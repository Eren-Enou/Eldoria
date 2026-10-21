'use strict';
// Native022 presentation. Policy inputs stay distinct from observer joins.
const NativeConcurrent = (() => {
 const is = data => ['022','023','024','025'].includes(data?.source?.experiment) && data.native_concurrency === true;
 function resolve(root, path) {
  const parts = path.match(/[A-Za-z_][A-Za-z_0-9]*|\[\d+\]/g) || [];
  const rebuilt = parts.map((part,index) => part.startsWith('[') ? part : (index ? '.' : '') + part).join('');
  if(rebuilt !== path)throw new Error('Unsupported native original reference');
  let value=root;
  for(const part of parts){const key=part.startsWith('[')?Number(part.slice(1,-1)):part;if(value===null||typeof value!=='object'||!Object.prototype.hasOwnProperty.call(value,key))throw new Error('Missing native original reference');value=value[key];}
  return value;
 }
 function resolveData(data,ref){if(ref.startsWith('material.')&&data.preparation)return resolve(data.preparation,ref);if(ref.startsWith('valuations[')&&data.valuation)return resolve(data.valuation,ref);return ref.startsWith('checkpoints[')&&data.continuous?resolve(data.continuous,ref):resolve(data.trial,ref);}
 function sourceRef(data,ref){return data.continuous&&!ref.startsWith('material.')&&!ref.startsWith('checkpoints[')&&!ref.startsWith('valuations[')?'history.'+ref:ref;}
 function project(data,moment,owner) {
  if(!Number.isInteger(owner)||owner<0||owner>3)return null;
  const trial=data.trial, ref=moment.ref;
  if(moment.kind==='decision'){
   const boundary=resolve(trial,ref), index=boundary.choices.findIndex(d=>d.input.legacy.own.id===owner);
   if(index>=0){const d=boundary.choices[index],view={...d,phase:'activity',ref:`${ref}.choices[${index}]`,own:d.input.legacy.own};
    if(data.valuation){const bi=Number(ref.match(/^boundaries\[(\d+)\]$/)[1]),vi=data.valuation.valuations.findIndex(v=>v.boundary===bi&&v.choice===index);if(vi<0)throw new Error('Missing owned024score audit');view.valuation=data.valuation.valuations[vi];view.valuation_ref=`valuations[${vi}]`;}
    return view;}
  }else if(moment.kind==='reply'){
   const r=resolve(trial,ref);if(r.input.own.own.id===owner)return {...r,phase:'recipient',ref,own:r.input.own.own};
  }else if(moment.kind==='resource'){
   const d=resolve(trial,ref).decision;if(d.actor===owner)return {...d,phase:'resource',ref:ref+'.decision'};
  }else if(moment.kind==='inquiry'){
   const d=resolve(trial,ref).decision;if(d.input.owner===owner)return {...d,phase:'inquiry selection',ref:ref+'.decision',input_tick:moment.input_tick};
  }else if(data.preparation&&moment.kind==='raw_memory'){
   const receipt=resolve(data.preparation,ref);if(receipt.owner===owner)return {phase:'owned raw receipt',ref,owner,raw_retained:receipt.after,receipt_kind:receipt.kind};
  }else if(moment.kind==='snapshot'){
   const own=resolve(trial,ref)[owner], view={phase:'owned snapshot',ref:`${ref}[${owner}]`,own};
   if(ref==='final_agents'){view.retained=trial.final_memory[owner];if(data.preparation)view.raw_retained=data.preparation.material.final_slots[owner];}return view;
  }
  return null;
 }
 function original(env,ref){
  const {S,go}=env;resolveData(S.data,ref);S.lens='observer';S.causalNode=ref;
  S.chain=S.data.causal.chains.find(ch=>ch.nodes.includes(ref))?.id??null;
  const node=S.data.causal.nodes[ref], fallback=ref.match(/^boundaries\[\d+\]/)?.[0];
  const index=S.data.timeline.findIndex(m=>m.ref===(node?.anchor??fallback??ref));
  if(index>=0)go(index);else renderCausal(env);
 }
 function graphButton(env,host,label,ref){return env.button(host,label,()=>original(env,ref));}
 function lanes(env,moment){
  const {S,$,el,note}=env, data=S.data,t=data.trial,host=$('concurrent-lanes');host.replaceChildren();host.hidden=S.lens!=='observer';
  if(host.hidden)return;
  host.append(el('h2',`Four individuals · original ${data.source.record} · world tick ${moment.tick}`));
  if(data.continuous)note(host,`Experiment${data.source.experiment} · ${data.valuation?data.valuation.config.valuation+" valuation · ":""}${data.continuous.config.mode} opportunities · reporting period ${data.continuous.config.period}`);
  note(host,'Actual participant reservations from start to release. Blank lanes can still be blocked by an exclusive inquiry.');
  const ns='http://www.w3.org/2000/svg',svg=document.createElementNS(ns,'svg');svg.setAttribute('viewBox','0 0 1000 235');svg.setAttribute('role','img');svg.setAttribute('aria-label','Four concurrent individual lanes on one shared world-time axis');
  const scale=870/Math.max(1,t.end-t.start),x=tick=>110+(tick-t.start)*scale;
  function node(tag,attrs,text){const e=document.createElementNS(ns,tag);for(const [key,value]of Object.entries(attrs))e.setAttribute(key,value);if(text!==undefined)e.textContent=text;svg.append(e);return e;}
  const colors={Resource:'#8fd2af',Claim:'#d5b57b',Meal:'#9bb9e5',Query:'#bea5e5',Send:'#426d73',Reply:'#789a9b',Prepare:'#e5a976'};
  for(let owner=0;owner<4;owner++){
   node('text',{x:4,y:40+owner*43,fill:'#dce9e2','font-size':14},`Individual ${owner}`);
   node('line',{x1:110,x2:980,y1:35+owner*43,y2:35+owner*43,stroke:'#344c50','stroke-width':24});
   for(const c of data.intervals.filter(c=>c.owners.includes(owner))){
    const r=node('rect',{x:x(c.start),y:23+owner*43,width:Math.max(2,(c.end-c.start)*scale),height:24,rx:2,fill:colors[c.kind],tabindex:0,role:'button','aria-label':`${c.kind} ${c.id}, individuals ${c.owners.join(' / ')}, ${c.start} to ${c.end}, actual release`});
    const title=document.createElementNS(ns,'title');title.textContent=`${c.kind} #${c.id} · ${c.start}–${c.end} · ${c.start_ref} → ${c.release_ref}`;r.append(title);r.onclick=()=>original(env,c.start_ref);r.onkeydown=e=>{if(e.key==='Enter')r.onclick();};
   }
   for(const [index,receipt]of t.memories.entries())if(receipt.owner===owner&&receipt.kind==='Sense'){
    const dot=node('circle',{cx:x(receipt.tick),cy:52+owner*43,r:2.5,fill:'#f7e9b9',tabindex:0,role:'button','aria-label':`Individual ${owner}, actual observation at ${receipt.tick}`});dot.onclick=()=>original(env,`memories[${index}]`);dot.onkeydown=e=>{if(e.key==='Enter')dot.onclick();};
   }
  }
  const step=Math.max(4,Math.ceil((t.end-t.start)/12/4)*4);
  for(let tick=t.start;tick<=t.end;tick+=step)node('text',{x:x(tick),y:225,fill:'#b7cfca','font-size':12},tick);
  if(data.continuous)for(const [index,checkpoint]of data.continuous.checkpoints.entries()){
   const nominal=checkpoint.labels.includes('NominalStop');node('line',{x1:x(checkpoint.tick),x2:x(checkpoint.tick),y1:8,y2:205,stroke:nominal?'#ec927b':'#719a9c','stroke-width':nominal?2:1,'stroke-dasharray':nominal?'2 2':'4 4'});const mark=node('rect',{x:x(checkpoint.tick)-5,y:8,width:10,height:197,fill:'transparent',tabindex:0,role:'button','aria-label':`${checkpoint.labels.join(' / ')} checkpoint at ${checkpoint.tick}; ${checkpoint.active.length} active commitments`});
   const title=document.createElementNS(ns,'title');title.textContent=`${checkpoint.labels.join(' / ')} @ ${checkpoint.tick} · checkpoints[${index}]`;mark.append(title);mark.onclick=()=>original(env,`checkpoints[${index}]`);mark.onkeydown=e=>{if(e.key==='Enter')mark.onclick();};
  }
  node('line',{x1:x(moment.tick),x2:x(moment.tick),y1:8,y2:205,stroke:'#f3d98f','stroke-width':2});const axis=el('div',undefined,'concurrent-axis');axis.append(svg);host.append(axis);
  if(data.preparation)for(const [index,receipt]of data.preparation.material.memories.entries())if(['Sense','Insufficient'].includes(receipt.kind)){
   const dot=node('circle',{cx:x(receipt.tick),cy:52+receipt.owner*43,r:3,fill:'#e5a976',tabindex:0,role:'button','aria-label':`Individual ${receipt.owner}, actual raw observation at ${receipt.tick}`});dot.onclick=()=>original(env,`material.memories[${index}]`);dot.onkeydown=e=>{if(e.key==='Enter')dot.onclick();};
  }
  note(host,'Green: resource · sand: claim · blue: meal · purple: inquiry · teal: send/reply. Dots mark observations. Click an interval for its original; scroll the timeline horizontally when needed.');
  if(data.preparation)note(host,'Orange: finite preparation, including paid failed attempts. Extraction and conversion are distinct actual phases; no automatic meal or sharing follows completion.');
  if(data.continuous)note(host,`Dashed lines: observer reporting checkpoints; red: nominal cutoff ${t.start+data.continuous.config.horizon}. Existing work drains with real time/needs to ${t.end}; no new ordinary choices after cutoff. Checkpoint markers open exact original references.`);
  const active=data.intervals.filter(c=>c.start<=moment.tick&&moment.tick<c.end);
  note(host,`After starts at tick ${moment.tick}: ${active.filter(c=>c.kind==='Resource').length} social scenes; ${active.length} physical commitments. Local input cards retain their original pre-reservation availability.`);
 }
 function ownInput(env,host,view){
  const {heading,note,table,details,human}=env;
  if(!view){note(host,'No owned decision input is captured for this individual at this record. Busy time, another person’s choice, or an atomic interior is not filled from observer history.');return;}
  heading(host,`${view.phase} · original ${view.ref}`);
  function raw(cell){
   if(!cell){note(host,'Raw source cognition unavailable / unknown, not zero. No previous receipt is restored from observer history.');return;}
   const o=cell.observation;note(host,`Own raw receipt ${cell.receipt}: source ${o.id}, observed at ${o.observed_at}; remembered stock ${o.stock}; own removals ${cell.own_takes}; retained raw upper bound ${Math.max(0,o.stock-cell.own_takes)}. Current shared stock is excluded.`);
   table(host,['Own observed preparation affordance','Captured value'],[['Accessible',o.accessible?'Yes':'No'],['Extraction / preparation ticks',`${o.extraction_ticks} / ${o.preparation_ticks}`],['Known conversion',`${o.input_units} raw → ${o.edible_units} food + ${o.waste_units} waste`]]);
   details(host,'Own retained raw source cell',cell);
  }
  if(Object.prototype.hasOwnProperty.call(view,'raw_retained'))raw(view.raw_retained);
  if(view.phase==='owned raw receipt'){note(host,`Actual owned slot transition: ${view.receipt_kind}. Only the after-state is shown; evicted before-state is not recovered.`);return;}
  if(view.own){
   table(host,['Owned state','Captured value'],[['Individual',view.own.id],['Food',view.own.food],['Hunger',view.own.hunger],['Generosity / caution / expectation',`${view.own.generosity} / ${view.own.caution} / ${view.own.expectation}`]]);
   details(host,'Own directed trust',view.own.trust);details(host,'Own retained episodes',view.own.memories);
  }
  if(view.phase==='activity'){
   const cell=view.input.retained,legacy=view.input.legacy;
   if(Object.prototype.hasOwnProperty.call(view.input,'raw_retained'))raw(view.input.raw_retained);
   const tagged=Object.prototype.hasOwnProperty.call(view.input,'raw_retained');
   note(host,cell?`${tagged?'Own ready-cache':'Own'} receipt ${cell.receipt}: stock ${cell.observation.stock} observed at ${cell.observation.observed_at}; own takes ${cell.own_takes}; retained upper bound ${Math.max(0,cell.observation.stock-cell.own_takes)}. Current observer stock is excluded.`:tagged?'Ready-cache observation unavailable / unknown, not zero.':'Environmental observation unavailable / unknown, not zero.');
   table(host,['Public individual','Captured availability'],legacy.people.map(p=>[p.id,p.available?'Available':'Busy']));
   table(host,['Activity','Score','Eligible'],view.candidates.map(c=>[c.action,c.score??c.final_score,c.eligible?'Yes':'No']));
   if(view.valuation){
    const prepare=view.valuation.selection.terms.some(t=>t.action==='Prepare');
    note(host,prepare?`Captured local Prepare = own H minus known total work ticks. Old four activity terms and stable old-first ties are preserved. Own H=${view.own.hunger}. Source ${view.valuation_ref}.`:`Captured ${view.valuation.policy} rule: eligible Consume = H${view.valuation.policy==='MealRelief'?' + min(H,40)':''}; other scores unchanged. Own H=${view.own.hunger}. Stable first maximum: Wait, Consume, Invite, Acquire. Source ${view.valuation_ref}.`);
    table(host,prepare?['Activity','Base','Meal relief','Work cost','Final','Eligible']:['Activity','Base','Meal relief','Final','Eligible'],view.valuation.selection.terms.map(t=>prepare?[t.action,t.base_score,t.meal_relief,t.work_effort,t.final_score,t.eligible?'Yes':'No']:[t.action,t.base_score,t.meal_relief,t.final_score,t.eligible?'Yes':'No']));
    details(host,'Own captured valuation terms',view.valuation);
   }
   table(host,['Contact candidate','Score','Eligible'],view.contact_candidates.map(c=>[human(c.intent),c.score,c.eligible?'Yes':'No']));
   const q=legacy.query.input.base;details(host,'Own concerns and encounter context',{context:q.scene,concerns:q.concerns});
   details(host,'Own captured inquiry state',legacy.query);
   note(host,`Exclusive inquiry lane ${view.input.inquiry_lane_free?'available':'blocked'}. A local query candidate is not an executed inquiry.`);
  }else if(view.phase==='recipient'){
   table(host,['Recipient alternative','Score'],view.candidates.map(c=>[human(c.reply),c.score]));
   details(host,'Actually received invitations',view.input.incoming);note(host,'Environmental quantity is not captured in this reply input; no earlier slot is backfilled. An Accept reply is provisional consent, not a completed resource agreement.');
  }else if(view.phase==='resource'){
   table(host,['Own captured resource input','Value'],[['Food / hunger',`${view.food} / ${view.hunger}`],['Directed trust to observed partner',view.trust],['Last-four retained episodic valence',view.episodic_valence]]);
   table(host,['Legal action','Score'],view.candidates.map(c=>[c.action,c.score]));details(host,'Public observation',view.observation);details(host,'Retained memory cause IDs',view.memory_causes);
   note(host,'The resource decision captures its own policy inputs; full environment and partner inventory are not imported.');
  }else if(view.phase==='inquiry selection'){
   note(host,`Owned inquiry selection input at tick ${view.input_tick}; the selected timeline record marks completion. Partner-private response decision context is excluded.`);
   details(host,'Own inquiry input',view.input);table(host,['Inquiry alternative','Score'],view.candidates.map(c=>[human(c.action),c.score]));
  }else{
   note(host,'Owned state snapshot, not a new decision.');
   if(Object.prototype.hasOwnProperty.call(view,'retained'))details(host,'Own final environmental slot',view.retained);
  }
  if(view.selected!==undefined)note(host,`Selected: ${human(view.selected)}.`);
 }
 function renderCausal(env){
  const {S,$,el,note,heading,details,button}=env,data=S.data,m=data.timeline[S.index],panel=$('causal-browser');
  panel.hidden=S.lens!=='observer';
  for(const id of ['causal-chain','causal-node','encounter-results','personal-summary'])$(id).replaceChildren();
  if(panel.hidden){$('chain-select').replaceChildren();return;}
  const select=$('chain-select');select.replaceChildren();const empty=el('option','Select a recorded activity cohort');empty.value='';select.append(empty);
  for(const chain of data.causal.chains){const option=el('option',chain.label);option.value=chain.id;select.append(option);}
  const chain=data.causal.chains.find(ch=>ch.id===S.chain)??data.causal.chains.find(ch=>ch.nodes.includes(m.ref));select.value=chain?.id??'';
  if(chain){S.chain=chain.id;for(const ref of chain.nodes)graphButton(env,$('causal-chain'),data.causal.nodes[ref]?.label??ref,ref);}else note($('causal-chain'),'Choose an original invitation or physical-work cohort. No causal chain is inferred merely from nearby timestamps.');
  note($('causal-chain'),data.causal.note);
  const ref=S.causalNode??m.ref;heading($('causal-node'),'Exact original record');note($('causal-node'),`${data.source.id} · original zero-based trial ${data.source.record} · ${sourceRef(data,ref)}`);details($('causal-node'),'Original recorded values',resolveData(data,ref));
  const filter=$('encounter-filter').value,after=Number($('encounter-after').value),rows=[];
  if(['unexecuted','declined','reciprocal'].includes(filter)){
   for(const [index,a]of data.trial.admissions.entries())if(a.tick>after&&(filter==='unexecuted'?!['Execute','Reciprocal'].includes(a.reason):a.reason===(filter==='declined'?'Declined':'Reciprocal')))rows.push({ref:`admissions[${index}]`,label:`${a.tick} · ${a.invitation.proposer} → ${a.invitation.recipient}: ${a.reason}`});
  }else{
   let base=data.trial.final_state;while(!base.events)base=base.base;
   for(const c of data.intervals.filter(c=>c.kind==='Resource'&&c.end>after)){
    const events=base.events.filter(e=>e.scene===c.scene),transfers=events.reduce((n,e)=>n+e.transferred,0),actions=events.map(e=>e.decision.selected).join('/');
    const matches=filter==='all'||filter==='transfer'&&transfers>0||filter==='zero'&&transfers===0||filter==='request-leave'&&actions==='Request/Leave'||filter==='request-refuse'&&actions==='Request/Refuse'||filter==='other'&&!['Request/Leave','Request/Refuse'].includes(actions);
    if(matches)rows.push({ref:c.start_ref,label:`${c.start}–${c.end} · scene ${c.scene}, individuals ${c.owners.join(' / ')} · ${actions} · transferred ${transfers}`});
   }
  }
  $('encounter-count').textContent=`${rows.length} matching original records · showing up to 80. Completed scenes and unexecuted invitations are separate.`;
  for(const row of rows.slice(0,80)){const item=el('li');graphButton(env,item,row.label,row.ref);$('encounter-results').append(item);}
  const own=data.intervals.filter(c=>c.owners.includes(S.actor));heading($('personal-summary'),`Individual ${S.actor} · recorded occupancy`);note($('personal-summary'),`${own.length} participant commitments, ${own.reduce((n,c)=>n+c.end-c.start,0)} occupied ticks. Unreserved time is not necessarily voluntary rest; exclusive inquiry may block decisions.`);
 }
 function render(env){
  const {S,$,el,note,heading,table,details,human}=env,d=S.data,t=d.trial,m=d.timeline[S.index],observer=S.lens==='observer';
  $('title').textContent=t.config.case.replaceAll('_',' ');$('mode').textContent=d.preparation?`${d.source.mode} · finite preparation · reporting period ${d.continuous.config.period}`:d.valuation?`${d.valuation.config.valuation} valuation · Continuous opportunities · reporting period ${d.continuous.config.period}`:d.continuous?`${d.continuous.config.mode} opportunities · reporting period ${d.continuous.config.period}`:`Social capacity ${t.config.capacity}`;
  $('identity').textContent=`Experiment ${d.source.experiment} · original zero-based trial ${d.source.record} · seed ${d.source.seed} · ticks ${t.start}–${t.end}`;
  $('configuration').textContent=d.continuous?`Observer metadata: ${d.continuous.config.phase} physical phase; nominal stop ${t.start+d.continuous.config.horizon}, charged drain to ${t.end}. Reporting checkpoints do not refresh cognition or reset the world. Original references use history.*; checkpoints[*] belong to the original ${d.source.experiment} envelope.${d.valuation?' Score terms use original valuations[*] references.':''} ${d.source.scientific_status}`:`Observer metadata: ${t.config.order} order; claim ${t.config.duration}; passive cadence ${t.config.cadence}; need +5 per 4 ticks from comparison start. This original finite-window research does not establish continuous opportunities or absolute-phase invariance.`;
  $('source').textContent=JSON.stringify(d.source,null,2);$('scrub').max=d.timeline.length-1;$('scrub').value=S.index;$('tick').value=m.tick;$('previous').disabled=S.index===0;$('next').disabled=S.index===d.timeline.length-1;
  $('actor-lens').setAttribute('aria-pressed',!observer);$('observer-lens').setAttribute('aria-pressed',observer);
  $('concurrent-link').hidden=!observer;
  $('perspective-panel').classList.toggle('observer',observer);
  $('moment-kind').textContent=`WORLD TICK ${m.tick} · ${m.kind.toUpperCase()} · MOMENT ${S.index+1}/${d.timeline.length}`;
  const view=project(d,m,S.actor);$('moment-title').textContent=observer?m.label:view?`Individual ${S.actor} · ${view.phase}${view.selected===undefined?'':': '+human(view.selected)}`:'No owned decision input at this record';$('moment-ref').textContent=sourceRef(d,m.ref);
  for(const id of ['cards','outcome','inspect','links'])$(id).replaceChildren();$('cards').classList.toggle('concurrent-cards',observer);$('cards').classList.toggle('concurrent-owned',!observer);
  $('lens-note').textContent=observer?'Objective recorded history and typed causal navigation. Physical vectors are reconstructed from actual operations; they never become actor knowledge.':'Only this individual’s original captured input or explicit owned snapshot. No observer stock, private response context, nearest-state interpolation or archive recovery.';
  if(observer){
   const point=d.observer_physical.find(p=>p.tick===m.tick);note($('outcome'),'Observer balances after all recorded physical operations at this world tick. The selected record retains its own exact before/after context.');
   if(point){for(let owner=0;owner<4;owner++){const card=el('article');card.append(el('h3',`Individual ${owner}`));note(card,`Food ${point.food[owner]} · hunger ${point.hunger[owner]}`);$('cards').append(card);}note($('outcome'),`Actual cache stock ${point.stock}; consumed ${point.consumed}.`);}
   if(d.preparation&&point){
    table($('outcome'),['Common-unit compartment','Actual after this tick'],[['Raw source0',point.raw],['Owned raw escrow',point.escrow.join(' / ')],['Ready cache0',point.stock],['Owned food total',point.food.reduce((a,b)=>a+b,0)],['Inert waste',point.waste],['Consumed',point.consumed],['Conserved total',point.raw+point.escrow.reduce((a,b)=>a+b,0)+point.stock+point.food.reduce((a,b)=>a+b,0)+point.waste+point.consumed]]);
    note($('outcome'),'Finite2raw → 1food + 1waste. Actual shared balances are observer-only; the actor may still retain an older upper bound.');
    const pointIndex=d.preparation.material.points.findLastIndex(p=>p.tick<=m.tick),materialPoint=d.preparation.material.points[pointIndex];
    if(materialPoint){
     table($('outcome'),['Active preparation','Owner','Actual phase','Extraction','Finish','Raw escrow'],materialPoint.active.map(p=>[p.process.commitment,p.process.owner,p.phase,p.process.extract_at,p.process.finish_at,p.escrow]));
     details($('outcome'),`Original material.points[${pointIndex}] · tick ${materialPoint.tick}`,materialPoint);
    }

    for(const span of d.intervals.filter(c=>c.kind==='Prepare'&&c.end>=m.tick).slice(0,6))graphButton(env,$('links'),`Prepare #${span.id} · individual ${span.owners[0]} · paid ${span.start}–${span.end}`,span.start_ref);
    for(const {index,op}of d.preparation.material.operations.map((op,index)=>({op,index})).filter(x=>x.op.tick>=m.tick).slice(0,6))graphButton(env,$('links'),`${op.outcome} · tick ${op.tick} · individual ${op.owner}`,`material.operations[${index}]`);
   }
   const active=d.intervals.filter(c=>c.start<=m.tick&&m.tick<c.end);table($('outcome'),['Actual commitment','Participants','Start','Release','Bound'],active.map(c=>[`${c.kind} #${c.id}`,c.owners.join(' / '),c.start,c.end,c.deadline]));
   if(d.valuation){
    heading($('links'),'Recorded activity evidence');
    const deferred=(d.valuation_deferrals??[]).filter(x=>x.tick>=m.tick);
    note($('links'),`${deferred.length} later eligible meal deferrals with actual time spent; showing the next 6. Completion hunger is an observer value after all operations at that tick.`);
    for(const x of deferred.slice(0,6))graphButton(env,$('links'),`Deferred eligible meal · tick ${x.tick} · individual ${x.owner} · food ${x.food}, H${x.hunger} · ${x.selected} until ${x.end}, H${x.hunger_after_completion_tick}`,x.ref);
    const b=t.final_state.base.base.base.base.base;
    const helpful=b.events.map((e,i)=>({e,i})).filter(({e,i})=>i>=t.bootstrap_events&&e.transferred&&e.tick>=m.tick).slice(0,3);
    for(const {e,i}of helpful)graphButton(env,$('links'),`Actual transfer · tick ${e.tick} · individuals ${e.participants.join(' / ')} · ${e.transferred} unit`, `final_state.base.base.base.base.base.events[${i}]`);
    for(const q of d.timeline.filter(q=>q.kind==='inquiry'&&q.tick>=m.tick).slice(0,2))graphButton(env,$('links'),`Actual inquiry completion · tick ${q.tick}`,q.ref);
   }
   details($('inspect'),'Selected original record',resolveData(d,m.ref));graphButton(env,$('links'),'Inspect exact original '+m.ref,m.ref);$('raw').textContent=JSON.stringify(resolveData(d,m.ref),null,2);
  }else{
   S.causalNode=null;ownInput(env,$('inspect'),view);note($('cards'),view?'Owned captured context; another person’s state is excluded.':'No replacement state is inferred from other records.');$('raw').textContent=JSON.stringify(view,null,2);
   if(view)note($('links'),`Exact owned source: ${view.ref}`);
  }
  lanes(env,m);renderCausal(env);
 }
 function renderPair(env){
  const {S,$,el,note,table,details}=env,c=S.comparison,host=$('compare-summary'),pair=$('pair');host.replaceChildren();pair.replaceChildren();host.hidden=S.lens==='actor';$('inspect-right').disabled=false;
  table(host,['Comparison scope','Result'],[['Same physical start/profiles',c.initial_state_and_profiles_match?'Yes':'NO'],[c.preparation_treatment?'Matched finite-preparation access treatment':c.valuation_treatment?'Matched activity-valuation treatment':c.opportunity_treatment?'Matched public-opportunity treatment':'Only capacity differs in configuration',c.preparation_treatment?(c.matched_except_access&&c.access_contrast?`${c.left_mode} / ${c.right_mode}`:'NO — not a matched access pair'):c.valuation_treatment?(c.matched_except_valuation&&c.valuation_contrast?`${c.left_mode} / ${c.right_mode}`:'NO — not a matched valuation pair'):c.opportunity_treatment?(c.matched_except_mode&&c.mode_contrast?`${c.left_mode} / ${c.right_mode}`:'NO — not a matched opportunity pair'):(c.matched_except_capacity&&c.capacity_contrast?'Yes':'NO — not a matched capacity treatment')],['Same evaluation endpoints',c.equal_horizon?'Yes':'NO'],['First admission difference',c.first_admission?.tick??'None'],['First executed difference',c.first_executed?.tick??'None'],['First missing ordinary decision opportunity',c.first_opportunity?.tick??'None'],['First different selected choice at a shared actor/tick',c.first_common_selected_choice?.tick??'None'],[c.preparation_treatment?'First edible-holdings difference (owned + ready)':'First food-ledger difference',c.first_material??'None'],[c.preparation_treatment?'Later edible-holdings reconvergence':'Later food-ledger reconvergence',c.material_reconvergence??'None'],[c.preparation_treatment?'Terminal edible holdings':'Terminal food ledger',c.terminal_food_equal?'Equal':'Different'],['Terminal hunger',c.terminal_hunger_equal?'Equal':'Different'],['Terminal directed trust (missing = 0)',c.semantic_trust_equal?'Equal':'Different'],[c.preparation_treatment?'Captured physical/cognitive history equality (includes received access flags)':c.valuation_treatment?'Complete physical/history body equality':c.opportunity_treatment?'Full state/choice trace equal apart from projected Input.end':'Complete trial equality except capacity',((c.opportunity_treatment||c.valuation_treatment||c.preparation_treatment)?c.full_state_trace_null:c.complete_null_except_capacity)?'Yes':'No'],['Hunger-area right minus left',c.hunger_area_right_minus_left.join(' / ')]]);
  if(c.preparation_treatment)table(host,['Finite material comparison','Result'],[['Same fixed valuation policy',c.same_valuation_policy?'Yes':'NO'],['Same initial received knowledge except declared access flag',c.same_initial_knowledge_except_access?'Yes':'NO'],['First completed conversion difference',c.first_conversion_difference??'None'],['First actual meal difference',c.first_meal_difference??'None'],['First interpersonal event difference',c.first_social_event_difference??'None'],['First raw/escrow/food/waste/consumed difference',c.first_full_material??'None'],['Later full-material reconvergence',c.full_material_reconvergence??'None'],['Terminal full material',c.terminal_full_material_equal?'Equal':'Different']]);
  note(host,c.note);details(host,'Actual material/contact counts',{left:c.left_metrics,right:c.right_metrics});
  if(c.valuation_treatment&&c.first_score_difference){
   const f=c.first_score_difference;note(host,`First different score: tick ${f.tick}, individual ${f.owner}; exact local inputs ${f.same_local_input?'match':'do not match'}. A score difference may leave the selected activity unchanged.`);
   for(const [side,data]of [['left',S.data],['right',S.right]]){
    const score=f[side],id=data.source.id;table(host,[`${side} rule`,'Activity','Base','Meal relief','Final'],score.audit.selection.terms.map(t=>[score.audit.policy,t.action,t.base_score,t.meal_relief,t.final_score]));
    env.button(host,`Inspect ${side} original score terms · ${score.audit_ref}`,env.guard(async()=>{if(S.data.source.id!==id)await env.openHistory(id);if(S.data.source.id===id)original(env,score.audit_ref);}));
   }
  }

  for(const [label,difference]of [['admission',c.first_admission],['execution',c.first_executed]])if(difference){
   for(const [side,data]of [['left',S.data],['right',S.right]]){
    const record=difference[side+'_difference'];if(!record)continue;
    const id=data.source.id,ref=record.ref;
    env.button(host,`Inspect ${side} first ${label} difference · ${ref}`,env.guard(async()=>{if(S.data.source.id!==id)await env.openHistory(id);if(S.data.source.id===id)original(env,ref);}));
   }
  }
  details(host,'Final recorded directed trust',{left:S.data.trial.final_agents.map(a=>({owner:a.id,trust:a.trust})),right:S.right.trial.final_agents.map(a=>({owner:a.id,trust:a.trust})),raw_map_shape_equal:c.raw_trust_equal});
  const tick=S.data.timeline[S.index].tick;
  for(const [data,points]of [[S.data,c.left_physical],[S.right,c.right_physical]]){
   const col=el('section');col.append(el('h3',data.preparation?`Original ${data.source.record} · ${data.source.mode} · ${data.valuation.config.valuation}`:data.valuation?`Original ${data.source.record} · ${data.valuation.config.valuation}`:data.continuous?`Original ${data.source.record} · ${data.continuous.config.mode}`:`Original ${data.source.record} · capacity ${data.trial.config.capacity}`));note(col,`Shared world tick ${tick}; source ${data.source.id}`);
   if(S.lens==='observer'){const p=points.find(p=>p.tick===tick);if(p){table(col,['Individual','Food','Hunger'],p.food.map((food,owner)=>[owner,food,p.hunger[owner]]));note(col,`Cache ${p.stock}; consumed ${p.consumed}. Physical reconstruction only; no cognitive interpolation.`);if(data.preparation)note(col,`Raw ${p.raw}; escrow ${p.escrow.join(' / ')}; waste ${p.waste}. These shared compartments are observer-only.`);}else note(col,'This tick is outside this recorded world.');}
   else {const moment=data.timeline.find(m=>m.kind==='decision'&&m.tick===tick),view=moment?project(data,moment,S.actor):null;ownInput(env,col,view);}
   pair.append(col);
  }
 }
 return {is,resolve,resolveData,sourceRef,project,original,render,renderCausal,renderPair};
})();
if(typeof module!=='undefined')module.exports=NativeConcurrent;
