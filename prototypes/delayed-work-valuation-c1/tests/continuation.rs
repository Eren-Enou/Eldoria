use world_of_individuals::{
    continuation_c1 as c, encounters as e, response_continuation as response,
};
fn fixture() -> e::Response {
    serde_json::from_str(include_str!("response-fixture.json")).unwrap()
}
#[test]
fn captured_response_is_exact_and_input_stays_owned() {
    let recorded = fixture();
    let before = recorded.input.clone();
    assert!(c::defined_response_equal(&recorded));
    let actual = response::respond(&recorded.input);
    assert_eq!(actual, recorded);
    assert_eq!(recorded.input, before);
}
#[test]
fn missing_inquiry_intent_uses_existing_zero_rule_without_archive_recovery() {
    let recorded = fixture();
    let mut input = recorded.input.clone();
    input.outgoing.selected = e::Intent::Invite {
        recipient: 1,
        kind: e::Kind::Inquiry,
    };
    let pending = input.outgoing.selected;
    input.outgoing.candidates.retain(|c| c.intent != pending);
    let actual = response::respond(&input);
    assert_eq!(actual.input, input);
    assert_eq!(actual.candidates, recorded.candidates);
    assert_eq!(actual.selected, recorded.selected);
    assert!(
        std::panic::catch_unwind(|| c::defined_response_equal(&actual)).is_err(),
        "original partial function must remain unchanged"
    );
}
#[test]
fn wait_and_first_incoming_ties_keep_original_order() {
    let mut input = fixture().input;
    let mut actual = response::respond(&input);
    let tied = actual
        .candidates
        .iter()
        .filter(|c| c.score == 58)
        .collect::<Vec<_>>();
    assert!(tied.len() >= 2);
    assert_eq!(actual.selected, tied[0].reply);
    input.incoming.clear();
    actual = response::respond(&input);
    assert_eq!(actual.selected, e::Reply::Wait);
    assert!(
        actual
            .candidates
            .iter()
            .any(|c| c.reply == e::Reply::Continue && c.score == 0)
    );
    input = fixture().input;
    input
        .own
        .people
        .iter_mut()
        .find(|p| p.id == input.own.own.id)
        .unwrap()
        .available = false;
    let busy = response::respond(&input);
    assert_eq!(busy.selected, e::Reply::Wait);
    assert!(
        busy.candidates
            .iter()
            .filter(|c| matches!(c.reply, e::Reply::Accept(_)))
            .all(|c| c.score == -1)
    );
}
