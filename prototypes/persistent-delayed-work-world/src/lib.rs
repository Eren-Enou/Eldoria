include!(concat!(env!("OUT_DIR"), "/lib_host.rs"));
pub mod decision_audit;
#[path = "../../persistent-material-world/src/material_audit.rs"]
pub mod material_audit;
#[path = "../../persistent-world/src/storage_codec.rs"]
pub mod storage_codec;
#[path = "verification.rs"]
pub mod verification;
pub fn validate_registration(config: &delayed::Config) -> Result<(), String> {
    if !delayed::matrix().contains(config) {
        return Err(
            "explicit legal registered026 Baseline/ProjectedNeed configuration required".into(),
        );
    }
    config.world.validate()
}
pub fn policy_identity(mode: delayed::Mode) -> String {
    format!("{}:{mode:?}", continuation_c1::SOURCE_FREEZE_SHA256)
}

#[cfg(test)]
mod tests;
