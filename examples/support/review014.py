"""Read-only014 independent activity/time/resource reconstruction; no Rust scorer.
Reuse independent006–013 cognition/provenance primitives, not014 outcome labels.
"""
import argparse,collections,copy,gzip,hashlib,itertools,json
from pathlib import Path
import review013 as prior
import review013_findings as cognition
ROOT=Path(__file__).resolve().parents[2]
MODES=('ResourceFirst','InquiryFirst','Local','NoDeadline','LocalPulse')
def read(directory,name):
    p=directory/(name+'.json')
    return json.loads(p.read_bytes() if p.exists() else gzip.decompress((directory/(name+'.json.gz')).read_bytes()))
def select(i,mode):
    eligible=lambda w:i['now']<w['closes'] and i['now']+w['reservation']<=min(w['closes'],i['end'])
    candidates=[dict(action='Wait',eligible=True,base=0,urgency=0,score=0)]
    for action in ['Resource','Inquiry']:
        w=next((w for w in i['windows'] if w['action']==action),None)
        if w is None:continue
        if action=='Resource':base=max(0,max(c['score'] for c in i['resource']['candidates']))+max(0,i['own']['hunger'])//2
        else:
            d=i['inquiry']['decision'];base=next(c['score'] for c in d['candidates'] if c['action']==d['selected'])
        urgency=25 if mode in ['Local','LocalPulse'] and any(o['action']!=action and eligible(o) and i['now']+o['reservation']+w['reservation']>min(w['closes'],i['end']) for o in i['windows']) else 0
        score=-1 if not eligible(w) else (2 if action=='Resource' else 1) if mode=='ResourceFirst' else (2 if action=='Inquiry' else 1) if mode=='InquiryFirst' else base+urgency
        candidates.append(dict(action=action,eligible=eligible(w),base=base,urgency=urgency,score=score))
    return candidates,max(candidates,key=lambda c:c['score'])['action']
def resource(events,agents):
    own=copy.deepcopy(agents)
    for e in events:
        d=e['decision'];a=own[d['actor']];partner=d['observation']['partner'];trust=a['trust'].get(str(partner),0)
        memories=[m for m in a['memories'] if m['partner']==partner];v=max(-40,min(40,sum(m['valence'] for m in memories[-4:])))
        assert (d['trust'],d['episodic_valence'],d['food'],d['hunger'])==(trust,v,a['food'],a['hunger'])
        last=d['observation']['last_signal'];last=last['action'] if last else None
        legal=['Leave']
        if last=='Offer':legal+=['Accept','Refuse']
        elif last=='Request':legal+=(['Offer'] if a['food']>=1 else [])+['Refuse']
        else:legal+=['Request']+(['Offer'] if a['food']>=1 else [])
        legal.sort(key=('Accept','Offer','Request','Refuse','Leave').index)
        h,g,c=a['hunger'],a['generosity'],a['caution']
        scores=dict(Accept=h+20+prior.old.trunc(trust,4)-c//5,Offer=g-h//2-c//2+trust+v-(60 if h>=60 and a['food']<=1 else 0),Request=h-35+prior.old.trunc(trust,4)-c//4,Refuse=h//2+c//2-g//2-prior.old.trunc(trust,2),Leave=0)
        assert d['candidates']==[dict(action=x,score=scores[x]) for x in legal]
        assert d['selected']==max(legal,key=lambda x:scores[x])
        assert e['balances_before']==[own[x]['food'] for x in e['participants']]
        assert sum(e['balances_before'])==sum(e['balances_after'])
        if e['transferred']:
            assert d['selected']=='Accept' and last=='Offer' and e['transferred']==1
            own[d['actor']]['food']+=1;own[partner]['food']-=1
        assert e['balances_after']==[own[x]['food'] for x in e['participants']]
        for id,m in zip(e['participants'],e['interpretations'],strict=True):
            actor=id==d['actor'];agent=own[id]
            if e['outcome']=='Inability':interpretation,valence='FailedTransfer',0
            elif e['transferred']:interpretation,valence=('HelpReceived',24+agent['expectation']//10) if actor else ('HelpGiven',8-agent['hunger']//10)
            elif d['selected']=='Refuse':interpretation,valence=('ProtectedOwnNeeds',0) if actor else ('Rejected',-(8+agent['expectation']//4+agent['hunger']//10))
            elif d['selected']=='Request':interpretation,valence='RequestHeard',0
            elif d['selected']=='Offer':interpretation,valence='OfferHeard',0
            else:interpretation,valence='Departure',0
            assert (m['interpretation'],m['valence'])==(interpretation,valence)
            p=str(m['partner']);agent['trust'][p]=max(-100,min(100,agent['trust'].get(p,0)+valence))
            agent['memories']=(agent['memories']+[m])[-16:]
    return own
def verify(t):
    snap=t['final_state'];compact=snap['base']['base']['base'];events=compact['base']['base']['events'];queries=compact['base']['inquiry']['records']
    prior.reconstruct(dict(final_state=snap,mode=snap['mode'],case=t['case'],config=t['config'],opportunities=[]))
    prefixes=cognition.bases(t)
    result=collections.Counter();steps=[];previous=t['initial']
    for p in t['periods']:
        for intervention in [x for x in t['interventions'] if x['period']==len(steps)]:
            assert intervention['before']==previous;previous=intervention['after']
            if intervention['kind']=='external relative hunger+20':
                for a,b in zip(intervention['before']['agents'],previous['agents'],strict=True):
                    assert b==dict(a,hunger=min(100,a['hunger']+20))
        assert p['start']==previous['tick'] and p['end']>p['start']
        used=[];expired=[];period_steps=[]
        for r in p['records']:
            assert r['before']==previous;d=r['decision'];i=d['input']
            assert (i['now'],i['end'],i['own'])==(previous['tick'],p['end'],previous['agents'][0])
            if i['inquiry']:
                preview=i['inquiry'];own=preview['input']['base'];local={k:preview['input'][k] for k in ['knowledge','hints','settings']}
                assert own['owner']==0 and own['hunger']==i['own']['hunger']
                assert own['scene']==len({e['scene'] for e in events[:previous['events']]})
                assert preview['input']['knowledge']==prior.provenance_prefix(compact,i['now']).get(0,[])
                items=prefixes[previous['assessment']].get('0',[])
                basis=[dict(concern=c['id'],event=c['event'],support=cognition.grouped(items,c['event']),context='EvidencePresent' if any(x['event']==c['event'] and x['quality']>0 and 'Claim' not in x['origin'] for x in items) else 'ClaimFallback') for c in own['concerns'] if c['status'] in ['Open','Partial']]
                assert basis==previous['basis']
                cells=[]
                for capture in snap['learning']['records']:
                    if capture['inquiry']>=previous['queries']:break
                    if capture['owner']!=0 or capture['after'] is None:continue
                    after=capture['after'];index=next((n for n,v in enumerate(cells) if (v['source'],v['strategy'],v['context'])==(after['source'],after['strategy'],after['context'])),None)
                    if index is not None:cells[index]=after
                    else:
                        if len(cells)==32:cells.pop(0)
                        cells.append(after)
                expected,chosen,_=cognition.scores(own,local,basis,cells,snap['mode'])
                assert preview['decision']['candidates']==expected and preview['decision']['selected']==chosen
            expected=[]
            if p['source'] and 'Inquiry' not in used and i['now']<p['qclose'] and i['inquiry'] and i['inquiry']['decision']['selected']!='Pause':expected.append('Inquiry')
            if p['partner'] and 'Resource' not in used and i['now']<p['rclose']:expected.append('Resource')
            assert [w['action'] for w in i['windows']]==expected
            for w in i['windows']:
                assert w['closes']==p['qclose' if w['action']=='Inquiry' else 'rclose']
                if w['action']=='Resource':assert w['reservation']==6
            candidates,selected=select(i,t['mode']);assert candidates==d['candidates'] and selected==d['selected']
            before,after=r['before'],r['after'];dt=after['tick']-before['tick'];assert 0<=dt and after['tick']<=p['end']
            if selected=='Wait':assert after['tick']==p['end'];assert after['agents']==before['agents']
            else:
                w=next(w for w in i['windows'] if w['action']==selected)
                assert candidates[['Wait','Resource','Inquiry'].index(selected) if len(candidates)==3 else 1]['eligible']
                assert 0<dt<=w['reservation'] and after['tick']<=w['closes'] and selected not in used
                used.append(selected)
            local_events=events[before['events']:after['events']];local_q=queries[before['queries']:after['queries']]
            if selected=='Resource':
                assert not local_q and dt==len(local_events) and dt<=6
                assert local_events[0]['decision']==i['resource']
                assert resource(local_events,before['agents'])==after['agents']
                result['resource_ticks']+=dt;result['resource_activities']+=1
                result['transfers']+=sum(e['transferred'] for e in local_events)
            elif selected=='Inquiry':
                assert not local_events and len(local_q)==1 and local_q[0]['time_spent']==dt
                assert local_q[0]['decision']==i['inquiry']['decision']
                assert [a['food'] for a in before['agents']]==[a['food'] for a in after['agents']]
                assert w['reservation']==(13 if local_q[0]['decision']['selected']['Ask']['strategy']=='Direct' else 31)
                result['inquiry_ticks']+=dt;result['inquiries']+=1
                outcome=next(x['outcome'] for x in snap['learning']['records'] if x['inquiry']==local_q[0]['id'])
                result['receipts']+=outcome['receipt_acquired'];result['changes']+=outcome['support_before']!=outcome['support_after'] or outcome['status_before']!=outcome['status_after']
                result['failed_inquiries']+=not outcome['receipt_acquired']
            else:assert not local_events and not local_q
            for w in i['windows']:
                if not next(c for c in candidates if c['action']==w['action'])['eligible']:result['reservation_blocks']+=1
            lost=[a for a,close,present in [('Inquiry',p['qclose'],p['source']),('Resource',p['rclose'],p['partner'])] if present and a not in used and a not in expired and after['tick']>=close]
            assert lost==r['expired'];expired+=lost;previous=after
            period_steps.append(selected)
        unused=[a for a,present in [('Inquiry',p['source']),('Resource',p['partner'])] if present and a not in used]
        assert unused==p['unused'];result['unused_inquiry']+='Inquiry' in unused;result['unused_resource']+='Resource' in unused
        result['wait_ticks']+=p['end']-p['start']-sum(r['after']['tick']-r['before']['tick'] for r in p['records'] if r['decision']['selected']!='Wait')
        agents=copy.deepcopy(previous['agents'])
        for offset,c in enumerate(p['consumption']):
            a=agents[c['actor']];assert c['tick']==p['end']+1+offset and c['food_before']==a['food'] and c['hunger_before']==a['hunger']
            consumed=int(a['food']>0 and a['hunger']>0);assert c['consumed']==consumed;a['food']-=consumed
            if consumed:a['hunger']=max(0,a['hunger']-40)
            assert (c['food_after'],c['hunger_after'])==(a['food'],a['hunger']);result['consumed']+=consumed
        assert p['after']['agents']==agents and p['after']['tick']==p['end']+3
        previous=p['after'];steps.append(period_steps)
    assert sum(a['food'] for a in t['initial']['agents'])==sum(a['food'] for a in previous['agents'])+result['consumed']
    assert sum(r['after']['queries']-r['before']['queries'] for p in t['periods'] for r in p['records'])==len(queries)-t['initial']['queries']
    assert sum(r['after']['events']-r['before']['events'] for p in t['periods'] for r in p['records'])==len(events)-t['initial']['events']
    trajectory=[dict(food=[a['food'] for a in p['after']['agents']],hunger=[a['hunger'] for a in p['after']['agents']],trust=p['after']['agents'][0]['trust']) for p in t['periods']]
    return dict(case=t['case'],mode=t['mode'],config=t['config'],steps=steps,metrics=dict(result),trajectory=trajectory,food=[a['food'] for a in previous['agents']],hunger=[a['hunger'] for a in previous['agents']],trust=previous['agents'][0]['trust'])
def main():
    parser=argparse.ArgumentParser();parser.add_argument('--directory',type=Path,default=ROOT/'experiments/014');parser.add_argument('--preview',action='store_true');parser.add_argument('--self-test',action='store_true');args=parser.parse_args()
    freeze=args.directory/'policy-freeze.json'
    if freeze.exists():
        for name,digest in json.loads(freeze.read_bytes()).items():
            assert hashlib.sha256((ROOT/name).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==digest, 'frozen014 policy/fixture changed: '+name
    if (args.directory/'archive-manifest.json').exists():
        for name,m in json.loads((args.directory/'archive-manifest.json').read_bytes()).items():
            compressed=(args.directory/(name+'.json.gz')).read_bytes();raw=gzip.decompress(compressed)
            assert (len(compressed),hashlib.sha256(compressed).hexdigest(),len(raw),hashlib.sha256(raw).hexdigest())==(m['gzip_bytes'],m['gzip_sha256'],m['json_bytes'],m['json_sha256'])
    rows=[];trials=[]
    for name in ['seed-42'] if args.preview else ['seed-42','held-out-seed-42']:trials+=read(args.directory,name)
    diagnostic=args.directory/'exclusive-budget.json.gz'
    if not args.preview and diagnostic.exists():trials+=read(args.directory,'exclusive-budget')
    for t in trials:rows.append(verify(t))
    groups=collections.defaultdict(dict)
    for row in rows:groups[(row['case'],prior.old.key(row['config']))][row['mode']]=row
    pairs=[]
    for (case,config),modes in groups.items():
        for left,right in itertools.combinations(MODES,2):
            a,b=modes[left],modes[right];activity=a['steps']!=b['steps'];material=(a['food'],a['hunger'],a['trust'],a['metrics'].get('transfers',0))!=(b['food'],b['hunger'],b['trust'],b['metrics'].get('transfers',0))
            pairs.append(dict(case=case,config=config,left=left,right=right,activity_changed=activity,persistent_changed=material,trajectory_changed=a['trajectory']!=b['trajectory']))
    for t in trials:
        if t['case']=='mirrored':
            other=next(o for o in trials if o['case']=='material_first' and o['config']==t['config'] and o['mode']==t['mode'])
            assert t['periods'][0]['records'][0]['decision']==other['periods'][0]['records'][0]['decision']
    if args.self_test:
        sample=trials[0]
        for field in ['tick','score','closes']:
            bad=copy.deepcopy(sample);r=bad['periods'][0]['records'][0]
            if field=='tick':r['after']['tick']=bad['periods'][0]['end']+1
            elif field=='score':r['decision']['candidates'][1]['score']+=1
            else:r['decision']['input']['windows'][0]['closes']=r['decision']['input']['now']
            try:verify(bad)
            except (AssertionError,KeyError):pass
            else:raise AssertionError('corruption accepted: '+field)
    print(json.dumps(dict(independent_trials=len(rows),trials=rows,comparisons=pairs),indent=2))
if __name__=='__main__':main()
