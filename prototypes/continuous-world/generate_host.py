"""Source-preserving host. Only module paths and an appended private child differ.
No policy or physical function body is rewritten. Root files are never written.
"""
from pathlib import Path
import hashlib,json,re,sys
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
OUT=Path(sys.argv[1]).resolve();OUT.mkdir(parents=True,exist_ok=True)
locks=json.loads((ROOT/'experiments/023/inherited-source-bindings.json').read_bytes())
for name,expected in locks.items():
    actual=hashlib.sha256((ROOT/name).read_bytes().replace(b'\r\n',b'\n')).hexdigest()
    assert actual==expected, 'Inherited source changed: '+name

def attribute(path):
    return '#[path = '+json.dumps(path.as_posix())+']\n'

def rewrite_modules(text,folder,overrides):
    def replace(match):
        name=match[2];path=overrides.get(name,folder/(name+'.rs'))
        assert path.exists() or path.parent==OUT
        return attribute(path)+match[0]
    return re.sub(r'^(pub )?mod (\w+);$',replace,text,flags=re.M)

concurrent=(ROOT/'src/simulation/concurrent.rs').read_text(encoding='utf-8')
# The private child is appended; all original scheduler functions stay byte-text-identical.
child=HERE/'src/coordinator.rs'
if child.exists():
    prefix='    pub(crate) fn concurrent_trial(config: x::Config) -> x::Trial {'
    begin=concurrent.index(prefix);stop=concurrent.index('        let mut boundaries = vec![];',begin)
    bootstrap=concurrent[begin:stop].replace(prefix,'    fn continuous_bootstrap(config: x::Config) -> (Simulation, u64, u64, Vec<Agent>, usize, x::LocalContexts, u32, u64) {',1)
    bootstrap+='        (sim, start, end, initial, bootstrap_events, initial_contexts, stock, total)\n    }\n'
    concurrent+='\n// Versioned023 host extension. Original functions above are untouched.\nimpl Simulation {\n'+bootstrap+'}\n'+attribute(child)+'mod continuous;\n'
(OUT/'concurrent_host.rs').write_text(concurrent,encoding='utf-8')
adaptive=(ROOT/'src/simulation/adaptive.rs').read_text(encoding='utf-8')
assert adaptive.count('#[path = "concurrent.rs"]')==1
adaptive=adaptive.replace('#[path = "concurrent.rs"]',attribute(OUT/'concurrent_host.rs').rstrip())
(OUT/'adaptive_host.rs').write_text(adaptive,encoding='utf-8')
sim=(ROOT/'src/simulation.rs').read_text(encoding='utf-8')
(OUT/'simulation_host.rs').write_text(rewrite_modules(sim,ROOT/'src/simulation',{'adaptive':OUT/'adaptive_host.rs'}),encoding='utf-8')
lib=(ROOT/'src/lib.rs').read_text(encoding='utf-8')
lib=rewrite_modules(lib,ROOT/'src',{'simulation':OUT/'simulation_host.rs'})
if (HERE/'src/model.rs').exists():lib+='\n'+attribute(HERE/'src/model.rs')+'pub mod continuous;\n'
(OUT/'lib_host.rs').write_text(lib,encoding='utf-8')
