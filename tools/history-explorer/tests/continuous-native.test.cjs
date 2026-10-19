const fs=require('node:fs'),vm=require('node:vm'),assert=require('node:assert/strict');
const code=fs.readFileSync(require('node:path').join(__dirname,'../native-concurrent.js'),'utf8');
const context=vm.createContext({});vm.runInContext(code+';this.native=NativeConcurrent;',context);const native=context.native;
for(const n of [1,13,19,21]){
 const data=JSON.parse(fs.readFileSync(require('node:path').join(process.argv[2],`${n}.json`),'utf8'));
 assert(native.is(data));assert.equal(data.trial.schema,'experiment-023-physical-history/v1');
 for(const moment of data.timeline){
  if(moment.kind==='checkpoint'){
   assert.equal(native.resolveData(data,moment.ref).tick,moment.tick);
   for(let owner=0;owner<4;owner++)assert.equal(native.project(data,moment,owner),null);
  }
 }
 const boundary=data.timeline.find(m=>m.kind==='decision'&&data.trial.boundaries[m.boundary].choices.length===4);
 if(boundary)for(let owner=0;owner<4;owner++){
  const p=native.project(data,boundary,owner);assert.equal(p.own.id,owner);assert(!JSON.stringify(p).includes('actual_root'));
  assert.equal(native.sourceRef(data,p.ref),'history.'+p.ref);
 }
 if(n===13){const moment=data.timeline.find(m=>m.kind==='inquiry'),actor=native.project(data,moment,0);assert.equal(actor.input_tick,6);assert(!('response'in actor));assert(!('concern_after'in actor));}
 assert.throws(()=>native.resolveData(data,'constructor.prototype'));
}
console.log('Main023 actual-source owned/query/checkpoint projections PASS');
