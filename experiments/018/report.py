"""Authorized018 report writer; all measurements remain distinct, no welfare score."""
import collections,csv,gzip,itertools,json
from pathlib import Path
D=Path(__file__).resolve().parent
PAIRS=[('Fixed','Recurring'),('FixedContact','ContactOnly'),('ContactOnly','Recurring'),('Serial','Recurring'),('Recurring','EatFirst'),('Recurring','NoHistory')]
def write_csv(name,rows):
    with (D/name).open('w',encoding='utf-8',newline='') as f:
        w=csv.DictWriter(f,fieldnames=list(rows[0]),lineterminator='\n');w.writeheader()
        for row in rows:w.writerow({k:json.dumps(v,sort_keys=True,separators=(',',':')) if isinstance(v,(dict,list)) else v for k,v in row.items()})
def counts(groups,independent):
    result={}
    for left,right in PAIRS:
        c=collections.Counter()
        for modes in groups.values():
            a,b=modes[left],modes[right];c['pairs']+=1
            if independent:
                food=lambda r:[x['food'] for x in r['final']];hunger=lambda r:[x['hunger'] for x in r['final']];n=lambda r,k:r['metrics'].get(k,0)
                periods=[(x['food'],x['hunger'])!=(y['food'],y['hunger']) for x,y in zip(a['period_states'],b['period_states'],strict=True)]
                c['any_period_material']+=any(periods);c['successive_period_material']+=any(x and y for x,y in zip(periods,periods[1:]));c['reconverged_material']+=any(periods) and not periods[-1]
                c['final_trust_changed']+=[x['trust'] for x in a['final']]!=[x['trust'] for x in b['final']]
                c['inquiry_selection_changed']+=[(q['owner'],q['selected']) for q in a['inquiries']]!=[(q['owner'],q['selected']) for q in b['inquiries']]
                c['resource_action_changed']+=[{k:v for k,v in e.items() if k!='tick'} for e in a['resource_actions']]!=[{k:v for k,v in e.items() if k!='tick'} for e in b['resource_actions']]
            else:food=lambda r:r['final_food'];hunger=lambda r:r['final_hunger'];n=lambda r,k:r[k]
            c['final_material_changed']+=(food(a),hunger(a))!=(food(b),hunger(b));c['transfer_count_changed']+=n(a,'transfers')!=n(b,'transfers');c['query_count_changed']+=n(a,'queries')!=n(b,'queries');c['consumed_more']+=n(b,'consumed')>n(a,'consumed');c['consumed_less']+=n(b,'consumed')<n(a,'consumed');c['someone_hungrier']+=any(y>x for x,y in zip(hunger(a),hunger(b),strict=True));c['someone_less_hungry']+=any(y<x for x,y in zip(hunger(a),hunger(b),strict=True))
        result[left+'/'+right]=dict(c)
    return result
def main():
    x=json.loads((D/'analysis.json').read_bytes());rows=x['trials'];groups=collections.defaultdict(dict)
    for r in rows:groups[r['case'],json.dumps(r['config'],sort_keys=True)][r['mode']]=r
    summaries=json.loads(gzip.decompress((D/'summary.json.gz').read_bytes()));all_groups=collections.defaultdict(dict)
    for r in summaries:all_groups[r['seed'],r['case'],json.dumps(r['config'],sort_keys=True)][r['mode']]=r
    core=counts(groups,True);all_seeds=counts(all_groups,False)
    (D/'all-seed-comparisons.json').write_text(json.dumps(all_seeds,indent=2)+'\n',encoding='utf-8');(D/'independent-comparisons.json').write_text(json.dumps(core,indent=2)+'\n',encoding='utf-8')
    write_csv('comparisons.csv',x['comparisons']);order=[]
    for (case,config),modes in groups.items():
        c=json.loads(config)
        if c['order']=='Ascending':continue
        baseline=groups[case,json.dumps(dict(c,order='Ascending'),sort_keys=True)]
        for mode,b in modes.items():
            a=baseline[mode];order.append(dict(case=case,mode=mode,order=c['order'],transfers_ascending=a['metrics'].get('transfers',0),transfers_other=b['metrics'].get('transfers',0),queries_ascending=a['metrics'].get('queries',0),queries_other=b['metrics'].get('queries',0),hunger_reduced_ascending=a['metrics'].get('hunger_reduced',0),hunger_reduced_other=b['metrics'].get('hunger_reduced',0),final_hunger_ascending=[p['hunger'] for p in a['final']],final_hunger_other=[p['hunger'] for p in b['final']]))
    assert len(order)==56;write_csv('order-comparisons.csv',order)
    trajectory=[dict(case=r['case'],mode=r['mode'],config=r['config'],period=index,**p) for r in rows for index,p in enumerate(r['period_states'])];write_csv('trajectories.csv',trajectory)
    metrics=[dict(case=r['case'],mode=r['mode'],config=r['config'],metrics=r['metrics'],per_actor=r['per_actor'],final_food=[a['food'] for a in r['final']],final_hunger=[a['hunger'] for a in r['final']],final_trust=[a['trust'] for a in r['final']],consumption_hunger_histogram=r['consumption_hunger_histogram']) for r in rows];write_csv('measurements-by-trial.csv',metrics)
    txt=['Experiment018:1064 registered worlds each exactly replayed;266 independently reconstructed fullseed42 histories.', '38 matched groups;798 overlapping comparisons;56 opening-matched order controls;10 corruption categories.', 'Seven matched focal privacy twins;preserved invalid hook candidate is rejected. No statistical replication or global welfare ranking.', 'Independent seed42 comparisons:']
    txt.extend(json.dumps(dict(pair=k,**v),sort_keys=True) for k,v in core.items());txt.append('All-seed summary comparisons:');txt.extend(json.dumps(dict(pair=k,**v),sort_keys=True) for k,v in all_seeds.items())
    order_totals=[]
    for mode in ('Recurring','Serial','EatFirst','NoHistory'):
        for order_name in ('Ascending','Descending','Rotating'):
            selected=[r for r in rows if r['case'] in ('competing_eater','information','refusal','ongoing') and r['mode']==mode and r['config']==dict(budget=24,periods=6,long_periods=24,order=order_name,interval=4,stock=12)]
            assert len(selected)==4
            totals=dict(mode=mode,order=order_name,worlds=4,transfers=sum(r['metrics'].get('transfers',0) for r in selected),queries=sum(r['metrics'].get('queries',0) for r in selected),consumed=sum(r['metrics'].get('consumed',0) for r in selected),hunger_reduced=sum(r['metrics'].get('hunger_reduced',0) for r in selected));order_totals.append(totals);txt.append(json.dumps(totals,sort_keys=True))
    (D/'ordering-totals.json').write_text(json.dumps(order_totals,indent=2)+'\n',encoding='utf-8');(D/'report.txt').write_text('\n'.join(txt)+'\n',encoding='utf-8')
if __name__=='__main__':main()
