"""Isolated continuation host. Original source files are never written.
Only storage derives, module visibility/paths and an appended child are added.
"""
from pathlib import Path
import hashlib, json, os, re, subprocess, sys
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
OUT=Path(sys.argv[1]).resolve()
for file, expected in json.loads((ROOT/'experiments/023/source-freeze.json').read_bytes()).items():
    assert hashlib.sha256((ROOT/file).read_bytes().replace(b'\r\n', b'\n')).hexdigest()==expected, file
for file, expected in json.loads((ROOT/'experiments/024/source-freeze.json').read_bytes()).items():
    assert hashlib.sha256((ROOT/file).read_bytes().replace(b'\r\n', b'\n')).hexdigest()==expected, file
subprocess.run([sys.executable, '-B', str(ROOT/'prototypes/activity-valuation/generate_host.py'), str(OUT)], check=True)
attribute=lambda p: '#[path = '+json.dumps(p.as_posix())+']\n'
def change(name, old, new):
    p=OUT/name
    s=p.read_text(encoding='utf-8')
    assert s.count(old)==1, (name,old)
    p.write_text(s.replace(old,new),encoding='utf-8')
change('simulation_host.rs','#[derive(Resource, Default)]\nstruct Runtime', '#[derive(Resource, Default, Clone, serde::Serialize, serde::Deserialize)]\nstruct Runtime')
change('simulation_host.rs','mod adaptive;', 'pub(crate) mod adaptive;')
change('adaptive_host.rs','mod concurrent;', 'pub(crate) mod concurrent;')
change('concurrent_host.rs','#[derive(Resource)]\nstruct State', '#[derive(Resource, Clone, serde::Serialize, serde::Deserialize)]\nstruct State')
coordinator=(ROOT/'prototypes/continuous-world/src/coordinator.rs').read_text(encoding='utf-8')
coordinator+='\n'+attribute(HERE/'src/runtime.rs')+'pub mod persistent;\n'
(OUT/'coordinator.rs').write_text(coordinator,encoding='utf-8')
change('concurrent_host.rs', attribute(ROOT/'prototypes/continuous-world/src/coordinator.rs')+'mod continuous;',attribute(OUT/'coordinator.rs')+'pub(crate) mod continuous;')
change('valuation_coordinator.rs','    fn valuation_boundary(', '    pub(super) fn valuation_boundary(')
for module in ['history','audit']:
    original=(ROOT/'src'/f'{module}.rs').read_text(encoding='utf-8')
    if module=='history':
        original=re.sub(r'#\[derive\(([^\n]+)\)\]', lambda m: '#[derive('+m[1]+', serde::Serialize, serde::Deserialize)]',original)
        helper='\nimpl HistoryIndex { pub(crate) fn saved_trust(&self, owner:u32, partner:u32) -> Option<i32> { self.relationships.get(&(owner,partner)).and_then(|r|r.entries.last().map(|e|e.trust_after)) } }\n'
        assert original.count('#[cfg(test)]')==1
        original=original.replace('#[cfg(test)]',helper+'#[cfg(test)]',1)
    else:
        original=original.replace('#[derive(Resource, Clone, Debug, Default)]', '#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize)]')
    (OUT/f'{module}_host.rs').write_text(original,encoding='utf-8')
    change('lib_host.rs',attribute(ROOT/'src'/f'{module}.rs'),attribute(OUT/f'{module}_host.rs'))
p=OUT/'lib_host.rs'
p.write_text(p.read_text(encoding='utf-8')+'\npub use simulation::adaptive::concurrent::continuous::persistent;\n',encoding='utf-8')
# Identity binds all inherited source plus successor source/config, normalized like freezes.
files=set(json.loads((ROOT/'experiments/023/inherited-source-bindings.json').read_bytes()))|set(json.loads((ROOT/'experiments/023/source-freeze.json').read_bytes()))
files|=set(json.loads((ROOT/'experiments/024/source-freeze.json').read_bytes()))
files|={(HERE/name).relative_to(ROOT).as_posix() for name in ['Cargo.toml','Cargo.lock','build.rs','generate_host.py','compatibility-v1.json']}
files|={p.relative_to(ROOT).as_posix() for p in (HERE/'src').glob('*.rs')}
digest=hashlib.sha256()
digest.update(subprocess.check_output(['rustc','--version']).strip())
digest.update(os.environ['TARGET'].encode())
if os.environ.get('CARGO_FEATURE_WORKBENCH_HOST'):
    files.add('tools/workbench/Cargo.lock')
    digest.update(b'workbench-host')
for file in sorted(files):
    digest.update(file.encode()+b'\0'+(ROOT/file).read_bytes().replace(b'\r\n',b'\n'))
(OUT/'runtime_identity.txt').write_text(digest.hexdigest(),encoding='ascii')
# Named legacy readers are allowed only with the same kernel and exact inherited
# package identities. New transport source still receives its own current identity.
import tomllib
compat=json.loads((HERE/'compatibility-v1.json').read_bytes())
kernel=(HERE/'src/runtime.rs').read_text(encoding='utf-8')
assert kernel.count('!session::accepts_runtime(&envelope.runtime)')==1
kernel=kernel.replace('!session::accepts_runtime(&envelope.runtime)', 'envelope.runtime != IDENTITY')
compatible=(hashlib.sha256(kernel.encode()).hexdigest()==compat['runtime_source_sha256']
    and subprocess.check_output(['rustc','--version']).decode().strip()==compat['compiler']
    and os.environ['TARGET']==compat['target'])
for lock, expected in compat['packages'].items():
    actual=tomllib.loads((ROOT/lock).read_text())['package']
    actual=[{k:v for k,v in p.items() if k in ['name','version','source','checksum','dependencies']} for p in actual]
    for package in actual:
        if package['name']=='woi-persistent-world':
            package['dependencies']=[d for d in package.get('dependencies',[]) if d not in compat['allowed_transport_dependencies']]
    names={p['name'] for p in expected}
    canonical=lambda values:sorted(json.dumps(p,sort_keys=True) for p in values)
    compatible &= canonical(p for p in actual if p['name'] in names)==canonical(expected)
for file,expected in compat['inherited_sources'].items():
    compatible &= hashlib.sha256((ROOT/file).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==expected
dependencies=tomllib.loads((HERE/'Cargo.toml').read_text())['dependencies']
compatible &= all(dependencies.get(name)==expected for name,expected in compat['kernel_dependencies'].items())
legacy=compat['workbench' if os.environ.get('CARGO_FEATURE_WORKBENCH_HOST') else 'standalone']
(OUT/'compatible_v1.txt').write_text(legacy if compatible else '',encoding='ascii')
