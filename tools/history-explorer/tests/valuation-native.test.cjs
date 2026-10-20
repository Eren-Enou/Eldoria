'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const model = require('../native-concurrent.js');
const load = n => JSON.parse(fs.readFileSync(path.join(process.argv[2], `${n}.json`)));
const baseline = load(0), treatment = load(1);
const moment = baseline.timeline.find(m => m.kind === 'decision' && m.tick === 18);
const left = model.project(baseline, moment, 0);
const right = model.project(treatment, moment, 0);
assert.deepEqual(left.input, right.input);
assert.equal(left.selected, 'Invite');
assert.equal(right.selected, 'Consume');
assert.deepEqual(left.valuation.selection.terms.map(t => t.final_score), [0,35,48,-1]);
assert.deepEqual(right.valuation.selection.terms.map(t => t.final_score), [0,70,48,-1]);
assert.equal(model.sourceRef(treatment, right.valuation_ref), right.valuation_ref);
assert.deepEqual(model.resolveData(treatment, right.valuation_ref), right.valuation);
assert.equal(model.sourceRef(treatment, right.ref), 'history.' + right.ref);
// Observer-only later state and another actor's captured input cannot change own view.
const before = structuredClone(right);
treatment.trial.final_agents[3].food = 9999;
treatment.observer_physical[0].stock = 9999;
treatment.valuation_deferrals = [{invented_future:9999}];
const other = treatment.trial.boundaries[12].choices.find(d => d.input.legacy.own.id !== 0);
if (other) other.input.legacy.own.food = 9999;
assert.deepEqual(model.project(treatment, moment, 0), before);
assert.equal(JSON.stringify(before).includes('9999'), false);
for (const n of [0,1,12,18,19,20]) {
  const data = load(n);
  assert.equal(model.is(data), true);
  for (const m of data.timeline) {
    model.resolveData(data,m.ref);
    if (m.kind === 'checkpoint') for(let owner=0;owner<4;owner++) assert.equal(model.project(data,m,owner),null);
    for(let owner=0;owner<4;owner++) {
      const view=model.project(data,m,owner);
      if (view?.phase==='activity') {
        assert.equal(view.own.id,owner);
        assert.equal(view.valuation.selection.selected,view.selected);
        model.resolveData(data,view.valuation_ref);
      }
    }
  }
}
const query=load(12), qm=query.timeline.find(m=>m.kind==='inquiry');
const own=model.project(query,qm,0);
assert.equal(own.phase,'inquiry selection');
assert.equal(Object.hasOwn(own,'response'),false);
assert.equal(own.input.owner,0);
console.log('024 actual owned score terms, local privacy, original refs and absent checkpoint cognition PASS');
