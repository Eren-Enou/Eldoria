use woi_workbench::{config::Configuration, execution, storage::Store};
fn main() {
    if let Err(e) = start() {
        eprintln!("Workbench: {e}");
        std::process::exit(1);
    }
}
fn start() -> woi_workbench::Result<()> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|s| s == "identity") && args.len() == 1 {
        println!(
            "{}",
            serde_json::to_string_pretty(&woi_workbench::portable::build_identity()).unwrap()
        );
        return Ok(());
    }
    let workspace = if args.first().is_some_and(|s| s == "--workspace") {
        if args.len() < 2 {
            return Err("Supply an absolute user-data workspace".into());
        }
        let value = std::path::PathBuf::from(args.remove(1));
        args.remove(0);
        Some(value)
    } else {
        None
    };
    let readonly = args.first().is_some_and(|s| {
        ["world-check", "world-storage", "world-verify", "world-list"].contains(&s.as_str())
    });
    let store = if readonly {
        Store::detect_readonly(workspace.as_deref())?
    } else {
        Store::detect(workspace.as_deref())?
    };
    #[cfg(feature = "world-sessions")]
    if args.first().is_some_and(|s| s.starts_with("world-")) {
        return world_command(&store, &args);
    }
    match args.first().map(String::as_str) {
        Some("help" | "--help" | "-h") => println!("World of Individuals Rust Workbench V2\nDesktop: launch without arguments (build with --features desktop)\nCLI: run [configuration.json] | list | inspect RUN-ID | reproduce RUN-ID | viewer RUN-ID | compare LEFT-ID RIGHT-ID | observe RUN-ID observation.json\nWorld sessions (feature world-sessions): world-create CONFIG | world-advance WORLD STEPS [--codec zlib] | world-inspect WORLD | world-check WORLD | world-storage WORLD [--budget-bytes N --max-generations N --project-saves N] | world-verify WORLD --generation N | world-recover WORLD NEW-ABSOLUTE-BRANCH/WORLD-ID --generation N | world-finalize WORLD | world-list\nGuide: docs/development/HUMAN_WORKBENCH_QUICKSTART.md; docs/development/PERSISTENT_WORLD_QUICKSTART.md"),
        Some("run") => {
            let mut c = Configuration::default();
            if let Some(path) = args.get(1) { c = woi_workbench::storage::read(std::path::Path::new(path))?; }
            let m = execution::run(&store, c, None)?;
            println!("{}", serde_json::to_string_pretty(&m).unwrap());
            if m.status != "Completed" { return Err("Execution incomplete; diagnostics saved".into()); }
        }
        Some("list") => for id in store.list()? { match execution::metadata(&store, &id) { Ok(m) => println!("{} | {} | {} | seed{}", id, m.status, m.configuration.scenario, m.configuration.seed), Err(e) => println!("{id} | Missing/corrupt metadata: {e}") } },
        Some("inspect") => println!("{}", serde_json::to_string_pretty(&execution::inspect(&store, args.get(1).ok_or("Supply a run ID")?)?).unwrap()),
        Some("notes") => println!("{}",serde_json::to_string_pretty(&woi_workbench::observations::load(&store,args.get(1).ok_or("Supply a run ID")?)?).unwrap()),
        Some("compare") => println!("{}",serde_json::to_string_pretty(&woi_workbench::comparison::compare(&store,args.get(1).ok_or("Supply left run ID")?,args.get(2).ok_or("Supply right run ID")?)?).unwrap()),
        Some("reproduce") => {
            let m=execution::reproduce(&store, args.get(1).ok_or("Supply a run ID")?)?;
            println!("{}",serde_json::to_string_pretty(&m).unwrap());
            if m.status!="Completed" { return Err("Reproduction incomplete; diagnostics saved".into()); }
        }
        Some("viewer") => println!("{}",woi_workbench::viewer::command_text(&store,args.get(1).ok_or("Supply a run ID")?,8766)?),
        Some("observe") => {
            let note: woi_workbench::observations::Observation=woi_workbench::storage::read(std::path::Path::new(args.get(2).ok_or("Supply observation JSON path")?))?;
            println!("{}",woi_workbench::observations::save(&store,args.get(1).ok_or("Supply a run ID")?,&note)?.display());
        }
        None => {
            #[cfg(feature = "desktop")]
            return desktop(store);
            #[cfg(not(feature = "desktop"))]
            println!("Run native desktop: cargo run --release --locked --manifest-path tools/workbench/Cargo.toml --features desktop\nCLI: run [configuration.json] | list | inspect RUN-ID | reproduce RUN-ID");
        }
        _ => return Err("Unknown command. Use run [configuration.json], list, inspect RUN-ID or reproduce RUN-ID".into()),
    }
    Ok(())
}
#[cfg(feature = "world-sessions")]
fn world_command(store: &Store, args: &[String]) -> woi_workbench::Result<()> {
    use woi_workbench::world_sessions::{Budget, Codec, Sessions, status};
    let sessions = if args.first().is_some_and(|s| s == "world-create") {
        Sessions::new(store)?
    } else {
        Sessions::existing(store)?
    };
    let selector = || {
        args.get(1)
            .map(String::as_str)
            .ok_or("supply world ID or absolute branch directory")
    };
    if args
        .first()
        .is_some_and(|s| s == "world-storage" || s == "world-check")
    {
        let mut budget = Budget::default();
        let options = &args[2.min(args.len())..];
        if !options.len().is_multiple_of(2) {
            return Err("storage warning flags need NUMBER values".into());
        }
        let mut seen = std::collections::BTreeSet::new();
        for pair in options.as_chunks::<2>().0 {
            if !seen.insert(pair[0].as_str()) {
                return Err(format!("duplicate storage flag {}", pair[0]));
            }
            match pair[0].as_str() {
                "--budget-bytes" => {
                    budget.bytes = Some(pair[1].parse().map_err(|_| "invalid byte budget")?)
                }
                "--max-generations" => {
                    budget.generations =
                        Some(pair[1].parse().map_err(|_| "invalid generation warning")?)
                }
                "--project-saves" => {
                    budget.projected_saves =
                        pair[1].parse().map_err(|_| "invalid projected saves")?
                }
                _ => return Err("unknown storage flag".into()),
            }
        }
        let report = sessions.inspect(selector()?, &budget)?;
        let valid = report.stable_snapshot
            && report.latest_committed.is_some()
            && report.generations.iter().all(|g| g.valid);
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        if args[0] == "world-check" && !valid {
            return Err(
                "world verification incomplete/invalid; inspect reported errors, no fallback"
                    .into(),
            );
        }
        return Ok(());
    }
    if args
        .first()
        .is_some_and(|s| s == "world-recover" || s == "world-verify")
    {
        let (generation_index, expected) = if args[0] == "world-recover" {
            (3, 5)
        } else {
            (2, 4)
        };
        if args.len() != expected || args[generation_index] != "--generation" {
            return Err("explicit --generation N required; recovery also needs a fresh absolute destination".into());
        }
        let generation = args[generation_index + 1]
            .parse()
            .map_err(|_| "invalid generation")?;
        if args[0] == "world-recover" {
            println!(
                "{}",
                serde_json::to_string_pretty(&sessions.recover(
                    selector()?,
                    std::path::Path::new(&args[2]),
                    generation
                )?)
                .unwrap()
            );
        } else {
            println!(
                "{}",
                serde_json::to_string_pretty(&sessions.verify(selector()?, generation)?).unwrap()
            );
        }
        return Ok(());
    }
    let result=match args.first().map(String::as_str) {
        Some("world-list")=> {println!("{}",serde_json::to_string_pretty(&sessions.list()?).unwrap());return Ok(());}
        Some("world-create")=> {
            let config=if let Some(path)=args.get(1) {woi_workbench::storage::read(std::path::Path::new(path))?}else{Configuration::default()};
            sessions.create(&config)?
        }
        Some("world-inspect"|"world-status")=>sessions.open_selected(selector()?)?,
        Some("world-advance")=> {
            let mut session=sessions.open_selected(selector()?)?;
            let steps=args.get(2).ok_or("supply bounded step count")?.parse().map_err(|_|"invalid step count")?;
            let codec=match &args[3.min(args.len())..]{[]=>Codec::Raw,[flag,value] if flag=="--codec" && value=="zlib"=>Codec::Zlib,_=>return Err("use --codec zlib or omit codec for raw".into())};
            session.advance(steps)?;session.save_with_codec(codec)?;session
        }
        Some("world-interrupt")=>{if args.len()!=3{return Err("world-interrupt WORLD STEPS".into());}let mut session=sessions.open_selected(selector()?)?;session.advance(args[2].parse().map_err(|_|"invalid step count")?)?;session}
        Some("world-finalize")=> {let mut session=sessions.open_selected(selector()?)?;session.finalize()?;session}
        _=>return Err("world commands: world-list | world-create [configuration.json] | world-inspect ID | world-advance ID STEPS | world-finalize ID".into()),
    };
    let mut response = status(&result);
    if args[0] == "world-interrupt" {
        response["checkpoint_saved"] = false.into();
        response["warning"] =
            "unsaved executed history preserved; explicit branch recovery required".into();
    }
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}
#[cfg(feature = "desktop")]
fn desktop(_store: Store) -> woi_workbench::Result<()> {
    woi_workbench::desktop::launch(_store)
}
