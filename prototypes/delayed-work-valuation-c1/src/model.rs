//! Explicit C1 evidence envelope;no new actor state or valuation signal.
use crate::{delayed as n, encounters as e, preparation as m};
use serde::{Deserialize, Serialize};
include!(concat!(env!("OUT_DIR"), "/c1_stamp.rs"));

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Extension {
    pub boundary: usize,
    pub response: usize,
    pub owner: u32,
    pub tick: u64,
    pub outgoing: e::Intent,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trial {
    pub schema: String,
    pub source_freeze_sha256: String,
    pub correction: String,
    pub trial: n::Trial,
    pub response_extensions: Vec<Extension>,
}
pub fn trial(config: n::Config) -> Trial {
    let trial = n::trial(config);
    let mut response_extensions = vec![];
    for (boundary, b) in trial.body.history.boundaries.iter().enumerate() {
        for (response, r) in b.responses.iter().enumerate() {
            let outgoing = &r.input.outgoing;
            if outgoing.selected != e::Intent::Wait
                && !outgoing
                    .candidates
                    .iter()
                    .any(|c| c.intent == outgoing.selected)
            {
                response_extensions.push(Extension {
                    boundary,
                    response,
                    owner: r.input.own.own.id,
                    tick: r.input.own.now,
                    outgoing: outgoing.selected,
                });
            }
        }
    }
    Trial {
        schema: "experiment-026-c1-trial/v1".into(),
        source_freeze_sha256: SOURCE_FREEZE_SHA256.into(),
        correction: "ContinueZeroBeforeLookup".into(),
        trial,
        response_extensions,
    }
}
pub fn baseline_projection(trial: &Trial) -> m::Trial {
    assert!(trial.response_extensions.is_empty());
    n::baseline_projection(&trial.trial)
}
pub fn defined_response_equal(response: &e::Response) -> bool {
    let old = crate::contact_learning::respond(&response.input);
    let new = crate::response_continuation::respond(&response.input);
    old == new && new == *response
}
