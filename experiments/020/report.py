"""Descriptive020 report from verified original histories. No outcome tuning."""
import collections,csv,gzip,hashlib,itertools,json
from pathlib import Path
D=Path(__file__).resolve().parent
def dump(name,x):(D/name).write_text(json.dumps(x,indent=2)+'\n',encoding='utf-8',newline='\n')
def csvfile(name,rows):
    with (D/name).open('w',encoding='utf-8',newline='') as f:
        w=csv.DictWriter(f,fieldnames=list(rows[0]),lineterminator='\n');w.writeheader()
        for r in rows:w.writerow({k:json.dumps(v,separators=(',',':'),sort_keys=True) if isinstance(v,(list,dict)) else v for k,v in r.items()})
def key(t):return t['seed'],t['case'],json.dumps(t['config'],sort_keys=True)
def legacy(t):return t['final_state']['base']['base']['base']['base']['base']
def selections(w):return [None if d is None else d['selected'] for d in w['choices']]
def first(a,b):
    if a['initial']['state']!=b['initial']['state']:return dict(kind='initial placement',left=None,right=None)
    for pno,(p,q) in enumerate(zip(a['periods'],b['periods'],strict=True)):
        for sno,(x,y) in enumerate(zip(p['steps'],q['steps'])):
            if selections(x)!=selections(y):
                assert a['points'][x['activity']['observed']]['state']==b['points'][y['activity']['observed']]['state']
                return dict(kind='voluntary activity',period=pno,step=sno,tick=a['points'][x['activity']['observed']]['state']['tick'],left=selections(x),right=selections(y))
    assert a['final_state']==b['final_state'],'unexplained divergence without different voluntary activity'
    return None
def comparison(a,b,ra,rb):
    assert a['profiles']==b['profiles'] and a['initial_total']==b['initial_total']
    x,y=ra['final'],rb['final'];pa,pb=ra['period_states'],rb['period_states'];assert [p['tick'] for p in pa]==[p['tick'] for p in pb]
    return dict(seed=a['seed'],case=a['case'],config=a['config'],left=a['mode'],right=b['mode'],initial_owned_equal=a['initial']['state']==b['initial']['state'],potential_food=a['initial_total'],first=first(a,b),complete_snapshot_changed=a['final_state']!=b['final_state'],final_material_changed=[(v['food'],v['hunger']) for v in x]!=[(v['food'],v['hunger']) for v in y],trust_changed=[v['trust'] for v in x]!=[v['trust'] for v in y],material_periods=sum((p['food'],p['hunger'])!=(q['food'],q['hunger']) for p,q in zip(pa,pb,strict=True)),left_area=ra['hunger_area'],right_area=rb['hunger_area'],someone_lower_area=any(v<u for u,v in zip(ra['hunger_area'],rb['hunger_area'],strict=True)),someone_higher_area=any(v>u for u,v in zip(ra['hunger_area'],rb['hunger_area'],strict=True)),left_metrics=ra['metrics'],right_metrics=rb['metrics'])
def main():
    analysis=json.loads(gzip.decompress((D/'analysis.json.gz').read_bytes()));rows=analysis['trials'];assert len(rows)==164
    measurements=[];slices=[];comparisons=[];group=[];summary={};longitudinal=[];inquiries=[]
    def finish():
        if not group:return
        for (ia,a),(ib,b) in itertools.combinations(group,2):comparisons.append(comparison(a,b,rows[ia],rows[ib]))
        modes={t['mode']:t for _,t in group}
        if 'Closed' in modes and 'Ignored' in modes:assert modes['Closed']['final_state']==modes['Ignored']['final_state']
    with gzip.open(D/'trials.jsonl.gz','rt',encoding='utf-8') as f:
        for index,line in enumerate(f):
            t=json.loads(line);r=rows[index];assert (r['seed'],r['case'],r['mode'],r['config'])==(t['seed'],t['case'],t['mode'],t['config'])
            if group and key(group[0][1])!=key(t):finish();group=[]
            group.append((index,t));base=legacy(t);steps=[w for p in t['periods'] for w in p['steps']]
            meals=[c for w in steps for c in w['activity']['consumption']];events=base['events'][t['initial']['state']['events']:]
            assert r['metrics'].get('Granted',0)==sum(x['outcome']=='Granted' for x in t['records'])
            assert sum(a['food'] for a in r['final'])+r['cache']['stock']+len(meals)==t['initial_total']
            m=dict(record=index,seed=t['seed'],case=t['case'],mode=t['mode'],config=t['config'],initial_total=t['initial_total'],granted=r['metrics'].get('Granted',0),failed=r['metrics'].get('Depleted',0),claim_actor_ticks=sum(x['attempt']['end']-x['attempt']['start'] for x in t['records']),meals_by_actor=[sum(c['actor']==i for c in meals) for i in range(3)],hunger_area=r['hunger_area'],transfers=sum(e['transferred'] for e in events),queries=r['metrics'].get('queries',0),final_food=[a['food'] for a in r['final']],final_hunger=[a['hunger'] for a in r['final']],final_cache=r['cache']['stock'],steps=len(steps),points=len(t['points']),json_bytes=len(line.encode())-1)
            measurements.append(m)
            inquiries.extend(dict(record=index,seed=t['seed'],case=t['case'],mode=t['mode'],config=t['config'],**q) for q in r['inquiries'])
            if index<70 and t['case'] in ('sharing','lost_contact','contested','information','stranded','ongoing') and t['mode'] in ('Closed','Need','Acquire'):
                start=t['initial']['state']['tick'];limit=start+24
                slices.append(dict(record=index,case=t['case'],mode=t['mode'],epoch=start,claims=t['records'][:4],steps=[dict(period=pno,step=sno,source=f'periods[{pno}].steps[{sno}]',before=w['activity']['before'],after=w['activity']['after'],start=t['points'][w['activity']['before']]['state']['tick'],selected=selections(w),admissions=w['admissions'],completed=w['completed'],skipped_responses=w['skipped_responses'],allocations=w['activity']['allocations'],consumption=w['activity']['consumption']) for pno,p in enumerate(t['periods']) for sno,w in enumerate(p['steps']) if t['points'][w['activity']['before']]['state']['tick']<limit],events=[e for e in events if e['tick']<=limit],inquiries=r['inquiries']))
            if t['case'] in ('ongoing','stranded'):
                last_positive=max((i for i,p in enumerate(t['points']) if sum(a['food'] for a in p['state']['agents'])),default=-1)
                exhausted=t['points'][last_positive+1]['state']['tick']
                terminals=[e for e in events if e['tick']>exhausted and e['outcome'] is not None]
                zero=sum(not any(x['transferred'] for x in events if x['scene']==e['scene']) for e in terminals)
                longitudinal.append(dict(record=index,seed=t['seed'],case=t['case'],mode=t['mode'],config=t['config'],owned_exhaustion=exhausted,cache_depletion=next((p['state']['tick'] for p in t['points'] if p['cache']['stock']==0),None),last_grant=max((a['completed_at'] for a in t['records'] if a['outcome']=='Granted'),default=None),terminal_scenes_after_owned_exhaustion=len(terminals),zero_transfer_scenes_after_owned_exhaustion=zero,final_cache=r['cache']['stock']))
    finish();assert len(comparisons)==274
    primary=[c for c in comparisons if c['seed']==42 and c['config']['duration']==4 and c['config']['order']==('Descending' if c['case']=='lost_contact' else 'Ascending') and c['case'] in {r['case'] for r in rows[:70]}]
    for left,right in [('Closed','Ignored'),('Ignored','Need'),('Closed','Need'),('Need','Acquire'),('Owned','Need')]:
        cs=[c for c in primary if (c['left'],c['right'])==(left,right)]
        assert len(cs)==14
        summary[left+'/'+right]=dict(pairs=len(cs),voluntary_differences=sum(c['first'] is not None and c['first']['kind']=='voluntary activity' for c in cs),snapshot_differences=sum(c['complete_snapshot_changed'] for c in cs),material_during_periods=sum(c['material_periods']>0 for c in cs),final_material_differences=sum(c['final_material_changed'] for c in cs),trust_differences=sum(c['trust_changed'] for c in cs),someone_lower_area=sum(c['someone_lower_area'] for c in cs),someone_higher_area=sum(c['someone_higher_area'] for c in cs))
    totals=dict(sum((collections.Counter(r['metrics']) for r in rows),collections.Counter()))
    csvfile('measurements.csv',measurements);csvfile('comparisons.csv',comparisons);csvfile('longitudinal.csv',longitudinal)
    dump('causal-slices.json',slices);dump('findings.json',dict(worlds=164,groups=40,pairs=274,primary=summary,totals=totals,corruptions=analysis['corruptions']))
    outcomes=[q['outcome'] for q in inquiries if q['outcome'] is not None]
    dump('inquiry-observations.json',dict(methods=dict(collections.Counter(q['selected']['Ask']['strategy'] for q in inquiries)),queries=len(inquiries),receipts=sum(o['receipt_acquired'] for o in outcomes),retained_changes=sum((o['support_before'],o['status_before'])!=(o['support_after'],o['status_after']) for o in outcomes),rows=inquiries))
    print(json.dumps(dict(primary=summary,totals=totals),sort_keys=True))
if __name__=='__main__':main()
