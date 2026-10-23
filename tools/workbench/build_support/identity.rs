//! Developer-only source verification. No Git or source files are needed by a
//! packaged ordinary run; the generated identity travels with the binaries.
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, env, fs, path::Path, process::Command};

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn git(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("developer build needs Git");
    assert!(out.status.success(), "Git identity command failed");
    String::from_utf8(out.stdout).unwrap().trim().into()
}
pub fn generate(root: &Path) {
    let mut locks = BTreeMap::new();
    let mut inputs = BTreeMap::new();
    for name in [
        "experiments/023/inherited-source-bindings.json",
        "experiments/023/source-freeze.json",
        "experiments/024/source-freeze.json",
    ] {
        let path = root.join(name);
        println!("cargo:rerun-if-changed={}", path.display());
        let bytes = fs::read(&path).unwrap();
        locks.insert(name, digest(&bytes));
        let expected: BTreeMap<String, String> = serde_json::from_slice(&bytes).unwrap();
        for (name, hash) in expected {
            let path = root.join(&name);
            println!("cargo:rerun-if-changed={}", path.display());
            let normalized = String::from_utf8(fs::read(&path).unwrap())
                .unwrap()
                .replace("\r\n", "\n");
            let actual = digest(normalized.as_bytes());
            assert_eq!(actual, hash, "frozen engine input {name}");
            inputs.insert(name, actual);
        }
    }
    let mut application = BTreeMap::new();
    for name in git(
        root,
        &[
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "--",
            "tools/workbench",
            "tools/portable-runner",
        ],
    )
    .lines()
    {
        if Path::new(name)
            .components()
            .any(|c| c.as_os_str() == "target")
        {
            continue;
        }
        let path = root.join(name);
        if !path.is_file() {
            continue;
        }
        println!("cargo:rerun-if-changed={}", path.display());
        application.insert(
            name.to_owned(),
            digest(&fs::read(path).unwrap().replace_crlf()),
        );
    }
    for path in [".git/HEAD", ".git/index", ".git/refs/heads/main"] {
        println!("cargo:rerun-if-changed={}", root.join(path).display());
    }
    let compiler = Command::new(env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
        .arg("--version")
        .output()
        .expect("developer compiler identity");
    assert!(compiler.status.success());
    let identity = serde_json::json!({
        "schema":"woi-build-identity/v1","revision":git(root,&["rev-parse","HEAD"]),
        "dirty":!git(root,&["status","--porcelain"]).is_empty(),
        "source_locks":locks,"engine_fingerprint":digest(&serde_json::to_vec(&inputs).unwrap()),
        "application_fingerprint":digest(&serde_json::to_vec(&application).unwrap()),
        "target":env::var("TARGET").unwrap(),"profile":env::var("PROFILE").unwrap(),
        "target_features":env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default(),
        "package_version":env::var("CARGO_PKG_VERSION").unwrap(),"compiler":String::from_utf8(compiler.stdout).unwrap().trim()
    });
    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("build-identity.json"),
        serde_json::to_vec_pretty(&identity).unwrap(),
    )
    .unwrap();
}
trait Normalize {
    fn replace_crlf(&self) -> Vec<u8>;
}
impl Normalize for Vec<u8> {
    fn replace_crlf(&self) -> Vec<u8> {
        self.iter()
            .enumerate()
            .filter_map(|(i, &b)| {
                if b == b'\r' && self.get(i + 1) == Some(&b'\n') {
                    None
                } else {
                    Some(b)
                }
            })
            .collect()
    }
}
