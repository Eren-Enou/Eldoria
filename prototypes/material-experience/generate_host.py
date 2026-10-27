"""027 additive generated successor; old source is read-only, seams reversible."""
from pathlib import Path
import hashlib, json, subprocess, sys
HERE=Path(__file__).resolve().parent; ROOT=HERE.parents[1]; OUT=Path(sys.argv[1]).resolve()
for n,h in json.loads((ROOT/'experiments/027/registration-bindings.json').read_bytes()).items():
    assert hashlib.sha256((ROOT/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,n
subprocess.run([sys.executable,'-B',str(ROOT/'prototypes/delayed-work-valuation-c1/generate_host.py'),str(OUT)],check=True)
audit=[]
def change(s, pairs):
    old=s
    for a,b,count in pairs:
        assert s.count(a)==count,(a,s.count(a));s=s.replace(a,b)
    back=s
    for a,b,count in reversed(pairs):
        assert back.count(b)==count;back=back.replace(b,a)
    assert back==old
    audit.extend(dict(before=a,after=b,count=c)for a,b,c in pairs)
    return s
attr=lambda p:'#[path = '+json.dumps(p.as_posix())+']\n'
p=OUT/'concurrent_host.rs';s=p.read_text(encoding='utf-8')
old=ROOT/'prototypes/material-preparation/src/coordinator.rs';new=OUT/'experience_physics.rs'
s=change(s,[(attr(old),attr(new),1)])
p.write_text(s+'\n'+attr(OUT/'experience_coordinator.rs')+'mod experience_coordinator;\n',encoding='utf-8')
s=old.read_text(encoding='utf-8')
s=change(s,[('        s.slots[owner as usize] = after;', '''        s.slots[owner as usize] = after;
        // NLL ends the material borrow before acquiring the learner.
        if let Some(mut learning) = self.world.get_resource_mut::<crate::experience::Learning>() {
            learning.receive(owner, tick, kind, id);
        }''',1)])
new.write_text(s,encoding='utf-8')
s=(OUT/'delayed_coordinator.rs').read_text(encoding='utf-8')
s=change(s,[
 ('delayed_boundary26','experience_boundary27',2),
 ('pub(crate) fn delayed_trial26(registered: m::Config, mode: crate::delayed::Mode) -> m::Trial {',
  'pub(crate) fn experience_trial27(registered: m::Config, mode: crate::delayed::Mode, learning: crate::experience::Mode) -> (m::Trial, crate::experience::Learning) {',1),
 ('            return m::from_legacy(registered, old);','            return (m::from_legacy(registered, old), crate::experience::Learning::new(learning));',1),
 ('        sim.world.insert_resource(m::Material::new(registered.preparation.clone(), start));',
  '        sim.world.insert_resource(crate::experience::Learning::new(learning));\n        sim.world.insert_resource(m::Material::new(registered.preparation.clone(), start));',1),
 ('            let selection = crate::delayed::rank(&input, &inherited.terms, mode);',
  '            let fixed = crate::delayed::rank(&input, &inherited.terms, mode);\n            let selection = self.world.resource_mut::<crate::experience::Learning>().choose(&input, fixed, index, boundary.choices.len());',1),
 ('        m::Trial {\n            schema:', '        let body = m::Trial {\n            schema:',1),
 ('            material,\n        }\n    }', '''            material,
        };
        sim.world.resource_mut::<crate::experience::Learning>().finish(end);
        (body, sim.world.resource::<crate::experience::Learning>().clone())
    }''',1),
])
(OUT/'experience_coordinator.rs').write_text(s,encoding='utf-8')
p=OUT/'lib_host.rs';p.write_text(p.read_text(encoding='utf-8')+'\n'+attr(HERE/'src/experience.rs')+'pub mod experience;\n',encoding='utf-8')
(OUT/'experience_generation_audit.json').write_text(json.dumps(audit,indent=2)+'\n',encoding='utf-8')
# Bind actual027 scientific input sources, excluding local paths/build artifacts.
names=['Cargo.toml','Cargo.lock','build.rs','generate_host.py','src/lib.rs','src/main.rs','src/experience.rs']
sources={('prototypes/material-experience/'+n):hashlib.sha256((HERE/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest() for n in names}
identity=hashlib.sha256(json.dumps(sources,sort_keys=True,separators=(',',':')).encode()).hexdigest()
(OUT/'experience_stamp.rs').write_text('pub const SOURCE_ID: &str = '+json.dumps(identity)+';\n',encoding='utf-8')

