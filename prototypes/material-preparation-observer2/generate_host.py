from pathlib import Path
import subprocess,sys,json,hashlib
here=Path(__file__).resolve().parent;root=here.parents[1];out=Path(sys.argv[1]).resolve()
subprocess.run([sys.executable,'-B',str(root/'prototypes/material-preparation/generate_host.py'),str(out)],check=True)
for manifest in ('source-freeze.json','validation-source-locks.json'):
 for n,h in json.loads((root/'experiments/025'/manifest).read_bytes()).items():assert hashlib.sha256((root/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,n
p=out/'preparation_coordinator.rs';s=p.read_text(encoding='utf-8');old='            final_state.validate().unwrap();';new='            crate::preparation_observer2::validate(&final_state).unwrap();'
assert s.count(old)==1;changed=s.replace(old,new);assert changed.replace(new,old)==s;p.write_text(changed,encoding='utf-8')
p=out/'lib_host.rs';s=p.read_text(encoding='utf-8');s+='\n#[path = '+json.dumps((here/'src/observer_validation.rs').as_posix())+']\npub mod preparation_observer2;\n';p.write_text(s,encoding='utf-8')
