"""Observer-only024 derived analysis. No policy implementation or outcome tuning."""
import argparse,collections,json
from pathlib import Path
import review024 as r
import review024_findings as f

def local(t,tick,owner):
 for bi,b in enumerate(t['history']['boundaries']):
  if b['tick']==tick:
   for di,d in enumerate(b['choices']):
    if d['input']['legacy']['own']['id']==owner:
     audit=next(v for v in t['valuations']if(v['boundary'],v['choice'])==(bi,di))
     return dict(ref=f'history.boundaries[{bi}].choices[{di}]',valuation_ref=f'valuations[{t["valuations"].index(audit)}]',input=d['input'],candidates=d['candidates'],selected=d['selected'],contact=d['contact'],contact_candidates=d['contact_candidates'],score_audit=audit)
 raise AssertionError('missing exact local choice')

def executions(t):
 h=t['history'];out=collections.defaultdict(list)
 for n,e in enumerate(r.core(h)['events']):
  if n>=h['bootstrap_events']:out[e['tick']].append(dict(ref=f'history.final_state.base.base.base.base.base.events[{n}]',kind='resource',owners=e['participants'],selected=e['decision']['selected'],transferred=e['transferred']))
 for n,g in enumerate(h['grants']):out[g['tick']].append(dict(ref=f'history.grants[{n}]',kind='claim',owner=g['owner'],outcome=g['outcome']))
 for n,m in enumerate(r.core(h)['cognition']['consumption']):out[m['tick']].append(dict(ref=f'history.final_state.base.base.base.base.base.cognition.consumption[{n}]',kind='meal',owner=m['actor'],food_before=m['food_before'],food_after=m['food_after'],hunger_before=m['hunger_before'],hunger_after=m['hunger_after']))
 for n,q in enumerate(h['final_state']['base']['base']['base']['base']['inquiry']['records']):out[q['tick']].append(dict(ref=f'history.final_state.base.base.base.base.inquiry.records[{n}]',kind='query',owner=q['decision']['input']['owner'],selected=q['decision']['selected']))
 return dict(out)

def inquiry(t,ordinal):
 snap=t['history']['final_state'];compact=snap['base']['base']['base'];prov=compact['provenance'];out=[]
 for n,q in enumerate(compact['base']['inquiry']['records']):
  pq=next(x for x in prov['queries']if x['inquiry']==q['id']);rid=pq['receipt'];receipt=None if rid is None else prov['receipts'][rid];learn=next(x for x in snap['learning']['records']if x['inquiry']==q['id'])
  out.append(dict(ordinal=ordinal,ref=f'history.final_state.base.base.base.base.inquiry.records[{n}]',start=q['tick']-q['time_spent'],end=q['tick'],owner=q['decision']['input']['owner'],selected=q['decision']['selected'],native_response=q['response'],provenance_receipt=rid,cognitive_receipt=None if receipt is None else receipt['cognition_receipt'],novelty_value=q['realized_value'],observed_progress=learn['outcome']))
 return out

def per_world(t,ordinal,metric,independent):
 h=t['history'];core=r.core(h);missed=[];consumed=[];acquired=[]
 for bi,b in enumerate(h['boundaries']):
  for di,d in enumerate(b['choices']):
   own=d['input']['legacy']['own'];eat=next(c for c in d['candidates']if c['action']=='Consume')
   row=dict(ref=f'history.boundaries[{bi}].choices[{di}]',tick=b['tick'],owner=own['id'],food=own['food'],hunger=own['hunger'],selected=d['selected'])
   if eat['eligible']and d['selected']!='Consume':missed.append(row)
   if d['selected']=='Consume':consumed.append(row)
   if d['selected']=='Acquire':acquired.append(row)
 material=[]
 for n,e in enumerate(core['events']):
  if n<h['bootstrap_events']or not e['transferred']:continue
  recipient=e['decision']['actor']
  if e['decision']['food']==0 and e['decision']['hunger']>0:
   later=next((dict(ref=f'history.final_state.base.base.base.base.base.cognition.consumption[{mi}]',record=m)for mi,m in enumerate(core['cognition']['consumption'])if m['actor']==recipient and m['tick']>e['tick']),None)
   material.append(dict(ref=f'history.final_state.base.base.base.base.base.events[{n}]',recipient=recipient,tick=e['tick'],hunger=e['decision']['hunger'],next_meal=later))
 assert metric['hunger_area']==independent['area']
 return dict(ordinal=ordinal,config=t['config'],metrics=metric,independent=independent,missed_eligible_meals=missed,first_consumed=consumed[:1],first_acquisition=acquired[:1],transfers_to_empty_hungry=material,inquiries=inquiry(t,ordinal))

def analyze(archive,metrics):
 independent=f.inspect(archive);assert len(metrics)==56;worlds=[];pairs=[];previous=None
 for n,t in enumerate(r.read(archive)):
  worlds.append(per_world(t,n,metrics[n],independent['worlds'][n]))
  if n>=54:continue
  if n%2==0:previous=t;continue
  p=dict(independent['pairs'][n//2]);first=p['first_selected']
  if first:p['first_selected_local']=dict(baseline=local(previous,first['tick'],first['owner']),meal_relief=local(t,first['tick'],first['owner']))
  tick=p['first_execution'];p['first_executed_records']=None if tick is None else dict(tick=tick,baseline=executions(previous).get(tick,[]),meal_relief=executions(t).get(tick,[]))
  p['baseline_metrics']=metrics[n-1];p['meal_relief_metrics']=metrics[n]
  pairs.append(p)
 summary=dict(independent['summary'])
 summary.update(pairs_more_meals=sum(p['meal_relief_metrics']['meals']>p['baseline_metrics']['meals']for p in pairs),pairs_fewer_meals=sum(p['meal_relief_metrics']['meals']<p['baseline_metrics']['meals']for p in pairs),pairs_more_social=sum(p['meal_relief_metrics']['social']>p['baseline_metrics']['social']for p in pairs),pairs_fewer_social=sum(p['meal_relief_metrics']['social']<p['baseline_metrics']['social']for p in pairs),pairs_any_higher_before_nominal_area=sum(any(x>0 for x in p['before_nominal_hunger_area_delta'])for p in pairs),pairs_any_lower_before_nominal_area=sum(any(x<0 for x in p['before_nominal_hunger_area_delta'])for p in pairs))
 return dict(summary=summary,pairs=pairs,worlds=worlds,qualification='Observer-only dimensions, not a welfare ranking. Transfers may circulate food; information receipt, novelty, retained progress and nutritional use are distinct. Held-out rows are deterministic parameter sensitivity.')

def main():
 p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('--metrics',required=True);p.add_argument('--output');p.add_argument('--expected');a=p.parse_args();result=analyze(a.archive,json.loads(Path(a.metrics).read_bytes()))
 if a.expected:assert result==json.loads(Path(a.expected).read_bytes())
 if a.output:
  with Path(a.output).open('x',encoding='utf-8')as out:json.dump(result,out,separators=(',',':'));out.write('\n')
 print(json.dumps(result['summary'],sort_keys=True))
if __name__=='__main__':main()
