'use strict';
// Pure observer-data helpers, shared by the browser and real-source Node tests.
function originalValue(trial,ref) {
 if(typeof ref!=='string'||!/^\w+(?:\[\d+\]|\.\w+)*$/.test(ref))throw new Error('Invalid original reference');
 let value=trial;for(const token of ref.matchAll(/(\w+)|\[(\d+)\]/g)) {
  const key=token[1]??Number(token[2]);if(value===null||typeof value!=='object'||!Object.hasOwn(value,key))throw new Error('Missing original reference: '+ref);value=value[key];
 }return value;
}
function resourceRecords(causal,kind,after) {
 if(['unexecuted','declined','reciprocal'].includes(kind))return Object.values(causal.nodes).filter(n=>n.source_kind==='Arbitration'&&n.tick>after&&(kind==='unexecuted'?n.outcome!=='Execute':n.outcome===(kind==='declined'?'Declined':'Reciprocal'))).map(n=>({tick:n.tick,ref:n.ref,anchor:n.anchor,description:`${n.outcome} invitation ${n.actor}→${n.recipient}`}));
 return causal.encounters.filter(e=>e.completed&&e.end>after&&(
  kind==='all'||kind==='transfer'&&e.transferred>0||kind==='zero'&&e.transferred===0||
  kind==='request-leave'&&e.actions.join('/')==='Request/Leave'||kind==='request-refuse'&&e.actions.join('/')==='Request/Refuse'||
  kind==='other'&&!['Request/Leave','Request/Refuse'].includes(e.actions.join('/'))
 )).map(e=>({tick:e.end,ref:e.ref,anchor:e.anchor,description:`Scene ${e.id} · ${e.participants.join('→')} · ${e.actions.join(' → ')} · ${e.outcome} · transferred ${e.transferred}`}));
}
function orderedChainNodes(causal,chain,timeline) {
 const order=new Map(timeline.map(m=>[m.ref,m.order]));
 const phase=n=>n.source_kind==='Completed resource encounter'?2:n.source_kind==='Recipient response'||n.source_kind==='Recorded interpretation'?1:0;
 return [...new Set([...chain.nodes,...chain.edges.map(e=>e.source)])].sort((a,b)=>{
  const x=causal.nodes[a],y=causal.nodes[b];return (x.tick??0)-(y.tick??0)||(order.get(x.anchor)??-1)-(order.get(y.anchor)??-1)||phase(x)-phase(y);
 });
}
if(typeof module!=='undefined')module.exports={originalValue,resourceRecords,orderedChainNodes};
