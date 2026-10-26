include!(concat!(env!("OUT_DIR"), "/lib_host.rs"));
pub mod material_audit;
#[path = "../../persistent-world/src/storage_codec.rs"]
pub mod storage_codec;
#[cfg(test)]
mod tests;
pub mod verification;
