'use strict';
// Native022 presentation. Policy inputs stay distinct from observer joins.
const NativeConcurrent = (() => {
 const is = data => data?.source?.experiment === '022' && data.native_concurrency === true;
 function resolve(root, path) {
  const parts = path.match(/[A-Za-z_][A-Za-z_0-9]*|\[\d+\]/g) || [];
  const rebuilt = parts.map((part,index) => part.startsWith('[') ? part : (index ? '.' : '') + part).join('');
  if(rebuilt !== path)throw new Error('Unsupported native original reference');
  let value=root;
  for(const part of parts){const key=part.startsWith('[')?Number(part.slice(1,-1)):part;if(value===null||typeof value!=='object'||!Object.prototype.hasOwnProperty.call(value,key))throw new Error('Missing native original reference');value=value[key];}
  return value;
 }
 function project(data,moment,owner) {
  if(!Number.isInteger(owner)||owner<0||owner>3)return null;
  const trial=data.trial, ref=moment.ref;
  if(moment.kind==='decision'){
   const boundary=resolve(trial,ref), index=boundary.choices.findIndex(d=>d.input.legacy.own.id===owner);
   if(index>=0){const d=boundary.choices[index];return {...d,phase:'activity',ref:`${ref}.choices[${index}]`,own:d.input.legacy.own};}
  }else if(moment.kind==='reply'){
   const r=resolve(trial,ref);if(r.input.own.own.id===owner)return {...r,phase:'recipient',ref,own:r.input.own.own};
  }else if(moment.kind==='resource'){
   const d=resolve(trial,ref).decision;if(d.actor===owner)return {...d,phase:'resource',ref:ref+'.decision'};
  }else if(moment.kind==='inquiry'){
   const d=resolve(trial,ref).decision;if(d.input.owner===owner)return {...d,phase:'inquiry selection',ref:ref+'.decision',input_tick:moment.input_tick};
  }else if(moment.kind==='snapshot'){
   const own=resolve(trial,ref)[owner], view={phase:'owned snapshot',ref:`${ref}[${owner}]`,own};
   if(ref==='final_agents')view.retained=trial.final_memory[owner];return view;
  }
  return null;
 }
 function original(env,ref){
  const {S,go}=env;resolve(S.data.trial,ref);S.lens='observer';S.causalNode=ref;
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
  note(host,'Actual participant reservations from start to release. Blank lanes can still be blocked by an exclusive inquiry.');
  const ns='http://www.w3.org/2000/svg',svg=document.createElementNS(ns,'svg');svg.setAttribute('viewBox','0 0 1000 235');svg.setAttribute('role','img');svg.setAttribute('aria-label','Four concurrent individual lanes on one shared world-time axis');
  const scale=870/Math.max(1,t.end-t.start),x=tick=>110+(tick-t.start)*scale;
  function node(tag,attrs,text){const e=document.createElementNS(ns,tag);for(const [key,value]of Object.entries(attrs))e.setAttribute(key,value);if(text!==undefined)e.textContent=text;svg.append(e);return e;}
  const colors={Resource:'#8fd2af',Claim:'#d5b57b',Meal:'#9bb9e5',Query:'#bea5e5',Send:'#426d73',Reply:'#789a9b'};
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
  node('line',{x1:x(moment.tick),x2:x(moment.tick),y1:8,y2:205,stroke:'#f3d98f','stroke-width':2});const axis=el('div',undefined,'concurrent-axis');axis.append(svg);host.append(axis);
  note(host,'Green: resource · sand: claim · blue: meal · purple: inquiry · teal: send/reply. Dots mark observations. Click an interval for its original; scroll the timeline horizontally when needed.');
  const active=data.intervals.filter(c=>c.start<=moment.tick&&moment.tick<c.end);
  note(host,`After starts at tick ${moment.tick}: ${active.filter(c=>c.kind==='Resource').length} social scenes; ${active.length} physical commitments. Local input cards retain their original pre-reservation availability.`);
 }
 function ownInput(env,host,view){
  const {heading,note,table,details,human}=env;
  if(!view){note(host,'No owned decision input is captured for this individual at this record. Busy time, another person’s choice, or an atomic interior is not filled from observer history.');return;}
  heading(host,`${view.phase} · original ${view.ref}`);
  if(view.own){
   table(host,['Owned state','Captured value'],[['Individual',view.own.id],['Food',view.own.food],['Hunger',view.own.hunger],['Generosity / caution / expectation',`${view.own.generosity} / ${view.own.caution} / ${view.own.expectation}`]]);
   details(host,'Own directed trust',view.own.trust);details(host,'Own retained episodes',view.own.memories);
  }
  if(view.phase==='activity'){
   const cell=view.input.retained,legacy=view.input.legacy;
   note(host,cell?`Own receipt ${cell.receipt}: stock ${cell.observation.stock} observed at ${cell.observation.observed_at}; own takes ${cell.own_takes}; retained upper bound ${Math.max(0,cell.observation.stock-cell.own_takes)}. Current observer stock is excluded.`:'Environmental observation unavailable / unknown, not zero.');
   table(host,['Public individual','Captured availability'],legacy.people.map(p=>[p.id,p.available?'Available':'Busy']));
   table(host,['Activity','Score','Eligible'],view.candidates.map(c=>[c.action,c.score,c.eligible?'Yes':'No']));
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
  const ref=S.causalNode??m.ref;heading($('causal-node'),'Exact original record');note($('causal-node'),`${data.source.id} · original zero-based trial ${data.source.record} · ${ref}`);details($('causal-node'),'Original recorded values',resolve(data.trial,ref));
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
  $('title').textContent=t.config.case.replaceAll('_',' ');$('mode').textContent=`Social capacity ${t.config.capacity}`;
  $('identity').textContent=`Experiment 022 · original zero-based trial ${d.source.record} · seed ${t.config.seed} · ticks ${t.start}–${t.end}`;
  $('configuration').textContent=`Observer metadata: ${t.config.order} order; claim ${t.config.duration}; passive cadence ${t.config.cadence}; need +5 per 4 ticks from comparison start. This original finite-window research does not establish continuous opportunities or absolute-phase invariance.`;
  $('source').textContent=JSON.stringify(d.source,null,2);$('scrub').max=d.timeline.length-1;$('scrub').value=S.index;$('tick').value=m.tick;$('previous').disabled=S.index===0;$('next').disabled=S.index===d.timeline.length-1;
  $('actor-lens').setAttribute('aria-pressed',!observer);$('observer-lens').setAttribute('aria-pressed',observer);
  $('concurrent-link').hidden=!observer;
  $('perspective-panel').classList.toggle('observer',observer);
  $('moment-kind').textContent=`WORLD TICK ${m.tick} · ${m.kind.toUpperCase()} · MOMENT ${S.index+1}/${d.timeline.length}`;
  const view=project(d,m,S.actor);$('moment-title').textContent=observer?m.label:view?`Individual ${S.actor} · ${view.phase}${view.selected===undefined?'':': '+human(view.selected)}`:'No owned decision input at this record';$('moment-ref').textContent=m.ref;
  for(const id of ['cards','outcome','inspect','links'])$(id).replaceChildren();$('cards').classList.toggle('concurrent-cards',observer);$('cards').classList.toggle('concurrent-owned',!observer);
  $('lens-note').textContent=observer?'Objective recorded history and typed causal navigation. Physical vectors are reconstructed from actual operations; they never become actor knowledge.':'Only this individual’s original captured input or explicit owned snapshot. No observer stock, private response context, nearest-state interpolation or archive recovery.';
  if(observer){
   const point=d.observer_physical.find(p=>p.tick===m.tick);note($('outcome'),'Observer balances after all recorded physical operations at this world tick. The selected record retains its own exact before/after context.');
   if(point){for(let owner=0;owner<4;owner++){const card=el('article');card.append(el('h3',`Individual ${owner}`));note(card,`Food ${point.food[owner]} · hunger ${point.hunger[owner]}`);$('cards').append(card);}note($('outcome'),`Actual cache stock ${point.stock}; consumed ${point.consumed}.`);}
   const active=d.intervals.filter(c=>c.start<=m.tick&&m.tick<c.end);table($('outcome'),['Actual commitment','Participants','Start','Release','Bound'],active.map(c=>[`${c.kind} #${c.id}`,c.owners.join(' / '),c.start,c.end,c.deadline]));
   details($('inspect'),'Selected original record',resolve(t,m.ref));graphButton(env,$('links'),'Inspect exact original '+m.ref,m.ref);$('raw').textContent=JSON.stringify(resolve(t,m.ref),null,2);
  }else{
   S.causalNode=null;ownInput(env,$('inspect'),view);note($('cards'),view?'Owned captured context; another person’s state is excluded.':'No replacement state is inferred from other records.');$('raw').textContent=JSON.stringify(view,null,2);
   if(view)note($('links'),`Exact owned source: ${view.ref}`);
  }
  lanes(env,m);renderCausal(env);
 }
 function renderPair(env){
  const {S,$,el,note,table,details}=env,c=S.comparison,host=$('compare-summary'),pair=$('pair');host.replaceChildren();pair.replaceChildren();host.hidden=S.lens==='actor';$('inspect-right').disabled=false;
  table(host,['Comparison scope','Result'],[['Same physical start/profiles',c.initial_state_and_profiles_match?'Yes':'NO'],['Only capacity differs in configuration',c.matched_except_capacity&&c.capacity_contrast?'Yes':'NO — not a matched capacity treatment'],['Same evaluation endpoints',c.equal_horizon?'Yes':'NO'],['First admission difference',c.first_admission?.tick??'None'],['First executed difference',c.first_executed?.tick??'None'],['First missing ordinary decision opportunity',c.first_opportunity?.tick??'None'],['First different selected choice at a shared actor/tick',c.first_common_selected_choice?.tick??'None'],['First food-ledger difference',c.first_material??'None'],['Later food-ledger reconvergence',c.material_reconvergence??'None'],['Terminal food ledger',c.terminal_food_equal?'Equal':'Different'],['Terminal hunger',c.terminal_hunger_equal?'Equal':'Different'],['Terminal directed trust (missing = 0)',c.semantic_trust_equal?'Equal':'Different'],['Complete trial equality except capacity',c.complete_null_except_capacity?'Yes':'No'],['Hunger-area right minus left',c.hunger_area_right_minus_left.join(' / ')]]);
  note(host,c.note);details(host,'Actual material/contact counts',{left:c.left_metrics,right:c.right_metrics});
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
   const col=el('section');col.append(el('h3',`Original ${data.source.record} · capacity ${data.trial.config.capacity}`));note(col,`Shared world tick ${tick}; source ${data.source.id}`);
   if(S.lens==='observer'){const p=points.find(p=>p.tick===tick);if(p){table(col,['Individual','Food','Hunger'],p.food.map((food,owner)=>[owner,food,p.hunger[owner]]));note(col,`Cache ${p.stock}; consumed ${p.consumed}. Physical reconstruction only; no cognitive interpolation.`);}else note(col,'This tick is outside this recorded world.');}
   else {const moment=data.timeline.find(m=>m.kind==='decision'&&m.tick===tick),view=moment?project(data,moment,S.actor):null;ownInput(env,col,view);}
   pair.append(col);
  }
 }
 return {is,resolve,project,original,render,renderCausal,renderPair};
})();
if(typeof module!=='undefined')module.exports=NativeConcurrent;
