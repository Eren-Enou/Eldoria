"""Derive017 descriptive comparisons from independently reconstructed evidence."""
import collections,csv,gzip,json
from pathlib import Path
D=Path(__file__).resolve().parent
x=json.loads((D/'analysis.json').read_bytes());rows=x['trials']
with (D/'comparisons.csv').open('w',newline='',encoding='utf-8') as f:
    w=csv.DictWriter(f,fieldnames=list(x['comparisons'][0]),lineterminator='\n');w.writeheader()
    for r in x['comparisons']:w.writerow(dict(r,config=json.dumps(r['config'],sort_keys=True,separators=(',',':'))))
groups=collections.defaultdict(dict)
for r in rows:groups[(r['case'],json.dumps(r['config'],sort_keys=True))][r['mode']]=r
orders=[]
for (case,config),modes in groups.items():
    c=json.loads(config)
    if c['order']=='Ascending':continue
    baseline=groups[(case,json.dumps(dict(c,order='Ascending'),sort_keys=True))]
    for mode,r in modes.items():
        a=baseline[mode]
        orders.append(dict(case=case,mode=mode,order=c['order'],transfers_ascending=a['metrics'].get('transfers',0),transfers_other=r['metrics'].get('transfers',0),consumed_ascending=a['metrics'].get('consumed',0),consumed_other=r['metrics'].get('consumed',0),material_changed=[(p['food'],p['hunger']) for p in a['final']]!=[(p['food'],p['hunger']) for p in r['final']]))
with (D/'order-comparisons.csv').open('w',newline='',encoding='utf-8') as f:
    w=csv.DictWriter(f,fieldnames=list(orders[0]),lineterminator='\n');w.writeheader();w.writerows(orders)
summary=json.loads(gzip.decompress((D/'summary.json.gz').read_bytes()));matched=collections.defaultdict(dict)
for r in summary:matched[(r['seed'],r['case'],json.dumps(r['config'],sort_keys=True))][r['mode']]=r
comparisons={}
for left,right in [('Serial','Committed'),('SerialActive','Active'),('Active','NoHistory'),('Committed','Active')]:
    count=collections.Counter()
    for g in matched.values():
        a,b=g[left],g[right];count['pairs']+=1
        count['material_changed']+=(a['final_food'],a['final_hunger'])!=(b['final_food'],b['final_hunger'])
        for field in ('invitations','queries','transfers','consumed'):
            count[field+'_more']+=b[field]>a[field];count[field+'_less']+=b[field]<a[field]
        count['someone_more_hungry']+=any(v>u for u,v in zip(a['final_hunger'],b['final_hunger']))
    comparisons[left+'/'+right]=dict(count)
(D/'all-seed-comparisons.json').write_text(json.dumps(comparisons,indent=2)+'\n',encoding='utf-8')
totals=collections.defaultdict(collections.Counter)
for r in rows:
    if r['config']['budget']==24 and r['config']['periods']==6:
        totals[(r['config']['order'],r['mode'])].update(r['metrics'])
text=['Experiment017:940 preregistered worlds,each exactly replayed;235 independently reconstructed seed42 histories.',
      '47 matched groups;470 overlapping mode comparisons;130 opening-matched ordering controls;seven scratch corruption categories.',
      'No postsetup hunger/profile/busy intervention. Resource commitments stepped;inquiry remains atomic. Counts are not welfare.']
for (order,mode),metrics in sorted(totals.items()):text.append(json.dumps(dict(order=order,mode=mode,**metrics),sort_keys=True))
text.append(json.dumps(comparisons,sort_keys=True))
(D/'report.txt').write_text('\n'.join(text)+'\n',encoding='utf-8')
