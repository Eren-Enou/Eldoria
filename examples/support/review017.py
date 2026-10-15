"""Read-only017 reconstruction, independent of Rust evaluator classifications.
Reuse reviewed006–015 cognition/resource primitives; derive017 scheduling/activities.
"""
import argparse,collections,copy,gzip,hashlib,itertools,json
from pathlib import Path
import review015 as old
ROOT=Path(__file__).resolve().parents[2]
MODES=('Serial','Committed','SerialActive','Active','NoHistory')
SERIAL={'Serial','SerialActive'}
EAT={'SerialActive','Active','NoHistory'}

def read(d,name):
    p=d/(name+'.json')
    return json.loads(p.read_bytes() if p.exists() else gzip.decompress((d/(name+'.json.gz')).read_bytes()))

def consume(agents,c,tick):
    a=agents[c['actor']]
    assert c['tick']==tick and (c['food_before'],c['hunger_before'])==(a['food'],a['hunger'])
    amount=int(a['food']>0 and a['hunger']>0)
    assert c['consumed']==amount
    a['food']-=amount
    if amount:a['hunger']=max(0,a['hunger']-40)
    assert (c['food_after'],c['hunger_after'])==(a['food'],a['hunger'])

def verify(t):
    points=t['points'];snap=t['final_state'];compact=snap['base']['base']['base'];base=compact['base']['base']
    events=base['events'];queries=compact['base']['inquiry']['records'];transitions=base['concerns']['transitions'];info=base['cognition']['information']
    old.cognitive.reconstruct(dict(final_state=snap,mode='Contextual',case=t['case'],config=t['config'],opportunities=[]))
    prefixes=old.assessment.bases(t);metrics=collections.Counter();timeline=[];meetings=[];details=[]
    active=None;wave=0;previous=t['initial'];new_events=set();new_queries=set();consumptions=[]
    def context(i,p,people,now,end):
        state=p['state'];owner=i['own']['id']
        assert set(i)=={'own','profile','now','end','people','query'}
        assert i['own']==state['agents'][owner] and i['profile']==t['profiles'][owner]
        assert i['people']==people and (i['now'],i['end'])==(now,end)
        q=i['query'];own=q['input']['base'];assert (own['owner'],own['hunger'],own['relationship_goal'],own['scene'])==(owner,i['own']['hunger'],i['profile']['relationship_goal'],p['scenes'])
        cs=old.concern_prefix(transitions,state['concerns']).get(owner,[]);assert own['concerns']==cs
        sources=[x['id'] for x in people if x['available'] and x['id']!=owner and not any(c['target']==x['id'] for c in cs)]
        assert [s['id'] for s in own['sources']]==sources and all(not s['offers'] for s in own['sources'])
        assert q['input']['knowledge']==old.cognitive.provenance_prefix(compact,now).get(owner,[])
        items=prefixes[state['assessment']].get(str(owner),[])
        basis=[dict(concern=c['id'],event=c['event'],support=old.assessment.grouped(items,c['event']),context='EvidencePresent' if any(x['event']==c['event'] and x['quality']>0 and 'Claim' not in x['origin'] for x in items) else 'ClaimFallback') for c in cs if c['status'] in ('Open','Partial')]
        candidates,selected,_=old.assessment.scores(own,{k:q['input'][k] for k in ('knowledge','hints','settings')},basis,old.cells_prefix(snap['learning']['records'],owner,state['queries']),'Contextual')
        assert (q['decision']['candidates'],q['decision']['selected'])==(candidates,selected)
    def resource(es,agents):
        for e in es:
            scene=base['scenes'][e['scene']];prior=[x for x in events[:e['id']] if x['scene']==e['scene']];turn=len(prior)
            ids=scene['participants'];assert e['participants']==ids and e['decision']['actor']==ids[turn%2]
            signal=None if not prior else dict(actor=prior[-1]['decision']['actor'],action=prior[-1]['decision']['selected'])
            assert e['decision']['observation']==dict(partner=ids[1-turn%2],last_signal=signal,amount=1,turns_left=6-turn)
            action=e['decision']['selected'];outcome='Agreement' if action=='Accept' else 'Refusal' if action=='Refuse' else 'Withdrawal' if action=='Leave' else 'Timeout' if turn+1==6 else None
            assert e['outcome']==outcome and e['transferred']==int(action=='Accept')
            metrics['transfers']+=e['transferred'];metrics['refusals']+=action=='Refuse';new_events.add(e['id'])
            agents=old.resources.resource([e],agents)
        return agents
    for pno,p in enumerate(t['periods']):
        assert points[p['before']]==previous and p['start']==previous['state']['tick'] and p['end']>p['start']
        for s in p['steps']:
            before=points[s['before']];observed=points[s['observed']];after=points[s['after']];now=before['state']['tick']
            assert before==previous and s['wave']==wave and s['active_before']==active
            busy=set() if active is None else {active['invitation']['proposer'],active['invitation']['recipient']}
            people=[dict(id=n,available=n not in busy) for n in range(3)]
            assert s['people']==people and all(set(x)=={'id','available'} for x in people)
            if active is None:
                assert observed==dict(before,state=dict(before['state'],concerns=observed['state']['concerns']))
                changes=transitions[before['state']['concerns']:observed['state']['concerns']]
                cs=old.concern_prefix(transitions,before['state']['concerns']);expected=[c for owner in sorted(cs) for c in cs[owner] if c['status'] in ('Open','Partial')]
                assert len(changes)==len(expected)
                for r,c in zip(changes,expected,strict=True):assert r['reason']=='015 observed encounter wave' and r['before']==c and r['after']==dict(c,age=min(2**32-1,c['age']+1))
            else:assert observed==before
            assert len(s['decisions'])==3
            eating=[];outgoing=[];selected=[]
            for owner,d in enumerate(s['decisions']):
                if owner in busy:assert d is None;outgoing.append(None);selected.append(None);continue
                assert d['mode']==t['mode'];c=d['contact'];i=c['input'];assert i['own']['id']==owner
                context(i,observed,people,now,p['end'])
                mode='NoHistory' if t['mode']=='NoHistory' else 'History';assert c['mode']==mode
                cs,choice=old.initiate(i,mode);assert (c['candidates'],c['selected'])==(cs,choice)
                eat=t['mode'] in EAT and i['own']['food']>0 and i['own']['hunger']>0 and now<p['end'];invite=choice!='Wait'
                score=next(x['score'] for x in cs if x['intent']==choice)
                candidates=[dict(action='Wait',eligible=True,score=0),dict(action='Consume',eligible=eat,score=i['own']['hunger'] if eat else -1),dict(action='Invite',eligible=invite,score=score if invite else -1)]
                assert d['candidates']==candidates and d['selected']==max(candidates,key=lambda x:x['score'])['action']
                actual=copy.deepcopy(c)
                if d['selected']!='Invite':actual['selected']='Wait'
                outgoing.append(actual);selected.append(d['selected'])
                if d['selected']=='Consume':eating.append(owner)
                metrics['busy_excluded_candidates']+=sum(x['intent']!='Wait' and not x['eligible'] and x['intent']['Invite']['recipient'] in busy for x in cs)
            assert [c['actor'] for c in s['consumption']]==eating and all(c['consumed']==1 for c in s['consumption'])
            consumptions+=s['consumption'];metrics['voluntary_consumed']+=len(eating)
            start_agents=copy.deepcopy(observed['state']['agents'])
            if active is not None:
                assert not s['responses'] and not s['allocations'] and s['sent'] is None and s['admitted'] is None and s['query'] is None and s['started'] is None
                assert after['state']['tick']==now+1 and after['scenes']==before['scenes']
                es=events[before['state']['events']:after['state']['events']];assert len(es)==1 and es[0]['tick']==now+1 and es[0]['scene']==active['scene']
                agents=resource(es,start_agents)
                for c in s['consumption']:assert c['actor'] not in busy;consume(agents,c,now+1)
                assert agents==after['state']['agents']
                finished=es[-1]['outcome'] is not None
                assert s['finished']==(active if finished else None)
                assert s['active_after']==(None if finished else active)
                metrics['occupied_steps']+=1
                if finished:meetings.append(dict(invitation=active['invitation'],start=active['start'],end=now+1,scene=active['scene']))
            else:
                wave+=1
                inv=[dict(proposer=n,recipient=d['selected']['Invite']['recipient'],kind=d['selected']['Invite']['kind'],expires=now+2) for n,d in enumerate(outgoing) if d['selected']!='Wait']
                metrics['invitations']+=len(inv)
                for c in s['consumption']:consume(start_agents,c,now+1)
                if not inv:
                    assert not s['responses'] and not s['allocations'] and s['sent'] is None and s['admitted'] is None and s['started'] is None and s['query'] is None and s['finished'] is None and s['active_after'] is None
                    assert after==dict(observed,state=dict(observed['state'],tick=now+1 if eating else p['end'],agents=start_agents))
                else:
                    sent=points[s['sent']];admitted=points[s['admitted']]
                    assert sent==dict(observed,state=dict(observed['state'],tick=now+1,agents=start_agents))
                    assert admitted==dict(sent,state=dict(sent['state'],tick=now+2))
                    assert len(s['responses'])==3
                    for owner,r in enumerate(s['responses']):
                        i=r['input'];context(i['own'],sent,people,now+1,p['end'])
                        assert i['outgoing']==outgoing[owner] and i['incoming']==[v for v in inv if v['recipient']==owner]
                        cs,_=old.response(i)
                        for c in cs:
                            if c['reply']=='Continue':c['score']=0
                        assert (r['candidates'],r['selected'])==(cs,max(cs,key=lambda x:x['score'])['reply'])
                    allocations=old.match(now+2,t['config']['order'],s['wave'],people,inv,s['responses']);assert s['allocations']==allocations
                    metrics.update(a['disposition'] for a in allocations)
                    executed=[a['invitation'] for a in allocations if a['disposition']=='Execute'];assert len(executed)<=1
                    if not executed:
                        assert after==admitted and s['started'] is None and s['finished'] is None and s['query'] is None and s['active_after'] is None
                    elif executed[0]['kind']=='Resource':
                        v=executed[0];commit=dict(invitation=v,scene=admitted['scenes'],start=now+2,first_event=admitted['state']['events'])
                        assert s['started']==commit and now+8<=p['end'] and s['query'] is None
                        if t['mode'] in SERIAL:
                            es=events[commit['first_event']:after['state']['events']]
                            assert es and len(es)<=6 and es[-1]['outcome'] is not None
                            assert [e['tick'] for e in es]==list(range(now+3,now+3+len(es))) and all(e['scene']==commit['scene'] for e in es)
                            assert after['state']['tick']==now+2+len(es) and s['finished']==commit and s['active_after'] is None
                            assert resource(es,start_agents)==after['state']['agents']
                            meetings.append(dict(invitation=v,start=now+2,end=after['state']['tick'],scene=commit['scene']))
                        else:
                            assert s['active_after']==commit and s['finished'] is None and after==dict(admitted,scenes=admitted['scenes']+1)
                        assert after['scenes']==admitted['scenes']+1
                    else:
                        v=executed[0];assert s['started'] is None and s['active_after'] is None and s['finished'] is None
                        assert s['query']==admitted['state']['queries'];q=queries[s['query']];new_queries.add(q['id'])
                        preview=outgoing[v['proposer']]['input']['query'];assert q['decision']==preview['decision'] and q['decision']['selected']['Ask']['source']==v['recipient']
                        assert q['time_spent']==after['state']['tick']-now-2 and 0<q['time_spent']<=31 and after['state']['queries']==admitted['state']['queries']+1
                        assert old.query_agents(start_agents,events,info,now+2,after['state']['tick'])==after['state']['agents'] and after['state']['events']==admitted['state']['events'] and after['scenes']==admitted['scenes']
                        learning=next(r for r in snap['learning']['records'] if r['inquiry']==q['id']);details.append(dict(inquiry=q['id'],owner=v['proposer'],source=v['recipient'],start=now+2,end=after['state']['tick'],outcome=learning['outcome']))
                        metrics['queries']+=1
            assert now<after['state']['tick']<=p['end']
            timeline.append(dict(start=now,end=after['state']['tick'],scene=None if active is None else active['scene'],available=[x['available'] for x in people],selected=selected,consume=eating,started=s['started'],finished=s['finished'],query=s['query']))
            active=s['active_after'];previous=after
        assert previous['state']['tick']==p['end'] and active is None
        agents=copy.deepcopy(previous['state']['agents'])
        assert len(p['consumption'])==3
        for owner,c in enumerate(p['consumption']):assert c['actor']==owner;consume(agents,c,p['end']+owner+1)
        consumptions+=p['consumption'];metrics['mandatory_consumed']+=sum(c['consumed'] for c in p['consumption'])
        assert points[p['after']]==dict(previous,state=dict(previous['state'],tick=p['end']+3,agents=agents))
        previous=points[p['after']]
    assert new_events==set(range(t['initial']['state']['events'],len(events)))
    assert new_queries==set(range(t['initial']['state']['queries'],len(queries)))
    assert consumptions==[c for c in base['cognition']['consumption'] if c['tick']>t['initial']['state']['tick']]
    assert previous['state']['agents']==base['agents'] and previous['state']['concerns']==len(transitions)
    assert [base['intentional']['profiles'][str(i)] for i in range(3)]==t['profiles']
    metrics['consumed']=metrics['voluntary_consumed']+metrics['mandatory_consumed']
    assert sum(a['food'] for a in t['initial']['state']['agents'])==sum(a['food'] for a in previous['state']['agents'])+metrics['consumed']
    return dict(case=t['case'],mode=t['mode'],config=t['config'],metrics=dict(metrics),meetings=meetings,inquiries=details,timeline=timeline,final=previous['state']['agents'],points=len(points))

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--directory',type=Path,default=ROOT/'experiments/017');ap.add_argument('--self-test',action='store_true');args=ap.parse_args()
    trials=read(args.directory,'seed-42')+read(args.directory,'held-out-seed-42');rows=[verify(t) for t in trials]
    groups=collections.defaultdict(dict)
    for t,r in zip(trials,rows):groups[(t['case'],json.dumps(t['config'],sort_keys=True))][t['mode']]=(t,r)
    for modes in groups.values():
        assert set(modes)==set(MODES)
        assert all(t['initial']==modes['Serial'][0]['initial'] and t['profiles']==modes['Serial'][0]['profiles'] for t,r in modes.values())
        assert modes['Serial'][0]['periods'][0]['steps'][0]['decisions']==modes['Committed'][0]['periods'][0]['steps'][0]['decisions'] or all(a['contact']==b['contact'] and a['selected']==b['selected'] for a,b in zip(modes['Serial'][0]['periods'][0]['steps'][0]['decisions'],modes['Committed'][0]['periods'][0]['steps'][0]['decisions']))
        assert modes['Serial'][0]['final_state']==modes['Committed'][0]['final_state']
        a,b=[modes[m][0]['periods'][0]['steps'][0]['decisions'] for m in ('SerialActive','Active')]
        assert all(x['contact']==y['contact'] and x['selected']==y['selected'] for x,y in zip(a,b))
    order_controls=0
    for (case,config),modes in groups.items():
        c=json.loads(config)
        if c['order']!='Ascending':
            baseline=groups[(case,json.dumps(dict(c,order='Ascending'),sort_keys=True))]
            for mode,(t,r) in modes.items():
                assert t['initial']==baseline[mode][0]['initial']
                assert t['periods'][0]['steps'][0]['decisions']==baseline[mode][0]['periods'][0]['steps'][0]['decisions']
                order_controls+=1
        if case=='private_mirror':
            twin=groups[('information',config)]
            for mode,(t,r) in modes.items():
                assert t['periods'][0]['steps'][0]['decisions'][0]==twin[mode][0]['periods'][0]['steps'][0]['decisions'][0]
    pairs=[]
    for (case,config),modes in groups.items():
        for l,r in itertools.combinations(MODES,2):
            a,b=modes[l][1],modes[r][1]
            pairs.append(dict(case=case,config=json.loads(config),left=l,right=r,material_changed=[(x['food'],x['hunger']) for x in a['final']]!=[(x['food'],x['hunger']) for x in b['final']],trust_changed=[x['trust'] for x in a['final']]!=[x['trust'] for x in b['final']],resource_meetings_changed=a['meetings']!=b['meetings'],inquiries_changed=a['inquiries']!=b['inquiries']))
    rejected=[]
    if args.self_test:
        sample=next(t for t in trials if t['case']=='competing_eater' and t['mode']=='Active')
        for category in ('commitment','release','observation','food','selection','time','consent'):
            bad=copy.deepcopy(sample);steps=bad['periods'][0]['steps'];occupied=next(s for s in steps if s['active_before']);wave=next(s for s in steps if s['responses'])
            if category=='commitment':occupied['active_before']['scene']+=999
            elif category=='release':next(s for s in steps if s['finished'])['finished']['start']+=1
            elif category=='observation':occupied['people'][0]['available']=True
            elif category=='food':bad['points'][bad['periods'][-1]['after']]['state']['agents'][0]['food']+=1
            elif category=='selection':next(d for d in occupied['decisions'] if d)['selected']='Invite'
            elif category=='time':bad['points'][occupied['after']]['state']['tick']+=1
            else:wave['responses'][1]['selected']='Wait'
            try:verify(bad)
            except (AssertionError,KeyError,StopIteration,IndexError):rejected.append(category)
            else:raise AssertionError('accepted corruption '+category)
    assert len(trials)==235 and len(groups)==47 and order_controls==130
    print(json.dumps(dict(histories=len(rows),matched_groups=len(groups),order_controls=order_controls,corruptions=rejected,trials=rows,comparisons=pairs),separators=(',',':')))

if __name__=='__main__':main()
