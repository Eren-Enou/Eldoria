"""Read-only016 independent contact feedback, policies and full causal reconstruction.
Adapt old015 verification to prospective controls; reuse independently reviewed
006–015 primitives, never Rust016 classifications or expected treatment outcomes.
"""
import argparse,collections,copy,gzip,hashlib,itertools,json
from pathlib import Path
import review015 as prior
cognitive=prior.cognitive
assessment=prior.assessment
resources=prior.resources
concern_prefix=prior.concern_prefix
cells_prefix=prior.cells_prefix
query_agents=prior.query_agents
match=prior.match
ROOT=Path(__file__).resolve().parents[2]
MODES=('Unchanged','Control','Recent','Typed','Untyped')

def initiate(i,mode,cells):
    cs,_=prior.initiate(i,'History')
    if mode in ('Recent','Typed','Untyped'):
        for c in cs:
            if c['intent']=='Wait':continue
            v=c['intent']['Invite']
            if any(r['partner']==v['recipient'] and r['kind']==v['kind'] and r['until']>i['now'] for r in cells):c['eligible']=False;c['score']=-1
    return cs,max(cs,key=lambda c:c['score'])['intent']

def response(i,mode):
    cs,_=prior.response(i)
    if mode!='Unchanged':
        for c in cs:
            if c['reply']=='Continue':c['score']=0
    return cs,max(cs,key=lambda c:c['score'])['reply']

def learn(cells,mode,signal):
    assert signal['received']>=signal['sent']+2
    before=copy.deepcopy(cells);after=[copy.deepcopy(c) for c in cells if c['until']>signal['received']];evicted=None
    if mode in ('Unchanged','Control'):after=[]
    else:
        if mode=='Recent':after=[]
        elif signal['outcome']=='Met':after=[c for c in after if (c['partner'],c['kind'])!=(signal['partner'],signal['kind'])]
        if signal['outcome']!='Met':
            delay=6 if mode=='Recent' else 12 if mode=='Untyped' or signal['outcome']=='Declined' else 2
            cell=dict(partner=signal['partner'],kind=signal['kind'],outcome=signal['outcome'],received=signal['received'],until=signal['received']+delay)
            at=next((i for i,c in enumerate(after) if (c['partner'],c['kind'])==(cell['partner'],cell['kind'])),None)
            if at is not None:after[at]=cell
            else:
                cap=1 if mode=='Recent' else 2
                if len(after)==cap:evicted=after.pop(0)
                after.append(cell)
    assert len(after)<=(1 if mode=='Recent' else 2) and all(c['partner']!=signal['owner'] for c in after)
    return dict(signal=signal,before=before,after=after,evicted=evicted)

def verify(t):
    snap=t['final_state'];compact=snap['base']['base']['base'];base=compact['base']['base'];events=base['events'];inquiries=compact['base']['inquiry']['records'];information=base['cognition']['information'];transitions=base['concerns']['transitions']
    cognitive.reconstruct(dict(final_state=snap,mode='Contextual',case=t['case'],config=t['config'],opportunities=[]))
    prefixes=assessment.bases(t);memory=[[],[],[]];profiles=copy.deepcopy(t['initial_profiles']);profile_changes=[];last_contact={};last_outcome={};intervals=[];metrics=collections.Counter();contacts=collections.Counter();previous=t['initial'];id=0;trajectory=[];choices=[];encounters=[];details=[]
    def context(i,point,busy,end,now):
        owner=i['own']['id'];assert i['own']==point['agents'][owner] and i['profile']==profiles[owner]
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
        for capture in p['waves']:
            w=capture['wave']
            assert capture['memory_before']==memory
            available=[[c for c in cs if c['until']>w['before']['tick']] for cs in memory]
            assert capture['available']==available
            metrics['expired_cells']+=sum(len(a)-len(b) for a,b in zip(memory,available));memory=available
            relative=w['before']['tick']-p['start']
            willing=(relative>=t['config']['shift']) if t['case']=='late_willing' else (t['config']['shift']<=relative<t['config']['shift']+10) if t['case']=='brief_willing' else (pno%3!=0) if t['case']=='longitudinal' else None
            if willing is not None:
                updated=dict(profiles[2],relationship_goal=100 if willing else 0,privacy=0 if willing else 100)
                if updated!=profiles[2]:
                    profile_changes.append(dict(tick=w['before']['tick'],actor=2,before=profiles[2],after=updated));profiles[2]=updated
            updates=[]
            for owner,d in enumerate(w['proposals']):
                if d['selected']=='Wait':continue
                outgoing=d['selected']['Invite'];recipient=outgoing['recipient'];kind=outgoing['kind']
                accepted=w['responses'][recipient]['selected']==dict(Accept=owner)
                x=w['execution']
                met=x is not None and {x['invitation']['proposer'],x['invitation']['recipient']}=={owner,recipient} and x['invitation']['kind']==kind
                free=all(w['before']['tick']+2>=busy[j] for j in [owner,recipient])
                outcome='Unavailable' if not free else 'Declined' if not accepted else 'Met' if met else 'AcceptedUnmet'
                signal=dict(owner=owner,partner=recipient,kind=kind,sent=w['before']['tick'],received=w['after']['tick'],outcome=outcome)
                u=learn(memory[owner],t['mode'],signal);updates.append(u);memory[owner]=u['after']
                metrics['feedback_'+outcome]+=1;metrics['feedback_evictions']+=u['evicted'] is not None
                metrics['feedback_expired_cells']+=sum(c['until']<=signal['received'] for c in u['before'])
                key=(owner,recipient,kind)
                if key in last_contact:intervals.append(dict(owner=owner,partner=recipient,kind=kind,previous=last_contact[key],sent=signal['sent'],interval=signal['sent']-last_contact[key]))
                if last_outcome.get(key)=='Declined' and outcome=='Declined':metrics['repeated_declines']+=1
                last_contact[key]=signal['sent'];last_outcome[key]=outcome
            assert capture['updates']==updates
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
                assert d['input']['own']['id']==owner and d['mode']=='History';people=context(d['input'],before,busy,end,before['tick'])
                base_candidates,base_selected=prior.initiate(d['input'],'History')
                candidates,selected=initiate(d['input'],t['mode'],capture['available'][owner]);
                metrics['decisions_changed_by_retained_feedback']+=base_selected!=selected
                metrics['deferred_candidates']+=sum(a['eligible'] and not b['eligible'] for a,b in zip(base_candidates,candidates))
                metrics['wait_with_deferred_options']+=selected=='Wait' and base_selected!='Wait'
                assert (d['candidates'],d['selected'])==(candidates,selected)
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
                candidates,selected=response(i,t['mode']);assert (r['candidates'],r['selected'])==(candidates,selected)
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
    assert memory==t['final_cells'] and profile_changes==t['profile_changes']
    assert profiles==[base['intentional']['profiles'][str(i)] for i in range(3)]
    return dict(case=t['case'],mode=t['mode'],config=t['config'],metrics=dict(metrics),contacts=dict(contacts),choices=choices,encounters=encounters,inquiry_details=details,retry_intervals=intervals,trajectory=trajectory,final=previous['agents'])

def read(directory,name):
    p=directory/(name+'.json')
    return json.loads(p.read_bytes() if p.exists() else gzip.decompress((directory/(name+'.json.gz')).read_bytes()))

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--directory',type=Path,default=ROOT/'experiments/016');ap.add_argument('--self-test',action='store_true');ap.add_argument('--preview',action='store_true');args=ap.parse_args();directory=args.directory
    if (directory/'policy-freeze.json').exists():
        for n,h in json.loads((directory/'policy-freeze.json').read_bytes()).items():assert hashlib.sha256((ROOT/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==h,n
    if (directory/'archive-manifest.json').exists():
        for name,m in json.loads((directory/'archive-manifest.json').read_bytes()).items():
            b=(directory/(name+'.json.gz')).read_bytes();raw=gzip.decompress(b)
            assert (len(b),hashlib.sha256(b).hexdigest(),len(raw),hashlib.sha256(raw).hexdigest())==(m['gzip_bytes'],m['gzip_sha256'],m['json_bytes'],m['json_sha256'])
    rows=[];groups=collections.defaultdict(dict);first={};samples={};initial={}
    for name in ['seed-42'] if args.preview else ['seed-42','held-out-seed-42']:
        for t in read(directory,name):
            row=verify(t);rows.append(row);key=(t['case'],json.dumps(t['config'],sort_keys=True));groups[key][t['mode']]=row
            setup=(t['initial'],t['initial_profiles']);assert key not in first or first[key]==setup;first[key]=setup
            w=t['periods'][0]['waves'][0]['wave'];initial[(t['case'],t['mode'],key[1])]=(w['proposals'],w['responses'])
            if name=='seed-42' and t['mode']=='Typed':samples[t['case']]=t
    for (case,config),modes in groups.items():
        assert set(modes)==set(MODES)
        # Initial local choices match before the first actual feedback.
        for mode in ('Recent','Typed','Untyped'):assert initial[(case,mode,config)]==initial[(case,'Control',config)]
    ordering=0
    for (case,mode,config),w in initial.items():
        c=json.loads(config)
        if c['order']!='Ascending':
            base=json.dumps(dict(c,order='Ascending'),sort_keys=True);assert w==initial[(case,mode,base)];ordering+=1
    for (case,mode,config),w in initial.items():
        if case=='decline' and ('information',mode,config) in initial:assert w[0][0]==initial[('information',mode,config)][0][0]
    pairs=[]
    for (case,config),modes in groups.items():
        for a,b in itertools.combinations(MODES,2):
            l,r=modes[a],modes[b]
            material=lambda x:[(a['food'],a['hunger']) for a in x['final']]
            pairs.append(dict(case=case,config=json.loads(config),left=a,right=b,choices_changed=l['choices']!=r['choices'],encounters_changed=l['encounters']!=r['encounters'],trajectory_changed=l['trajectory']!=r['trajectory'],material_changed=material(l)!=material(r),trust_changed=[a['trust'] for a in l['final']]!=[a['trust'] for a in r['final']]))
    if args.self_test:
        for field in ['feedback','eligibility','time']:
            bad=copy.deepcopy(samples['decline']);capture=bad['periods'][0]['waves'][0];w=capture['wave']
            if field=='feedback':capture['updates'][0]['signal']['outcome']='Met'
            elif field=='eligibility':w['proposals'][0]['candidates'][1]['eligible']=not w['proposals'][0]['candidates'][1]['eligible']
            else:w['after']['tick']=bad['periods'][0]['end']+1
            try:verify(bad)
            except (AssertionError,KeyError,StopIteration):pass
            else:raise AssertionError('accepted corruption '+field)
    print(json.dumps(dict(independent_trials=len(rows),ordering_contrasts=ordering,trials=rows,comparisons=pairs),separators=(',',':')))
if __name__=='__main__':main()
