"""Read-only observational diagnosis of original023; no policy replacement/probes."""
from pathlib import Path
import argparse,collections,gzip,json
from review023_findings import base,resource_value
ROOT=Path(__file__).resolve().parents[2]

def div(n,d):return (abs(n)//d)*(-1 if n<0 else 1)

def terms(own,partner):
 trust=own['trust'].get(str(partner),0)
 memories=[m for m in own['memories'] if m['partner']==partner][-4:]
 valence=max(-40,min(40,sum(m['valence'] for m in memories)))
 request=own['hunger']-35+div(trust,4)-own['caution']//4
 offer=None if not own['food'] else own['generosity']-own['hunger']//2-own['caution']//2+trust+valence-(60 if own['hunger']>=60 and own['food']<=1 else 0)
 result=dict(partner=partner,trust=trust,retained_valence=valence,retained_events=[m['event'] for m in memories],request=request,offer=offer,leave=0,hunger_term=own['hunger']//2,protocol_penalty=2)
 result['score']=max(0,request,offer if offer is not None else -10**9)+result['hunger_term']-2
 assert result['score']==resource_value(own,partner)
 return result

def inspect():
 worlds=[];saved={};counter=collections.Counter()
 with gzip.open(ROOT/'experiments/023/trials.jsonl.gz','rt',encoding='utf-8') as stream:
  for ordinal,line in enumerate(stream):
   t=json.loads(line);h=t['history'];core=base(h);events=core['events'][h['bootstrap_events']:];stats=collections.Counter();examples={}
   for bi,b in enumerate(h['boundaries']):
    for di,d in enumerate(b['choices']):
     i=d['input'];e=i['legacy'];own=e['own'];eat=next(c for c in d['candidates'] if c['action']=='Consume');stats['decisions']+=1
     stats['owned_food_decisions']+=own['food']>0
     can=bool(eat['eligible']);missed=can and d['selected']!='Consume'
     stats['eligible_consumption']+=can;stats['nonselected_eligible_consumption']+=missed
     stats['nonselected_consumption_at_hunger100']+=missed and own['hunger']==100
     if d['selected']=='Consume':key='consumed_choice'
     elif missed and own['hunger']==100:key='owned_food_neglect'
     elif missed:key='eligible_food_deferred'
     elif d['selected']=='Acquire':key='environmental_choice'
     else:key=None
     admissions=[a for a in h['admissions'] if a['source']==bi and a['invitation']['proposer']==own['id']]
     selected=d['contact'];contact_terms=None
     if selected!='Wait' and selected['Invite']['kind']=='Resource':contact_terms=terms(own,selected['Invite']['recipient'])
     data=dict(ordinal=ordinal,ref=f'history.boundaries[{bi}].choices[{di}]',tick=b['tick'],owner=own['id'],food=own['food'],hunger=own['hunger'],retained_environment=i['retained'],inquiry_lane=i['inquiry_lane_free'],candidates=d['candidates'],contact=selected,contact_terms=contact_terms,selected=d['selected'],admissions=admissions)
     if key and key not in examples:examples[key]=data
     if missed:
      stats['deferred_with_pending']+=any(p['invitation']['proposer']==own['id'] for p in b['pending'])
      for a in admissions:
       stats['deferred_admission_'+a['reason']]+=1
       if a['commitment'] is not None:
        c=next(x['commitment'] for x in h['transitions'] if not x['completed'] and x['commitment']['id']==a['commitment'])
        if isinstance(c['work'],dict) and 'Resource'in c['work']:
         scene=c['work']['Resource']['scene'];actual=[v for v in events if v['scene']==scene]
         transfer=sum(v['transferred'] for v in actual);stats['deferred_contact_with_transfer']+=transfer>0;stats['deferred_contact_zero_transfer']+=not transfer
         category='deferred_then_transfer' if transfer else 'deferred_then_zero_transfer'
         if category not in examples:examples[category]=dict(data,executed_scene=scene,event_refs=[f'history.final_state.base.base.base.base.base.events[{core["events"].index(v)}]' for v in actual],executed_actions=[v['decision']['selected'] for v in actual],transferred=transfer,actual_start=c['start'],actual_end=actual[-1]['tick'])
     if ordinal==19 and own['id']==2 and b['tick'] in (72,77):saved[f'19_actor2_{b["tick"]}']=data
     if ordinal in (10,11) and own['id']==2 and b['tick']==7:saved[f'{ordinal}_actor2_7']=data
   scenes=collections.defaultdict(list)
   for event in events:scenes[event['scene']].append(event)
   social=[]
   for scene,es in scenes.items():
    if any(e['transferred'] for e in es):
     transfer=next(e for e in es if e['transferred']);recipient=transfer['decision']['actor'];before=transfer['decision']['food'];after=transfer['balances_after'][transfer['participants'].index(recipient)]
     meals=[(mi,m) for mi,m in enumerate(core['cognition']['consumption']) if m['actor']==recipient and m['tick']>transfer['tick']]
     if before==0 and transfer['decision']['hunger']>0:
      social.append(dict(scene=scene,transfer_event=transfer['id'],tick=transfer['tick'],recipient=recipient,recipient_food_before=before,recipient_food_after=after,recipient_hunger=transfer['decision']['hunger'],next_meal_ref=None if not meals else f'history.final_state.base.base.base.base.base.cognition.consumption[{meals[0][0]}]',next_meal=None if not meals else meals[0][1],qualification='Actual food delivery to an empty recipient; later consumption is recorded only when present. No unique-unit or universal-welfare claim'))
   worlds.append(dict(ordinal=ordinal,case=t['config']['case'],mode=t['config']['mode'],counts=dict(stats),examples=examples,material_to_empty_recipient=social[:2]))
   counter.update(stats)
 return dict(schema='observational-023-activity-diagnosis/v1',reference='336ab08354bda6dbdb5171e4066e28ea28344b6b',worlds=worlds,summary=dict(counter),exact_witnesses=saved,qualification='Observer classifies only after execution. No outcome class is policy input; scene counts are not a universal utility ranking. No treatment/probe is run.')

def main():
 p=argparse.ArgumentParser();p.add_argument('--output');p.add_argument('--expected');a=p.parse_args();result=inspect()
 if a.expected:assert result==json.loads(Path(a.expected).read_bytes())
 if a.output:
  with Path(a.output).open('x',encoding='utf-8')as f:json.dump(result,f,indent=2);f.write('\n')
 print(json.dumps(result['summary'],sort_keys=True))
if __name__=='__main__':main()
