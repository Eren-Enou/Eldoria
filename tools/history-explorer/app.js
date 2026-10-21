'use strict';
// Observational presentation only. Never score a policy or reconstruct actor cognition.
const $ = id => document.getElementById(id);
const S = {catalog: [], data: null, right: null, comparison: null, index: 0, actor: 0, lens: 'actor', timer: null, generation: 0, pairGeneration: 0, causalNode: null, chain: null};
const significant = m => !['snapshot', 'need', 'window', 'decision'].includes(m.kind);
const text = value => typeof value === 'string' ? value : JSON.stringify(value);
function human(value) {
 if(value?.Invite)return `${value.Invite.kind==='Inquiry'?'Inquiry':'Resource contact'} with individual ${value.Invite.recipient}`;
 if(value?.Accept!==undefined)return `Accept invitation from individual ${value.Accept}`;
 if(value?.Ask)return `Ask individual ${value.Ask.source}, concern ${value.Ask.concern}, ${value.Ask.strategy}`;
 return text(value);
}
function el(tag, content, cls) { const e = document.createElement(tag); if (content !== undefined) e.textContent = text(content); if (cls) e.className = cls; return e; }
function note(parent, content, warning=false) { parent.append(el('p', content, 'note' + (warning ? ' warning' : ''))); }
function details(parent, title, value) { const d=el('details'); d.append(el('summary',title),el('pre',JSON.stringify(value,null,2))); parent.append(d); }
function heading(parent, title) { parent.append(el('h3',title)); }
function table(parent, headers, rows, selected=-1) { const t=el('table'), head=el('tr'); headers.forEach(h=>head.append(el('th',h))); t.append(head); rows.forEach((r,i)=>{const tr=el('tr'); if(i===selected)tr.className='selected'; r.forEach(c=>tr.append(el('td',c === undefined ? 'Not captured' : c)));t.append(tr);});parent.append(t); }
function button(parent, label, action) { const b=el('button',label);b.addEventListener('click',action);parent.append(b);return b; }
function status(message, error=false) {$('status').textContent=message;$('status').classList.toggle('error',error);}
async function api(path, options) { const r=await fetch(path,options);const data=await r.json();if(!r.ok)throw new Error(data.error || r.statusText);return data; }
function guard(fn) { return async (...args)=>{try{await fn(...args);}catch(e){pause();status(e.message,true);}}; }
function label(e) {const c=e.config;if(e.experiment==='025')return `025 · ${e.registered?'registered':'custom prototype'} · ${c.family} · ${e.mode} · work ${c.preparation.extraction_ticks}+${c.preparation.preparation_ticks} · nominal ${c.world.horizon} · original zero-based trial ${e.record}`;if(e.experiment==='024')return `024 · ${e.registered?'registered':'custom prototype'} · ${e.case} · ${c.valuation} · seed ${e.seed} · period ${c.world.period} / nominal ${c.world.horizon} / ${c.world.phase} · original zero-based trial ${e.record}`;if(e.experiment==='023')return `023 · ${e.registered?'registered':'custom prototype'} · ${e.case} · ${c.mode} · seed ${e.seed} · period ${c.period} / nominal ${c.horizon} / ${c.phase} · original zero-based trial ${e.record}`;if(e.experiment==='022')return `022 · ${e.case} · social capacity ${c.capacity} · seed ${e.seed} · claim ${c.duration} / horizon ${c.horizon} / ${c.order} · original zero-based trial ${e.record}`;if(e.experiment==='021')return `021 · ${e.case} · ${e.policy} / ${e.mode} · seed ${e.seed} · glimpse ${c.cadence} / window ${c.window} / attempt ${c.duration} / ${c.order} · original zero-based trial ${e.record}`;if(e.experiment==='020')return `020 · ${e.case} · ${e.mode} · seed ${e.seed} · window ${c.window} / attempt ${c.duration} / ${c.order} · original zero-based trial ${e.record}`;return `${e.experiment} · ${e.case} · ${e.mode} · seed ${e.seed} · window ${c.window ?? c.budget ?? 'not captured'} / gap ${c.idle_gap ?? 'not a recorded treatment'} / I${c.interval} / ${c.order} / stock ${c.stock} / H parameter ${c.hunger ?? 'not recorded'} · #${e.record}`;}
function options(select, entries, selected) {select.replaceChildren();entries.forEach(e=>{const o=el('option',label(e));o.value=e.id;select.append(o);});if(selected)select.value=selected;}
function nativeEnv(){return {S,$,el,note,heading,table,details,button,human,go,openHistory,guard,status};}
function configureActorOptions(data){
 let fourth=$('actor').querySelector('option[value="3"]');
 if(!fourth){fourth=el('option','Individual 3');fourth.value='3';$('actor').append(fourth);}
 const native=NativeConcurrent.is(data);fourth.hidden=!native;fourth.disabled=!native;
 if(!native&&S.actor>2)S.actor=0;$('actor').value=String(S.actor);
}
function configure(data) {
 if(data.schema !== 'woi-history-explorer/v1')throw new Error('Unknown export schema');
 if(data.source.experiment==='025'){
  if(!NativeConcurrent.is(data)||data.source.source_schema!=='experiment-025-trial/v1'||data.trial.schema!=='experiment-025-physical-history/v1'||data.extractor_version!==9||!data.valuation||!data.continuous||!data.preparation)throw new Error('025 requires its explicit finite-preparation adapter');
  data.stepMap=new Map();return data;
 }
 if(data.source.experiment==='024'){
  if(!NativeConcurrent.is(data)||data.source.source_schema!=='experiment-024-trial/v1'||data.trial.schema!=='experiment-024-physical-history/v1'||data.extractor_version!==8||!data.valuation||!data.continuous)throw new Error('024 requires its explicit valuation adapter');
  data.stepMap=new Map();return data;
 }
 if(data.source.experiment==='023'){
  if(!NativeConcurrent.is(data)||data.source.source_schema!=='experiment-023-trial/v1'||data.trial.schema!=='experiment-023-physical-history/v1'||data.extractor_version!==7||!data.continuous)throw new Error('023 requires its explicit continuous-world adapter');
  data.stepMap=new Map();return data;
 }
 if(data.source.experiment==='022'){
  if(!NativeConcurrent.is(data)||data.source.source_schema!=='experiment-022-trial/v1'||data.extractor_version!==5)throw new Error('022 requires the explicit native four-person adapter');
  data.stepMap=new Map();return data;
 }
 data.stepMap = new Map();let flat=0;data.trial.periods.forEach((p,pn)=>p.steps.forEach((w,sn)=>data.stepMap.set(`${pn}/${sn}`,flat++)));
 return data;
}
function clearComparison() {S.right=null;S.comparison=null;$('inspect-right').disabled=true;$('compare-summary').replaceChildren();$('pair').replaceChildren();}
async function openHistory(id) {
 pause();const generation=++S.generation;status('Loading one verified trial…');
 const d=configure(await api('/api/trial?id='+encodeURIComponent(id)));if(generation!==S.generation)return;S.data=d;S.causalNode=null;S.chain=null;configureActorOptions(d);clearComparison();S.index=Math.max(0,d.timeline.findIndex(m=>m.kind==='decision'));$('compare-panel').hidden=true;if([...$('trial').options].some(o=>o.value===id))$('trial').value=id;
 render();status('Loaded verified original '+d.source.archive+' · record '+d.source.record+'. No simulation was run.');
}
function current() {return S.data?.timeline[S.index];}
function followOriginal(ref,anchor) {
 if(NativeConcurrent.is(S.data))return NativeConcurrent.original(nativeEnv(),ref);
 originalValue(S.data.trial,ref);
 const priorChain=S.data.causal.chains.find(ch=>ch.id===$('chain-select').value);
 const i=S.data.timeline.findIndex(m=>m.ref===anchor);
 if(i<0){const n=S.data.causal.nodes[ref];if(n&&n.tick<S.data.trial.clock.epoch){S.causalNode=ref;S.lens='observer';render();status('Exact original record shown below; it predates the captured replay epoch. No new moment is invented.');$('causal-node').scrollIntoView({block:'center'});return;}throw new Error('No captured navigation anchor for '+ref);}
 S.causalNode=ref;S.chain=priorChain?.nodes.includes(ref)?priorChain.id:null;S.lens='observer';go(i);$('causal-node').scrollIntoView({block:'center'});
}
function renderCausal() {
 if(NativeConcurrent.is(S.data))return NativeConcurrent.renderCausal(nativeEnv());
 const panel=$('causal-browser');panel.hidden=S.lens!=='observer';
 if(panel.hidden){$('causal-chain').replaceChildren();$('causal-node').replaceChildren();$('encounter-results').replaceChildren();$('personal-summary').replaceChildren();return;}
 const c=S.data.causal;if(!c){note($('causal-chain'),'Typed links are unavailable in this older export. Reopen the verified source.');return;}
 if(S.causalSource!==S.data.source.id){S.causalSource=S.data.source.id;S.causalNode=null;S.chain=null;$('chain-select').replaceChildren();for(const ch of c.chains){const option=el('option',`${ch.tick} · ${ch.id}${ch.scenes.length?' · scene '+ch.scenes.join(','):''}`);option.value=ch.id;$('chain-select').append(option);}}
 const m=current();let chain=c.chains.find(ch=>ch.id===S.chain);
 if(!chain)chain=c.chains.find(ch=>m.step&&ch.id===`periods[${m.step[0]}].steps[${m.step[1]}].activity`)??c.chains.find(ch=>ch.nodes.includes(m.ref))??c.chains[0];
 $('chain-select').value=chain.id;const box=$('causal-chain');box.replaceChildren();
 note(box,`Recorded cohort at tick ${chain.tick}. Its outgoing choices may differ from the edge that executed. Missing later links remain unknown; they are not inferred from time alone.`);
 const list=el('ol');const ids=orderedChainNodes(c,chain,S.data.timeline);
 for(const id of ids){const n=c.nodes[id],li=el('li'),o=n.outcome;let description=human(o??'Original reference');
  if(n.source_kind==='Resource action'&&o)description=`${o.action}; transferred ${o.transferred}; food [${o.before}]→[${o.after}]; ${o.termination??'continues'}`;
  if(n.source_kind==='Recorded interpretation'&&o)description=`${o.interpretation}; valence ${o.valence}; event ${o.event}`;
  if(n.source_kind==='Executed inquiry'&&o)description=`${human({Ask:o.selected})}; ${o.actual_time} ticks; native field ${o.native_response===null?'null':'present'}; ${o.status_before}→${o.status_after}`;
  if(n.source_kind==='Provenance query'&&o)description=`observed ${o.observed_at}, completed ${o.completed_at}; receipt ${o.receipt??'none'}`;
  if(n.source_kind==='Provenance delivery'&&o)description=`receipt ${o.receipt}; received claim ${o.received_claim}; quality ${o.received_quality}`;
  if(n.source_kind==='Retained assessment update'&&o)description=`support ${o.before}→${o.support}; ${o.retained.length} retained items`;
  if(n.source_kind==='Observed learning consequence'&&o)description=`${o.novelty}; support ${o.support_before}→${o.support_after}; ${o.status_before}→${o.status_after}; progress is not truth`;
  button(li,`${n.tick??'Time not captured'} · ${n.source_kind}${n.actor!==null?' · individual '+n.actor:''}${n.recipient!==null?' → '+n.recipient:''} · ${description}`,guard(()=>followOriginal(n.ref,n.anchor)));list.append(li);}box.append(list);
 details(box,'Exact typed edges (observer references, not a universal causal graph)',chain.edges);
 const source=$('causal-node');source.replaceChildren();if(S.causalNode){const n=c.nodes[S.causalNode];heading(source,'Original linked record');source.append(el('p',S.causalNode,'mono'));details(source,'Original value at this exact reference',originalValue(S.data.trial,S.causalNode));if(n){for(const key of ['before','after'])if(n[key])button(source,`Open ${key}: ${n[key]}`,guard(()=>followOriginal(n[key],n[key])));}}
 const kind=$('encounter-filter').value,rows=resourceRecords(c,kind,Math.max(0,Number($('encounter-after').value)||0));
 $('encounter-count').textContent=`${rows.length} ${['unexecuted','declined','reciprocal'].includes(kind)?'allocation dispositions (not completed encounters)':'completed resource encounters'} match · showing up to 80. Counts cover the captured replay periods, not only nearby timeline entries; pre-epoch preparation is excluded.`;
 const result=$('encounter-results');result.replaceChildren();for(const r of rows.slice(0,80)){const li=el('li');button(li,`${r.tick} · ${r.description}`,guard(()=>followOriginal(r.ref,r.anchor)));result.append(li);}
 const summary=$('personal-summary');summary.replaceChildren();heading(summary,`Individual ${S.actor} · recorded activity summary`);note(summary,`Captured replay from epoch ${S.data.trial.clock.epoch}; earlier preparation is excluded. No private thoughts are interpolated.`);
 const own=S.data.timeline.filter(m=>m.kind==='decision').map(m=>projection(S.data,m,S.actor)).filter(a=>a?.phase==='activity');
 const meals=S.data.timeline.filter(m=>m.kind==='meal'&&m.data.actor===S.actor),acts=S.data.timeline.filter(m=>m.kind==='resource'&&m.data.decision.actor===S.actor),queries=S.data.timeline.filter(m=>m.kind==='inquiry'&&m.data.decision.input.owner===S.actor);
 table(summary,['Recorded quantity','Count'],[['Selected Consume',own.filter(a=>a.selected==='Consume').length],['Selected Invite (not execution)',own.filter(a=>a.selected==='Invite').length],['Selected Wait',own.filter(a=>a.selected==='Wait').length],['Executed meals',meals.length],['Executed resource actions',acts.length],['Completed inquiries',queries.length],['Completed scenes participated',c.encounters.filter(e=>e.completed&&e.participants.includes(S.actor)).length]]);
}
function stepFor(data,m) {return m.step ? data.trial.periods[m.step[0]].steps[m.step[1]].activity : null;}
function projection(data,m,owner) {
 if(NativeConcurrent.is(data))return NativeConcurrent.project(data,m,owner);
 if(m.kind==='resource')return m.data.decision.actor===owner?data.resource_actors?.[m.data.id]??null:null;
 if(m.step){const index=data.stepMap.get(m.step.join('/'));const local=m.kind==='decision'?data.actors[index][owner]:m.kind==='consent'?data.reply_actors?.[index]?.[owner]??null:null;if(local)return local;}
 return m.point!==null?capturedOwnedPoint(data.trial,m.point,owner):null;
}

function snapshot(data,tick) {let found=null,index=null;for(let i=0;i<data.trial.points.length;i++){const p=data.trial.points[i];if(p.state.tick>tick)break;found=p;index=i;}return found ? {point:found,index} : {point:data.trial.initial,index:'initial'};}
function exactSnapshot(data,m) {return m.point !== null ? {point:data.trial.points[m.point],index:m.point} : snapshot(data,m.tick);}
function nearest(data,tick) {const i=data.timeline.findIndex(m=>m.tick>=tick);return i<0?data.timeline.length-1:i;}
function go(index) {if(!S.data)return;S.index=Math.max(0,Math.min(S.data.timeline.length-1,index));render();}
function nextWhere(test, direction=1) {if(!S.data)return;for(let i=S.index+direction;i>=0&&i<S.data.timeline.length;i+=direction){if(test(S.data.timeline[i])){go(i);return;}}status('No further matching recorded moment.');}
function pause() {if(S.timer)clearInterval(S.timer);S.timer=null;$('play').textContent='Play';}
function play() {if(S.timer){pause();return;}if(!S.data)return;$('play').textContent='Pause';S.timer=setInterval(()=>{if(S.index>=S.data.timeline.length-1){pause();return;}go(S.index+1);},1000/Number($('speed').value));}
function render() {
 if(!S.data)return;
 if(NativeConcurrent.is(S.data)){NativeConcurrent.render(nativeEnv());renderList();renderPair();return;}
 $('concurrent-lanes').hidden=true;$('concurrent-lanes').replaceChildren();$('concurrent-link').hidden=true;$('cards').classList.remove('concurrent-cards','concurrent-owned');
 const d=S.data,t=d.trial,m=current();
 $('title').textContent=t.case.replaceAll('_',' ');$('mode').textContent=t.mode;
 $('identity').textContent=`Experiment ${d.source.experiment} · Seed ${d.source.seed} · ticks ${t.clock.epoch}–${t.clock.last} · ${t.config.order} order`;
 $('configuration').textContent=`Archive configuration (observer metadata): window ${t.config.window??t.config.budget}; observation gap ${t.config.idle_gap??'not a recorded treatment'}; hunger +${t.clock.settings.increment} per ${t.clock.settings.interval} ticks; elapsed horizon ${t.clock.last-t.clock.epoch}.`;
 if(d.source.experiment==='021')$('configuration').textContent=`Archive configuration (observer metadata): ${t.policy} acquisition; ${t.mode} knowledge; passive cadence ${t.config.cadence} ticks (${t.mode==='FreshReference'?'fresh sensor at every free choice':t.mode==='Unobserved'?'channel disabled':'fixed slots, busy actors miss them'}); attempt ${t.config.duration}; window ${t.config.window}; hunger +${t.clock.settings.increment} per ${t.clock.settings.interval} ticks.`;
 $('source').textContent=JSON.stringify(d.source,null,2);
 $('scrub').max=d.timeline.length-1;$('scrub').value=S.index;$('tick').value=m.tick;
 $('previous').disabled=S.index===0;$('next').disabled=S.index===d.timeline.length-1;
 $('moment-kind').textContent=`WORLD TICK ${m.tick} · ${m.kind.toUpperCase()} · MOMENT ${S.index+1}/${d.timeline.length}`;
 $('moment-title').textContent=S.lens==='observer'?m.label:(projection(d,m,S.actor)?`Individual ${S.actor}: ${projection(d,m,S.actor).phase==='owned snapshot'?'owned state capture (no decision)':human(projection(d,m,S.actor).selected)}`:'Observer record selected; switch perspective for objective details.');$('moment-ref').textContent=m.ref;
 renderCards();renderInspect();renderLinks();renderList();renderPair();renderCausal();
}
function renderCards() {
 const p=$('cards');p.replaceChildren(); const m=current();
 if(S.lens==='actor') {
  const a=projection(S.data,m,S.actor);
  const c=el('div',undefined,'card');c.append(el('strong',`Individual ${S.actor} · Actor Perspective`));
  if(a){c.append(el('p',`Owned food ${a.own.food} · hunger ${a.own.hunger}`, 'stat'),el('p',`Captured at tick ${a.now}; window end ${a.end ?? 'not captured in this record'}`));}
  else c.append(el('p','Not captured at this moment. Navigate to this individual’s recorded decision.'));
  p.append(c);
 } else {
  const x=exactSnapshot(S.data,m);
  x.point.state.agents.forEach(a=>{const c=el('div',undefined,'card');c.append(el('strong',`Individual ${a.id}`),el('p',`Food ${a.food}`, 'stat'),el('p',`Hunger ${a.hunger}`),el('small',`Observer snapshot tick ${x.point.state.tick} · point ${x.index}`));p.append(c);});
 }
 const outcome=$('outcome');outcome.replaceChildren();
 if(S.lens==='observer') {
  const x=exactSnapshot(S.data,m),total=x.point.state.agents.reduce((n,a)=>n+a.food,0);
  note(outcome,`Observer balance: ${total} food owned in recorded point ${x.index} at tick ${x.point.state.tick}. No interpolation between snapshots.`);
  if(x.point.cache){heading(outcome,'Environmental observer · finite cache');table(outcome,['Cache','Stock / capacity','Access','Pending claimants'],[[x.point.cache.id,`${x.point.cache.stock} / ${x.point.cache.capacity}`,x.point.cache.accessible?'Open':'Closed',x.point.pending.map(a=>`${a.actor}: ${a.start}→${a.end}`).join('; ')||'None']]);details(outcome,'Pending claims and deadlines (observer only)',x.point.pending);}
  if(typeof x.index==='number') {
   const capture=S.data.timeline.find(z=>z.kind==='snapshot'&&z.point===x.index);
   const consumed=S.data.timeline.filter(z=>z.kind==='meal'&&(z.tick<x.point.state.tick||z.tick===x.point.state.tick&&z.order<=capture.order)).reduce((sum,z)=>sum+z.data.consumed,0);
   const initial=S.data.trial.initial.state.agents.reduce((sum,a)=>sum+a.food,0)+(S.data.trial.initial.cache?.stock??0),stock=x.point.cache?.stock??0;
   note(outcome,`Recorded food ledger: initial ${initial} = owned ${total} + cache ${stock} + consumed ${consumed}. ${initial===total+stock+consumed?'Balanced at this capture.':'MISMATCH — inspect original references.'}`);
  }
  if(m.kind==='resource') {
   heading(outcome,`Executed ${m.data.decision.selected} by individual ${m.data.decision.actor}`);
   table(outcome,['Participant','Food before','Food after'],m.data.participants.map((id,i)=>[id,m.data.balances_before[i],m.data.balances_after[i]]));
   note(outcome,`${m.data.transferred} food transferred. Recorded termination: ${m.data.outcome??'interaction continues'}. This is not a global benefit ranking.`);
  }
  if(m.kind==='meal')table(outcome,['Recorded meal','Before','After'],[['Owned food',m.data.food_before,m.data.food_after],['Hunger',m.data.hunger_before,m.data.hunger_after]]);
  if(m.kind==='claim'){heading(outcome,'Actual claim admission');table(outcome,['Claim','Actor','Cache','Busy interval'],[[m.data.id,m.data.actor,m.data.cache,`${m.data.start} → ${m.data.end}`]]);note(outcome,'The linked point captures state before this cohort’s reservations. Admission records the new busy interval; no physical unit is reserved and no result is yet known.');}
  if(m.kind==='acquisition'){const r=m.data;heading(outcome,`Actual ${r.outcome} · individual ${r.attempt.actor}`);table(outcome,['Physical result','Before','After'],[['Cache stock',r.cache_before,r.cache_after],['Own food',r.food_before,r.food_after]]);note(outcome,`Claim ${r.attempt.id}, ${r.attempt.start} → ${r.completed_at}; completion rank ${r.rank}. Full time charged for failure as well as success. The linked after-point may include other completions at this same tick; this row retains this operation’s own exact balances.`);}
  if(m.kind==='environmental_memory'){const r=m.data;heading(outcome,`Owned environmental ${r.kind} · individual ${r.owner}`);table(outcome,['Owned field','Before','After'],[['Receipt',r.before?.receipt??'Unavailable',r.after?.receipt??'Unavailable'],['Observed tick',r.before?.observation.observed_at??'Unknown',r.after?.observation.observed_at??'Unknown'],['Last-seen stock',r.before?.observation.stock??'Unknown',r.after?.observation.stock??'Unknown'],['Own takes',r.before?.own_takes??'Unknown',r.after?.own_takes??'Unknown']]);note(outcome,`Recorded transition at tick ${r.tick}; ${r.before?`previous observation age ${r.tick-r.before.observation.observed_at}`:'no prior available record'}.`);details(outcome,'Original retention transition (observer audit)',r);note(outcome,'This audit preserves a past record for inspection. Actor Perspective reads only the slot captured in an actual decision; it does not restore this content.');}
  if(S.data.source.experiment==='021'&&m.step){const w=S.data.trial.periods[m.step[0]].steps[m.step[1]],p=S.data.trial.points[w.activity.observed],now=p.state.tick;heading(outcome,'Dated knowledge versus actual stock · observer comparison');table(outcome,['Individual','Available own receipt','Observed tick / age','Local upper estimate','Actual stock at choice'],w.choices.map((c,o)=>{const k=c?.retained;return[o,c===null?'No choice: occupied':k?k.receipt:'Unknown',k?`${k.observation.observed_at} / ${now-k.observation.observed_at}`:'Unavailable',k?Math.max(0,k.observation.stock-k.own_takes):'Unknown',p.cache.stock];}));note(outcome,'Rival grants can change actual stock without updating these owned records. The actual-stock column is observer-only.');}
  if(m.kind==='inquiry') {
   note(outcome,`Executed inquiry ${m.data.id}: ${human(m.data.decision.selected)}. Actual effort ${m.data.time_spent} ticks. ${m.data.response===null?'Native response receipt field is empty. This does not exclude a provenance-channel delivery; inspect the recorded assessment and concern changes before judging information acquired.':'Native response receipt recorded; correctness and useful change are separate questions.'}`);
   table(outcome,['Native query stage','Before','After'],[['Concern status',m.data.concern_before?.status,m.data.concern_after?.status]]);
  }
  if(m.data)details(outcome,'Actual selected record',m.data);
 } else note(outcome,'Private partner food, traits, hidden provenance and objective truth: Not known to this individual. Later outcomes are available in Observer Perspective.');
}
function renderInspect() {
 const box=$('inspect');box.replaceChildren();const m=current(),s=stepFor(S.data,m),a=projection(S.data,m,S.actor);
 $('actor-lens').setAttribute('aria-pressed',S.lens==='actor');$('observer-lens').setAttribute('aria-pressed',S.lens==='observer');document.querySelector('.inspector').classList.toggle('observer',S.lens==='observer');
 $('lens-note').textContent=S.lens==='actor'?`ACTOR PERSPECTIVE · individual ${S.actor} · selected tick ${m.tick} · captured local information only`:`OBSERVER PERSPECTIVE · selected tick ${m.tick} · includes private and objective records`;
 if(S.lens==='actor') {
  if(!a) {
   note(box,'Not captured at this moment. This view does not restore a local perspective from observer history.',true);
   button(box,'Find next captured decision',()=>nextWhere(x=>x.kind==='decision' && projection(S.data,x,S.actor)?.phase==='activity'));
   $('raw').textContent='Not captured at this moment.';return;
  }
  note(box,`Individual ${S.actor}: owned food ${a.own.food}, hunger ${a.own.hunger}; captured tick ${a.now}.`);
  if(a.phase==='owned snapshot') {
   heading(box,'Captured own state · no decision record');
   note(box,`Own Agent component at original point ${a.source_point}, tick ${a.now}. No action or policy input is inferred from this state.`);
   details(box,'Own retained episodes',a.own.memories);details(box,'Own directed trust',a.own.trust);
   details(box,'Own recorded traits',{generosity:a.own.generosity,caution:a.own.caution,expectation:a.own.expectation});
   note(box,'Concerns, full assessment, source knowledge, progress cells, public observations and action candidates: Not captured in this point record. Nothing is filled from observer history.');
   button(box,'Next captured activity decision',()=>nextWhere(x=>x.kind==='decision'&&projection(S.data,x,S.actor)?.phase==='activity'));
   $('raw').textContent=JSON.stringify(a,null,2);return;
  }
  if(a.phase==='resource') {
   heading(box,'Executed resource decision · captured local input');
   note(box,`Recorded selection: ${human(a.selected)}. This is the actual resource action, distinct from the earlier invitation.`);
   table(box,['Legal candidate','Recorded score'],a.candidates.map(c=>[c.action,c.score]));
   details(box,'Public resource observation',a.observation);
   details(box,'Recorded own relationship inputs',{trust_toward_partner:a.trust,episodic_valence:a.episodic_valence,memory_cause_references:a.memory_causes});
   note(box,'Full retained episodes, concerns, assessment and source inventory: Not captured in this resource decision. Reference IDs do not restore their content.');
   $('raw').textContent=JSON.stringify(a,null,2);return;
  }
  heading(box,`Why this action? · ${a.phase==='recipient'?'Recipient response':S.data.trial.mode}`);
  note(box,`Recorded selection: ${human(a.selected)}. These are historical scores and priorities, not calibrated utility or a psychological explanation.`);
  table(box,['Activity','Eligible','Score'],a.candidates.map(c=>[human(c.action??c.reply),c.eligible===undefined?'Not captured':c.eligible?'Yes':'No',c.score]),a.candidates.findIndex(c=>text(c.action??c.reply)===text(a.selected)));
  if(a.phase==='recipient')details(box,'Incoming invitations legitimately observed',a.incoming);
  if(a.cache){heading(box,'Legitimate current environmental observation');table(box,['Cache','Observed stock','Observed at','Access','Attempt ticks'],[[a.cache.id,a.cache.stock,a.cache.observed_at,a.cache.accessible?'Open':'Closed',a.cache.duration]]);note(box,`Recorded acquisition policy ${a.acquisition_policy}. ${a.acquisition_policy==='Acquire'?'Eligible acquisition has explicit priority.':'Strict first-maximum ties retain earlier candidates.'} No later stock or claimant outcome is part of this input.`);}
  if(a.environmental_knowledge){const k=a.environmental_knowledge,c=k.retained;heading(box,'Own dated environmental knowledge');if(c){table(box,['Own local field','Recorded value'],[['Cache',c.observation.id],['Owner-local receipt',c.receipt],['Observed tick',c.observation.observed_at],['Age in ticks',k.age],['Last-seen stock',c.observation.stock],['Own successful takes',c.own_takes],['Remaining upper estimate',k.estimate]]);note(box,`This is an available retained record at this choice. Current objective stock is unknown. Known access: ${c.observation.accessible?'open':'closed'}; current attempt duration ${k.duration}. Own takes do not refresh the observation date.`);}else{note(box,'Unknown / unavailable. No quantity is available to this choice; this is not a known empty source. An older receipt in observer history cannot restore it.');}note(box,`Recorded acquisition policy ${a.acquisition_policy}. ${a.acquisition_policy==='Acquire'?'Eligible acquisition has explicit priority.':'Strict first-maximum ties preserve the earlier candidate.'}`);}
  if(a.phase==='activity'&&S.data.trial.mode==='Deadline')note(box,'Historical priority: eat when eligible except when a meal would consume the last admissible invitation slot. Scores alone do not explain this priority.');
  if(a.phase==='activity'&&S.data.trial.mode==='EatFirst')note(box,'Historical priority: eligible consumption precedes score comparison.');
  heading(box,a.phase==='recipient'?'Earlier outgoing contact alternatives (captured in this response)':'Contact alternatives and public bounds');table(box,['Intent','Eligible','Score','Reserved ticks'],a.contact_candidates.map(c=>[human(c.intent),c.eligible?'Yes':'No',c.score,c.reservation]));
  details(box,'Own retained episodes',a.own.memories);details(box,'Own directed trust',a.own.trust);details(box,'Own concerns at decision',a.inquiry.base.concerns);
  details(box,'Public individuals and availability',a.people);details(box,'Captured source knowledge and attribution',{sources:a.inquiry.base.sources,knowledge:a.inquiry.knowledge,hints:a.inquiry.hints});details(box,'Captured inquiry learning and candidates',{cells:a.inquiry.base.cells,seen:a.inquiry.base.seen,candidates:a.query_candidates,selected:a.query_selected});
  note(box,'Full retained Grouped assessment basis and separate Contextual progress cells: Not captured in this activity input. They are not filled from observer audit records.');
  details(box,'Owned traits and profile',{generosity:a.own.generosity,caution:a.own.caution,expectation:a.own.expectation,profile:a.profile});
  $('raw').textContent=JSON.stringify(a,null,2);
 } else {
  const x=exactSnapshot(S.data,m);heading(box,'Objective recorded state');
  note(box,`Latest captured point at or before this tick: ${x.index}, captured tick ${x.point.state.tick}. Without an explicit point link, a tick snapshot can precede this event or include other phases at the same tick. Follow original before/after references for exact scope.`,x.point.state.tick!==m.tick);
  details(box,'All current recorded agents (observer only)',x.point.state.agents);
  if(s) {
   heading(box,'Intention → consent → matching → execution');
   details(box,'Selected outgoing activities',s.decisions.map((d,i)=>({individual:i,selected:d?.selected??'No new choice (occupied)',contact:d?.contact.selected})));
   details(box,'Actual provisional recipient responses',s.responses.map(r=>({owner:r.input.own.own.id,selected:r.selected,candidates:r.candidates})));
   details(box,'Arbitration dispositions',s.allocations);
   details(box,'Actual execution references',{started:s.started,finished:s.finished,query:s.query,active_before:s.active_before,active_after:s.active_after,consumption:s.consumption});
   note(box,s.query===null?'This step has no executed inquiry. A planned query, if present, did not become a query here.':`Executed query id ${s.query}; follow its native record below.`);
   if(['020','021'].includes(S.data.source.experiment)){const w=S.data.trial.periods[m.step[0]].steps[m.step[1]];details(box,'Actual environmental choices, admissions, omitted responses and completions (observer)',{choices:w.choices,admissions:w.admissions,skipped_responses:w.skipped_responses,completed:w.completed,atomic_blocked:w.atomic_blocked});}
  }
  const b=layers(S.data.trial);
  details(box,'Recorded concern transitions through captured point',b.base.concerns.transitions.slice(0,x.point.state.concerns));
  details(box,'Recorded assessment audit through captured point (observer)',b.assessment.records.slice(0,x.point.state.assessment));
  $('raw').textContent=JSON.stringify(m.data??s??x.point,null,2);
 }
}
function layers(t) {return {base:t.final_state.base.base.base.base.base,inquiry:t.final_state.base.base.base.base.inquiry,assessment:t.final_state.base.base.assessment};}
function renderLinks() {
 const box=$('links');box.replaceChildren();const m=current(),s=stepFor(S.data,m);let refs=[];
 if(S.lens==='actor'){
  note(box,'Objective results and future links are hidden in Actor Perspective. The timeline is a historical navigation index, not the individual’s knowledge.');
  button(box,'Explore recorded consequences (Observer)',()=>{S.lens='observer';render();$('causal-browser').scrollIntoView({block:'start'});});
  button(box,'Next own captured activity decision',()=>nextWhere(x=>x.kind==='decision'&&projection(S.data,x,S.actor)?.phase==='activity'));
  return;
 }
 if(s) {
  refs=['before','observed','sent','admitted','after'].filter(k=>s[k]!==null).map(k=>({label:`${k}: point ${s[k]}`,ref:`points[${s[k]}]`}));
  if(s.query!==null)refs.push({label:`Executed inquiry ${s.query}`,ref:`final_state.base.base.base.base.inquiry.records[${s.query}]`});
  const scene=s.started?.scene??s.active_before?.scene??s.finished?.scene;
  if(scene!==undefined)S.data.timeline.filter(x=>x.kind==='resource'&&x.data.scene===scene).forEach(x=>refs.push({label:`Event ${x.data.id}: ${x.data.decision.selected} @ ${x.tick}`,ref:x.ref}));
 } else if(m.kind==='resource') {
  const scene=m.data.scene;
  const i=S.data.timeline.findIndex(x=>x.kind==='commitment'&&x.data.scene===scene);
  if(i>=0)button(box,'Related commitment',()=>go(i));
  for(const x of S.data.timeline.filter(x=>x.kind==='resource'&&x.data.scene===scene))refs.push({label:`Scene event ${x.data.id} @ ${x.tick}`,ref:x.ref});
 }
 refs.forEach(r=>{const i=S.data.timeline.findIndex(x=>x.ref===r.ref);if(i>=0)button(box,(S.lens==='actor'?'Observer reference: ':'')+r.label,()=>{S.lens='observer';go(i);});});
 button(box,'Next actual resource action (Observer)',()=>{S.lens='observer';nextWhere(significantResource);});
 button(box,'Next executed inquiry (Observer)',()=>{S.lens='observer';nextWhere(x=>x.kind==='inquiry');});
 button(box,'Next recorded state',()=>nextWhere(x=>x.kind==='snapshot'));
 button(box,'Trace recorded chain (Observer)',()=>{S.lens='observer';render();$('causal-browser').scrollIntoView({block:'start'});});
 if(!refs.length)note(box,'No explicit direct precursor link captured here. Chronological navigation does not establish sole causation.');
}
const significantResource=x=>x.kind==='resource';
function renderList() {
 if(!S.data)return;const kind=$('kind').value;
 let moments=S.data.timeline.filter(m=>kind==='all'||(kind==='significant'?significant(m):m.kind===kind));
 let at=moments.findIndex(x=>x.id>=S.index);if(at<0)at=moments.length-1;const start=Math.max(0,at-8);const shown=moments.slice(start,start+45);
 $('list-note').textContent=moments.length?`${moments.length} matching records · displaying ${shown.length} near current tick`:'No records of this kind in this history. Change the filter or inspect the other history.';
 const list=$('events');list.replaceChildren();shown.forEach(m=>{const li=el('li'),description=S.lens==='actor'?'Historical navigation reference · objective details hidden':m.label,b=button(li,`${m.tick} · ${m.kind} · ${description}`,()=>go(m.id));b.setAttribute('aria-current',m.id===S.index);list.append(li);});
}
async function compareWith(id) {
 if(!S.data)return;pause();clearComparison();status('Loading paired recorded history and comparing source values…');
 const generation=S.generation,pairGeneration=++S.pairGeneration,left=S.data.source.id;
 const right=configure(await api('/api/trial?id='+encodeURIComponent(id)));
 const result=await api(`/api/compare?left=${left}&right=${id}`);
 if(generation!==S.generation||pairGeneration!==S.pairGeneration)return;S.right=right;S.comparison=result;
 $('compare-panel').hidden=false;$('other').value=id;renderPair();status('Paired comparison ready. Descriptions are derived; original references remain available.');
}
function renderPair() {
 if(S.data&&S.right&&S.comparison?.native_concurrency)return NativeConcurrent.renderPair(nativeEnv());
 if(!S.data||!S.right||!S.comparison)return;$('inspect-right').disabled=false;const c=S.comparison,p=$('compare-summary');p.replaceChildren();p.hidden=S.lens==='actor';
 if(c.first_executed_difference){const a=c.first_executed_difference.left,b=c.first_executed_difference.right;note(p,`First actual execution difference: left ${a?a.kind+' at tick '+a.tick:'has no further action'}; right ${b?b.kind+' at tick '+b.tick:'has no further action'}. Follow the source references for the actual changes.`);}
 table(p,['Comparison scope','Recorded result'],[['Initial state / profiles',c.initial_state_and_profiles_match?'Matched':'DIFFERENT'],['Configurations',c.configurations_equal?'Equal':'Different — timing or other configuration treatment'],['Physical clock settings',c.physical_settings_equal?'Equal':'DIFFERENT — not a policy-only comparison'],['Endpoints',c.equal_horizon?'Same epoch and final tick':'UNEQUAL — qualify endpoint comparison'],['First choice',c.first_choice?JSON.stringify(c.first_choice):'No observed voluntary choice difference'],['First executed difference (all action types)',c.first_executed_difference??'No difference'],['First resource event difference',c.first_resource_event_difference??'No difference'],['Intermediate material differences',`${c.material_different_shared_ticks.length} shared recorded ticks`],[c.potential_supply_equal===undefined?'Final food / hunger':'Final owned + cache material / hunger',c.final_material_equal?'Equal':'Different'],['Material reconvergence',c.material_reconvergence?'Yes; historical differences remain':'Not demonstrated'],[c.potential_supply_equal===undefined?'Complete final snapshot':'Complete captured final state (legacy + environment)',c.full_final_snapshot_equal?'Equal':'Different — material equality does not imply full-state equality']] .map(r=>[r[0],text(r[1])]));
 if(c.potential_supply_equal!==undefined)table(p,['Environmental physical qualification','Result'],[['Total potential supply',c.potential_supply_equal?'Equal':'Different'],['Initial owned placement',c.owned_placement_equal?'Equal':'Different — placement intervention'],['Cache access',c.cache_access_equal?'Equal':'Different — access intervention'],['Final owned food / hunger',c.owned_final_material_equal?'Equal':'Different'],['Final cache stock',c.final_cache_stock_equal?'Equal':'Different — owned equality does not imply material equality'],['Complete legacy export (includes audit history)',c.legacy_final_snapshot_equal?'Equal':'Different'],['Final directed trust',c.final_trust_equal?'Equal':'Different']]);
 if(c.environmental_comparison){table(p,['021 retention comparison','Result'],[['Knowledge modes',`${c.environmental_comparison.left_mode} / ${c.environmental_comparison.right_mode}`],['Acquisition policies',`${c.environmental_comparison.left_policy} / ${c.environmental_comparison.right_policy}`],['Same external glance slots',c.environmental_comparison.passive_slots_match?'Yes':'No'],['Both modes isolate retention',c.environmental_comparison.pure_retention_modes?'Yes, subject to matched initial conditions':'No: sensing frequency/channel control'],['Final owned environmental slots',c.final_environmental_memory_equal?'Equal':'Different'],['Final semantic directed trust',c.final_trust_equal?'Equal (missing = 0)':'Different values'],['Raw trust-map shape',c.raw_final_trust_equal?'Equal':'Different; may be only stored zeros']]);note(p,c.environmental_comparison.note);}
 if(c.model_hunger_area){table(p,['Individual','Left model hunger area','Right model hunger area'],c.model_hunger_area.left.map((v,i)=>[i,v,c.model_hunger_area.right[i]]));note(p,c.model_hunger_area.note);}
 if(c.first_choice?.tick!==undefined)button(p,'Jump to first voluntary difference',()=>go(nearest(S.data,c.first_choice.tick)));
 if(c.material_different_shared_ticks.length)button(p,'Jump to first material difference',()=>go(nearest(S.data,c.material_different_shared_ticks[0].tick)));
 button(p,'Compare final recorded states',()=>go(S.data.timeline.length-1));
 note(p,c.note,true);
 const pair=$('pair');pair.replaceChildren();const tick=current().tick;
 for(const d of [S.data,S.right]) {
  const col=el('div');col.append(el('h3',`${d.trial.mode} · ${d.trial.case}`));const x=snapshot(d,tick);
  col.append(el('p',`World tick ${tick}; snapshot captured at ${x.point.state.tick}, point ${x.index}`));
  if(S.lens==='observer')table(col,['Individual','Food','Hunger'],x.point.state.agents.map(a=>[a.id,a.food,a.hunger]));
  if(S.lens==='observer'&&x.point.cache)note(col,`Observer cache ${x.point.cache.id}: stock ${x.point.cache.stock}; ${x.point.pending.length} pending claims. Exact captured time above.`);
  const moments=d.timeline.filter(m=>m.kind==='decision'&&m.tick===tick);const m=moments[0];const a=m?projection(d,m,S.actor):null;
  if(a&&a.phase==='activity'){col.append(el('p',`Individual ${S.actor}: ${human(a.selected)}`));table(col,['Activity','Score','Eligible'],a.candidates.map(c=>[c.action,c.score,c.eligible?'Yes':'No']));}
  else note(col,'No captured activity decision for this individual at this exact tick.');
  if(S.lens==='observer')details(col,'Trust in captured state',x.point.state.agents.map(a=>({id:a.id,trust:a.trust})));
  pair.append(col);
 }
}
function setupPresets() {
 if(S.catalog.some(e=>e.experiment==='025'&&e.registered))for(const [title,left,right]of [['025 · Prepare then eat / stale failure',0,1],['025 · Competing final batch',4,5],['025 · Costly sharing',16,17],['025 · Unknown raw null',2,3],['025 · Escrow at cutoff',26,27],['025 · Work across reports',30,31],['025 · Longer finite supply',42,43]])button($('presets'),title,guard(async()=>{const a=S.catalog.find(e=>e.experiment==='025'&&e.registered&&e.record===left),b=S.catalog.find(e=>e.experiment==='025'&&e.registered&&e.record===right);if(!a||!b)throw new Error('025preset source absent');await openHistory(a.id);if(S.data?.source.id===a.id)await compareWith(b.id);}));

 if(S.catalog.some(e=>e.experiment==='024'&&e.registered))for(const [title,left,right]of [['024 · Costly earlier meal',0,1],['024 · Meal relief and continued contact',6,7],['024 · Finite exhaustion / persistent divergence',18,19],['024 · Genuine valuation null',20,21],['024 · Unchanged inquiry with real receipt',12,13],['024 · Longer exhaustion',52,53]])button($('presets'),title,guard(async()=>{const a=S.catalog.find(e=>e.experiment==='024'&&e.registered&&e.record===left),b=S.catalog.find(e=>e.experiment==='024'&&e.registered&&e.record===right);if(!a||!b)throw new Error('024preset source absent');await openHistory(a.id);if(S.data?.source.id===a.id)await compareWith(b.id);}));
 if(S.catalog.some(e=>e.experiment==='023'&&e.registered))for(const [title,left,right]of [['023 · Reporting crossing / mixed costs',0,1],['023 · Continuous inquiry / short control',12,13],['023 · Long finite resource exhaustion',18,19],['023 · Genuine behavior null',20,21],['023 · Longer inquiry control',36,37]])button($('presets'),title,guard(async()=>{const a=S.catalog.find(e=>e.experiment==='023'&&e.registered&&e.record===left),b=S.catalog.find(e=>e.experiment==='023'&&e.registered&&e.record===right);if(!a||!b)throw new Error('023preset source absent');await openHistory(a.id);if(S.data?.source.id===a.id)await compareWith(b.id);}));
 for(const entry of S.catalog.filter(e=>e.experiment==='023'&&!e.registered))button($('presets'),'023 · Open actual prototype run',guard(()=>openHistory(entry.id)));
 if(S.catalog.some(e=>e.experiment==='022'))for(const [title,left,right] of [
  ['022 · Two simultaneous resource scenes',0,1],['022 · Different completion times',4,5],
  ['022 · Claim and meal beside resource work',12,13],['022 · Busy recipient / missed contact',10,11],
  ['022 · One grant / one depleted claim',14,15],['022 · Atomic inquiry / capacity null',20,21],
  ['022 · Complete capacity null',30,31],['022 · Long resource exhaustion',34,35],['022 · Mixed individual costs',2,3]
 ])button($('presets'),title,guard(async()=>{const a=S.catalog.find(e=>e.experiment==='022'&&e.record===left),b=S.catalog.find(e=>e.experiment==='022'&&e.record===right);if(!a||!b)throw new Error('Native022 preset not indexed');await openHistory(a.id);if(S.data?.source.id===a.id)await compareWith(b.id);}));
 for(const [title,left,right] of [['021 · Useful continuity / later social difference',81,82],['021 · Rival depletion / old knowledge',80,82],['021 · Full-cost stale information',31,32],['021 · Mixed individual distribution',1,2],['021 · Expiry removes work / loses continuity',162,163],['021 · Complete null',121,122],['021 · Stored-zero trust control',161,163]])button($('presets'),title,guard(async()=>{const a=S.catalog.find(e=>e.experiment==='021'&&e.record===left),b=S.catalog.find(e=>e.experiment==='021'&&e.record===right);if(!a||!b)throw new Error('021 preset not indexed');await openHistory(a.id);if(S.data?.source.id===a.id)await compareWith(b.id);}));
 const finiteFind=(name,mode)=>S.catalog.find(e=>e.experiment==='020'&&e.seed==='42'&&e.case===name&&e.mode===mode&&e.config.duration===4&&e.config.window===24&&e.config.order===(name==='lost_contact'?'Descending':'Ascending'));
 for(const [title,name,left,right] of [['020 · Sharing after contention','sharing','Closed','Need'],['020 · Missed donor / costly failure','lost_contact','Closed','Acquire'],['020 · Last unit, three claimants','contested','Need','Acquire'],['020 · Exhaustion and empty contact','ongoing','Owned','Need'],['020 · Complete null','empty','Ignored','Need'],['020 · Material reconvergence','contact_bias','Need','Acquire']])button($('presets'),title,guard(async()=>{const a=finiteFind(name,left),b=finiteFind(name,right);if(!a||!b)throw new Error('020 preset not indexed');await openHistory(a.id);if(S.data?.source.id===a.id)await compareWith(b.id);}));
 const defs=[['A · Small score change','useful_contact','Baseline','Scale110',{}],['B · Private cooperation reversal','unproductive_contact','Baseline','Scale110',{}],['C · Deadline / delayed meal','ongoing','EatFirst','Deadline',{stock:4}],['D · Planned inquiry loses arbitration','information','Baseline','Scale90',{order:'Descending'}],['E · Conservative short window','useful_contact','Baseline','Baseline',{}, {window:6}],['F · Complete null','no_food','Baseline','Scale90',{}],['G · Observation gaps','recurring','Baseline','Baseline',{idle_gap:5}, {}]];
 const find=(name,mode,changes)=>S.catalog.find(e=>e.experiment==='019'&&e.seed==='42'&&e.case===name&&e.mode===mode&&Object.entries({...{window:24,idle_gap:1,interval:4,stock:12,hunger:20,order:'Ascending'},...changes}).every(([k,v])=>e.config[k]===v));
 defs.forEach(([title,name,l,r,c,rc])=>button($('presets'),title,guard(async()=>{const a=find(name,l,c),b=find(name,r,rc??c);if(!a||!b)throw new Error('Preset not in indexed catalog');await openHistory(a.id);if(S.data?.source.id===a.id)await compareWith(b.id);})));
}
$('load').addEventListener('click',guard(()=>openHistory($('trial').value)));
$('search').addEventListener('input',()=>{const matches=S.catalog.filter(e=>label(e).toLowerCase().includes($('search').value.toLowerCase()));options($('trial'),matches);$('load').disabled=matches.length===0;status(matches.length?`${matches.length} recorded trials match. Select one to open.`:'No matching trials. Change or clear the search.');});
$('actor').addEventListener('change',()=>{S.actor=Number($('actor').value);render();});
$('chain-select').addEventListener('change',()=>{S.chain=$('chain-select').value;if(NativeConcurrent.is(S.data))S.causalNode=null;renderCausal();});
$('encounter-filter').addEventListener('change',renderCausal);$('encounter-after').addEventListener('input',renderCausal);
$('actor-lens').addEventListener('click',()=>{S.lens='actor';render();});$('observer-lens').addEventListener('click',()=>{S.lens='observer';render();});
$('previous').addEventListener('click',()=>go(S.index-1));$('next').addEventListener('click',()=>go(S.index+1));$('play').addEventListener('click',play);
$('speed').addEventListener('change',()=>{if(S.timer){pause();play();}});$('scrub').addEventListener('input',()=>{pause();go(Number($('scrub').value));});
$('prev-event').addEventListener('click',()=>nextWhere(significant,-1));$('next-event').addEventListener('click',()=>nextWhere(significant));
$('next-decision').addEventListener('click',()=>nextWhere(m=>m.kind==='decision'&&projection(S.data,m,S.actor)?.phase==='activity'));
$('jump').addEventListener('click',()=>{if(S.data)go(nearest(S.data,Number($('tick').value)));});$('kind').addEventListener('change',renderList);
$('open-compare').addEventListener('click',()=>{$('compare-panel').hidden=false;$('compare-panel').scrollIntoView({block:'start',behavior:'smooth'});});
$('compare').addEventListener('click',guard(()=>compareWith($('other').value)));
$('inspect-right').addEventListener('click',guard(async()=>{if(!S.right)return;const key=S.right.source.id,tick=current().tick;await openHistory(key);if(S.data.source.id===key&&tick>(NativeConcurrent.is(S.data)?S.data.trial.start:S.data.trial.clock.epoch))go(nearest(S.data,tick));}));
$('close-compare').addEventListener('click',()=>{S.pairGeneration++;clearComparison();$('compare-panel').hidden=true;});
$('download').addEventListener('click',()=>{if(!S.data)return;const a=el('a');a.href='/api/export?id='+encodeURIComponent(S.data.source.id);a.download=`history-${S.data.source.id}.json`;document.body.append(a);a.click();a.remove();});
$('import').addEventListener('change',guard(async()=>{const file=$('import').files[0];if(!file)return;pause();const generation=++S.generation;$('import').value='';if(file.size>64*1024*1024)throw new Error('Export exceeds 64 MiB bound');status('Checking imported export against source membership…');const imported=configure(await api('/api/import',{method:'POST',headers:{'Content-Type':'application/json'},body:await file.text()}));if(generation!==S.generation)return;S.data=imported;S.causalNode=null;S.chain=null;configureActorOptions(imported);S.index=0;clearComparison();$('search').value='';options($('trial'),S.catalog,S.data.source.id);$('load').disabled=false;$('compare-panel').hidden=true;render();status('Imported export verified against original registered trial.');}));
document.addEventListener('keydown',e=>{if(['INPUT','SELECT','TEXTAREA','BUTTON','SUMMARY'].includes(document.activeElement?.tagName))return;if(e.key==='ArrowRight'){e.preventDefault();go(S.index+1);}if(e.key==='ArrowLeft'){e.preventDefault();go(S.index-1);}if(e.key===' '){e.preventDefault();play();}if(e.key.toLowerCase()==='n')$('next-decision').click();if(e.key==='[')nextWhere(significant,-1);if(e.key===']')nextWhere(significant);});
const narrowLayout=window.matchMedia('(max-width:700px)');
function arrangeLibrary(){ $('library-drawer').open=!narrowLayout.matches; }
arrangeLibrary();narrowLayout.addEventListener('change',arrangeLibrary);
guard(async()=>{const data=await api('/api/catalog');S.catalog=data.entries;
 const query=$('search').value.toLowerCase(),matches=S.catalog.filter(e=>label(e).toLowerCase().includes(query));
 options($('trial'),matches);$('load').disabled=matches.length===0;options($('other'),S.catalog);setupPresets();
 const demo=query?null:S.catalog.find(e=>e.experiment==='019'&&e.case==='useful_contact'&&e.mode==='Baseline'&&e.seed==='42'&&e.archive==='seed-42.jsonl.gz');
 if(demo)await openHistory(demo.id);else status(matches.length?'Catalog ready. Select a trial.':'No matching trials. Change or clear the search.');})();
