"""Complementary019 primitive prefix/locality review and causal references.
No scorer, evaluator or original reader imported. Stream one history at a time.
"""
import collections,gzip,hashlib,itertools,json
from pathlib import Path
R=Path(__file__).resolve().parents[2];D=R/'experiments/019'
MODES=('Baseline','EatFirst','Deadline','Scale90','Scale110','NoHistory')
DEFAULT=dict(window=24,idle_gap=1,interval=4,stock=12,hunger=20,order='Ascending')
def digest(x):return hashlib.sha256(json.dumps(x,sort_keys=True,separators=(',',':')).encode()).hexdigest()
def stream(name):
    with gzip.open(D/(name+'.jsonl.gz'),'rt',encoding='utf-8') as f:
        for line in f:yield json.loads(line)
def choice(d):
    if d is None:return None
    action=d['selected'];intent=d['contact']['selected'] if action=='Invite' else None
    query=d['contact']['input']['query']['decision']['selected'] if intent!='Wait' and isinstance(intent,dict) and intent['Invite']['kind']=='Inquiry' else None
    return action,intent,query
def compact(t,archive,line):
    steps=[]
    for pno,p in enumerate(t['periods']):
        for sno,w in enumerate(p['steps']):
            s=w['activity'];point=t['points'][s['observed']]
            details=[]
            for d in s['decisions']:
                if d is None:details.append(None);continue
                i=d['contact']['input'];details.append(dict(owner=i['own']['id'],food=i['own']['food'],hunger=i['own']['hunger'],remaining=i['end']-i['now'],scores=[c['score'] for c in d['candidates']],selected=choice(d),contact=d['contact']['selected'],input_digest=digest(i),reservation=next(c['reservation'] for c in d['contact']['candidates'] if c['intent']==d['contact']['selected'])))
            steps.append(dict(period=pno,step=sno,point=s['observed'],tick=point['state']['tick'],state=digest(point),choices=[choice(d) for d in s['decisions']],details=details,query=s['query'],started=s['started']))
    base=t['final_state']['base']['base']['base']['base']['base']
    events=[dict(id=e['id'],tick=e['tick'],actor=e['decision']['actor'],action=e['decision']['selected'],food=e['decision']['food'],hunger=e['decision']['hunger'],transferred=e['transferred']) for e in base['events'][t['initial']['state']['events']:]]
    return dict(seed=t['seed'],case=t['case'],mode=t['mode'],config=t['config'],archive=archive,record=line,initial=digest((t['initial'],t['profiles'])),final=digest(t['final_state']),steps=steps,events=events,meals=[c for p in t['periods'] for w in p['steps'] for c in w['activity']['consumption']],end=t['clock']['last'],epoch=t['clock']['epoch'])
def first(a,b):
    assert a['initial']==b['initial']
    for x,y in zip(a['steps'],b['steps']):
        assert x['state']==y['state'],('world diverged before voluntary choice',a['case'],a['mode'],b['mode'],x['tick'],y['tick'])
        if x['choices']!=y['choices']:
            actors=[i for i,(l,r) in enumerate(zip(x['choices'],y['choices'],strict=True)) if l!=r]
            for actor in actors:assert x['details'][actor]['input_digest']==y['details'][actor]['input_digest']
            return dict(tick=x['tick'],actors=actors,left={k:v for k,v in x.items() if k not in ('state','choices')},right={k:v for k,v in y.items() if k not in ('state','choices')})
    assert len(a['steps'])==len(b['steps']) and a['final']==b['final']
    return None
def main():
    groups=collections.defaultdict(dict);opening={};matched_timing=0;count=0
    for name in ('seed-42','held-out-seed-42','seed-1-focus'):
        for line,t in enumerate(stream(name),1):
            d=compact(t,name+'.jsonl.gz',line);key=(t['seed'],t['case'],json.dumps(t['config'],sort_keys=True));groups[key][t['mode']]=d;count+=1
            opening[(t['seed'],t['case'],t['mode'],json.dumps(t['config'],sort_keys=True))]=d['initial']
    assert count==336 and len(groups)==56
    pairs=[];unchanged=0
    for (seed,case,config),modes in groups.items():
        for left,right in itertools.combinations(MODES,2):
            a,b=modes[left],modes[right];f=first(a,b);unchanged+=f is None
            pairs.append(dict(seed=seed,case=case,config=json.loads(config),left=left,right=right,first=f,left_reference=dict(archive=a['archive'],record=a['record']),right_reference=dict(archive=b['archive'],record=b['record']),final_snapshot_equal=a['final']==b['final']))
        c=json.loads(config);changed=[k for k in DEFAULT if c[k]!=DEFAULT[k]]
        if seed==42 and len(changed)==1 and changed[0] in ('window','idle_gap','order'):
            default=json.dumps(DEFAULT,sort_keys=True)
            for mode in MODES:assert opening[seed,case,mode,config]==opening[seed,case,mode,default];matched_timing+=1
    assert matched_timing==162
    # Admission witness:long-window first resource really fits the short window,
    # while the short configuration cannot legally offer that same bound8 action.
    default=json.dumps(DEFAULT,sort_keys=True);short=json.dumps(dict(DEFAULT,window=6),sort_keys=True)
    a,b=groups[42,'useful_contact',default]['Baseline'],groups[42,'useful_contact',short]['Baseline'];assert a['initial']==b['initial']
    started=next(s for s in a['steps'] if s['started']);c=started['started'];scene=c['scene'];es=[e for e in a['events'] if e['id']>=c['first_event']];end=next(e['tick'] for e in es if e['action'] in ('Accept','Refuse','Leave'))
    assert end-a['epoch']<=6 and started['tick']==a['epoch'] and not any(s['started'] for s in b['steps'])
    witness=dict(case='useful_contact',mode='Baseline',long=dict(archive=a['archive'],record=a['record']),short=dict(archive=b['archive'],record=b['record']),start=a['epoch'],actual_end=end,conservative_reservation=8,short_window=6,events=[e for e in es if e['tick']<=end])
    print(json.dumps(dict(histories=count,matched_timing_order_initials=matched_timing,paired_prefixes=len(pairs),no_voluntary_difference_full_snapshot_equal=unchanged,admission_witness=witness,pairs=pairs),separators=(',',':')))
if __name__=='__main__':main()
