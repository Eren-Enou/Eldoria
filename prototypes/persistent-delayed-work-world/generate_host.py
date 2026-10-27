"""Exact persistent C1 projection; original physics/storage retain ownership."""
from pathlib import Path
import hashlib,json,os,subprocess,sys
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];OUT=Path(sys.argv[1]).resolve()
sys.path.insert(0,str(ROOT/'examples/support'))
from foundation026 import identity
identity()
subprocess.run(['git','merge-base','--is-ancestor','ab96bd237ea1582a2fb5bd0922e3cb9a41820768','HEAD'],cwd=ROOT,check=True)
subprocess.run([sys.executable,'-B',str(ROOT/'prototypes/persistent-material-world/generate_host.py'),str(OUT)],check=True)
reference=OUT/'c1-reference';reference.mkdir(exist_ok=True)
subprocess.run([sys.executable,'-B',str(ROOT/'prototypes/delayed-work-valuation-c1/generate_host.py'),str(reference)],check=True)
seams=json.loads((OUT/'source-seams.json').read_bytes())
def attr(p):return '#[path = '+json.dumps(p.as_posix())+']'
def output(n,s):(OUT/n).write_text(s,encoding='utf-8')
def changed(name,s,pairs):
 original=s
 for old,new,count in pairs:
  assert s.count(old)==count,(name,old[:80],s.count(old),count);s=s.replace(old,new)
 reverse=s
 for old,new,count in reversed(pairs):
  assert reverse.count(new)==count,(name,'reverse',new[:80]);reverse=reverse.replace(new,old)
 assert reverse==original,name
 seams.append(dict(source=name,substitutions=sum(p[2]for p in pairs),reverse_exact=True,original_sha256=hashlib.sha256(original.encode()).hexdigest()))
 return s
def patch(n,pairs):output(n,changed(n,(OUT/n).read_text(encoding='utf-8'),pairs))
source=(reference/'delayed_coordinator.rs').read_text(encoding='utf-8')
output('delayed_coordinator.rs',changed('C1 boundary visibility',source,[('    fn delayed_boundary26(', '    pub(super) fn delayed_boundary26(',1)]))
with (OUT/'concurrent_host.rs').open('a',encoding='utf-8')as f:f.write('\n'+attr(OUT/'delayed_coordinator.rs')+'\nmod delayed_coordinator;\n')
output('continuation_response.rs',(reference/'continuation_response.rs').read_text(encoding='utf-8'))
for n in ['source_stamp.rs','c1_stamp.rs']:output(n,(reference/n).read_text(encoding='utf-8'))
# Factor observer wrappers only; original functions retain identical delegation.
s=(ROOT/'prototypes/delayed-work-valuation/src/model.rs').read_text(encoding='utf-8')
s=changed('026 observer envelope factorization',s,[
 ('"../../../experiments/026/registered-matrix.json"',json.dumps((ROOT/'experiments/026/registered-matrix.json').as_posix()),1),
 ('    let mut body =\n', '    let body =\n',1),
 ('    let mut projection_audit = vec![];', '    from_body(config, body)\n}\npub fn from_body(config: Config, mut body: m::Trial) -> Trial {\n    let mut projection_audit = vec![];',1)])
output('delayed_model.rs',s)
s=(ROOT/'prototypes/delayed-work-valuation-c1/src/model.rs').read_text(encoding='utf-8')
s=changed('C1 observer envelope factorization',s,[('    let mut response_extensions = vec![];', '    from_trial(trial)\n}\npub fn from_trial(trial: n::Trial) -> Trial {\n    let mut response_extensions = vec![];',1)])
output('c1_model.rs',s)
with (OUT/'lib_host.rs').open('a',encoding='utf-8')as f:
 for filename,module in [('delayed_model.rs','delayed'),('c1_model.rs','continuation_c1'),('continuation_response.rs','response_continuation')]:f.write('\n'+attr(OUT/filename)+'\npub mod '+module+';\n')
patch('runtime.rs',[
 ('"woi-material-world-checkpoint/v1"','"woi-delayed-work-world-checkpoint/v1"',1),
 ('    valuation: Option<m::Config>,','    valuation: Option<m::Config>,\n    delayed: crate::delayed::Config,',2),
 ('    execution_phase: ExecutionPhase,','    execution_phase: ExecutionPhase,\n    policy_identity: String,',1),
 ('pub fn new_valuation(registered: m::Config)','pub fn new_valuation(registered: crate::delayed::Config)',1),
 ('pub fn new(registered: m::Config) -> Result<Self, String> {\n        registered.validate()?;', 'pub fn new(delayed: crate::delayed::Config) -> Result<Self, String> {\n        crate::validate_registration(&delayed)?;\n        let registered = delayed.world.clone();\n        registered.validate()?;',1),
 ('            valuation: Some(registered),', '            delayed, valuation: Some(registered),',1),
 ('            execution_phase: self.execution_phase(),','            execution_phase: self.execution_phase(),\n            policy_identity: crate::policy_identity(self.delayed.mode),\n            delayed: self.delayed.clone(),',1),
 ('        saved.config.validate()?;', '        saved.config.validate()?;\n        crate::validate_registration(&saved.delayed)?;\n        if saved.policy_identity != crate::policy_identity(saved.delayed.mode) { return Err("changed policy identity".into()); }',1),
 ('            valuation: saved.valuation,','            valuation: saved.valuation,\n            delayed: saved.delayed,',1),
 ('    pub fn validate(&self) -> Result<(), String> {','    pub fn validate(&self) -> Result<(), String> {\n        crate::validate_registration(&self.delayed)?;\n        if self.valuation.as_ref() != Some(&self.delayed.world) { return Err("inconsistent saved026 registration".into()); }\n        crate::decision_audit::validate(&serde_json::json!({"boundaries":self.boundaries,"pending":self.pending()}), &format!("{:?}",self.delayed.mode))?;',1),
 ('self.sim.preparation_boundary25(', 'self.sim.delayed_boundary26(',1),
 ('registered.valuation, &mut self.valuations)', 'registered.valuation, self.delayed.mode, &mut self.valuations)',1),
 ('    pub fn trial(&self) -> Result<m::Trial, String>', '    pub fn material_trial(&self) -> Result<m::Trial, String>',1),
 ('serde_json::to_vec(&self.trial()?).map_err(|e|e.to_string())', 'serde_json::to_vec(&serde_json::json!({"schema":"woi-persistent-C1-reference-projection/v1","runtime":IDENTITY,"policy_identity":crate::policy_identity(self.delayed.mode),"c1_projection":self.trial()?})).map_err(|e|e.to_string())',1),
 ('    pub fn material(&self) -> &m::Material {', '    pub fn delayed_registration(&self) -> &crate::delayed::Config { &self.delayed }\n    pub fn trial(&self) -> Result<crate::continuation_c1::Trial, String> { Ok(crate::continuation_c1::from_trial(crate::delayed::from_body(self.delayed.clone(), self.material_trial()?))) }\n    pub fn material(&self) -> &m::Material {',1),
 (attr(ROOT/'prototypes/persistent-material-world/src/material_validation.rs'),attr(OUT/'material_validation.rs'),1),
])
with (OUT/'runtime.rs').open('a',encoding='utf-8')as f:f.write('\n#[cfg(test)]\n'+attr(HERE/'src/semantic_tests.rs')+'\nmod semantic_tests;\n')
s=(ROOT/'prototypes/persistent-material-world/src/material_validation.rs').read_text(encoding='utf-8')
before,separator,tests=s.partition('\n#[cfg(test)]\n');assert separator
output('material_validation.rs',before+'\n')
seams.append(dict(source='historical025-only test module',historical_tests_retained_in_original_host=True,production_prefix_exact=True))
patch('session.rs',[
 ('pub fn create(path: &Path, config: m::Config)', 'pub fn create(path: &Path, config: crate::delayed::Config)',1),
 ('pub fn create_valuation(path: &Path, config: m::Config)', 'pub fn create_valuation(path: &Path, config: crate::delayed::Config)',1),
 ('"valuation":world.valuation,','"valuation":world.valuation,"delayed":world.delayed,"policy_identity":crate::policy_identity(world.delayed.mode),',1),
])
extension='''    let state_json=serde_json::json!({"pending":serde_json::to_value(field(physical,"pending")?).map_err(|e|e.to_string())?,"delayed":serde_json::to_value(field(&state,"delayed")?).map_err(|e|e.to_string())?,"policy_identity":serde_json::to_value(field(&state,"policy_identity")?).map_err(|e|e.to_string())?,"boundaries":serde_json::to_value(field(&state,"boundaries")?).map_err(|e|e.to_string())?});
    let captured=records.iter().skip(1).map(|r|r["boundary"].clone()).collect::<Vec<_>>();
    if state_json["boundaries"]!=serde_json::Value::Array(captured) {return Err("independent saved decision/journal mismatch".into());}
    let retained=serde_json::json!({"agents":serde_json::to_value(field(&state,"agents")?).map_err(|e|e.to_string())?,"initial":serde_json::to_value(field(&state,"initial")?).map_err(|e|e.to_string())?,"needs":serde_json::to_value(field(physical,"needs")?).map_err(|e|e.to_string())?,"meals":serde_json::to_value(field(field(&state,"cognition")?,"consumption")?).map_err(|e|e.to_string())?,"events":serde_json::to_value(field(runtime,"events")?).map_err(|e|e.to_string())?});
    crate::decision_audit::retained_primitives(&retained)?;
    let delayed=&state_json["delayed"];
    if delayed != &header["delayed"] || state_json["policy_identity"] != header["policy_identity"] {
        return Err("independent026 mode/source journal binding mismatch".into());
    }
    let config: crate::delayed::Config=serde_json::from_value(delayed.clone()).map_err(|e|e.to_string())?;
    crate::validate_registration(&config)?;
    if state_json["policy_identity"] != crate::policy_identity(config.mode) { return Err("independent026 policy identity".into()); }
    crate::decision_audit::validate(&state_json,delayed["mode"].as_str().ok_or("missing mode")?)?;
    Ok(Verified {'''
patch('verification.rs',[
 ('"woi-material-world-checkpoint/v1"','"woi-delayed-work-world-checkpoint/v1"',1),
 ('    Ok(Verified {',extension,1),
 ('scope: "Independent raw CBOR/JSON binding, events, commitment lifecycle, common mass, material phases, extraction/conversion and environmental retention; not a full cognition or behavioral equivalence proof",','scope: "Independent raw binding, explicit026 mode, local scores/ties, pending state, need/meals, episode membership, material/time/environment retention; not full cognitive content or behavioral equivalence",',1),

])
# Storage/recovery algorithms remain generated by their canonical owners.
files=set()
for manifest in ['experiments/026/source-freeze.json','experiments/026/continuation-c1/source-freeze.json','experiments/026/continuation-c1/source-bindings.json']:
 files.add(manifest);files.update(json.loads((ROOT/manifest).read_bytes()))
for directory in [HERE,ROOT/'prototypes/persistent-world',ROOT/'prototypes/persistent-material-world',ROOT/'prototypes/continuous-world',ROOT/'prototypes/activity-valuation',ROOT/'prototypes/material-preparation',ROOT/'prototypes/material-preparation-observer2']:
 files.update(p.relative_to(ROOT).as_posix()for p in directory.rglob('*')if p.is_file()and'target'not in p.parts and p.suffix in ('.rs','.py','.inc','.toml','.lock'))
files.update(p.relative_to(ROOT).as_posix()for p in (ROOT/'src').rglob('*.rs'))
digest=hashlib.sha256();digest.update(subprocess.check_output(['rustc','--version']).strip());digest.update(os.environ['TARGET'].encode())
if os.environ.get('CARGO_FEATURE_FAILURE_PROBES'):digest.update(b'failure-probes')
for n in sorted(files):digest.update(n.encode()+b'\0'+(ROOT/n).read_bytes().replace(b'\r\n',b'\n'))
output('runtime_identity.txt',digest.hexdigest())
output('source-seams.json',json.dumps(seams,indent=2))
