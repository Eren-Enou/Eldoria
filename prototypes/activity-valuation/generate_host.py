"""024 source host: append a versioned ordinary-scoring seam; never edit originals.
The registered law lives in model.rs. All physical code below is mechanically
inherited from frozen023. Exact disabled-path replay is a mandatory runtime gate.
"""
from pathlib import Path
import hashlib,json,subprocess,sys
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
OUT=Path(sys.argv[1]).resolve()
for p,h in json.loads((ROOT/'experiments/023/source-freeze.json').read_bytes()).items():
    assert hashlib.sha256((ROOT/p).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,p
subprocess.run([sys.executable,'-B',str(ROOT/'prototypes/continuous-world/generate_host.py'),str(OUT)],check=True)
original=(ROOT/'prototypes/continuous-world/src/coordinator.rs').read_text(encoding='utf-8')
new=original.replace('use crate::continuous as c;','use crate::{continuous as c, valuation as v};')
new=new.replace('continuous_boundary','valuation_boundary').replace('continuous_trial23','valuation_trial24')
new=new.replace('        cutoff: u64,\n','        cutoff: u64,\n        policy: v::Mode,\n        valuations: &mut Vec<v::Record>,\n',1)
needle='            boundary.choices.push(decision.into());'
assert new.count(needle)==1
new=new.replace(needle,'''            let selection = (policy.selector())(&decision.input, &decision.candidates);
            for (candidate, term) in decision.candidates.iter_mut().zip(&selection.terms) {
                assert_eq!(candidate.action, term.action);
                candidate.score = term.final_score;
            }
            decision.selected = selection.selected;
            valuations.push(v::Record { boundary:index, choice:boundary.choices.len(), policy, selection });
            boundary.choices.push(decision.into());''')
new=new.replace('fn valuation_trial24(cfg: c::Config) -> c::Trial {','fn valuation_trial24(registered: v::Config) -> v::Trial {\n        registered.validate().unwrap();\n        let cfg = registered.world.clone();\n        let mut valuations = vec![];')
new=new.replace('sim.valuation_boundary(boundaries.len(), &cfg, start, cutoff)','sim.valuation_boundary(boundaries.len(), &cfg, start, cutoff, registered.valuation, &mut valuations)')
new=new.replace('experiment-023-physical-history/v1','experiment-024-physical-history/v1')
new=new.replace('        c::Trial {\n            schema: "experiment-023-trial/v1".into(),\n            config: cfg,','        v::Trial {\n            schema: "experiment-024-trial/v1".into(),\n            config: registered,')
new=new.replace('            checkpoints,\n','            checkpoints,\n            valuations,\n')
assert 'fn valuation_trial24(registered: v::Config)'in new and 'c::Trial {'not in new
# Reverse every declared seam to independently verify the rest of the coordinator.
back=new.replace('use crate::{continuous as c, valuation as v};','use crate::continuous as c;').replace('valuation_boundary','continuous_boundary').replace('valuation_trial24','continuous_trial23')
back=back.replace('        policy: v::Mode,\n        valuations: &mut Vec<v::Record>,\n','')
start=back.index('            let selection = (policy.selector())');end=back.index(needle,start)
back=back[:start]+back[end:]
back=back.replace('fn continuous_trial23(registered: v::Config) -> v::Trial {\n        registered.validate().unwrap();\n        let cfg = registered.world.clone();\n        let mut valuations = vec![];','fn continuous_trial23(cfg: c::Config) -> c::Trial {')
back=back.replace(', registered.valuation, &mut valuations)',' )').replace('cutoff )','cutoff)')
back=back.replace('experiment-024-physical-history/v1','experiment-023-physical-history/v1').replace('        v::Trial {\n            schema: "experiment-024-trial/v1".into(),\n            config: registered,','        c::Trial {\n            schema: "experiment-023-trial/v1".into(),\n            config: cfg,').replace('            valuations,\n','')
assert back==original,'Undeclared physical/coordinator body change'
(OUT/'valuation_coordinator.rs').write_text(new,encoding='utf-8')
attribute=lambda p:'#[path = '+json.dumps(p.as_posix())+']\n'
p=OUT/'concurrent_host.rs';p.write_text(p.read_text(encoding='utf-8')+'\n'+attribute(OUT/'valuation_coordinator.rs')+'mod valuation;\n',encoding='utf-8')
p=OUT/'lib_host.rs';p.write_text(p.read_text(encoding='utf-8')+'\n'+attribute(HERE/'src/model.rs')+'pub mod valuation;\n',encoding='utf-8')
