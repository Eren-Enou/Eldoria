"""025-only generated type/phase/ledger seams; never write historical sources.

Every substitution is counted and reversed. That text guard complements, rather
than replaces, compiled disabled parity and independent physical verification.
"""
from pathlib import Path
import hashlib
import json
import subprocess
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OUT = Path(sys.argv[1]).resolve()
OUT.mkdir(parents=True, exist_ok=True)
for name, expected in json.loads((ROOT/'experiments/024/source-freeze.json').read_bytes()).items():
    assert hashlib.sha256((ROOT/name).read_bytes().replace(b'\r\n', b'\n')).hexdigest() == expected, name
subprocess.run([sys.executable, '-B', str(ROOT/'prototypes/activity-valuation/generate_host.py'), str(OUT)], check=True)


def attribute(path):
    return '#[path = '+json.dumps(path.as_posix())+']'


def changed(original, substitutions):
    value = original
    for before, after in substitutions:
        assert value.count(before) == 1, ('025 seam count', before[:100])
        value = value.replace(before, after, 1)
    back = value
    for before, after in reversed(substitutions):
        assert back.count(after) == 1, ('025 reverse count', after[:100])
        back = back.replace(after, before, 1)
    assert back == original, 'Undeclared025source change'
    return value


original = (ROOT/'src/experiment022.rs').read_text(encoding='utf-8')
record_types = changed(original, [
    ('    Query,\n}', '    Query,\n    Prepare,\n}'),
    ('include_str!("../experiments/022/registered-matrix.json")',
     'include_str!('+json.dumps((ROOT/'experiments/022/registered-matrix.json').as_posix())+')'),
])
(OUT/'records025.rs').write_text(record_types, encoding='utf-8')
original = (ROOT/'src/experiment.rs').read_text(encoding='utf-8')
experiment = changed(original, [
    ('#[path = "experiment022.rs"]', attribute(OUT/'records025.rs')),
])
(OUT/'experiment025_host.rs').write_text(experiment, encoding='utf-8')

path = OUT/'concurrent_host.rs'
original = path.read_text(encoding='utf-8')
ledger_begin = original.index('    fn concurrent_ledger(&self) {')
ledger_end = original.index('        let actors = s', ledger_begin)
old_ledger = original[ledger_begin:ledger_end]
new_ledger = old_ledger.replace('            s.total,',
                               '            s.total.checked_add(self.material_parts25().0).unwrap(),', 1)
new_ledger = new_ledger.replace('                    .sum::<u64>()',
                               '                    .sum::<u64>() + self.material_parts25().1', 1)
assert new_ledger != old_ledger and new_ledger.count('material_parts25') == 2
memory_begin = original.index('    fn concurrent_memory(')
memory_end = original.index('        let tick = self.activity_tick();', memory_begin)
memory_signature = original[memory_begin:memory_end]
physical = changed(original, [
    ('                _ => c.deadline <= next,',
     '                x::Work::Prepare => false,\n                _ => c.deadline <= next,'),
    ('        self.concurrent_ledger();\n    }\n    fn concurrent_ledger',
     '        self.material_tick25();\n        self.concurrent_ledger();\n    }\n    fn concurrent_ledger'),
    (old_ledger, new_ledger),
    (memory_signature, memory_signature+'        self.material_cache25(owner, &after);\n'),
])
physical += '\n'+attribute(HERE/'src/coordinator.rs')+'\nmod preparation;\n'
path.write_text(physical, encoding='utf-8')
path = OUT/'lib_host.rs'
original = path.read_text(encoding='utf-8')
library = changed(original, [
    (attribute(ROOT/'src/experiment.rs'), attribute(OUT/'experiment025_host.rs')),
])
library += '\n'+attribute(HERE/'src/model.rs')+'\npub mod preparation;\n'
path.write_text(library, encoding='utf-8')

# Preserve the inherited response/admission/bootstrap/drain path. Only the
# ordinary observation, five-way selection and actual Prepare dispatch differ.
original = (OUT/'valuation_coordinator.rs').read_text(encoding='utf-8')
begin = original.index('            let selection = (policy.selector())')
end = original.index('\n        }\n        for d in &boundary.choices', begin)
old_selection = original[begin:end]
new_selection = '''            let inherited = (policy.selector())(&decision.input, &decision.candidates);
            let input = m::Input {
                inherited: decision.input,
                raw_retained: self.world.resource::<m::Material>().slots[owner as usize].clone(),
            };
            let selection = m::rank(&input, &inherited.terms);
            valuations.push(m::Record { boundary:index, choice:boundary.choices.len(), policy, selection:selection.clone() });
            boundary.choices.push(m::Choice {
                input, candidates:selection.terms, selected:selection.selected,
                contact_candidates:decision.base.contact.candidates,
                contact:decision.base.contact.selected,
            });'''
begin = original.index('        for d in &boundary.choices')
end = original.index('        self.concurrent_ledger();', begin)
old_dispatch = original[begin:end]
new_dispatch = old_dispatch.replace('d.input.legacy', 'd.input.inherited.legacy').replace('a::Action::', 'm::Action::')
new_dispatch = new_dispatch.replace('            match d.selected {', '''            match d.selected {
                m::Action::Prepare => { self.material_reserve25(owner, index); }''', 1)
old_sense = '''            if sense {
                let stock = self.world.resource::<State>().stock;
                self.concurrent_sense(owner, k::Change::Sense, stock, None);
            }'''
new_registration = '''        registered.validate().unwrap();
        if !registered.preparation.feature_enabled {
            let old = Self::valuation_trial24(v::Config {world:registered.world.clone(), valuation:registered.valuation, family:registered.family.clone(), source023:None});
            return m::from_legacy(registered, old);
        }'''
coordinator = changed(original, [
    ('use crate::{continuous as c, valuation as v};', 'use crate::{continuous as c, valuation as v, preparation as m};'),
    ('    fn valuation_boundary(', '    fn preparation_boundary25('),
    ('valuations: &mut Vec<v::Record>', 'valuations: &mut Vec<m::Record>'),
    (') -> x::Boundary {', ') -> m::Boundary {'),
    ('let mut boundary = x::Boundary {', 'let mut boundary = m::Boundary {'),
    ('            queries: self.world.resource::<q::Inquiry>().records.len(),',
     '            queries: self.world.resource::<q::Inquiry>().records.len(),\n            material_point: self.world.resource::<m::Material>().points.len()-1,'),
    ('        let duration = state.config.duration;\n        let sense = (now - state.epoch).is_multiple_of(state.config.cadence);\n',
     '        let duration = state.config.duration;\n'),
    (old_sense, '            self.material_glance25(owner);'),
    (old_selection, new_selection),
    (old_dispatch, new_dispatch),
    ('pub(crate) fn valuation_trial24(registered: v::Config) -> v::Trial {',
     'pub(crate) fn preparation_trial25(registered: m::Config) -> m::Trial {'),
    ('        registered.validate().unwrap();', new_registration),
    ('        let cutoff = start + cfg.horizon;',
     '        sim.world.insert_resource(m::Material::new(registered.preparation.clone(), start));\n        sim.concurrent_ledger();\n        let cutoff = start + cfg.horizon;'),
    ('sim.valuation_boundary(', 'sim.preparation_boundary25('),
    ('        assert!(sim.world.resource::<State>().pending.is_empty());',
     '        assert!(sim.world.resource::<State>().pending.is_empty());\n        assert!(sim.world.resource::<m::Material>().active.is_empty());'),
    ('            x::Trial {', '            m::History {'),
    ('experiment-024-physical-history/v1', 'experiment-025-physical-history/v1'),
    ('                initial_total: total,',
     '                initial_total: total.checked_add(u64::from(registered.preparation.raw_initial)).unwrap(),'),
    ('        let checkpoints = c::checkpoints(&history, &cfg);',
     '        let material = sim.world.resource::<m::Material>().history();\n        let checkpoints = m::checkpoints(&history, &cfg, &material.points);'),
    ('        v::Trial {', '        m::Trial {'),
    ('experiment-024-trial/v1', 'experiment-025-trial/v1'),
    ('            valuations,\n', '            valuations,\n            material,\n'),
])
(OUT/'preparation_coordinator.rs').write_text(coordinator, encoding='utf-8')
path = OUT/'concurrent_host.rs'
path.write_text(path.read_text(encoding='utf-8')+'\n'+attribute(OUT/'preparation_coordinator.rs')+'\nmod preparation_coordinator;\n', encoding='utf-8')
