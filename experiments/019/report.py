"""Authorized019 report writer. No policy tuning or global welfare score."""
import collections,csv,gzip,itertools,json
from pathlib import Path
D=Path(__file__).resolve().parent
PAIRS=[('Baseline','EatFirst'),('Baseline','Deadline'),('EatFirst','Deadline'),('Baseline','Scale90'),('Baseline','Scale110'),('Baseline','NoHistory')]
def key(r):return r['seed'],r['case'],r['mode'],json.dumps(r['config'],sort_keys=True)
def write_csv(name,rows):
    with (D/name).open('w',encoding='utf-8',newline='') as f:
        w=csv.DictWriter(f,fieldnames=list(rows[0]),lineterminator='\n');w.writeheader()
        for r in rows:w.writerow({k:json.dumps(v,sort_keys=True,separators=(',',':')) if isinstance(v,(dict,list)) else v for k,v in r.items()})
def dump(name,value):(D/name).write_text(json.dumps(value,indent=2)+'\n',encoding='utf-8',newline='\n')
def main():
    x=json.loads((D/'analysis.json').read_bytes());rows=x['trials'];summaries=json.loads(gzip.decompress((D/'summary.json.gz').read_bytes()));lookup={key(r):r for r in summaries};assert len(lookup)==1296
    for r in rows:
        s=lookup[key(r)];assert r['hunger_area']==s['hunger_area'];assert [a['food'] for a in r['final']]==s['final_food'] and [a['hunger'] for a in r['final']]==s['final_hunger']
        for name in ('transfers','queries','consumed','idle_ticks'):assert r['metrics'].get(name,0)==s[name],(key(r),name)
        assert sum(p['to']-p['from'] for p in r['pauses'])==s['pause_ticks']
    prefix=json.loads((D/'findings.json').read_bytes());prefixes={(p['seed'],p['case'],json.dumps(p['config'],sort_keys=True),p['left'],p['right']):p for p in prefix['pairs']}
    contrasts={};primary={key(r):r for r in rows if r['seed']==42}
    for left,right in PAIRS:
        c=collections.Counter()
        for r in x['comparisons']:
            if r['seed']!=42 or (r['left'],r['right'])!=(left,right):continue
            c['pairs']+=1
            first=prefixes[r['seed'],r['case'],json.dumps(r['config'],sort_keys=True),r['left'],r['right']]
            c['voluntary_choice_changed']+=first['first'] is not None
            c['complete_snapshot_changed']+=not first['final_snapshot_equal']
            for f in ('activity_trace_changed','inquiry_selection_changed','inquiry_outcome_changed','resource_action_changed','trust_changed','material_changed','reconverged','someone_lower_area','someone_higher_area'):c[f]+=r[f]
            c['any_period_material']+=r['material_periods']>0;c['successive_material']+=r['successive_material_periods']
        assert c['pairs']==54;contrasts[left+'/'+right]=dict(c)
    dump('independent-comparisons.json',contrasts)
    groups=collections.defaultdict(dict)
    for r in summaries:groups[r['seed'],r['case'],json.dumps(r['config'],sort_keys=True)][r['mode']]=r
    all_seeds={}
    for left,right in PAIRS:
        c=collections.Counter()
        for modes in groups.values():
            a,b=modes[left],modes[right];c['pairs']+=1
            c['material_changed']+=(a['final_food'],a['final_hunger'])!=(b['final_food'],b['final_hunger'])
            for n in ('transfers','queries','consumed'):c[n+'_changed']+=a[n]!=b[n]
            c['someone_lower_area']+=any(v<u for u,v in zip(a['hunger_area'],b['hunger_area'],strict=True));c['someone_higher_area']+=any(v>u for u,v in zip(a['hunger_area'],b['hunger_area'],strict=True))
            c['someone_hungrier']+=any(v>u for u,v in zip(a['final_hunger'],b['final_hunger'],strict=True));c['someone_less_hungry']+=any(v<u for u,v in zip(a['final_hunger'],b['final_hunger'],strict=True))
        assert c['pairs']==216;all_seeds[left+'/'+right]=dict(c)
    dump('all-seed-comparisons.json',all_seeds);write_csv('comparisons.csv',x['comparisons'])
    default=dict(window=24,idle_gap=1,interval=4,stock=12,hunger=20,order='Ascending');variations=[]
    for r in rows:
        if r['seed']!=42 or r['config']==default:continue
        b=primary[(42,r['case'],r['mode'],json.dumps(default,sort_keys=True))]
        changed=[k for k in default if r['config'][k]!=default[k]];assert len(changed)==1
        axis=changed[0];variations.append(dict(case=r['case'],mode=r['mode'],axis=axis,value=r['config'][axis],kind='timing' if axis in ('window','idle_gap') else 'order' if axis=='order' else 'physics_or_initial',final_food_base=[a['food'] for a in b['final']],final_food_other=[a['food'] for a in r['final']],final_hunger_base=[a['hunger'] for a in b['final']],final_hunger_other=[a['hunger'] for a in r['final']],area_base=b['hunger_area'],area_other=r['hunger_area'],transfers_base=b['metrics'].get('transfers',0),transfers_other=r['metrics'].get('transfers',0),queries_base=b['metrics'].get('queries',0),queries_other=r['metrics'].get('queries',0),gap_ticks=r['metrics'].get('sampling_gap_ticks',0)))
    assert len(variations)==210;write_csv('sensitivity-comparisons.csv',variations)
    trajectory=[dict(seed=r['seed'],case=r['case'],mode=r['mode'],config=r['config'],period=i,**state) for r in rows for i,state in enumerate(r['period_states'])];write_csv('trajectories.csv',trajectory)
    trial_metrics=[dict(seed=r['seed'],case=r['case'],mode=r['mode'],config=r['config'],metrics=r['metrics'],hunger_area=r['hunger_area'],per_actor=r['per_actor'],final_food=[a['food'] for a in r['final']],final_hunger=[a['hunger'] for a in r['final']],final_trust=[a['trust'] for a in r['final']],meal_hunger_histogram=r['consumption_hunger_histogram']) for r in rows];write_csv('measurements-by-trial.csv',trial_metrics)
    report=['019:1296 registered worlds,336 independent full histories;54 primary groups/810 overlapping pairs,12 preregistered seed1 histories,13 corruption categories.','No statistical replication or global welfare ranking.','Primary independent contrasts:']
    report.extend(json.dumps(dict(pair=k,**v),sort_keys=True) for k,v in contrasts.items());report.append('All-seed summary contrasts:');report.extend(json.dumps(dict(pair=k,**v),sort_keys=True) for k,v in all_seeds.items());(D/'report.txt').write_text('\n'.join(report)+'\n',encoding='utf-8',newline='\n')
    print(json.dumps(dict(primary=contrasts,all_seeds=all_seeds),sort_keys=True))
if __name__=='__main__':main()
