"""Derive compact015 reports from independently reconstructed evidence; no simulation."""
import csv,json,collections
from pathlib import Path
P=Path(__file__).resolve().parent
a=json.loads((P/'analysis.json').read_bytes());rows=a['trials'];core=dict(budget=24,periods=6,long_periods=24,order='Ascending')
with (P/'comparisons.csv').open('w',newline='',encoding='utf-8') as f:
    w=csv.DictWriter(f,fieldnames=list(a['comparisons'][0]));w.writeheader();w.writerows(a['comparisons'])
orders=[]
for row in rows:
    if row['config']!=core:continue
    for order in ['Descending','Rotating']:
        b=next(b for b in rows if b['case']==row['case'] and b['mode']==row['mode'] and b['config']==dict(core,order=order))
        material=lambda r:[(v['food'],v['hunger']) for v in r['final']]
        orders.append(dict(case=row['case'],mode=row['mode'],order=order,transfers_ascending=row['metrics'].get('transfers',0),transfers_other=b['metrics'].get('transfers',0),consumed_ascending=row['metrics'].get('consumed',0),consumed_other=b['metrics'].get('consumed',0),material_changed=material(row)!=material(b),trajectory_changed=row['trajectory']!=b['trajectory']))
with (P/'order-comparisons.csv').open('w',newline='',encoding='utf-8') as f:
    w=csv.DictWriter(f,fieldnames=list(orders[0]));w.writeheader();w.writerows(orders)
lines=['Experiment015:195 independent seed42 histories;780 preregistered worlds exact-replayed.','195 overlapping within-config initiation comparisons;78 matched ordering contrasts.','No contact/transfer count is universal utility. Reciprocal invitations share one execution.']
configs=[]
for row in rows:
    if row['config'] not in configs:configs.append(row['config'])
for c in configs:
    for mode in ['Simple','History','NoHistory']:
        metrics=collections.Counter()
        for row in rows:
            if row['config']==c and row['mode']==mode:metrics.update(row['metrics'])
        lines.append(json.dumps(dict(config=c,mode=mode,**metrics),sort_keys=True))
lines.append('See results.md for causal references, missed/redundant outcomes, ordering effects and ecological limits.')
(P/'report.txt').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print('195 comparisons,78 ordering contrasts and15 aggregate rows written from independent data.')
