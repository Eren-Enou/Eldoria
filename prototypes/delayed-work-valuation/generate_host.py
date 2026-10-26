"""026 one counted ranking seam; original corrected025 producer stays available."""
from pathlib import Path
import hashlib,json,subprocess,sys
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
OUT=Path(sys.argv[1]).resolve()
sha=lambda p:hashlib.sha256(p.read_bytes().replace(b'\r\n',b'\n')).hexdigest()
prospective=json.loads((ROOT/'experiments/026/prospective-validation.json').read_bytes())
for manifest in (prospective['registered_documents_sha256'],json.loads((ROOT/'experiments/026/inherited-source-bindings.json').read_bytes())):
    for name,digest in manifest.items():assert sha(ROOT/name)==digest,('026 registration/source',name)
subprocess.run([sys.executable,'-B',str(ROOT/'prototypes/material-preparation-observer2/generate_host.py'),str(OUT)],check=True)
original=(OUT/'preparation_coordinator.rs').read_text(encoding='utf-8')
seams=[
 ('preparation_boundary25','delayed_boundary26',2),
 ('    fn delayed_boundary26(', '    #[allow(clippy::too_many_arguments)] // preserve inherited arguments plus one explicit mode\n    fn delayed_boundary26(',1),
 ('preparation_trial25(registered: m::Config)','delayed_trial26(registered: m::Config, mode: crate::delayed::Mode)',1),
 ('        policy: v::Mode,','        policy: v::Mode,\n        mode: crate::delayed::Mode,',1),
 ('m::rank(&input, &inherited.terms)','crate::delayed::rank(&input, &inherited.terms, mode)',1),
 ('registered.valuation, &mut valuations)','registered.valuation, mode, &mut valuations)',1),
]
changed=original
for before,after,count in seams:
    assert changed.count(before)==count,('026 seam',before,changed.count(before))
    changed=changed.replace(before,after)
back=changed
for before,after,count in reversed(seams):
    assert back.count(after)==count
    back=back.replace(after,before)
assert back==original,'Undeclared026 coordinator change'
(OUT/'delayed_coordinator.rs').write_text(changed,encoding='utf-8')
attribute=lambda p:'#[path = '+json.dumps(p.as_posix())+']\n'
p=OUT/'concurrent_host.rs'
p.write_text(p.read_text(encoding='utf-8')+'\n'+attribute(OUT/'delayed_coordinator.rs')+'mod delayed_coordinator;\n',encoding='utf-8')
p=OUT/'lib_host.rs'
p.write_text(p.read_text(encoding='utf-8')+'\n'+attribute(HERE/'src/model.rs')+'pub mod delayed;\n',encoding='utf-8')
freeze=ROOT/'experiments/026/source-freeze.json'
stamp='UNFROZEN'
if freeze.exists():
    for name,digest in json.loads(freeze.read_bytes()).items():assert sha(ROOT/name)==digest,('026 runtime source',name)
    stamp=sha(freeze)
(OUT/'source_stamp.rs').write_text('pub const SOURCE_FREEZE_SHA256: &str = '+json.dumps(stamp)+';\n',encoding='utf-8')
