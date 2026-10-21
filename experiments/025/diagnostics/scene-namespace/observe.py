from pathlib import Path
import subprocess,sys,json,hashlib
r=Path(__file__).resolve().parents[2];out=Path(sys.argv[1])
subprocess.run([sys.executable,'-B',str(r/'prototypes/material-preparation/generate_host.py'),str(out)],check=True)
p=out/'preparation_coordinator.rs';original=p.read_text(encoding='utf-8');needle='            final_state.validate().unwrap();'
assert original.count(needle)==1
addition='            if let Err(error) = final_state.validate() {\n                let capture = std::env::var("DIAGNOSTIC025_CAPTURE").unwrap();\n                let mut file = std::fs::OpenOptions::new().create_new(true).write(true).open(capture).unwrap();\n                serde_json::to_writer(&mut file, &final_state).unwrap();\n                eprintln!("DIAGNOSTIC ONLY: immutable failed snapshot captured; original assertion follows: {error}");\n            }\n'
observed=original.replace(needle,addition+needle);assert observed.replace(addition+needle,needle)==original
p.write_text(observed,encoding='utf-8')
record=dict(kind='scratch observer-only capture before unchanged failure assertion',original_sha256=hashlib.sha256(original.encode()).hexdigest(),instrumented_sha256=hashlib.sha256(observed.encode()).hexdigest(),inserted_text=addition,original_assertion_retained=True)
(Path(__file__).parent/'instrumentation.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
