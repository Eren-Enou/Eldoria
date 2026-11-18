use eldoria_observer::ui::ObserverApp;
use eldoria_settlement::Config;
use std::path::PathBuf;

fn main() -> eframe::Result {
    let mut config = Config {
        seed: 42,
        population: 8,
        horizon: 512,
    };
    let mut open = None;
    let mut workspace = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("Eldoria")
        .join("settlement-observer");
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help") {
        println!(
            "eldoria-observer [--open CHECKPOINT] [--workspace DIR] [--seed N] [--population N] [--horizon N]"
        );
        return Ok(());
    }
    let (pairs, remainder) = args.as_chunks::<2>();
    if !remainder.is_empty() {
        eprintln!("Every option requires a value");
        std::process::exit(1);
    }
    for pair in pairs {
        let result = match pair[0].as_str() {
            "--open" => {
                open = Some(PathBuf::from(&pair[1]));
                Ok(())
            }
            "--workspace" => {
                workspace = PathBuf::from(&pair[1]);
                Ok(())
            }
            "--seed" => pair[1].parse().map(|n| config.seed = n),
            "--population" => pair[1].parse().map(|n| config.population = n),
            "--horizon" => pair[1].parse().map(|n| config.horizon = n),
            _ => {
                eprintln!("Unknown option: {}", pair[0]);
                std::process::exit(1);
            }
        };
        if result.is_err() {
            eprintln!("Invalid number for {}", pair[0]);
            std::process::exit(1);
        }
    }
    if let Err(error) = config.validate() {
        eprintln!("{error}");
        std::process::exit(1);
    }
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1380.0, 900.0])
            .with_min_inner_size([1100.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Eldoria · Settlement Observer",
        options,
        Box::new(move |cc| {
            Ok(Box::new(
                ObserverApp::new(cc, config, workspace, open).map_err(std::io::Error::other)?,
            ))
        }),
    )
}
