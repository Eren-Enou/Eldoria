use woi_workbench::{config::Configuration, execution, storage::Store};
fn main() {
    if let Err(e) = start() {
        eprintln!("Workbench: {e}");
        std::process::exit(1);
    }
}
fn start() -> woi_workbench::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let store = Store::open(&woi_workbench::repository())?;
    match args.first().map(String::as_str) {
        Some("help" | "--help" | "-h") => println!("World of Individuals Rust Workbench V1\nDesktop: launch without arguments (build with --features desktop)\nCLI: run [configuration.json] | list | inspect RUN-ID | reproduce RUN-ID | viewer RUN-ID | observe RUN-ID observation.json\nGuide: docs/development/HUMAN_WORKBENCH_QUICKSTART.md"),
        Some("run") => {
            let mut c = Configuration::default();
            if let Some(path) = args.get(1) { c = woi_workbench::storage::read(std::path::Path::new(path))?; }
            let m = execution::run(&store, c, None)?;
            println!("{}", serde_json::to_string_pretty(&m).unwrap());
            if m.status != "Completed" { return Err("Execution incomplete; diagnostics saved".into()); }
        }
        Some("list") => for id in store.list()? { match execution::metadata(&store, &id) { Ok(m) => println!("{} | {} | {} | seed{}", id, m.status, m.configuration.scenario, m.configuration.seed), Err(e) => println!("{id} | Missing/corrupt metadata: {e}") } },
        Some("inspect") => println!("{}", serde_json::to_string_pretty(&execution::inspect(&store, args.get(1).ok_or("Supply a run ID")?)?).unwrap()),
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
#[cfg(feature = "desktop")]
fn desktop(_store: Store) -> woi_workbench::Result<()> {
    woi_workbench::desktop::launch(_store)
}
