//! Read-only local budget counterfactual; never executes a world or writes archives.
use std::io::{self, Read};
use world_of_individuals::{acquisition as a, environmental_knowledge as k};
fn main() {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text).unwrap();
    let inputs: Vec<k::Input> = serde_json::from_str(&text).unwrap();
    let rows = inputs.into_iter().map(|input| {
        let before = k::policy(&input, a::Policy::Need);
        let mut projected = input.clone();
        projected.legacy.end = projected.legacy.now + 33;
        let after = k::policy(&projected, a::Policy::Need);
        serde_json::json!({"owner":input.legacy.own.id,"tick":input.legacy.now,"original_end":input.legacy.end,"before":before.selected,"after":after.selected,"before_candidates":before.candidates,"after_candidates":after.candidates,"contacts":after.base.contact.candidates,"retained":input.retained,"public_people":input.legacy.people,"food":input.legacy.own.food,"hunger":input.legacy.own.hunger})
    }).collect::<Vec<_>>();
    println!("{}", serde_json::to_string(&rows).unwrap());
}
