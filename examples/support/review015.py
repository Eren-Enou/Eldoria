"""Read-only independent015 policies, matching, actual execution and local cognition.

No Rust evaluator flags or summary conclusions are used. Existing independently
reviewed cognitive primitives remain authoritative for old006–012 arithmetic.
"""
import argparse, collections, copy, gzip, hashlib, itertools, json
from pathlib import Path
import review013 as cognitive
import review013_findings as assessment
import review014 as resources

ROOT=Path(__file__).resolve().parents[2]
MODES=('Simple','History','NoHistory')
def trunc(a,b):return (abs(a)//b)*(-1 if a<0 else 1)
def resource_value(a,partner):
    trust=a['trust'].get(str(partner),0)
    valence=max(-40,min(40,sum(m['valence'] for m in [m for m in a['memories'] if m['partner']==partner][-4:])))
    h,g,c=a['hunger'],a['generosity'],a['caution']
    scores=[0,h-35+trunc(trust,4)-c//4]
    if a['food']>=1:scores.append(g-h//2-c//2+trust+valence-(60 if h>=60 and a['food']<=1 else 0))
    return max(scores)+max(h,0)//2-2
def selected_query(i):
    d=i['query']['decision'];a=d['selected']
    if a=='Pause':return None
    return a['Ask'],next(c['score'] for c in d['candidates'] if c['action']==a)
def initiate(i,mode):
    available={p['id']:p['available'] for p in i['people']};own=copy.deepcopy(i['own'])
    if mode=='NoHistory':own['trust']={};own['memories']=[]
    result=[dict(intent='Wait',reservation=0,eligible=True,score=0)]
    for p in i['people']:
        if p['id']==own['id']:continue
        legal=available[own['id']] and p['available'] and i['now']+8<=i['end']
        result.append(dict(intent=dict(Invite=dict(recipient=p['id'],kind='Resource')),reservation=8,eligible=legal,score=-1 if not legal else 1 if mode=='Simple' else resource_value(own,p['id'])))
    q=selected_query(i)
    if q:
        a,score=q;reserve=15 if a['strategy']=='Direct' else 33
        legal=available[own['id']] and available[a['source']] and i['now']+reserve<=i['end']
        result.append(dict(intent=dict(Invite=dict(recipient=a['source'],kind='Inquiry')),reservation=reserve,eligible=legal,score=-1 if not legal else 1 if mode=='Simple' else score-2))
    return result,max(result,key=lambda c:c['score'])['intent']
def response(i):
    result=[dict(reply='Wait',score=0)];own=i['own'];available=next(p['available'] for p in own['people'] if p['id']==own['own']['id'])
    for inv in i['incoming']:
        score=-1 if not available else resource_value(own['own'],inv['proposer']) if inv['kind']=='Resource' else own['profile']['relationship_goal']-own['profile']['privacy']-2
        result.append(dict(reply=dict(Accept=inv['proposer']),score=score))
    outgoing=i['outgoing']
    if outgoing['selected']!='Wait':result.append(dict(reply='Continue',score=next(c['score'] for c in outgoing['candidates'] if c['intent']==outgoing['selected'])))
    return result,max(result,key=lambda c:c['score'])['reply']
def match(now,order,wave,people,invitations,responses):
    available={p['id']:p['available'] for p in people};occupied=set();executed=[];result=[]
    key=lambda i:i['proposer'] if order=='Ascending' else 2-i['proposer'] if order=='Descending' else (i['proposer']+3-wave%3)%3
    for i in sorted(invitations,key=key):
        if now!=i['expires']:reason='Expired'
        elif not available[i['proposer']] or not available[i['recipient']]:reason='Unavailable'
        elif responses[i['recipient']]['selected']!=dict(Accept=i['proposer']):reason='Declined'
        elif any(p['proposer']==i['recipient'] and p['recipient']==i['proposer'] and p['kind']==i['kind'] for p in executed):reason='Reciprocal'
        elif occupied.intersection([i['proposer'],i['recipient']]):reason='Contention'
        else:reason='Execute';occupied.update([i['proposer'],i['recipient']]);executed.append(i)
        result.append(dict(invitation=i,disposition=reason))
    assert len(executed)<=1
    return result
def concern_prefix(transitions,n):
    live=collections.defaultdict(list)
    for r in transitions[:n]:
        b,a=r['before'],r['after'];owner=(a or b)['owner'];id=(a or b)['id']
        at=next((i for i,c in enumerate(live[owner]) if c['id']==id),None)
        if a is None:
            assert at is not None;live[owner].pop(at)
        elif at is None:live[owner].append(a)
        else:live[owner][at]=a
    assert all(len(cs)<=8 for cs in live.values())
    return live
def cells_prefix(records,owner,n):
    live=[]
    for r in records:
        if r['inquiry']>=n:break
        if r['owner']!=owner or r['after'] is None:continue
        a=r['after'];key=lambda c:(c['source'],c['strategy'],c['context']);at=next((i for i,c in enumerate(live) if key(c)==key(a)),None)
        if at is not None:live[at]=a
        else:
            if len(live)==32:live.pop(0)
            live.append(a)
    return live
def query_agents(agents,events,information,start,end):
    agents=copy.deepcopy(agents)
    for info in information:
        if not start<info['tick']<=end:continue
        a=agents[info['listener']];partner=str(info['speaker'])
        assert a['trust'].get(partner,0)==info['trust_before']
        change=info['revision']
        if change:
            at=next(i for i,m in enumerate(a['memories']) if m['event']==info['event']);assert a['memories'][at]==change['before']
            event=events[info['event']];original=event['interpretations'][event['participants'].index(info['listener'])]
            expected=dict(original,interpretation='PossibleSelfProtection',valence=0) if info['after']['support']>=60 else original
            assert expected==change['after'];a['memories'][at]=expected
            replacements={r['event']:r['revision']['after']['valence'] for r in information[:info['id']+1] if r['listener']==info['listener'] and r['revision']}
            trust=0
            for event in events:
                if event['tick']>info['tick']:break
                if info['listener'] not in event['participants']:continue
                m=event['interpretations'][event['participants'].index(info['listener'])]
                if m['partner']==info['speaker']:trust=max(-100,min(100,trust+replacements.get(event['id'],m['valence'])))
            a['trust'][partner]=trust
        assert a['trust'].get(partner,0)==info['trust_after']
    return agents

def verify(t):
    snap=t['final_state'];compact=snap['base']['base']['base'];base=compact['base']['base'];events=base['events'];inquiries=compact['base']['inquiry']['records'];information=base['cognition']['information'];transitions=base['concerns']['transitions']
    cognitive.reconstruct(dict(final_state=snap,mode='Contextual',case=t['case'],config=t['config'],opportunities=[]))
    prefixes=assessment.bases(t);metrics=collections.Counter();contacts=collections.Counter();previous=t['initial'];id=0;trajectory=[];choices=[];encounters=[];details=[]
    def context(i,point,busy,end,now):
        owner=i['own']['id'];assert i['own']==point['agents'][owner] and i['profile']==base['intentional']['profiles'][str(owner)]
        people=[dict(id=j,available=now>=busy[j]) for j in range(3)];assert (i['people'],i['now'],i['end'])==(people,now,end)
        q=i['query'];own=q['input']['base'];assert (own['owner'],own['hunger'],own['relationship_goal'])==(owner,i['own']['hunger'],i['profile']['relationship_goal'])
        concerns=concern_prefix(transitions,point['concerns']).get(owner,[]);assert own['concerns']==concerns
        sources=[p['id'] for p in people if p['available'] and p['id']!=owner and not any(c['target']==p['id'] for c in concerns)]
        assert [s['id'] for s in own['sources']]==sources
        assert all(not s['offers'] for s in own['sources'])
        assert own['scene']==len({e['scene'] for e in events[:point['events']]})
        assert q['input']['knowledge']==cognitive.provenance_prefix(compact,now).get(owner,[])
        items=prefixes[point['assessment']].get(str(owner),[])
        basis=[dict(concern=c['id'],event=c['event'],support=assessment.grouped(items,c['event']),context='EvidencePresent' if any(x['event']==c['event'] and x['quality']>0 and 'Claim' not in x['origin'] for x in items) else 'ClaimFallback') for c in concerns if c['status'] in ['Open','Partial']]
        candidates,selected,_=assessment.scores(own,{k:q['input'][k] for k in ['knowledge','hints','settings']},basis,cells_prefix(snap['learning']['records'],owner,point['queries']),'Contextual')
        assert (q['decision']['candidates'],q['decision']['selected'])==(candidates,selected)
        return people
    for pno,p in enumerate(t['periods']):
        if pno:
            intervention=t['interventions'][pno-1];assert intervention['period']==pno and intervention['before']==previous['agents']
            assert intervention['after']==[dict(a,hunger=min(100,a['hunger']+20)) for a in previous['agents']]
            previous=dict(previous,agents=intervention['after'])
        assert p['before']==previous and p['start']==previous['tick'] and p['end']>p['start']
        busy=p['busy_until'].copy();end=p['end']
        for w in p['waves']:
            assert w['id']==id;id+=1;before=w['before'];after=w['after']
            assert before==dict(previous,concerns=before['concerns'])
            live=concern_prefix(transitions,previous['concerns'])
            changes=transitions[previous['concerns']:before['concerns']]
            expected=[c for cs in live.values() for c in cs if c['status'] in ['Open','Partial']];expected.sort(key=lambda c:(c['owner'],live[c['owner']].index(c)))
            assert len(changes)==len(expected)
            for change,c in zip(changes,expected,strict=True):assert (change['reason'],change['before'],change['after'])==('015 observed encounter wave',c,dict(c,age=min(2**32-1,c['age']+1)))
            invitations=[]
            assert len(w['proposals'])==3
            for owner,d in enumerate(w['proposals']):
                assert d['input']['own']['id']==owner and d['mode']==t['mode'];people=context(d['input'],before,busy,end,before['tick'])
                candidates,selected=initiate(d['input'],t['mode']);assert (d['candidates'],d['selected'])==(candidates,selected)
                metrics['unaffordable_candidates']+=sum(not c['eligible'] for c in candidates)
                if selected!='Wait':invitations.append(dict(proposer=owner,recipient=selected['Invite']['recipient'],kind=selected['Invite']['kind'],expires=before['tick']+2));metrics['invitations']+=1
            choices.append([d['selected'] for d in w['proposals']])
            if not invitations:
                assert not w['responses'] and not w['allocations'] and w['execution'] is None
                assert after==dict(before,tick=end);metrics['waiting_ticks']+=end-before['tick'];previous=after;continue
            assert len(w['responses'])==3 and before['tick']+2<=end
            metrics['handshake_elapsed']+=2;metrics['sender_active_ticks']+=len(invitations)
            metrics['recipient_active_ticks']+=len({i['recipient'] for i in invitations})
            for owner,r in enumerate(w['responses']):
                i=r['input'];assert i['outgoing']==w['proposals'][owner] and i['own']['own']['id']==owner
                context(i['own'],before,busy,end,before['tick']+1)
                assert i['incoming']==[i for i in invitations if i['recipient']==owner]
                candidates,selected=response(i);assert (r['candidates'],r['selected'])==(candidates,selected)
            people=[dict(id=j,available=before['tick']+2>=busy[j]) for j in range(3)]
            allocations=match(before['tick']+2,t['config']['order'],w['id'],people,invitations,w['responses']);assert allocations==w['allocations']
            for a in allocations:metrics[a['disposition'].lower()]+=1
            executed=[a for a in allocations if a['disposition']=='Execute'];x=w['execution']
            if not executed:assert x is None and after==dict(before,tick=before['tick']+2)
            else:
                assert x['invitation']==executed[0]['invitation'] and x['before']==dict(before,tick=before['tick']+2) and x['after']==after
                i=x['invitation'];start=x['before'];dt=after['tick']-start['tick'];assert 0<dt and after['tick']<=end
                encounters.append(dict(wave=w['id'],proposer=i['proposer'],recipient=i['recipient'],kind=i['kind'],start=start['tick'],end=after['tick'],events=[start['events'],after['events']],queries=[start['queries'],after['queries']]))
                assert all(start['tick']>=busy[j] for j in [i['proposer'],i['recipient']])
                for j in [i['proposer'],i['recipient']]:busy[j]=after['tick'];contacts[j]+=1
                current=events[start['events']:after['events']];q=inquiries[start['queries']:after['queries']]
                if i['kind']=='Resource':
                    assert not q and dt==len(current) and dt<=6
                    assert current[0]['participants']==[i['proposer'],i['recipient']]
                    assert resources.resource(current,start['agents'])==after['agents']
                    metrics['resource_ticks']+=dt;metrics['transfers']+=sum(e['transferred'] for e in current);metrics['resource_refusals']+=sum(e['outcome']=='Refusal' for e in current)
                else:
                    assert not current and len(q)==1 and q[0]['time_spent']==dt
                    preview=w['proposals'][i['proposer']]['input']['query'];assert q[0]['decision']==preview['decision']
                    assert q[0]['decision']['selected']['Ask']['source']==i['recipient']
                    assert dt<=(13 if q[0]['decision']['selected']['Ask']['strategy']=='Direct' else 31)
                    assert query_agents(start['agents'],events,information,start['tick'],after['tick'])==after['agents']
                    capture=next(r for r in snap['learning']['records'] if r['inquiry']==q[0]['id']);o=capture['outcome']
                    details.append(dict(inquiry=q[0]['id'],owner=i['proposer'],source=i['recipient'],selected=q[0]['decision']['selected'],outcome=o))
                    metrics['inquiries']+=1;metrics['inquiry_ticks']+=dt;metrics['receipts']+=o['receipt_acquired'];metrics['changes']+=o['support_before']!=o['support_after'] or o['status_before']!=o['status_after'];metrics['no_receipt']+=not o['receipt_acquired'];metrics['redundant_receipts']+=o['receipt_acquired'] and o['novelty_value']==0
            previous=after
        assert previous['tick']==end
        agents=copy.deepcopy(previous['agents'])
        for offset,c in enumerate(p['consumption']):
            a=agents[c['actor']];assert c['actor']==offset and c['tick']==end+offset+1
            assert (c['food_before'],c['hunger_before'])==(a['food'],a['hunger']);used=int(a['food']>0 and a['hunger']>0)
            assert c['consumed']==used;a['food']-=used
            if used:a['hunger']=max(0,a['hunger']-40)
            assert (c['food_after'],c['hunger_after'])==(a['food'],a['hunger']);metrics['consumed']+=used
        assert p['after']==dict(previous,tick=end+3,agents=agents);previous=p['after'];trajectory.append(dict(food=[a['food'] for a in agents],hunger=[a['hunger'] for a in agents],trust=[a['trust'] for a in agents]))
    assert len(events)==previous['events'] and len(inquiries)==previous['queries'] and len(transitions)==previous['concerns']
    assert sum(a['food'] for a in t['initial']['agents'])==sum(a['food'] for a in previous['agents'])+metrics['consumed']
    assert previous['agents']==base['agents']
    return dict(case=t['case'],mode=t['mode'],config=t['config'],metrics=dict(metrics),contacts=dict(contacts),choices=choices,encounters=encounters,inquiry_details=details,trajectory=trajectory,final=previous['agents'])

def read(directory,name):
    raw=directory/(name+'.json')
    return json.loads(raw.read_bytes() if raw.exists() else gzip.decompress((directory/(name+'.json.gz')).read_bytes()))
def main():
    parser=argparse.ArgumentParser();parser.add_argument('--directory',type=Path,default=ROOT/'experiments/015');parser.add_argument('--self-test',action='store_true');parser.add_argument('--preview',action='store_true');args=parser.parse_args();directory=args.directory
    freeze=directory/'policy-freeze.json'
    if freeze.exists():
        for name,h in json.loads(freeze.read_bytes()).items():assert hashlib.sha256((ROOT/name).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,name
    manifest=directory/'archive-manifest.json'
    if manifest.exists():
        for name,m in json.loads(manifest.read_bytes()).items():
            b=(directory/(name+'.json.gz')).read_bytes();raw=gzip.decompress(b);assert (len(b),hashlib.sha256(b).hexdigest(),len(raw),hashlib.sha256(raw).hexdigest())==(m['gzip_bytes'],m['gzip_sha256'],m['json_bytes'],m['json_sha256'])
    rows=[];groups=collections.defaultdict(dict);first={};samples=[];initial_waves={}
    for name in ['seed-42'] if args.preview else ['seed-42','held-out-seed-42']:
        for t in read(directory,name):
            row=verify(t);rows.append(row);key=(t['case'],json.dumps(t['config'],sort_keys=True));groups[key][t['mode']]=row
            setup=json.dumps(t['initial'],sort_keys=True)
            assert key not in first or first[key]==setup;first[key]=setup
            initial_waves[(t['case'],t['mode'],json.dumps(t['config'],sort_keys=True))]=(t['periods'][0]['waves'][0]['proposals'],t['periods'][0]['waves'][0]['responses'])
            if name=='seed-42' and t['mode']=='History' and t['case'] in ['information','private_mirror']:samples.append(t)
    configs={key[2] for key in initial_waves}
    for config in configs:
        for mode in MODES:assert initial_waves[('information',mode,config)][0][0]==initial_waves[('private_mirror',mode,config)][0][0]
    for (case,mode,config),wave in initial_waves.items():
        c=json.loads(config)
        if c['budget']==24 and c['order']!='Ascending':
            baseline=json.dumps(dict(c,order='Ascending'),sort_keys=True)
            assert wave==initial_waves[(case,mode,baseline)]
    pairs=[]
    for (case,config),modes in groups.items():
        assert set(modes)==set(MODES)
        for a,b in itertools.combinations(MODES,2):
            left,right=modes[a],modes[b]
            material=lambda r:[(a['food'],a['hunger']) for a in r['final']]
            trust=lambda r:[a['trust'] for a in r['final']]
            pairs.append(dict(case=case,config=config,left=a,right=b,choices_changed=left['choices']!=right['choices'],encounters_changed=left['encounters']!=right['encounters'],trajectory_changed=left['trajectory']!=right['trajectory'],material_changed=material(left)!=material(right),trust_changed=trust(left)!=trust(right),final_changed=left['final']!=right['final']))
    if args.self_test and samples:
        for field in ['score','time','invitation']:
            bad=copy.deepcopy(samples[0]);w=bad['periods'][0]['waves'][0]
            if field=='score':w['proposals'][0]['candidates'][1]['score']+=1
            elif field=='time':w['after']['tick']=bad['periods'][0]['end']+1
            else:w['responses'][2]['selected']='Wait'
            try:verify(bad)
            except (AssertionError,KeyError,StopIteration):pass
            else:raise AssertionError('accepted corruption '+field)
    print(json.dumps(dict(independent_trials=len(rows),trials=rows,comparisons=pairs),indent=2))
if __name__=='__main__':main()
