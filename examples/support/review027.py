"""Independent027 local retention/scoring plus unchanged full physical/cognitive reader.
No simulator transitions, treatment evaluator findings or policy implementation imported.
"""
import argparse, copy, gzip, hashlib, json, types
from pathlib import Path
import material025, review025_findings, review026, review026_c1
ROOT=Path(__file__).resolve().parents[2]
def sha(p):return hashlib.sha256(p.read_bytes().replace(b'\r\n',b'\n')).hexdigest()
def identity():
    for n,h in json.loads((ROOT/'experiments/027/registration-bindings.json').read_bytes()).items():assert sha(ROOT/n)==h,('registration',n)
    lock=ROOT/'experiments/027/source-freeze.json'
    assert lock.exists(),'027 runtime must be published before evidence acceptance'
    sources=json.loads(lock.read_bytes())
    for n,h in sources.items():assert sha(ROOT/n)==h,('027 source',n)
    names=['Cargo.toml','Cargo.lock','build.rs','generate_host.py','src/lib.rs','src/main.rs','src/experience.rs']
    runtime={('prototypes/material-experience/'+n):sha(ROOT/'prototypes/material-experience'/n) for n in names}
    return hashlib.sha256(json.dumps(runtime,sort_keys=True,separators=(',',':')).encode()).hexdigest()
def read(path):
    with (gzip.open(path,'rb')if str(path).endswith('.gz')else open(path,'rb'))as f:
        for line in f:yield json.loads(line)
def learning(t):
    mode=t['config']['mode'];body=t['body'];h=body['history'];audit=t['learning']
    assert set(audit)=={'mode','cells','receipts','decisions'}and audit['mode']==mode
    timeline=[];expected_receipts=[];expected_decisions=[];states=[[]for _ in range(4)];at={};stats=dict(expired=0,capacity_evicted=0,score_changes=0,counterfactual_changes=0,eligible_with_retained=0)
    for memory in body['material']['memories']:
        if mode=='Experience'and memory['kind']in('Taken','Insufficient'):
            timeline.append((memory['tick'],0,memory['id'],memory))
    for bi,b in enumerate(h['boundaries']):
        for ci,c in enumerate(b['choices']):timeline.append((c['input']['legacy']['now'],1,(bi,ci),c))
    for now,kind,ref,row in sorted(timeline,key=lambda e:(e[0],e[1],e[2])):
        owner=row['owner']if kind==0 else row['input']['legacy']['own']['id']
        before=copy.deepcopy(states[owner]);after=[v for v in before if now-v['received']<24]
        assert all(v['owner']==owner and v['received']<=now for v in before)
        stats['expired']+=len(before)-len(after)
        if kind==0:
            signal=dict(owner=owner,received=now,success=row['kind']=='Taken')
            evicted=after.pop(0)if len(after)==2 else None;stats['capacity_evicted']+=int(evicted is not None)
            after.append(signal)
            expected_receipts.append(dict(memory=ref,signal=signal,before=before,after=copy.deepcopy(after),evicted=evicted))
        else:
            bi,ci=ref
            expect=50 if not after else 100*sum(v['success']for v in after)//len(after)
            delta=(1 if expect>=50 else -1)*(abs(expect-50)//5)if mode=='Experience'and row['candidates'][4]['eligible']else 0
            expected_decisions.append(dict(boundary=bi,choice=ci,before=before,retained=copy.deepcopy(after),expected=expect,adjustment=delta))
            at[(owner,now)]=copy.deepcopy(after)
            stats['eligible_with_retained']+=int(row['candidates'][4]['eligible']and bool(after))
            stats['score_changes']+=int(delta!=0)
            fixed=copy.deepcopy(row['candidates']);fixed[4]['base_score']-=delta;fixed[4]['final_score']-=delta
            stats['counterfactual_changes']+=int(row['selected']!=max(fixed,key=lambda c:c['final_score'])['action'])
        states[owner]=after
    assert audit['receipts']==expected_receipts,'live personal receipt/retention reconstruction'
    assert audit['decisions']==expected_decisions,'decision memory/expectation reconstruction'
    final=[[c for c in s if h['end']-c['received']<24]for s in states]
    stats['expired']+=sum(map(len,states))-sum(map(len,final))
    assert audit['cells']==final,'terminal retained state'
    return at,stats
def verify(t, source=None):
    assert set(t)=={'schema','source','config','body','learning'}and t['schema']=='experiment-027/v1'
    assert t['source']==(source or identity()),'wrong source identity'
    cfg=t['config'];matrix=json.loads((ROOT/'experiments/027/registered-matrix.json').read_bytes())
    assert cfg==matrix[2*cfg['source025']+int(cfg['mode']=='Experience')],'unregistered parameters'
    body=t['body'];assert body['config']==cfg['world']
    at,stats=learning(t)
    def selection(i,o,v):
        r=review026.selection(i,o,v,'ProjectedNeed')
        if cfg['mode']=='Experience'and r['terms'][4]['eligible']:
            cells=at[(i['legacy']['own']['id'],i['legacy']['now'])]
            expected=50 if not cells else 100*sum(c['success']for c in cells)//len(cells)
            delta=(1 if expected>=50 else -1)*(abs(expected-50)//5)
            r['terms'][4]['base_score']+=delta;r['terms'][4]['final_score']+=delta
            r['selected']=max(r['terms'],key=lambda c:c['final_score'])['action']
        return r
    path=ROOT/'examples/support/review025.py';text=path.read_text(encoding='utf-8')
    for seam in ('import material025\n','import review015 as old\n'):
        assert text.count(seam)==1;text=text.replace(seam,'')
    adapter=types.SimpleNamespace(Physical=material025.Physical,Slots=material025.Slots,selection=selection)
    ns={'__file__':str(path),'__name__':'independent027','material025':adapter,'old':review026_c1.OldProxy()}
    exec(compile(text,str(path),'exec'),ns)
    metrics=ns['verify'](body)
    # Separate scalar reconstruction also checks hunger integration and full mass.
    physical=review025_findings.physical(body)
    core=review025_findings.base(body['history']);events=core['events'][body['history']['bootstrap_events']:]
    result=dict(source025=cfg['source025'],mode=cfg['mode'],learning=stats,metrics=metrics,hunger_area=physical['area'],ordinary_hunger_area=physical['pre'],receipts=len(t['learning']['receipts']),work_ticks=sum(p['finish_at']-p['start']for p in body['material']['processes']),transfers=sum(e['transferred']for e in events),meals=[sum(m['consumed']for m in core['cognition']['consumption']if m['actor']==i)for i in range(4)])
    return result
def main():
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('--output');args=p.parse_args()
    source=identity();rows=[]
    for n,t in enumerate(read(args.archive)):
        assert t['config']==json.loads((ROOT/'experiments/027/registered-matrix.json').read_bytes())[n]
        try:rows.append(verify(t,source))
        except Exception as e:raise AssertionError(('027 independent row',n))from e
    assert len(rows)==100
    if args.output:
        with open(args.output,'x',encoding='utf-8')as f:json.dump(rows,f,indent=2);f.write('\n')
    print('027 full independent local learning/physics/cognition/consent PASS100')
if __name__=='__main__':main()
