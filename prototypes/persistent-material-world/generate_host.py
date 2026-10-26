"""025 continuation adapter; original simulation and storage algorithms stay owned
by their existing hosts. Every substitution is counted and reversible.
"""
from pathlib import Path
import hashlib, json, os, re, subprocess, sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
BASE = ROOT / 'prototypes/persistent-world'
OUT = Path(sys.argv[1]).resolve()
subprocess.run([sys.executable, '-B', str(ROOT/'prototypes/material-preparation-observer2/generate_host.py'), str(OUT)], check=True)
for name,expected in json.loads((ROOT/'experiments/025/source-freeze-observer2.json').read_bytes()).items():
    assert hashlib.sha256((ROOT/name).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==expected,name

def attr(p):
    return '#[path = '+json.dumps(p.as_posix())+']'

seams = []
def changed(name, text, pairs):
    original = text
    for old, new in pairs:
        assert text.count(old) == 1, (name, old[:120], text.count(old))
        text = text.replace(old, new, 1)
    back = text
    for old, new in reversed(pairs):
        assert back.count(new) == 1, (name, 'reverse', new[:120])
        back = back.replace(new, old, 1)
    assert back == original, name
    seams.append({'source':name,'sha256':hashlib.sha256(original.encode()).hexdigest(),'substitutions':len(pairs)})
    return text

def output(name, text):
    (OUT/name).write_text(text, encoding='utf8')

def patch(name, pairs):
    output(name, changed(name, (OUT/name).read_text(encoding='utf8'), pairs))

patch('simulation_host.rs', [
    ('#[derive(Resource, Default)]\nstruct Runtime', '#[derive(Resource, Default, Clone, serde::Serialize, serde::Deserialize)]\nstruct Runtime'),
    ('mod adaptive;', 'pub(crate) mod adaptive;')])
patch('adaptive_host.rs', [('mod concurrent;', 'pub(crate) mod concurrent;')])
patch('concurrent_host.rs', [
    ('#[derive(Resource)]\nstruct State', '#[derive(Resource, Clone, serde::Serialize, serde::Deserialize)]\nstruct State'),
    (attr(ROOT/'prototypes/continuous-world/src/coordinator.rs')+'\nmod continuous;', attr(OUT/'coordinator.rs')+'\npub(crate) mod continuous;')])
patch('valuation_coordinator.rs', [('    fn valuation_boundary(', '    pub(super) fn valuation_boundary(')])
patch('preparation_coordinator.rs', [('    fn preparation_boundary25(', '    pub(super) fn preparation_boundary25(')])
output('coordinator.rs', (ROOT/'prototypes/continuous-world/src/coordinator.rs').read_text(encoding='utf8')+'\n'+attr(OUT/'runtime.rs')+'\npub mod persistent;\n')

model = (ROOT/'prototypes/material-preparation/src/model.rs').read_text(encoding='utf8')
model = changed('025 model', model, [
    ('#[derive(Resource, Clone, Debug)]\npub struct Material', '#[derive(Resource, Clone, Debug, Serialize, Deserialize)]\n#[serde(deny_unknown_fields)]\npub struct Material'),
    ('"../../../experiments/025/registered-matrix.json"', json.dumps((ROOT/'experiments/025/registered-matrix.json').as_posix()))])
output('material_model.rs', model)
patch('lib_host.rs', [(attr(ROOT/'prototypes/material-preparation/src/model.rs'), attr(OUT/'material_model.rs'))])

for module in ['history', 'audit']:
    original = (ROOT/'src'/f'{module}.rs').read_text(encoding='utf8')
    if module == 'history':
        original = re.sub(r'#\[derive\(([^\n]+)\)\]', lambda m: '#[derive('+m[1]+', serde::Serialize, serde::Deserialize)]', original)
        helper = '\nimpl HistoryIndex { pub(crate) fn saved_trust(&self, owner:u32, partner:u32) -> Option<i32> { self.relationships.get(&(owner,partner)).and_then(|r|r.entries.last().map(|e|e.trust_after)) } }\n'
        assert original.count('#[cfg(test)]') == 1
        original = original.replace('#[cfg(test)]', helper+'#[cfg(test)]', 1)
    else:
        original = original.replace('#[derive(Resource, Clone, Debug, Default)]', '#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize)]')
    output(f'{module}_host.rs', original)
    patch('lib_host.rs', [(attr(ROOT/'src'/f'{module}.rs'), attr(OUT/f'{module}_host.rs'))])
with (OUT/'lib_host.rs').open('a', encoding='utf8') as f:
    f.write('\npub use simulation::adaptive::concurrent::continuous::persistent;\n')

# Same complete-state adapter; only the material compartment/type/export seams differ.
runtime = (BASE/'src/runtime.rs').read_text(encoding='utf8').split('\n#[cfg(test)]\nmod continuation_checks')[0]
pairs = [
    ('use crate::valuation as v;', 'use crate::{valuation as v, preparation as m};'),
    ('"woi-world-checkpoint/v1"', '"woi-material-world-checkpoint/v1"'),
    ('#[path = "session.rs"]', attr(OUT/'session.rs')),
]
original_types=runtime
type_seams=[('Option<v::Config>', 'Option<m::Config>'), ('Vec<v::Record>', 'Vec<m::Record>'), ('Vec<x::Boundary>', 'Vec<m::Boundary>')]
for old,new in type_seams:
    # Distinct occurrences are declared in their complete enclosing lines.
    for line in set(l for l in runtime.splitlines() if old in l):
        assert runtime.count(line+'\n') == 2, (old,line)
        # Saved and live fields have the same shape; enumerate these two seams.
        runtime = runtime.replace(line+'\n', line.replace(old,new)+'\n')
back=runtime
for old,new in reversed(type_seams):
    assert back.count(new)==2,(new,back.count(new))
    back=back.replace(new,old)
assert back==original_types
seams.append({'source':'persistent Saved/World field types','substitutions':6})
runtime = changed('persistent runtime envelope', runtime, pairs)
begin = runtime.index('    pub fn new_valuation(')
end = runtime.index('    pub fn tick(', begin)
runtime = changed('persistent constructor', runtime, [(runtime[begin:end], (HERE/'src/constructor.inc').read_text(encoding='utf8'))])
begin = runtime.index('        let boundary = if let Some(registered)')
end = runtime.index('        self.boundaries.push(boundary);', begin)
runtime = changed('persistent step', runtime, [(runtime[begin:end], (HERE/'src/step.inc').read_text(encoding='utf8'))])
runtime = changed('persistent material fields', runtime, [
    ('    local: x::LocalContexts,\n}', '    local: x::LocalContexts,\n    material: m::Material,\n}'),
    ('            local: world.resource::<x::LocalContexts>().clone(),', '            local: world.resource::<x::LocalContexts>().clone(),\n            material: world.resource::<m::Material>().clone(),'),
    ('            local\n        );', '            local,\n            material\n        );'),
    ('pub fn boundaries(&self) -> &[x::Boundary]', 'pub fn boundaries(&self) -> &[m::Boundary]'),
    ('+ consumed\n            != self.total', '+ consumed + self.sim.world.resource::<m::Material>().remaining_mass()\n            != self.total + u64::from(self.sim.world.resource::<m::Material>().settings.raw_initial)'),
    ('            if food.1 != agent.food || hunger.1 != agent.hunger {', '''            for operation in self.sim.world.resource::<m::Material>().operations.iter().filter(|o| o.owner == agent.id && o.outcome == m::Outcome::Prepared) {
                if (operation.tick, 4) >= food.0 { food = ((operation.tick, 4), operation.food_after); }
            }
            if food.1 != agent.food || hunger.1 != agent.hunger {'''),
    ('        Ok(())\n    }\n    pub fn trial(', '        self.validate_material()?;\n        Ok(())\n    }\n    pub fn trial('),
])
begin = runtime.index('    pub fn trial(')
runtime = runtime[:begin]+(HERE/'src/export.inc').read_text(encoding='utf8')+'\n}\n'
runtime += '\n'+attr(HERE/'src/material_validation.rs')+'\nmod material_validation;\n'
output('runtime.rs', runtime)

# Transport implementation is generated from its canonical owner, never copied/forked.
session = (BASE/'src/session.rs').read_text(encoding='utf8')
begin=session.index('#[cfg(test)]\nmod binding_tests')
end=session.index('\nfn hash(', begin)
session = changed('Session historical-only test module', session, [(session[begin:end], '// Historical023-only binding test lives in canonical host.\n')])
session = changed('Session typed projection', session, [
    ('#[path = "reliability.rs"]', attr(OUT/'reliability.rs')),
    ("boundaries: &'a [x::Boundary]", "boundaries: &'a [m::Boundary]"),
    ("valuations: &'a [v::Record]", "valuations: &'a [m::Record]"),
    ("    learning: &'a crate::self_evaluation::Learning,", "    learning: &'a crate::self_evaluation::Learning,\n    material: &'a m::Material,"),
    ('            learning: data.resource::<crate::self_evaluation::Learning>(),', '            learning: data.resource::<crate::self_evaluation::Learning>(),\n            material: data.resource::<m::Material>(),'),
    ('    learning: usize,', '    learning: usize,\n    material_operations: usize,\n    material_processes: usize,\n    material_memories: usize,\n    material_points: usize,'),
    ('            learning: s.learning.records.len(),', '            learning: s.learning.records.len(),\n            material_operations: s.material.operations.len(),\n            material_processes: s.material.processes.len(),\n            material_memories: s.material.memories.len(),\n            material_points: s.material.points.len(),'),
    ('pub fn create(path: &Path, config: c::Config)', 'pub fn create(path: &Path, config: m::Config)'),
    ('pub fn create_valuation(path: &Path, config: v::Config)', 'pub fn create_valuation(path: &Path, config: m::Config)'),
    ('"learning":initial.learning}', '"learning":initial.learning,"material":initial.material}'),
    ('                "events":&saved.runtime.events[prior.events..],', '''                "material_operations":&saved.material.operations[prior.material_operations..],
                "material_processes":&saved.material.processes[prior.material_processes..],
                "material_memories":&saved.material.memories[prior.material_memories..],
                "material_points":&saved.material.points[prior.material_points..],
                "events":&saved.runtime.events[prior.events..],'''),
])
output('session.rs',session)
with (OUT/'session.rs').open('a',encoding='utf8') as f:
    f.write('\nimpl Session { pub fn generation(&self)->u64 { self.generation } pub fn journal_bytes(&self)->usize { self.journal_bytes.len() } }\n')
output('reliability.rs',changed('recovery typed material audit',(BASE/'src/reliability.rs').read_text(encoding='utf8'),[
    ('    Ok((commit, journal, world))','''    crate::material_audit::journal(&serde_json::to_value(world.material()).map_err(|e|e.to_string())?,&records)?;
    Ok((commit, journal, world))''')]))
output('compatible_v1.txt','')
verification=(BASE/'src/verification.rs').read_text(encoding='utf8')
verification=changed('independent primitive reader material extension',verification,[
    ('//! Narrow independent storage/primitive reader: no World, Session or simulation validation calls.', '// Independent primitive reader, extended with material reconstruction.'),
    ('"woi-world-checkpoint/v1"','"woi-material-world-checkpoint/v1"'),
    ('pub conserved_food: u64,','pub conserved_material: u64,'),
    ('conserved_food: food,','conserved_material: food,'),
    ('    if food != n(&state, "total")? || food != n(physical, "total")? {', '''    let material = field(&state, "material")?;
    let initial_raw = n(field(material,"settings")?,"raw_initial")?;
    food = food.checked_add(n(material,"raw")?).and_then(|v|v.checked_add(n(material,"waste").ok()?)).ok_or("material mass overflow")?;
    for active in array(field(material,"active")?)? { food=food.checked_add(n(active,"escrow")?).ok_or("escrow mass overflow")?; }
    if food != n(&state, "total")? + initial_raw || food != n(physical, "total")? + initial_raw {'''),
    ('    Ok(Verified {', '''    let material_json=serde_json::to_value(material).map_err(|e|e.to_string())?;
    let physical_json=serde_json::to_value(physical).map_err(|e|e.to_string())?;
    crate::material_audit::validate(&material_json,&physical_json,tick)?;
    crate::material_audit::journal(&material_json,&records)?;
    if !same_json(field(material,"settings")?, &header["valuation"]["preparation"])? {
        return Err("independent material configuration mismatch".into());
    }
    Ok(Verified {'''),
    ('scope: "Independent raw CBOR/JSON hashes, configuration, event ordering, commitment lifecycle and aggregate food ledger; not a cognition validator or behavioral equivalence proof",',
     'scope: "Independent raw CBOR/JSON binding, events, commitment lifecycle, common mass, material phases, extraction/conversion and environmental retention; not a full cognition or behavioral equivalence proof",'),
])
output('verification.rs',verification)

files=set()
for manifest in ['experiments/023/inherited-source-bindings.json', 'experiments/023/source-freeze.json', 'experiments/024/source-freeze.json', 'experiments/025/source-freeze.json', 'experiments/025/validation-source-locks.json']:
    files.add(manifest)
    files.update(json.loads((ROOT/manifest).read_bytes()))
for directory in [HERE, BASE, ROOT/'prototypes/material-preparation-observer2']:
    files.update(p.relative_to(ROOT).as_posix() for p in directory.rglob('*') if p.is_file() and 'target' not in p.parts and p.suffix in ['.rs','.py','.toml','.lock','.inc'])
digest=hashlib.sha256()
digest.update(subprocess.check_output(['rustc','--version']).strip())
digest.update(os.environ['TARGET'].encode())
if os.environ.get('CARGO_FEATURE_FAILURE_PROBES'): digest.update(b'failure-probes')
for file in sorted(files):
    digest.update(file.encode()+b'\0'+(ROOT/file).read_bytes().replace(b'\r\n',b'\n'))
output('runtime_identity.txt',digest.hexdigest())
output('source-seams.json',json.dumps(seams,indent=2))
