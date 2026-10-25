use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use woi_persistent_world::{
    continuous as c,
    persistent::session::{Session, reliability},
    storage_codec::{self, Codec},
};
#[derive(Deserialize, Serialize)]
struct Packed {
    schema: String,
    codec: String,
    uncompressed_bytes: u64,
    uncompressed_sha256: String,
    #[serde(with = "serde_bytes")]
    payload: Vec<u8>,
}
fn encode(p: &Packed) -> Vec<u8> {
    let mut bytes = vec![];
    ciborium::into_writer(p, &mut bytes).unwrap();
    bytes
}
fn actual() -> (PathBuf, Vec<u8>, Vec<u8>) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/recovery-validation")
        .join(format!(
            "codec-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&root).unwrap();
    let path = root.canonicalize().unwrap().join("world");
    let mut config = c::matrix()
        .into_iter()
        .find(|c| c.case == "paired" && c.mode == c::Mode::Continuous)
        .unwrap();
    config.horizon = 64;
    let mut s = Session::create(&path, config).unwrap();
    s.advance(3).unwrap();
    let expected = s.world().checkpoint().unwrap();
    s.save_with_codec(Codec::Zlib).unwrap();
    drop(s);
    let packed = fs::read(path.join("checkpoint-00000002.cbor")).unwrap();
    (path, packed, expected)
}
#[test]
fn opt_in_compression_preserves_state_recovery_and_raw_default() {
    let (path, packed, expected) = actual();
    let raw = storage_codec::decode(&packed).unwrap();
    assert!(packed.len() < raw.len());
    assert_eq!(
        storage_codec::decode(&storage_codec::encode(raw.as_ref(), Codec::Raw).unwrap())
            .unwrap()
            .as_ref(),
        raw.as_ref()
    );
    reliability::verify_independent(&path, 2).unwrap();
    let mut s = Session::open(&path).unwrap();
    assert_eq!(s.world().checkpoint().unwrap(), expected);
    s.advance(2).unwrap();
    drop(s);
    let base = path.parent().unwrap().join("fork");
    fs::create_dir(&base).unwrap();
    let branch = base.join("world");
    reliability::recover(&path, &branch, 2).unwrap();
    let mut resumed = Session::open(&branch).unwrap();
    assert_eq!(resumed.world().checkpoint().unwrap(), expected);
    resumed.advance(2048).unwrap();
    resumed.finalize().unwrap();
    drop(resumed);
    let raw_saved = fs::read(branch.join("checkpoint-00000003.cbor")).unwrap();
    assert!(matches!(
        storage_codec::decode(&raw_saved).unwrap(),
        std::borrow::Cow::Borrowed(_)
    ));
    reliability::verify_independent(&branch, 3).unwrap();
}
#[test]
fn compressed_corruption_sizes_trailing_data_and_unknown_versions_fail() {
    let (_, packed, _) = actual();
    let edit = |change: fn(&mut Packed)| {
        let mut p: Packed = ciborium::from_reader(packed.as_slice()).unwrap();
        change(&mut p);
        assert!(storage_codec::decode(&encode(&p)).is_err());
    };
    edit(|p| p.uncompressed_bytes = 160 * 1024 * 1024 + 1);
    edit(|p| p.uncompressed_bytes = 1);
    edit(|p| p.uncompressed_sha256 = "0".repeat(64));
    edit(|p| {
        p.payload.truncate(p.payload.len() - 2);
    });
    edit(|p| p.payload.extend(b"trailing"));
    edit(|p| p.codec = "unknown".into());
    edit(|p| p.schema = "woi-world-storage/v99".into());
    let nested = storage_codec::encode(&packed, Codec::Zlib).unwrap();
    assert!(storage_codec::decode(&nested).is_err());
}
