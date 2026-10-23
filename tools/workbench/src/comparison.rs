//! Conservative observer comparisons; never a utility score or causal verdict.
use crate::{Result, execution, history::Summary, storage::Store};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Difference {
    pub field: String,
    pub left: Value,
    pub right: Value,
}
#[derive(Debug, Serialize)]
pub struct Comparison {
    pub left_id: String,
    pub right_id: String,
    pub left_configuration: crate::config::Configuration,
    pub right_configuration: crate::config::Configuration,
    pub classification: String,
    pub differences: Vec<Difference>,
    pub left: Summary,
    pub right: Summary,
}
fn differences(path: &str, left: &Value, right: &Value, out: &mut Vec<Difference>) {
    if left == right {
        return;
    }
    if let (Some(a), Some(b)) = (left.as_object(), right.as_object()) {
        let keys: std::collections::BTreeSet<_> = a.keys().chain(b.keys()).collect();
        for key in keys {
            differences(&format!("{path}{key}."), &left[key], &right[key], out);
        }
    } else {
        out.push(Difference {
            field: path.trim_end_matches('.').into(),
            left: left.clone(),
            right: right.clone(),
        });
    }
}
pub fn compare(store: &Store, left: &str, right: &str) -> Result<Comparison> {
    let a = execution::metadata(store, left)?;
    let b = execution::metadata(store, right)?;
    let mut av = serde_json::to_value(&a.configuration).map_err(|e| e.to_string())?;
    let mut bv = serde_json::to_value(&b.configuration).map_err(|e| e.to_string())?;
    let mut delta = Vec::new();
    differences("", &av, &bv, &mut delta);
    // Only the explicitly selectable valuation may vary in this conservative
    // matched classification. All material settings/world inputs must match.
    av.as_object_mut().unwrap().remove("valuation");
    bv.as_object_mut().unwrap().remove("valuation");
    let left_summary = execution::inspect(store, left)?;
    let right_summary = execution::inspect(store, right)?;
    let matched = av == bv
        && a.installed_release == b.installed_release
        && a.source.runner_fingerprint == b.source.runner_fingerprint
        && left_summary.initial_agents == right_summary.initial_agents
        && left_summary.initial_total == right_summary.initial_total
        && left_summary.start == right_summary.start;
    Ok(Comparison {
        left_id: left.into(),
        right_id: right.into(),
        left_configuration: a.configuration,
        right_configuration: b.configuration,
        classification: if matched {
            "Matched exploratory comparison"
        } else {
            "Recorded difference — conditions differ; not a matched comparison"
        }
        .into(),
        differences: delta,
        left: left_summary,
        right: right_summary,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn differences_include_missing_and_nested_fields() {
        let mut out = Vec::new();
        super::differences(
            "",
            &serde_json::json!({"preparation":{"raw_initial":6}}),
            &serde_json::json!({"preparation":{"raw_initial":0},"valuation":"Baseline"}),
            &mut out,
        );
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].field, "preparation.raw_initial");
        assert!(out[1].left.is_null());
    }
}
