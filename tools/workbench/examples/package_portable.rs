//! Developer-only packaging. No runtime dependency on Cargo, tar or source files.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use woi_workbench::{
    Result,
    config::Mode,
    execution,
    portable::{self, Binary, BuildIdentity, ReleaseManifest},
    storage,
};
fn identity(path: &Path) -> Result<BuildIdentity> {
    let out = Command::new(path)
        .arg("identity")
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err("Executable identity command failed".into());
    }
    serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())
}
fn notices(source: &Path, dest: &Path) -> Result<()> {
    for entry in fs::read_dir(source).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let ty = entry.file_type().map_err(|e| e.to_string())?;
        if ty.is_symlink() {
            continue;
        }
        let name = entry.file_name();
        let lower = name.to_string_lossy().to_lowercase();
        if ty.is_dir() {
            if ["target", ".git"].contains(&lower.as_str()) {
                continue;
            }
            notices(&entry.path(), &dest.join(&name))?;
        } else if lower.starts_with("license")
            || lower.starts_with("licence")
            || lower.starts_with("copying")
            || lower.starts_with("notice")
            || lower.contains("license")
            || lower.contains("licence")
            || (lower.ends_with(".txt") && source.components().any(|c| c.as_os_str() == "fonts"))
        {
            fs::create_dir_all(dest).map_err(|e| e.to_string())?;
            fs::copy(entry.path(), dest.join(name)).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
fn run() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 3 {
        return Err("package_portable APP.exe RUNNER.exe NEW_RELEASE_DIRECTORY".into());
    }
    let app = PathBuf::from(&args[0])
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let runner = PathBuf::from(&args[1])
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let output = PathBuf::from(&args[2]);
    let app_identity = identity(&app)?;
    let runner_identity = identity(&runner)?;
    if app_identity.package_version != env!("CARGO_PKG_VERSION")
        || runner_identity.package_version != env!("CARGO_PKG_VERSION")
        || app_identity.continuation_runtime.is_none()
    {
        return Err("this package requires matching0.3 application/runner versions and an embedded continuation runtime".into());
    }
    storage::no_links(&output)?;
    fs::create_dir(&output).map_err(|e| format!("Release directory must be NEW: {e}"))?;
    let manifest = ReleaseManifest {
        schema: portable::RELEASE_SCHEMA.into(),
        version: "0.3.0-rc.1".into(),
        platform: "windows-x86_64".into(),
        application: Binary {
            file: portable::APP_FILE.into(),
            sha256: execution::hash(&app)?,
            build: app_identity,
        },
        runner: Binary {
            file: portable::RUNNER_FILE.into(),
            sha256: execution::hash(&runner)?,
            build: runner_identity,
        },
        modes: vec![Mode::ContinuousWorld, Mode::ActivityValuation],
        history_schemas: vec![
            "experiment-023-trial/v1".into(),
            "experiment-024-trial/v1".into(),
        ],
        runtime_requirements: vec![
            "Windows x64; system graphics/OpenGL driver and operating-system DLLs".into(),
            "No developer tools, source checkout, Python or network for ordinary operation".into(),
        ],
    };
    manifest.validate()?;
    for binary in [&manifest.application, &manifest.runner] {
        if !binary
            .build
            .target_features
            .split(',')
            .any(|f| f == "crt-static")
        {
            return Err("Portable release requires a verified static-CRT build".into());
        }
    }
    fs::copy(app, output.join(portable::APP_FILE)).map_err(|e| e.to_string())?;
    fs::copy(runner, output.join(portable::RUNNER_FILE)).map_err(|e| e.to_string())?;
    storage::write_new(&output.join("release.json"), &manifest)?;
    let root = woi_workbench::repository();
    fs::copy(
        root.join("docs/development/PERSISTENT_WORLD_QUICKSTART.md"),
        output.join("QUICKSTART.md"),
    )
    .map_err(|e| e.to_string())?;
    let mut inventory = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for member in [
        "tools/workbench/Cargo.toml",
        "tools/portable-runner/Cargo.toml",
    ] {
        let mut command = Command::new("cargo");
        command.args([
            "metadata",
            "--locked",
            "--offline",
            "--format-version",
            "1",
            "--filter-platform",
            "x86_64-pc-windows-msvc",
            "--manifest-path",
            member,
        ]);
        if member == "tools/workbench/Cargo.toml" {
            command.args(["--features", "desktop,portable"]);
        }
        let out = command
            .current_dir(&root)
            .output()
            .map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).into());
        }
        let metadata: serde_json::Value =
            serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
        for p in metadata["packages"]
            .as_array()
            .ok_or("Missing package metadata")?
        {
            let id = p["id"].as_str().ok_or("Missing package id")?;
            if p["source"].is_null() || !seen.insert(id.to_owned()) {
                continue;
            }
            let name = format!(
                "{}-{}",
                p["name"].as_str().unwrap(),
                p["version"].as_str().unwrap()
            );
            let source = Path::new(p["manifest_path"].as_str().unwrap())
                .parent()
                .unwrap();
            notices(source, &output.join("licenses").join(name))?;
            inventory.push(serde_json::json!({"name":p["name"],"version":p["version"],"license":p["license"],"source":p["source"]}));
        }
    }
    for required in ["eframe", "egui", "epaint_default_fonts", "winit"] {
        if !inventory.iter().any(|p| p["name"] == required) {
            return Err(format!(
                "Missing desktop dependency notice inventory: {required}"
            ));
        }
    }
    let fonts = inventory
        .iter()
        .find(|p| p["name"] == "epaint_default_fonts")
        .unwrap();
    let directory = output
        .join("licenses")
        .join(format!(
            "epaint_default_fonts-{}",
            fonts["version"].as_str().unwrap()
        ))
        .join("fonts");
    for name in [
        "OFL.txt",
        "UFL.txt",
        "Hack-Regular.txt",
        "emoji-icon-font-mit-license.txt",
    ] {
        if !directory.join(name).is_file() {
            return Err(format!("Missing bundled font notice: {name}"));
        }
    }
    storage::write_new(&output.join("dependency-licenses.json"), &inventory)?;
    fs::write(output.join("NOTICES.txt"),"World of Individuals portable local evaluation build.\nNo first-party redistribution license is declared in this repository; this package does not invent one.\nThird-party license expressions and copied upstream notices are provided in dependency-licenses.json and licenses/.\nThe inventory is conservative and can include build/optional dependencies. Windows system components and graphics drivers are not bundled.\n").map_err(|e|e.to_string())?;
    portable::InstalledRelease::load(&output)?;
    let mut zip_name = output.as_os_str().to_os_string();
    zip_name.push(".zip");
    let zip = PathBuf::from(zip_name);
    if zip.exists() {
        return Err("ZIP already exists".into());
    }
    let zip = zip
        .parent()
        .unwrap()
        .canonicalize()
        .map_err(|e| e.to_string())?
        .join(zip.file_name().unwrap());
    let status = Command::new("tar.exe")
        .args(["-a", "-cf"])
        .arg(&zip)
        .arg("-C")
        .arg(&output)
        .arg(".")
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("Windows ZIP packaging failed".into());
    }
    println!("{}\nSHA256 {}", zip.display(), execution::hash(&zip)?);
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("Package: {e}");
        std::process::exit(1);
    }
}
