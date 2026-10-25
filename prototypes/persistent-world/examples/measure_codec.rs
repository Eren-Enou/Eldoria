//! Investigation only: genuine validated checkpoints, no storage writer changes.
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use std::{
    env, fs,
    io::{Read, Write},
    path::PathBuf,
    time::Instant,
};
use woi_persistent_world::persistent::{
    IDENTITY,
    session::{check_path, reliability},
};
fn main() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let output = PathBuf::from(args.first().ok_or("measure_codec NEW-JSON CHECKPOINT...")?);
    check_path(&output)?;
    if args.len() < 2 {
        return Err("supply at least one genuine checkpoint".into());
    }
    let mut rows = vec![];
    for input in &args[1..] {
        let input = PathBuf::from(input);
        check_path(&input)?;
        let name = input
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("invalid checkpoint filename")?;
        let generation = name
            .strip_prefix("checkpoint-")
            .and_then(|n| n.strip_suffix(".cbor"))
            .ok_or("expected committed checkpoint filename")?
            .parse()
            .map_err(|_| "invalid generation")?;
        let started = Instant::now();
        let verified = reliability::verify_independent(
            input.parent().ok_or("missing world directory")?,
            generation,
        )?;
        let verification_ms = started.elapsed().as_secs_f64() * 1000.;
        let raw = fs::read(&input).map_err(|e| e.to_string())?;
        let started = Instant::now();
        let mut encoder = ZlibEncoder::new(vec![], Compression::new(6));
        encoder.write_all(&raw).map_err(|e| e.to_string())?;
        let packed = encoder.finish().map_err(|e| e.to_string())?;
        let encode_ms = started.elapsed().as_secs_f64() * 1000.;
        let started = Instant::now();
        let mut decoded = vec![];
        ZlibDecoder::new(packed.as_slice())
            .take(160 * 1024 * 1024 + 1)
            .read_to_end(&mut decoded)
            .map_err(|e| e.to_string())?;
        let decode_ms = started.elapsed().as_secs_f64() * 1000.;
        if raw != decoded {
            return Err("codec roundtrip differs from original bytes".into());
        }
        rows.push(serde_json::json!({"input":input,"tick":verified.tick,"raw_bytes":raw.len(),"zlib_bytes":packed.len(),"encode_ms":encode_ms,"decode_ms":decode_ms,"verification_ms":verification_ms,"exact_roundtrip":true}));
    }
    let report = serde_json::json!({"schema":"woi-codec-investigation/v1","runtime":IDENTITY,"rows":rows,"scope":"Wall-clock under host/cache load; raw zlib investigation excludes prospective container overhead; not simulation evidence, live heap or a storage-format adoption"});
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)
        .map_err(|e| e.to_string())?;
    serde_json::to_writer_pretty(&mut file, &report).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    println!("{report}");
    Ok(())
}
