"""Fresh read-only026 review: independent arithmetic/masks/ties and chronology.
No simulation transitions or archived evaluator conclusion is imported here.
The full isolated gate separately supplies exhaustive physical/cognitive checking.
"""
import argparse,copy,gzip,hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
REVISIONS=[
 ('registration','759385d3e6a4dc3e40e816ef0585e335c2640d56'),
 ('v1_runtime','42d3ad359b7a002926ed3f57e65e99e96ad8545a'),
 ('v1_activation','d2321f1b5d8f15fb62f584a95e27b77abaeee240'),
 ('failure_and_C1_declaration','6f44686c1d3b7a85656a1ce9244c9a37bc6799ab'),
 ('C1_runtime','22b51485326eeb297d8740074c0ea57d1387247c'),
 ('C1_activation','2a5bec1a17646a951de612e01362230c01f6d36d'),
 ('C1_results_and_I1','6b05b388afc995bc9615db595e0c683dae04cf67')]
def git(*a):return subprocess.check_output(['git',*a],cwd=ROOT)
def digest(b):return hashlib.sha256(b).hexdigest()
def score(choice,mode):
 i=choice['input'];assert set(i)=={'legacy','retained','duration','inquiry_lane_free','raw_retained'}
 own=i['legacy']['own'];now=i['legacy']['now'];end=i['legacy']['end'];cell=i['raw_retained']
 assert i['retained'] is None or cell is None
 duration=None
 if cell:
  o=cell['observation']
  if (cell['owner']==own['id'] and o['id']==0 and o['observed_at']<=now and
      max(0,o['stock']-cell['own_takes'])>=2 and o['accessible'] and o['extraction_ticks']>0 and
      o['preparation_ticks']>0 and (o['input_units'],o['edible_units'],o['waste_units'])==(2,1,1)):
   duration=o['extraction_ticks']+o['preparation_ticks']
 eligible=bool(own['food']==0 and own['hunger']>0 and any(p['id']==own['id'] and p['available'] for p in i['legacy']['people']) and duration and 0<duration<=33 and now+duration<=end)
 terms=choice['candidates'];assert [t['action']for t in terms]==['Wait','Consume','Invite','Acquire','Prepare']
 t=terms[4];base=-1 if not eligible else own['hunger'] if mode=='Baseline' else min(100,own['hunger']+5*(duration//4))
 expected=dict(action='Prepare',eligible=eligible,base_score=base,meal_relief=0,work_effort=duration if eligible else 0,final_score=base-(duration if eligible else 0))
 assert t==expected,('Prepare reconstruction',t,expected)
 assert not(t['eligible']and terms[1]['eligible'])
 assert choice['selected']==max(terms,key=lambda t:t['final_score'])['action'],'strict old-first tie'
 return eligible

def material_and_need(body):
 h=body['history'];m=body['material'];processes={p['commitment']:p for p in m['processes']}
 assert len(processes)==len(m['processes'])
 seen=set();counts={k:0 for k in ('Extracted','Insufficient','Prepared','Failed')}
 raw=m['initial_raw'];waste=0;escrow={}
 for o in m['operations']:
  cid=o['commitment'];p=processes[cid];kind=o['outcome'];phase=0 if kind in ('Extracted','Insufficient')else 1
  assert (cid,phase)not in seen;seen.add((cid,phase))
  assert o['tick']==p['extract_at'if phase==0 else'finish_at'] and o['owner']==p['owner']
  assert o['raw_before']==raw and o['waste_before']==waste
  if phase==0:
   acquired=min(2,raw)if raw>=2 else 0
   assert kind==('Extracted'if acquired else'Insufficient') and o['escrow_before']==0
   raw-=acquired;escrow[cid]=acquired
   assert o['food_before']==o['food_after']
  else:
   assert (cid,0)in seen and o['escrow_before']==escrow[cid]
   success=int(escrow[cid]==2);assert kind==('Prepared'if success else'Failed')
   assert o['food_after']-o['food_before']==success
   waste+=success;escrow[cid]=0
  assert (o['raw_after'],o['waste_after'],o['escrow_after'])==(raw,waste,escrow[cid]);counts[kind]+=1
 assert len(seen)==2*len(processes) and not any(escrow.values())and not m['final_active']
 assert (raw,waste)==(m['final_raw'],m['final_waste'])
 core=h['final_state']
 while 'base'in core:core=core['base']
 meals=core['cognition']['consumption'];areas=[];meal_counts=[]
 for actor in h['initial']:
  owner=actor['id'];hunger=actor['hunger'];last=h['start'];area=0;events=[]
  for need in h['needs']:events.append((need['tick'],0,need['before'][owner],need['after'][owner]))
  actual=[a for a in meals if a['actor']==owner];meal_counts.append(sum(a['consumed']for a in actual))
  for meal in actual:
   assert meal['food_before']-meal['food_after']==meal['consumed']==1
   assert meal['hunger_after']==max(0,meal['hunger_before']-40)
   events.append((meal['tick'],1,meal['hunger_before'],meal['hunger_after']))
  for tick,kind,before,after in sorted(events):
   area+=(tick-last)*hunger;last=tick;assert hunger==before
   if kind==0:assert after==min(100,before+5)
   hunger=after
  area+=(h['end']-last)*hunger;areas.append(area)
  assert hunger==h['final_agents'][owner]['hunger']
 assert raw+waste+h['final_stock']+sum(a['food']for a in h['final_agents'])+sum(meal_counts)==h['initial_total']
 return dict(outcomes=counts,hunger_area=areas,meals=meal_counts)

def inspect():
 chronology=[]
 for index,(label,rev) in enumerate(REVISIONS):
  subprocess.run(['git','merge-base','--is-ancestor',rev,'HEAD'],cwd=ROOT,check=True)
  if index:subprocess.run(['git','merge-base','--is-ancestor',REVISIONS[index-1][1],rev],cwd=ROOT,check=True)
  chronology.append(dict(label=label,revision=rev))
 for name in ('specification.md','policy.json','registered-matrix.json'):
  path='experiments/026/'+name
  assert git('show',REVISIONS[0][1]+':'+path)==git('show',REVISIONS[-1][1]+':'+path),'prospective design drift'
 for path,rev in [('experiments/026/source-freeze.json',REVISIONS[1][1]),('experiments/026/continuation-c1/source-freeze.json',REVISIONS[4][1])]:
  for name,sha in json.loads((ROOT/path).read_bytes()).items():
   assert digest(git('show',rev+':'+name).replace(b'\r\n',b'\n'))==sha
 assert not any(n.startswith('experiments/026/continuation-c1/trials') for n in git('ls-tree','-r','--name-only',REVISIONS[5][1]).decode().splitlines())
 assert git('show',REVISIONS[3][1]+':experiments/026/failure/manifest.json')
 assert git('show',REVISIONS[-1][1]+':examples/support/verify026_c1_isolated.py')
 matrix=json.loads((ROOT/'experiments/026/registered-matrix.json').read_bytes())
 rows=[];by_mode={'Baseline':0,'ProjectedNeed':0};extensions=[];witness=None
 with gzip.open(ROOT/'experiments/026/continuation-c1/trials.jsonl.gz','rb')as f:
  for index,line in enumerate(f):
   t=json.loads(line);inner=t['trial'];cfg=inner['config'];assert cfg==matrix[index]
   body=inner['body'];physical=material_and_need(body);choices=eligible=responses=0
   for b in body['history']['boundaries']:
    for choice in b['choices']:
     is_eligible=score(choice,cfg['mode']);eligible+=is_eligible;choices+=1
     if is_eligible and cfg['mode']=='Baseline' and witness is None:witness=copy.deepcopy(choice)
    for response in b['responses']:
     cs=response['candidates'];assert cs[0]==dict(reply='Wait',score=0)
     assert all(c['score']==0 for c in cs if c['reply']=='Continue')
     assert response['selected']==max(cs,key=lambda c:c['score'])['reply']
     responses+=1
   rows.append(dict(row=index,source025=cfg['source025'],mode=cfg['mode'],choices=choices,eligible=eligible,responses=responses,physical=physical))
   by_mode[cfg['mode']]+=eligible
   if t['response_extensions']:extensions.append(dict(row=index,references=t['response_extensions']))
 assert len(rows)==100 and len(extensions)==1 and extensions[0]['row']==43 and len(extensions[0]['references'])==2
 assert witness is not None
 controls=[]
 for kind in ('hunger','score','duration','eligibility','candidate_order','selected'):
  forged=copy.deepcopy(witness)
  if kind=='hunger':forged['input']['legacy']['own']['hunger']+=1
  elif kind=='score':forged['candidates'][4]['final_score']+=1
  elif kind=='duration':forged['input']['raw_retained']['observation']['preparation_ticks']+=1
  elif kind=='eligibility':forged['candidates'][4]['eligible']=False
  elif kind=='candidate_order':forged['candidates'].reverse()
  else:forged['selected']='Consume'
  try:score(forged,'Baseline')
  except AssertionError:controls.append(kind)
  else:raise AssertionError(('independent review accepted scratch corruption',kind))
 comp=json.loads((ROOT/'experiments/026/continuation-c1/comparisons.json').read_bytes())
 assert comp['primary']['changed_selection']==[15,17,19,21,23,39,41,43] and len(comp['primary']['execution_null'])==40
 for row in rows:
  summary=comp['pairs'][row['source025']]['baseline'if row['mode']=='Baseline'else'treatment']
  physical=row['physical'];assert physical['hunger_area']==summary['hunger_area']and physical['meals']==summary['meals']
  assert physical['outcomes']['Prepared']==summary['success']and physical['outcomes']['Failed']==summary['failures']
 # Independent same-input policy restriction across every allowed scalar H,D.
 for h in range(101):
  for d in range(1,34):assert min(100,h+5*(d//4))-d>=h-d
 return dict(schema='experiment-026-fresh-foundation-review/v1',chronology=chronology,worlds=rows,total_worlds=len(rows),eligible_by_mode=by_mode,response_extensions=extensions,decision='ACCEPT WITH QUALIFICATION',qualification='Finite causal existence only; v1 failed43, C1 prospective undefined-domain extension, I1 post-run transport. Full physical/cognitive/consent reconstruction and native replay are separate mandatory gates; no utility/default adoption.',scalar_monotonicity_cases=101*33,scratch_corruptions_rejected=controls)

def main():
 p=argparse.ArgumentParser();p.add_argument('--output');p.add_argument('--expected');a=p.parse_args();result=inspect()
 if a.expected:assert result==json.loads(Path(a.expected).read_bytes())
 if a.output:
  with open(a.output,'x',encoding='utf-8')as out:json.dump(result,out,indent=2);out.write('\n')
 print('Fresh026 chronology/local eligibility/arithmetic/ties/Continue0 review PASS',result['total_worlds'])
if __name__=='__main__':main()
