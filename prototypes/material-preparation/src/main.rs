//! Opt-in025 bounded evaluator. Outputs must be new and outside archives.
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    process::{Command, Stdio},
};
use world_of_individuals::{preparation as m, valuation as v};
fn output(name: &str) -> BufWriter<std::fs::File> {
    let path = Path::new(name);
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .canonicalize()
        .expect("output parent must already exist");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    assert!(
        !parent.starts_with(root.join("experiments"))
            && !parent.starts_with(root.join("reinforcement")),
        "scientific archives are not runtime output locations"
    );
    BufWriter::new(
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .expect("output must be NEW"),
    )
}
fn write(out: &mut impl Write, trial: &impl serde::Serialize) {
    serde_json::to_writer(&mut *out, trial).unwrap();
    out.write_all(b"\n").unwrap();
    out.flush().unwrap();
}
fn read(path: &str) -> std::process::Child {
    Command::new("python").args(["-B","-c","import gzip,sys;p=sys.argv[1];f=gzip.open(p,'rb') if p.endswith('.gz') else open(p,'rb');[sys.stdout.buffer.write(line) for line in f]",path]).stdout(Stdio::piped()).spawn().unwrap()
}
fn preflight() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let code = r#"import pathlib,json,hashlib,runpy,sys
r=pathlib.Path(sys.argv[1]);g=json.loads((r/'experiments/025/runtime-gate.json').read_bytes())
assert g['status']=='PASS before first registered treatment'
for manifest in ('source-freeze.json','validation-source-locks.json'):
 for n,h in json.loads((r/'experiments/025'/manifest).read_bytes()).items():assert hashlib.sha256((r/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,('025source',n)
for n,h in json.loads((r/'experiments/025/prospective-validation.json').read_bytes())['registered_documents_sha256'].items():assert hashlib.sha256((r/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,('registration',n)
for n,h in json.loads((r/'docs/development/finite-material-foundation-validation.json').read_bytes())['source_sha256'].items():assert hashlib.sha256((r/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,('foundation gate source',n)
runpy.run_path(str(r/'examples/support/foundation024.py'))['identity']()
"#;
    assert!(
        Command::new("python")
            .args(["-B", "-c", code])
            .arg(root)
            .status()
            .unwrap()
            .success(),
        "025 pre-treatment source/gate identity"
    );
}
/// Reverse only declared025schema/additive audit fields, keeping actual actor
/// inputs/actions/physical/cognitive/consent history, and verify exact old bytes.
fn legacy_projection(new: &m::Trial, original: &v::Config) -> v::Trial {
    assert_eq!(new.config.world, original.world);
    assert_eq!(new.config.valuation, original.valuation);
    assert_eq!(new.config.preparation, m::Settings::default());
    assert!(
        new.material.processes.is_empty()
            && new.material.operations.is_empty()
            && new.material.memories.is_empty()
    );
    assert_eq!((new.material.final_raw, new.material.final_waste), (0, 0));
    let mut value = serde_json::to_value(new).unwrap();
    value.as_object_mut().unwrap().remove("material");
    value["schema"] = "experiment-024-trial/v1".into();
    value["config"] = serde_json::to_value(original).unwrap();
    value["history"]["schema"] = "experiment-024-physical-history/v1".into();
    for b in value["history"]["boundaries"].as_array_mut().unwrap() {
        b.as_object_mut().unwrap().remove("material_point");
        for d in b["choices"].as_array_mut().unwrap() {
            assert!(d["input"]["raw_retained"].is_null());
            d["input"].as_object_mut().unwrap().remove("raw_retained");
            let mut candidates = d["candidates"].as_array().unwrap().clone();
            let absent = candidates.pop().unwrap();
            assert_eq!(absent["action"], "Prepare");
            assert_eq!(absent["eligible"], false);
            assert_eq!(absent["final_score"], -1);
            d["candidates"] = candidates.into_iter().map(|c|serde_json::json!({"action":c["action"],"eligible":c["eligible"],"score":c["final_score"]})).collect();
        }
    }
    for cp in value["checkpoints"].as_array_mut().unwrap() {
        cp.as_object_mut().unwrap().remove("material_point");
    }
    for r in value["valuations"].as_array_mut().unwrap() {
        let terms = r["selection"]["terms"].as_array_mut().unwrap();
        assert_eq!(terms.len(), 5);
        assert_eq!(terms.pop().unwrap()["action"], "Prepare");
        for t in terms {
            assert_eq!(t["work_effort"], 0);
            t.as_object_mut().unwrap().remove("work_effort");
        }
    }
    serde_json::from_value(value).unwrap()
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str).unwrap_or("help") {
        "compatibility" => {
            let mut exported = args.get(3).map(|path| output(path));
            let mut child = read(
                args.get(2)
                    .map(String::as_str)
                    .unwrap_or("experiments/024/trials.jsonl.gz"),
            );
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let line = line.unwrap();
                let original: v::Trial = serde_json::from_str(&line).unwrap();
                assert_eq!(
                    serde_json::to_string(&v::trial(original.config.clone())).unwrap(),
                    line,
                    "inherited024byte replay{count}"
                );
                let new = m::trial(m::Config {
                    family: original.config.family.clone(),
                    pair: None,
                    world: original.config.world.clone(),
                    valuation: original.config.valuation,
                    preparation: m::Settings::default(),
                });
                assert_eq!(
                    serde_json::to_string(&legacy_projection(&new, &original.config)).unwrap(),
                    line,
                    "025feature-off original body{count}"
                );
                if let Some(out) = &mut exported {
                    write(out, &new);
                }
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            assert_eq!(count, 56);
            println!(
                "025 pre-treatment parity PASS:56inherited024exact byte replays;56feature-off histories/inputs/choices/checkpoints/audits exact after explicit schema projection;both old policies preserved"
            );
        }
        "matrix" => {
            preflight();
            let mut out = output(args.get(2).expect("NEW output file"));
            for (n, config) in m::matrix().into_iter().enumerate() {
                eprintln!(
                    "025row{n}:{} access{}",
                    config.family, config.preparation.access_enabled
                );
                write(&mut out, &m::trial(config));
            }
        }
        "replay" => {
            preflight();
            let mut child = read(args.get(2).expect("ORIGINAL025.jsonl/gz"));
            let mut count = 0;
            for line in BufReader::new(child.stdout.take().unwrap()).lines() {
                let line = line.unwrap();
                let original: m::Trial = serde_json::from_str(&line).unwrap();
                assert_eq!(
                    serde_json::to_string(&m::trial(original.config)).unwrap(),
                    line,
                    "025exact byte replay{count}"
                );
                count += 1;
            }
            assert!(child.wait().unwrap().success());
            println!("025 {count} exact byte replays PASS");
        }
        "run" => {
            preflight();
            let config: m::Config =
                serde_json::from_slice(&std::fs::read(args.get(2).expect("CONFIG.json")).unwrap())
                    .unwrap();
            let mut out = output(args.get(3).expect("NEW output file"));
            write(&mut out, &m::trial(config));
        }
        _ => println!(
            "material-preparation compatibility ORIGINAL024.gz\nmaterial-preparation matrix NEW.jsonl\nmaterial-preparation replay ORIGINAL025.gz\nmaterial-preparation run CONFIG.json NEW.jsonl\nNo default preparation execution;matrix/run/replay require the published pre-treatment gate/source locks."
        ),
    }
}
