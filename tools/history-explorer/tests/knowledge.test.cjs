const assert=require('node:assert/strict');
const {execFileSync}=require('node:child_process');
const path=require('node:path');
const {orderedChainNodes}=require('../causal.js');
const f=JSON.parse(execFileSync('python',['-B',path.join(__dirname,'knowledge-fixtures.py')],{maxBuffer:4*1024*1024}));
const ids=orderedChainNodes(f.causal,f.chain,f.timeline);
const node=kind=>ids.find(id=>f.causal.nodes[id].source_kind===kind);
for(const [a,b] of [['Legitimate environmental observation','Owned environmental receipt'],
 ['Owned environmental receipt','Captured retained environmental input'],
 ['Captured retained environmental input','Selected acquisition'],
 ['Selected acquisition','Claim admission / busy interval'],
 ['Claim admission / busy interval','Physical acquisition outcome']]) {
 assert(node(a)&&node(b));assert(ids.indexOf(node(a))<ids.indexOf(node(b)),a+' must precede '+b);
}
assert.equal(f.causal.nodes[node('Physical acquisition outcome')].tick,32);
assert(ids.some(id=>f.causal.nodes[id].tick===33&&f.causal.nodes[id].source_kind==='Executed meal'));
console.log('Actual021 JS: original observation → own receipt → retained input → choice → admission → grant32 → meal33 PASS.');
