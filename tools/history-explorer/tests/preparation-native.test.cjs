'use strict';
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const model = require('../native-concurrent.js');
const load = n => JSON.parse(fs.readFileSync(path.join(process.argv[2], `${n}.json`)));
const data = load(1);
assert.equal(model.is(data), true);
const moment = data.timeline.find(m => m.kind === 'decision' && m.tick === 7);
const own = model.project(data, moment, 0);
assert.equal(own.selected, 'Prepare');
assert.equal(own.input.raw_retained.observation.stock - own.input.raw_retained.own_takes, 6);
assert.equal(data.observer_physical.find(p => p.tick === 7).raw, 0);
assert.equal(own.input.raw_retained.observation.observed_at, 0);
const before = structuredClone(own);
data.preparation.material.final_raw = 9999;
data.preparation.material.points[0].raw = 9999;
data.observer_physical[0].raw = 9999;
data.trial.final_agents[3].food = 9999;
const foreign = data.trial.boundaries[7].choices.find(d => d.input.legacy.own.id !== 0);
if (foreign) foreign.input.raw_retained = {invented_partner_private:9999};
assert.deepEqual(model.project(data, moment, 0), before);
assert.equal(JSON.stringify(before).includes('9999'), false);
for (const ordinal of [0,1,15,17,19,27]) {
 const value = load(ordinal);
 for (const m of value.timeline) {
  model.resolveData(value, m.ref);
  if (m.kind === 'preparation' || m.kind === 'checkpoint') for(let owner=0;owner<4;owner++) assert.equal(model.project(value,m,owner),null);
  if (m.kind === 'raw_memory') {
   const receipt = model.resolveData(value, m.ref);
   const view = model.project(value,m,receipt.owner);
   assert.deepEqual(view.raw_retained,receipt.after);
   assert.equal(Object.hasOwn(view,'before'),false);
   assert.equal(model.project(value,m,(receipt.owner+1)%4),null);
   assert.equal(model.sourceRef(value,m.ref),m.ref);
  }
 }
 for(const ref of Object.keys(value.causal.nodes))model.resolveData(value,ref);
}
const overwritten = load(15);
const receiptMoment = overwritten.timeline.find(m => m.kind === 'raw_memory' && model.resolveData(overwritten,m.ref).kind === 'OverwrittenByReady');
assert.ok(receiptMoment,'actual raw/ready overwrite witness required');
const receipt = model.resolveData(overwritten,receiptMoment.ref);
assert.ok(receipt.before);
assert.equal(model.project(overwritten,receiptMoment,receipt.owner).raw_retained,null);
assert.equal(model.sourceRef(overwritten,'boundaries[0]'),'history.boundaries[0]');
assert.equal(model.sourceRef(overwritten,'material.operations[0]'),'material.operations[0]');
console.log('025 actual stale local state, no observer/private recovery, owned receipt eviction, physical phases and all original paths PASS');
