'use strict';
// Exact own component at an explicit captured point. No nearest-time lookup,
// audit-prefix reconstruction, partner state, policy evaluation or new knowledge.
function capturedOwnedPoint(trial, point, owner) {
 if(!Number.isInteger(point)||point<0)return null;
 const capture=trial.points[point];
 const agent=capture?.state?.agents?.find(a=>a.id===owner);
 if(!agent)return null;
 const own={};
 for(const key of ['id','food','hunger','generosity','caution','expectation','trust','memories']) {
  if(Object.prototype.hasOwnProperty.call(agent,key))own[key]=JSON.parse(JSON.stringify(agent[key]));
 }
 return {phase:'owned snapshot',own,now:capture.state.tick,source_point:point};
}
if(typeof module!=='undefined')module.exports={capturedOwnedPoint};
