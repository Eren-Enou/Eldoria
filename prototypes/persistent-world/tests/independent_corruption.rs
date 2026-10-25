use ciborium::value::Value;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use woi_persistent_world::{continuous as c, persistent::session::Session, verification};
#[derive(Serialize, Deserialize)]
struct Commit {
    binding_sha256: String,
    schema: String,
    world: String,
    runtime: String,
    generation: u64,
    tick: u64,
    journal_bytes: u64,
    journal_sha256: String,
    state_sha256: String,
    #[serde(with = "serde_bytes")]
    state: Vec<u8>,
}
#[derive(Serialize, Deserialize)]
struct Envelope {
    schema: String,
    runtime: String,
    checksum: String,
    #[serde(with = "serde_bytes")]
    payload: Vec<u8>,
}
fn encode<T: Serialize>(v: &T) -> Vec<u8> {
    let mut b = vec![];
    ciborium::into_writer(v, &mut b).unwrap();
    b
}
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn field<'a>(v: &'a Value, key: &str) -> &'a Value {
    if let Value::Map(m) = v {
        &m.iter()
            .find(|(k, _)| matches!(k,Value::Text(s) if s==key))
            .unwrap()
            .1
    } else {
        panic!("expected map")
    }
}
fn field_mut<'a>(v: &'a mut Value, key: &str) -> &'a mut Value {
    if let Value::Map(m) = v {
        &mut m
            .iter_mut()
            .find(|(k, _)| matches!(k,Value::Text(s) if s==key))
            .unwrap()
            .1
    } else {
        panic!("expected map")
    }
}
fn mutate(raw: &[u8], edit: impl FnOnce(&mut Value)) -> Vec<u8> {
    let mut c: Commit = ciborium::from_reader(raw).unwrap();
    let mut e: Envelope = ciborium::from_reader(c.state.as_slice()).unwrap();
    let mut payload: Value = ciborium::from_reader(e.payload.as_slice()).unwrap();
    edit(&mut payload);
    e.payload = encode(&payload);
    e.checksum = digest(&e.payload);
    c.state = encode(&e);
    c.state_sha256 = digest(&c.state);
    c.binding_sha256 = digest(
        &serde_json::to_vec(&(
            &c.schema,
            &c.world,
            &c.runtime,
            c.generation,
            c.tick,
            c.journal_bytes,
            &c.journal_sha256,
            &c.state_sha256,
        ))
        .unwrap(),
    );
    encode(&c)
}
fn actual_checkpoint() -> (Vec<u8>, Vec<u8>) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/recovery-validation")
        .join(format!(
            "raw-corrupt-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let path = path
        .parent()
        .unwrap()
        .canonicalize()
        .unwrap()
        .join(path.file_name().unwrap());
    let mut config = c::matrix()
        .into_iter()
        .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
        .unwrap();
    config.horizon = 32;
    config.seed = 43;
    let mut s = Session::create(&path, config).unwrap();
    s.advance(3).unwrap();
    s.save().unwrap();
    drop(s);
    (
        fs::read(path.join("checkpoint-00000002.cbor")).unwrap(),
        fs::read(path.join("journal.jsonl")).unwrap(),
    )
}
#[test]
fn independent_reader_rejects_rehashed_primitive_corruption() {
    let (raw, prefix) = actual_checkpoint();
    verification::verify(&raw, &prefix).unwrap();
    let bad_food = mutate(&raw, |payload| {
        if let Value::Array(a) = field_mut(payload, "agents") {
            *field_mut(&mut a[0], "food") = Value::Integer(99u64.into());
        }
    });
    assert!(
        verification::verify(&bad_food, &prefix)
            .unwrap_err()
            .contains("conservation")
    );
    let bad_config = mutate(&raw, |payload| {
        *field_mut(field_mut(payload, "config"), "seed") = Value::Integer(1u64.into());
    });
    assert!(
        verification::verify(&bad_config, &prefix)
            .unwrap_err()
            .contains("configuration")
    );
    let duplicate = mutate(&raw, |payload| {
        if let Value::Array(transitions) = field_mut(field_mut(payload, "state"), "transitions") {
            let index = transitions
                .iter()
                .position(|t| matches!(field(t, "completed"), Value::Bool(true)))
                .expect("genuine completed send/reply");
            transitions.insert(index + 1, transitions[index].clone());
        }
    });
    assert!(
        verification::verify(&duplicate, &prefix)
            .unwrap_err()
            .contains("duplicate")
    );
    let mut damaged = prefix.clone();
    damaged[3] ^= 1;
    assert!(verification::verify(&raw, &damaged).is_err());
    assert!(verification::verify(&raw, &prefix[..prefix.len() - 1]).is_err());
    let mut damaged = raw.clone();
    damaged[3] ^= 1;
    assert!(verification::verify(&damaged, &prefix).is_err());
    // All mutations were disposable byte copies. The original still verifies.
    verification::verify(&raw, &prefix).unwrap();
}
