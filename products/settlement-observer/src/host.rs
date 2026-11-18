use crate::notebook::{self, Anchor, Availability, Bookmark, NotebookView, Store, Summary};
use crate::projection::{self, Direction, EventFilter, Frame, Index, Perspective};
use eldoria_settlement::{Config, Settlement, storage};
use std::{
    path::PathBuf,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, SyncSender, TryRecvError},
    },
    thread,
    time::{Duration, Instant},
};

pub enum Command {
    New(Config),
    Open(PathBuf),
    Save(PathBuf),
    Step,
    Running(bool),
    Rate(u32),
    View {
        perspective: Perspective,
        filter: EventFilter,
        page: usize,
        selected: Option<u64>,
    },
    Navigate(Direction),
    SaveBookmark {
        anchor: Anchor,
        expected_revision: u32,
        title: String,
        body: String,
    },
    JumpBookmark(String),
    ReloadNotebook,
    Shutdown,
}
pub struct Reply {
    pub id: u64,
    pub result: Result<String, String>,
}
pub struct Request {
    pub id: u64,
    pub command: Command,
}
pub struct Client {
    sender: SyncSender<Request>,
    replies: Receiver<Reply>,
    latest: Arc<Mutex<Option<Frame>>>,
    next: u64,
}
impl Client {
    pub fn spawn(config: Config) -> Result<Self, String> {
        Self::spawn_host(Host::new(config)?)
    }
    pub fn spawn_with_notebook(config: Config, root: PathBuf) -> Result<Self, String> {
        Self::spawn_host(Host::with_notebook(config, root)?)
    }
    fn spawn_host(host: Host) -> Result<Self, String> {
        let (sender, requests) = mpsc::sync_channel::<Request>(16);
        let (reply_sender, replies) = mpsc::channel();
        let latest = Arc::new(Mutex::new(None));
        let sink = latest.clone();
        thread::Builder::new()
            .name("eldoria-settlement".into())
            .spawn(move || {
                let mut host = host;
                let mut deadline = Instant::now();
                let mut last_frame = Instant::now() - Duration::from_secs(1);
                publish(&host, &sink);
                loop {
                    let timeout = if host.running {
                        deadline
                            .saturating_duration_since(Instant::now())
                            .min(Duration::from_millis(100))
                    } else {
                        Duration::from_millis(100)
                    };
                    match requests.recv_timeout(timeout) {
                        Ok(request) => {
                            if matches!(request.command, Command::Shutdown) {
                                break;
                            }
                            let reset_deadline = !matches!(
                                request.command,
                                Command::View { .. }
                                    | Command::Navigate(_)
                                    | Command::SaveBookmark { .. }
                                    | Command::JumpBookmark(_)
                                    | Command::ReloadNotebook
                            );
                            let result = host.apply(request.command);
                            publish(&host, &sink);
                            last_frame = Instant::now();
                            if reply_sender
                                .send(Reply {
                                    id: request.id,
                                    result,
                                })
                                .is_err()
                            {
                                break;
                            }
                            if reset_deadline {
                                deadline = Instant::now() + host.interval();
                            }
                        }
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                        Err(mpsc::RecvTimeoutError::Timeout) => (),
                    }
                    if host.running && Instant::now() >= deadline {
                        if let Err(error) = host.tick() {
                            host.running = false;
                            let _ = reply_sender.send(Reply {
                                id: 0,
                                result: Err(error),
                            });
                        }
                        deadline = Instant::now() + host.interval();
                        if last_frame.elapsed() >= Duration::from_millis(100) || !host.running {
                            publish(&host, &sink);
                            last_frame = Instant::now();
                        }
                    }
                }
            })
            .map_err(|e| e.to_string())?;
        Ok(Self {
            sender,
            replies,
            latest,
            next: 1,
        })
    }
    pub fn send(&mut self, command: Command) -> Result<u64, String> {
        let id = self.next;
        self.sender
            .try_send(Request { id, command })
            .map_err(|e| format!("Simulation worker unavailable or busy: {e}"))?;
        self.next += 1;
        Ok(id)
    }
    pub fn poll(&self) -> Result<Option<Reply>, String> {
        match self.replies.try_recv() {
            Ok(reply) => Ok(Some(reply)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => {
                Err("Simulation worker stopped. Reopen the last committed checkpoint.".into())
            }
        }
    }
    pub fn take_frame(&self) -> Option<Frame> {
        self.latest.lock().ok()?.take()
    }
}
impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.sender.try_send(Request {
            id: 0,
            command: Command::Shutdown,
        });
    }
}
fn publish(host: &Host, sink: &Mutex<Option<Frame>>) {
    if let Ok(frame) = host.frame()
        && let Ok(mut slot) = sink.lock()
    {
        *slot = Some(frame);
    }
}

pub struct Host {
    pub world: Settlement,
    index: Index,
    perspective: Perspective,
    filter: EventFilter,
    page: usize,
    selected: Option<u64>,
    pub running: bool,
    pub dirty: bool,
    rate: u32,
    last_save: Option<String>,
    notebook: Option<Store>,
    bookmarks: Vec<Bookmark>,
    notebook_error: Option<String>,
    world_key: String,
}
impl Host {
    pub fn new(config: Config) -> Result<Self, String> {
        let world = Settlement::new(config)?;
        let world_key = notebook::world_key(&world);
        Ok(Self {
            world,
            index: Index::default(),
            perspective: Perspective::Observer,
            filter: EventFilter::All,
            page: 0,
            selected: None,
            running: false,
            dirty: true,
            rate: 8,
            last_save: None,
            notebook: None,
            bookmarks: vec![],
            notebook_error: None,
            world_key,
        })
    }
    pub fn with_notebook(config: Config, root: PathBuf) -> Result<Self, String> {
        let mut host = Self::new(config)?;
        host.notebook = Some(Store::new(root));
        let _ = host.reload_notebook();
        Ok(host)
    }
    fn reload_notebook(&mut self) -> Result<(), String> {
        let result = self
            .notebook
            .as_ref()
            .ok_or("Notebook workspace is not configured")?
            .load(&self.world_key);
        match result {
            Ok(entries) => {
                self.bookmarks = entries;
                self.notebook_error = None;
                Ok(())
            }
            Err(error) => {
                self.bookmarks.clear();
                self.notebook_error = Some(error.clone());
                Err(error)
            }
        }
    }
    fn availability(&self, anchor: &Anchor) -> Availability {
        if anchor.world != self.world_key {
            return Availability::DifferentHistory;
        }
        match self.index.prefix(anchor.record) {
            None => Availability::BeyondPrefix,
            Some(prefix)
                if prefix == anchor.prefix
                    && self.world.state().history[anchor.record as usize].tick == anchor.tick =>
            {
                Availability::Available
            }
            _ => Availability::DifferentHistory,
        }
    }
    pub fn interval(&self) -> Duration {
        Duration::from_secs_f64(1.0 / f64::from(self.rate))
    }
    pub fn tick(&mut self) -> Result<(), String> {
        if self.world.advance(1)? > 0 {
            self.dirty = true;
        }
        self.index.update(&self.world);
        if self.world.complete() {
            self.running = false;
        }
        Ok(())
    }
    pub fn frame(&self) -> Result<Frame, String> {
        let mut frame = projection::frame(
            &self.world,
            &self.index,
            projection::Selection {
                perspective: self.perspective,
                filter: self.filter,
                page: self.page,
                selected: self.selected,
            },
            self.running,
            self.dirty,
            self.last_save.clone(),
        )?;
        if self.perspective == Perspective::Observer && self.notebook.is_some() {
            let mut entries: Vec<_> = self
                .bookmarks
                .iter()
                .map(|note| Summary {
                    key: note.anchor.key(),
                    record: note.anchor.record,
                    tick: note.anchor.tick,
                    revision: note.revision,
                    title: note.title.clone(),
                    availability: self.availability(&note.anchor),
                })
                .collect();
            entries.sort_by_key(|b| b.record);
            let selected = self
                .bookmarks
                .iter()
                .find(|note| {
                    Some(note.anchor.record) == self.selected
                        && self.availability(&note.anchor) == Availability::Available
                })
                .cloned();
            frame.notebook = Some(NotebookView {
                world: self.world_key.clone(),
                entries,
                selected,
                error: self.notebook_error.clone(),
            });
        }
        Ok(frame)
    }
    pub fn apply(&mut self, command: Command) -> Result<String, String> {
        match command {
            Command::New(config) => {
                let world = Settlement::new(config)?;
                self.replace(world, None, true);
                Ok("New settlement is paused.".into())
            }
            Command::Open(path) => {
                self.running = false;
                let world = storage::load(&path)?;
                self.replace(world, Some(path.display().to_string()), false);
                Ok(format!("Opened {}. Time is paused.", path.display()))
            }
            Command::Save(path) => {
                self.running = false;
                storage::check_output(&path)?;
                if let Some(parent) = path.parent()
                    && !parent.as_os_str().is_empty()
                {
                    std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                storage::save(&self.world, &path)?;
                self.last_save = Some(path.display().to_string());
                self.dirty = false;
                Ok(format!(
                    "Saved tick {} to {}",
                    self.world.tick(),
                    path.display()
                ))
            }
            Command::Step => {
                self.running = false;
                self.tick()?;
                Ok(format!("Paused at tick {}.", self.world.tick()))
            }
            Command::Running(running) => {
                self.running = running && !self.world.complete();
                Ok(if self.running { "Running" } else { "Paused" }.into())
            }
            Command::Rate(rate) => {
                if !(1..=64).contains(&rate) {
                    return Err("Speed must be1–64 ticks per second".into());
                }
                self.rate = rate;
                Ok(format!("Speed: {rate} ticks/s"))
            }
            Command::View {
                perspective,
                filter,
                page,
                selected,
            } => {
                if let Perspective::Character(id) = perspective {
                    self.world.owned_view(id)?;
                }
                self.perspective = perspective;
                self.filter = filter;
                self.page = page;
                self.selected = selected;
                Ok("View updated".into())
            }
            Command::Navigate(direction) => {
                if let Some(id) = self.index.navigate(
                    &self.world,
                    self.perspective,
                    self.filter,
                    self.selected,
                    direction,
                )? {
                    self.selected = Some(id);
                    self.page =
                        self.index
                            .page_for(&self.world, self.perspective, self.filter, id)?;
                }
                Ok("View updated".into())
            }
            Command::SaveBookmark {
                anchor,
                expected_revision,
                title,
                body,
            } => {
                if self.perspective != Perspective::Observer {
                    return Err("Notebook editing is an external Observer operation".into());
                }
                if self.availability(&anchor) != Availability::Available {
                    return Err(
                        "Bookmark anchor does not match this executed history prefix".into(),
                    );
                }
                let store = self
                    .notebook
                    .as_ref()
                    .ok_or("Notebook workspace is not configured")?;
                let note = store.save(anchor, expected_revision, title, body)?;
                self.reload_notebook()?;
                Ok(format!(
                    "Observer bookmark saved · revision {}",
                    note.revision
                ))
            }
            Command::JumpBookmark(key) => {
                if self.perspective != Perspective::Observer {
                    return Err("Notebook navigation requires Observer perspective".into());
                }
                let note = self
                    .bookmarks
                    .iter()
                    .find(|b| b.anchor.key() == key)
                    .ok_or("Bookmark not found in this notebook")?;
                if self.availability(&note.anchor) != Availability::Available {
                    return Err(
                        "This bookmark is not in the loaded history prefix; no time was advanced"
                            .into(),
                    );
                }
                self.selected = Some(note.anchor.record);
                self.filter = EventFilter::All;
                self.page = self.index.page_for(
                    &self.world,
                    self.perspective,
                    self.filter,
                    note.anchor.record,
                )?;
                Ok("Bookmarked record selected; current simulation time is unchanged".into())
            }
            Command::ReloadNotebook => {
                if self.perspective != Perspective::Observer {
                    return Err("Notebook access requires Observer perspective".into());
                }
                self.reload_notebook()?;
                Ok("Observer notebook reloaded".into())
            }
            Command::Shutdown => {
                self.running = false;
                Ok("Stopped".into())
            }
        }
    }
    fn replace(&mut self, world: Settlement, last_save: Option<String>, dirty: bool) {
        self.world = world;
        self.world_key = notebook::world_key(&self.world);
        self.index = Index::default();
        self.index.update(&self.world);
        self.running = false;
        self.dirty = dirty;
        self.last_save = last_save;
        self.perspective = Perspective::Observer;
        self.page = 0;
        self.selected = None;
        self.bookmarks.clear();
        self.notebook_error = None;
        if self.notebook.is_some() {
            let _ = self.reload_notebook();
        }
    }
}
