"""Complementary018 review from primitive archives; no evaluator/reader imports.
Re-derive elapsed pressure, all point/decision hunger, and observed scope/null counts.
"""
import collections,gzip,hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
D=ROOT/'experiments/018'
def records(directory,name):
    with gzip.open(directory/(name+'.jsonl.gz'),'rt',encoding='utf-8') as f:
        for line in f:yield json.loads(line)
def physical(t):
    clock=t['clock'];epoch=clock['epoch'];interval=clock['settings']['interval']
    inc=0 if t['mode'] in ('Fixed','FixedContact') else 5
    assert clock['settings']==dict(interval=t['config']['interval'],increment=inc)
    h=[a['hunger'] for a in t['initial']['state']['agents']];at={epoch:list(h)}
    meals=sorted([c for p in t['periods'] for w in p['steps'] for c in w['activity']['consumption']],key=lambda c:c['tick'])
    cursor=epoch;j=0
    def eat(until):
        nonlocal j
        while j<len(meals) and meals[j]['tick']<=until:
            c=meals[j];i=c['actor'];assert c['hunger_before']==h[i] and c['consumed']==1 and c['food_before']>0
            assert c['food_after']==c['food_before']-1
            h[i]=max(0,h[i]-40);assert c['hunger_after']==h[i];at[c['tick']]=list(h);j+=1
    for n in t['needs']:
        eat(cursor)
        assert n['from']==cursor and n['to']>cursor
        assert n['before']==h,('lost progression',t['case'],t['mode'],n['from'],n['before'],h)
        pulses=(n['to']-epoch)//interval-(cursor-epoch)//interval
        assert n['pulses']==pulses
        h[:]=[min(100,x+inc*pulses) for x in h]
        assert n['after']==h
        cursor=n['to'];at[cursor]=list(h)
    eat(cursor)
    assert cursor==clock['last']==t['points'][t['periods'][-1]['after']]['state']['tick'] and j==len(meals)
    for p in t['points']:
        assert [a['hunger'] for a in p['state']['agents']]==at[p['state']['tick']],('point hunger',t['case'],t['mode'],p['state']['tick'])
    base=t['final_state']['base']['base']['base']['base']['base']
    assert [a['hunger'] for a in base['agents']]==h
    for p in t['periods']:
        for w in p['steps']:
            s=w['activity'];now=t['points'][s['observed']]['state']['tick']
            for i,d in enumerate(s['decisions']):
                if d:assert d['contact']['input']['own']['hunger']==at[now][i]
    initial=sum(a['food'] for a in t['initial']['state']['agents'])
    assert sum(a['food'] for a in base['agents'])+len(meals)==initial
    return dict(needs=len(t['needs']),meals=len(meals),points=len(t['points']))
def main():
    groups=collections.defaultdict(dict);counts=collections.Counter();invalid=None;keep={}
    for name in ('seed-42','held-out-seed-42'):
        for t in records(D,name):
            v=physical(t);counts.update(v);counts['histories']+=1
            key=(t['case'],json.dumps(t['config'],sort_keys=True));assert t['mode'] not in groups[key]
            groups[key][t['mode']]=dict(final=hashlib.sha256(json.dumps(t['final_state'],sort_keys=True).encode()).hexdigest(),periods=[[dict(food=a['food'],hunger=a['hunger']) for a in t['points'][p['after']]['state']['agents']] for p in t['periods']])
            if t['case']=='information' and t['mode']=='EatFirst' and t['config']['interval']==2:invalid=True
            if name=='seed-42' and t['case']=='history' and t['mode'] in ('Serial','Recurring'):keep[t['mode']]=t
    assert counts['histories']==266 and len(groups)==38
    material=lambda agents:[(a['food'],a['hunger']) for a in agents]
    period_diff=final_diff=reconverge=0
    for modes in groups.values():
        a,b=[modes[m] for m in ('Serial','Recurring')]
        ds=[material(x)!=material(y) for x,y in zip(a['periods'],b['periods'],strict=True)]
        period_diff+=any(ds);final_diff+=ds[-1];reconverge+=any(ds) and not ds[-1]
    assert (period_diff,final_diff,reconverge)==(24,5,19)
    for (case,_),modes in groups.items():
        if case=='no_food':assert modes['Serial']['final']==modes['Recurring']['final']
    # Actual primitive requests and donor decisions, not outcome classifications.
    a,b=[keep[m] for m in ('Serial','Recurring')]
    def at(t,tick):return next(w['activity'] for p in t['periods'] for w in p['steps'] if t['points'][w['activity']['observed']]['state']['tick']==tick)
    x,y=at(a,98),at(b,98)
    assert x['decisions'][0]['contact']==y['decisions'][0]['contact']
    assert x['responses'][2]['selected']==y['responses'][2]['selected']==dict(Accept=0)
    pa,pb=[s['responses'][2]['input']['own']['own'] for s in (x,y)]
    assert (pa['food'],pb['food'])==(4,0) and dict(pa,food=0)==pb
    def base(t):return t['final_state']['base']['base']['base']['base']['base']
    ea,eb=[base(t)['events'] for t in (a,b)]
    assert ea[24]['decision']==eb[24]['decision'] and ea[24]['tick']==eb[24]['tick']==101
    assert (ea[25]['decision']['selected'],eb[25]['decision']['selected'])==('Offer','Leave')
    assert ea[26]['transferred']==1 and not any(e['transferred'] for e in eb if 101<=e['tick']<=103)
    # A replayed invalid development history must independently fail this physics.
    bad=next(t for t in records(D/'development/initial-clock-hook','held-out-seed-42') if t['case']=='information' and t['mode']=='EatFirst' and t['config']['interval']==2)
    try:physical(bad)
    except AssertionError as error:failure=str(error)
    else:raise AssertionError('invalid development accepted')
    assert invalid is True
    rows=json.loads(gzip.decompress((D/'summary.json.gz').read_bytes()));sg=collections.defaultdict(dict)
    for row in rows:sg[row['seed'],row['case'],json.dumps(row['config'],sort_keys=True)][row['mode']]=row
    assert len(rows)==1064 and len(sg)==152
    summary=collections.Counter()
    for modes in sg.values():
        x,y=[modes[m] for m in ('Serial','Recurring')]
        summary['changed_material']+=(x['final_food'],x['final_hunger'])!=(y['final_food'],y['final_hunger'])
        summary['consumed_more']+=y['consumed']>x['consumed'];summary['consumed_less']+=y['consumed']<x['consumed']
        summary['someone_hungrier']+=any(v>u for u,v in zip(x['final_hunger'],y['final_hunger'],strict=True))
        summary['someone_less_hungry']+=any(v<u for u,v in zip(x['final_hunger'],y['final_hunger'],strict=True))
    assert dict(summary)==dict(changed_material=20,consumed_more=0,consumed_less=0,someone_hungrier=20,someone_less_hungry=0)
    contrasts={}
    for left,right in [('Fixed','Recurring'),('FixedContact','ContactOnly'),('ContactOnly','Recurring'),('Serial','Recurring'),('Recurring','EatFirst'),('Recurring','NoHistory')]:
        c=collections.Counter()
        for modes in sg.values():
            x,y=modes[left],modes[right];c['pairs']+=1
            c['final_material_changed']+=(x['final_food'],x['final_hunger'])!=(y['final_food'],y['final_hunger'])
            c['transfer_count_changed']+=x['transfers']!=y['transfers'];c['query_count_changed']+=x['queries']!=y['queries']
            c['consumed_more']+=y['consumed']>x['consumed'];c['consumed_less']+=y['consumed']<x['consumed']
            c['someone_hungrier']+=any(v>u for u,v in zip(x['final_hunger'],y['final_hunger'],strict=True));c['someone_less_hungry']+=any(v<u for u,v in zip(x['final_hunger'],y['final_hunger'],strict=True))
        contrasts[left+'/'+right]=dict(c)
    assert contrasts==json.loads((D/'all-seed-comparisons.json').read_bytes())
    print(json.dumps(dict(counts=counts,groups=38,serial_recurring=dict(intermediate=period_diff,terminal=final_diff,reconverged=reconverge),all_seed_serial_recurring=summary,invalid_candidate_rejected=failure,corrected_same_case_passes=True),sort_keys=True))
if __name__=='__main__':main()
