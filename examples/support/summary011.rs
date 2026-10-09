//! Shared immutable observer summary; no output or policy input channel.
use world_of_individuals::{experiment011 as e, inquiry_value as v, model::Observation};
pub fn summary(t: &e::Trial, held: bool) -> serde_json::Value {
    let view = Observation {
        partner: t.resource_partner,
        last_signal: None,
        amount: t.config.amount,
        turns_left: 6,
    };
    let probe = |p: &e::Point| world_of_individuals::behavior::scarcity_policy(&p.agents[0], &view);
    let queries = e::queries(t);
    let steps: Vec<_>=queries.iter().zip(&t.opportunities).map(|(r,o)| {
        serde_json::json!({"inquiry":r.id,"selected":r.decision.selected,"concerns_before":o.before.concerns,
            "basis_before":o.before.basis,"public_sources":r.decision.input.sources,"cells_before":r.decision.input.cells,
            "candidates":r.decision.candidates,"status_components":r.decision.candidates.iter().map(|c|v::components(c,&o.before.basis)).collect::<Vec<_>>(),
            "response":r.response,"realized_value":r.realized_value,"novelty":r.novelty,
            "cell_after":r.cell_after,"time":r.time_spent,"concerns_after":o.after.concerns,"basis_after":o.after.basis,
            "assessment_before":o.before.assessment_end,"assessment_after":o.after.assessment_end,"remaining":o.remaining,
            "readiness_before":probe(&o.before),"readiness_after":probe(&o.after)})
    }).collect();
    let compact = &t.final_state.base.base.base;
    let start = t.opportunities.last().unwrap().after.event_end;
    let events = &compact.base.base.events[start..];
    serde_json::json!({"seed":t.seed,"held":held,"case":t.case,"config":t.config,"mode":t.mode,
        "targets":t.targets,"resource_partner":t.resource_partner,"steps":steps,
        "actions":events.iter().map(|e|e.decision.selected).collect::<Vec<_>>(),
        "event_ids":events.iter().map(|e|e.id).collect::<Vec<_>>(),
        "transferred":events.iter().map(|e|e.transferred).sum::<u32>(),
        "pre_readiness":probe(&t.opportunities[0].before),"post_readiness":probe(&t.opportunities.last().unwrap().after),
        "food_after_action":t.after_action.agents.iter().map(|a|a.food).collect::<Vec<_>>(),
        "persistent_food":t.persistent.agents.iter().map(|a|a.food).collect::<Vec<_>>(),"consumption":t.consumption,
        "unresolved_before_resource":t.opportunities.last().unwrap().after.concerns.iter().filter(|c|c.status.active()).count(),
        "unresolved_after_resource":t.persistent.concerns.iter().filter(|c|c.status.active()).count(),
        "trial_bytes":serde_json::to_vec(t).unwrap().len(),"audit_bytes":serde_json::to_vec(&t.final_state.value).unwrap().len(),
        "audit_records":t.final_state.value.records.len()})
}
