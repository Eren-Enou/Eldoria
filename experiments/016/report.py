"""Derive tables from independent016 reconstruction; never generate outcomes."""
import csv,json,collections,gzip,itertools
from pathlib import Path
D=Path(__file__).resolve().parent
x=json.loads((D/'analysis.json').read_bytes()); rows=x['trials']
with (D/'comparisons.csv').open('w',newline='',encoding='utf-8') as f:
 fields=list(x['comparisons'][0]);w=csv.DictWriter(f,fieldnames=fields,lineterminator='\n');w.writeheader()
 for p in x['comparisons']:w.writerow(dict(p,config=json.dumps(p['config'],sort_keys=True,separators=(',',':'))))
by={(r['case'],r['mode'],json.dumps(r['config'],sort_keys=True)):r for r in rows};orders=[]
for t in rows:
 c=t['config']
 if c['order']=='Ascending':continue
 a=by[t['case'],t['mode'],json.dumps(dict(c,order='Ascending'),sort_keys=True)]
 orders.append(dict(case=t['case'],mode=t['mode'],order=c['order'],choices_changed=a['choices']!=t['choices'],encounters_changed=a['encounters']!=t['encounters'],final_material_changed=[(x['food'],x['hunger']) for x in a['final']]!=[(x['food'],x['hunger']) for x in t['final']],ascending_transfers=a['metrics'].get('transfers',0),other_transfers=t['metrics'].get('transfers',0)))
with (D/'order-comparisons.csv').open('w',newline='',encoding='utf-8') as f:
 w=csv.DictWriter(f,fieldnames=list(orders[0]),lineterminator='\n');w.writeheader();w.writerows(orders)
groups=collections.defaultdict(collections.Counter)
for t in rows:groups[(json.dumps(t['config'],sort_keys=True),t['mode'])].update(t['metrics'])
text=['Experiment016:1000 exact replayed preregistered worlds;250 independent seed42 histories.', '500 overlapping within-config pairs,140 initial-input-matched ordering contrasts.', 'Supplementary10 seed1 traces are selected post-evaluation from the same matrix,not new held-out evidence.', 'Counts are not welfare. Unchanged vs Control changes recipient control; learning comparisons use Control.']
for (c,m),values in sorted(groups.items()):text.append(json.dumps(dict(config=json.loads(c),mode=m,**values),sort_keys=True))
(D/'report.txt').write_text('\n'.join(text)+'\n',encoding='utf-8')
summary=json.loads(gzip.decompress((D/'summary.json.gz').read_bytes()));matched=collections.defaultdict(dict)
for t in summary:matched[(t['seed'],t['case'],json.dumps(t['config'],sort_keys=True))][t['mode']]=t
stats={}
for mode in ['Recent','Typed','Untyped']:
 out=collections.Counter()
 for modes in matched.values():
  a,b=modes['Control'],modes[mode];out['pairs']+=1
  for name,condition in [('fewer_invitations',b['invitations']<a['invitations']),('fewer_declines',b['declined']<a['declined']),('more_declines',b['declined']>a['declined']),('more_consumed',b['consumed']>a['consumed']),('less_consumed',b['consumed']<a['consumed']),('lower_hunger_sum',sum(b['final_hunger'])<sum(a['final_hunger'])),('higher_hunger_sum',sum(b['final_hunger'])>sum(a['final_hunger']))]:out[name]+=condition
 stats[mode]=dict(out)
(D/'all-seed-comparisons.json').write_text(json.dumps(stats,indent=2)+'\n',encoding='utf-8')
