'use strict';
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const native=require('../native-concurrent.js');
const directory=process.argv[2];
assert(directory,'Provide a temporary directory of verified original exports');
const read=n=>JSON.parse(fs.readFileSync(path.join(directory,`${n}.json`),'utf8'));
const first=read(1);
assert(native.is(first));assert.equal(first.trial.schema,'experiment-022-trial/v1');
assert(!Object.hasOwn(first.trial,'periods'));assert(!Object.hasOwn(first.trial,'clock'));
const start=first.timeline.find(m=>m.kind==='decision'&&m.tick===6);
for(let owner=0;owner<4;owner++){
 const view=native.project(first,start,owner);
 assert.equal(view.own.id,owner);assert.equal(view.input.legacy.own.id,owner);
 assert.equal(view.own.food,2);assert.equal(view.own.hunger,20);
 assert(!Object.hasOwn(view.input,'stock'));assert(!Object.hasOwn(view,'observer_physical'));
}
assert.equal(native.project(first,first.timeline.find(m=>m.kind==='admission'),0),null);
const event=first.timeline.find(m=>m.kind==='resource'&&m.tick===10&&native.resolve(first.trial,m.ref).decision.actor===1);
assert.equal(native.project(first,event,0),null);
assert.equal(native.project(first,event,1).selected,'Accept');
assert(!Object.hasOwn(native.project(first,event,1),'balances_before'));
const mask=read(13),boundary=mask.timeline.find(m=>m.kind==='decision'&&m.tick===14);
assert.equal(native.project(mask,boundary,3).selected,'Acquire');
assert.equal(native.project(mask,boundary,3).input.inquiry_lane_free,false);
assert.equal(native.project(mask,boundary,2),null);
const query=read(18);
assert.equal(native.project(query,query.timeline.find(m=>m.kind==='need'&&m.tick===8),0),null);
const completion=query.timeline.find(m=>m.kind==='inquiry'&&m.tick===19);
const selection=native.project(query,completion,0);
assert.equal(selection.input_tick,6);assert.equal(selection.input.owner,0);
assert(!Object.hasOwn(selection,'response'));assert(!Object.hasOwn(selection,'concern_after'));
assert.equal(native.project(query,completion,2),null);
const helpful=read(24),cautious=read(26);
const a=native.project(helpful,helpful.timeline.find(m=>m.kind==='decision'),0);
const b=native.project(cautious,cautious.timeline.find(m=>m.kind==='decision'),0);
assert.deepEqual(a,b,'Partner-private trait changes must not enter focal captured input');
assert.notEqual(helpful.trial.initial[1].generosity,cautious.trial.initial[1].generosity);
const serial=read(0);
let stale=0;
for(const m of serial.timeline.filter(m=>m.kind==='decision'))for(let owner=0;owner<4;owner++){
 const view=native.project(serial,m,owner),cell=view?.input?.retained;
 if(cell&&Math.max(0,cell.observation.stock-cell.own_takes)>serial.observer_physical.find(p=>p.tick===m.tick).stock){
  stale++;assert.deepEqual(view.input.retained,native.resolve(serial.trial,view.ref).input.retained);
 }
}
assert(stale>0,'Original stale local knowledge remains mistaken in Actor Perspective');
assert.throws(()=>native.resolve(first.trial,'__proto__.constructor'));
console.log('native022: actual four-owner inputs, private mirror, no busy/interior backfill, response privacy, stale information and typed references PASS');
