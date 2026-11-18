use crate::projection::{self, Frame, Index, Perspective};
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
        page: usize,
        selected: Option<u64>,
    },
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
        let host = Host::new(config)?;
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
                            let reset_deadline = !matches!(request.command, Command::View { .. });
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
    page: usize,
    selected: Option<u64>,
    pub running: bool,
    pub dirty: bool,
    rate: u32,
    last_save: Option<String>,
}
impl Host {
    pub fn new(config: Config) -> Result<Self, String> {
        Ok(Self {
            world: Settlement::new(config)?,
            index: Index::default(),
            perspective: Perspective::Observer,
            page: 0,
            selected: None,
            running: false,
            dirty: true,
            rate: 8,
            last_save: None,
        })
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
        projection::frame(
            &self.world,
            &self.index,
            projection::Selection {
                perspective: self.perspective,
                page: self.page,
                selected: self.selected,
            },
            self.running,
            self.dirty,
            self.last_save.clone(),
        )
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
                page,
                selected,
            } => {
                if let Perspective::Character(id) = perspective {
                    self.world.owned_view(id)?;
                }
                self.perspective = perspective;
                self.page = page;
                self.selected = selected;
                Ok("View updated".into())
            }
            Command::Shutdown => {
                self.running = false;
                Ok("Stopped".into())
            }
        }
    }
    fn replace(&mut self, world: Settlement, last_save: Option<String>, dirty: bool) {
        self.world = world;
        self.index = Index::default();
        self.index.update(&self.world);
        self.running = false;
        self.dirty = dirty;
        self.last_save = last_save;
        self.perspective = Perspective::Observer;
        self.page = 0;
        self.selected = None;
    }
}
