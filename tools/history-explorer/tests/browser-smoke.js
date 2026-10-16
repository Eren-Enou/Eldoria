/* Codex cua_repl browser smoke recipe. Bind `tab` to the local app first.
   Paste this function into cua_repl. Run each request and verification in separate
   tool calls (browser-control calls have a short shared execution deadline):
     await historyExplorerSmoke(tab, 'core')
     await historyExplorerSmoke(tab, 'request:A')
     await historyExplorerSmoke(tab, 'verify:A')
   Repeat request/verify for B,C,D,E,F,G,long, then run 'keyboard' and 'empty'.
   Uses UI/DOM only, never app globals, policy calls or fetch.
*/
async function historyExplorerSmoke(tab, stage) {
 const report=[], check=(ok,message)=>{if(!ok)throw new Error(message);report.push(message);};
 const n=id=>tab.playwright.locator('#'+id);
 const ready=()=>n('status').filter({hasText:'Loaded verified original'}).waitFor({state:'visible',timeoutMs:30000});
 const titles={A:'A · Small score change',B:'B · Private cooperation reversal',C:'C · Deadline / delayed meal',D:'D · Planned inquiry loses arbitration',E:'E · Conservative short window',F:'F · Complete null',G:'G · Observation gaps'};
 const [action,part]=stage.split(':');
 if(stage==='core') {
  await tab.reload();await ready();
  check((await n('cards').innerText()).includes('Owned food 2 · hunger 80'),'Authentic record103 actor0 state');
  check(/Consume\s+Yes\s+80/.test(await n('inspect').innerText()),'Recorded candidate scores');
  historyExplorerSmoke.focal=await n('raw').textContent();
  await n('actor').selectOption('1');check((await n('cards').innerText()).includes('Owned food 6'),'Individual selection');await n('actor').selectOption('0');
  const before=await n('scrub').evaluate(e=>e.value);await n('next').click();await n('previous').click();check(await n('scrub').evaluate(e=>e.value)===before,'Previous/next');
  await n('speed').selectOption('4');const title=await n('moment-title').innerText();await n('play').click();await n('moment-title').filter({hasNotText:title}).waitFor({state:'visible'});await n('play').click();check(await n('play').innerText()==='Play','Play progresses; pause stops');
  await n('tick').fill('0');await n('jump').click();await n('next-event').click();check((await n('moment-kind').innerText()).includes('MEAL'),'Significant event navigation');
  await n('observer-lens').click();check((await n('cards').innerText()).includes('Food 1'),'Objective meal consequence');
  await tab.playwright.getByRole('button',{name:'Next actual resource action (Observer)',exact:true}).click();check((await n('moment-title').innerText()).includes('Offer'),'Executed resource reference');
  await n('actor').selectOption('1');await n('actor-lens').click();check((await n('inspect').innerText()).includes('Executed resource decision'),'Resource local-input adapter');await n('actor').selectOption('0');
 } else if(action==='request') {
  if(!await n('library-drawer').evaluate(e=>e.open))await tab.playwright.locator('#library-drawer>summary').click();
  if(part==='long') {
   await n('search').fill('longitudinal · Scale110');const labels=await tab.playwright.locator('#trial option').allTextContents();await n('trial').selectOption({label:labels[0]});await n('load').click();
  } else await tab.playwright.getByRole('button',{name:titles[part],exact:true}).click();
  report.push('Requested '+part+' through real UI');
 } else if(action==='verify') {
  if(part==='long') {
   await ready();await n('kind').selectOption('all');await n('scrub').press('End');check(await n('scrub').evaluate(e=>e.value===e.max),'Long history reaches final record');check(await tab.playwright.locator('#events li').count()<=45,'Bounded timeline DOM');await n('scrub').press('Home');check(await tab.playwright.locator('#events li').count()===45,'Long list renders a bounded real page');
  } else {
   await n('status').filter({hasText:'Paired comparison ready'}).waitFor({state:'visible',timeoutMs:30000});
   if(part==='A') {
    await n('actor-lens').click();check(!await n('compare-summary').isVisible(),'Whole-world comparison hidden in Actor Perspective');await n('observer-lens').click();const summary=await n('compare-summary').innerText();check(summary.includes('First recorded voluntary choice difference'),'Matched first voluntary difference');check(summary.includes('"tick":1'),'Actual execution includes earlier meal');await n('close-compare').click();check(!await n('compare-panel').isVisible(),'Return to single history');
   } else if(part==='B') {
    await n('actor-lens').click();check(await n('raw').textContent()===historyExplorerSmoke.focal,'Private twin leaves displayed focal input identical');await n('observer-lens').click();const agents=await tab.playwright.locator('#inspect details').filter({hasText:'All current recorded agents (observer only)'}).locator('pre').textContent();check(JSON.parse(agents).find(a=>a.id===1).generosity===0,'Observer reveals differing private state');
   } else if(part==='C') {
    await n('observer-lens').click();check((await n('compare-summary').innerText()).includes('Yes; historical differences remain'),'Material reconvergence preserves history');await tab.playwright.getByRole('button',{name:'Jump to first voluntary difference',exact:true}).click();check((await n('moment-kind').innerText()).includes('TICK 16'),'Jump to actual causal choice');
   } else if(part==='D') {
    await n('observer-lens').click();await n('tick').fill('45');await n('jump').click();if(!(await n('moment-kind').innerText()).includes('DECISION'))await n('next-decision').click();check((await n('inspect').innerText()).includes('This step has no executed inquiry'),'Unexecuted inquiry remains unexecuted');
   } else if(part==='E') check((await n('compare-summary').innerText()).includes('Different — timing or other configuration treatment'),'Timing mismatch explicit');
   else if(part==='F') {const summary=await n('compare-summary').innerText();check(summary.includes('No observed voluntary choice difference'),'Complete null');check(/Complete final snapshot\s+Equal/.test(summary),'Complete final Snapshot equality');}
   else if(part==='G') {await n('observer-lens').click();await n('kind').selectOption('gap');await tab.playwright.locator('#events button').first().click();check((await n('moment-title').innerText()).includes('Observation gap'),'Elapsed gap navigation');}
  }
 } else if(stage==='keyboard') {
  const before=await n('scrub').evaluate(e=>Number(e.value));await tab.playwright.locator('main').press('ArrowRight');check(await n('scrub').evaluate(e=>Number(e.value))===before+1,'Keyboard moment navigation');await tab.playwright.locator('main').press('n');check((await n('moment-kind').innerText()).includes('DECISION'),'Keyboard next individual decision');await tab.playwright.locator('main').press('Space');check(await n('play').innerText()==='Pause','Keyboard play');await tab.playwright.locator('main').press('Space');check(await n('play').innerText()==='Play','Keyboard pause');
 } else if(stage==='empty') {await n('search').fill('no such registered trial');check(!await n('load').isEnabled(),'Empty search state');await n('search').fill('');}
 else throw new Error('Unknown smoke stage');
 return report;
}
