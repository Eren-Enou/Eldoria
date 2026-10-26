"""C1: one response-call seam and one discarded-score substitution;reverse exact."""
from pathlib import Path
import hashlib,json,subprocess,sys
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];OUT=Path(sys.argv[1]).resolve()
sha=lambda p:hashlib.sha256(p.read_bytes().replace(b'\r\n',b'\n')).hexdigest()
d=ROOT/'experiments/026/continuation-c1'
for manifest in (json.loads((d/'prospective-validation.json').read_bytes())['source_bound_design'],json.loads((d/'source-bindings.json').read_bytes())):
    for name,digest in manifest.items():assert sha(ROOT/name)==digest,('C1 declaration',name)
subprocess.run([sys.executable,'-B',str(ROOT/'prototypes/delayed-work-valuation/generate_host.py'),str(OUT)],check=True)
path=OUT/'delayed_coordinator.rs';old=path.read_text(encoding='utf-8')
before='let r = crate::contact_learning::respond(&e::ResponseInput {'
after='let r = crate::response_continuation::respond(&e::ResponseInput {'
assert old.count(before)==1
changed=old.replace(before,after);assert changed.replace(after,before)==old
path.write_text(changed,encoding='utf-8')
source=(ROOT/'src/encounters.rs').read_text(encoding='utf-8')
start=source.index('pub fn respond(input: &ResponseInput) -> Response {');end=source.index('\n#[derive(',start)
original=source[start:end]
lookup='''        let score = input
            .outgoing
            .candidates
            .iter()
            .find(|c| c.intent == input.outgoing.selected)
            .unwrap()
            .score;'''
zero='        let score = 0; // C1: established wrapper clamps Continue before reselection'
assert original.count(lookup)==1
function=original.replace(lookup,zero);assert function.replace(zero,lookup)==original
(OUT/'continuation_response.rs').write_text('//! C1 preserves every defined contact-learning response;totalizes missing membership.\nuse crate::encounters::*;\n'+function+'\n',encoding='utf-8')
attribute=lambda p:'#[path = '+json.dumps(p.as_posix())+']\n'
path=OUT/'lib_host.rs';path.write_text(path.read_text(encoding='utf-8')+'\n'+attribute(OUT/'continuation_response.rs')+'pub mod response_continuation;\n'+attribute(HERE/'src/model.rs')+'pub mod continuation_c1;\n',encoding='utf-8')
freeze=d/'source-freeze.json';stamp='UNFROZEN'
if freeze.exists():
    for name,digest in json.loads(freeze.read_bytes()).items():assert sha(ROOT/name)==digest,('C1 source',name)
    stamp=sha(freeze)
(OUT/'c1_stamp.rs').write_text('pub const SOURCE_FREEZE_SHA256: &str = '+json.dumps(stamp)+';\n',encoding='utf-8')
(OUT/'c1_generation_audit.json').write_text(json.dumps(dict(coordinator_substitutions=1,response_substitutions=1,reverse_exact=True,
    original_response_function_sha256=hashlib.sha256(original.encode()).hexdigest(),scope='only copied026 coordinator;original producers and functions untouched'),indent=2)+'\n',encoding='utf-8')
