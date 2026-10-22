pub mod config;
#[cfg(feature = "desktop")]
pub mod desktop;
pub mod execution;
pub mod history;
pub mod observations;
pub mod storage;
pub mod viewer;
pub type Result<T> = std::result::Result<T, String>;

pub fn repository() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
}
