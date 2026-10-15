"""Read-only018 reconstruction: independent elapsed physics and actor execution.
Reuse reviewed006–015 cognitive/resource arithmetic, never evaluator classifications.
Stream original JSONL archives; bounded to one full trial rather than the whole suite.
"""
import argparse,collections,copy,gzip,hashlib,itertools,json
from pathlib import Path
import review015 as old
ROOT=Path(__file__).resolve().parents[2]
MODES=('Fixed','Recurring','Serial','FixedContact','ContactOnly','EatFirst','NoHistory')
SERIAL={'Serial'}
EAT={'Fixed','Recurring','Serial','EatFirst','NoHistory'}
def mapped(mode):return 'SerialActive' if mode=='Serial' else 'Committed' if mode in ('FixedContact','ContactOnly') else 'NoHistory' if mode=='NoHistory' else 'Active'
def read(d,name):
    compressed=d/(name+'.jsonl.gz');p=compressed if compressed.exists() else d/(name+'.jsonl')
    with (gzip.open(p,'rt',encoding='utf-8') if p.suffix=='.gz' else p.open(encoding='utf-8')) as f:
        for line in f:yield json.loads(line)
def consume(agents,c,tick):
    a=agents[c['actor']];assert c['tick']==tick and (c['food_before'],c['hunger_before'])==(a['food'],a['hunger'])
    assert a['food']>0 and a['hunger']>0 and c['consumed']==1
    a['food']-=1;a['hunger']=max(0,a['hunger']-40)
    assert (c['food_after'],c['hunger_after'])==(a['food'],a['hunger'])
def verify(t):
    points=t['points'];snap=t['final_state'];compact=snap['base']['base']['base'];base=compact['base']['base']
    events=base['events'];queries=compact['base']['inquiry']['records'];transitions=base['concerns']['transitions'];info=base['cognition']['information']
    old.cognitive.reconstruct(dict(final_state=snap,mode='Contextual',case=t['case'],config=t['config'],opportunities=[]))
    prefixes=old.assessment.bases(t);metrics=collections.Counter();timeline=[];meetings=[];details=[]
    active=None;wave=0;previous=t['initial'];new_events=set();new_queries=set();consumptions=[]
    index=0;clock=t['clock'];cursor=clock['epoch'];assert cursor==t['initial']['state']['tick']
    increment=0 if t['mode'] in ('Fixed','FixedContact') else 5
    assert clock['settings']==dict(interval=t['config']['interval'],increment=increment) and clock['settings']['interval']>=2
    def sync(agents,to):
        nonlocal index,cursor
        assert to>=cursor
        while cursor<to:
            n=t['needs'][index];assert n['from']==cursor and cursor<n['to']<=to
            pulses=(n['to']-clock['epoch'])//t['config']['interval']-(cursor-clock['epoch'])//t['config']['interval']
            assert n['pulses']==pulses and n['before']==[a['hunger'] for a in agents]
            expected=[min(100,h+5*pulses) if increment else h for h in n['before']]
            assert n['after']==expected
            for a,h in zip(agents,expected,strict=True):a['hunger']=h
            metrics['actual_hunger_added']+=sum(b-a for a,b in zip(n['before'],expected,strict=True));metrics['need_updates']+=1
            cursor=n['to'];index+=1
    period_states=[]

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
            sync(agents,e['tick']);scene=base['scenes'][e['scene']];prior=[x for x in events[:e['id']] if x['scene']==e['scene']];turn=len(prior)
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
        for wrapper in p['steps']:
            assert wrapper['need_before']==index
            s=wrapper['activity']
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
                assert d['mode']==mapped(t['mode']);c=d['contact'];i=c['input'];assert i['own']['id']==owner
                context(i,observed,people,now,p['end'])
                mode='NoHistory' if t['mode']=='NoHistory' else 'History';assert c['mode']==mode
                cs,choice=old.initiate(i,mode);assert (c['candidates'],c['selected'])==(cs,choice)
                eat=t['mode'] in EAT and i['own']['food']>0 and i['own']['hunger']>0 and now<p['end'];invite=choice!='Wait'
                score=next(x['score'] for x in cs if x['intent']==choice)
                candidates=[dict(action='Wait',eligible=True,score=0),dict(action='Consume',eligible=eat,score=i['own']['hunger'] if eat else -1),dict(action='Invite',eligible=invite,score=score if invite else -1)]
                assert d['candidates']==candidates and d['selected']==('Consume' if t['mode']=='EatFirst' and eat else max(candidates,key=lambda x:x['score'])['action'])
                actual=copy.deepcopy(c)
                if d['selected']!='Invite':actual['selected']='Wait'
                outgoing.append(actual);selected.append(d['selected'])
                if d['selected']=='Consume':eating.append(owner)
                if d['selected']=='Invite':
                    metrics['invitations_actor_'+str(owner)]+=1
                    metrics['inquiry_invitations' if c['selected']['Invite']['kind']=='Inquiry' else 'resource_invitations']+=1
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
                sync(start_agents,now+1)
                for c in s['consumption']:consume(start_agents,c,now+1)
                if not inv:
                    assert not s['responses'] and not s['allocations'] and s['sent'] is None and s['admitted'] is None and s['started'] is None and s['query'] is None and s['finished'] is None and s['active_after'] is None
                    assert after==dict(observed,state=dict(observed['state'],tick=now+1,agents=start_agents))
                else:
                    sent=points[s['sent']];admitted=points[s['admitted']]
                    assert sent==dict(observed,state=dict(observed['state'],tick=now+1,agents=start_agents))
                    admitted_agents=copy.deepcopy(start_agents);sync(admitted_agents,now+2)
                    assert admitted==dict(sent,state=dict(sent['state'],tick=now+2,agents=admitted_agents))
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
                            assert resource(es,admitted_agents)==after['state']['agents']
                            meetings.append(dict(invitation=v,start=now+2,end=after['state']['tick'],scene=commit['scene']))
                        else:
                            assert s['active_after']==commit and s['finished'] is None and after==dict(admitted,scenes=admitted['scenes']+1)
                        assert after['scenes']==admitted['scenes']+1
                    else:
                        v=executed[0];assert s['started'] is None and s['active_after'] is None and s['finished'] is None
                        assert s['query']==admitted['state']['queries'];q=queries[s['query']];new_queries.add(q['id'])
                        preview=outgoing[v['proposer']]['input']['query']
                        assert q['decision']['input']==dict(preview['input']['base'],hunger=admitted_agents[v['proposer']]['hunger'])
                        assert q['decision']['selected']==preview['decision']['selected'] and q['decision']['selected']['Ask']['source']==v['recipient']
                        ask=q['decision']['selected']['Ask'];response_tick=now+2+1+(10 if ask['strategy']=='Direct' else 28)
                        sync(admitted_agents,response_tick)
                        if q['response'] is not None:
                            ri=q['response']['decision']['input'];assert ri['own']==admitted_agents[v['recipient']] and ri['profile']==t['profiles'][v['recipient']]
                        else:
                            exchange=next(x for x in compact['provenance']['exchanges'] if now+2<x['tick']<=after['state']['tick'])
                            assert exchange['tick']-exchange['time_spent']==response_tick
                            ri=exchange['decision']['input'];assert ri['own']==admitted_agents[v['recipient']] and ri['profile']==t['profiles'][v['recipient']]
                        sync(admitted_agents,after['state']['tick'])
                        assert q['time_spent']==after['state']['tick']-now-2 and 0<q['time_spent']<=31 and after['state']['queries']==admitted['state']['queries']+1
                        calculated=old.query_agents(admitted_agents,events,info,now+2,after['state']['tick'])
                        assert calculated==after['state']['agents'],(t['case'],t['mode'],t['config'],pno,s['query'],[(i,{k:(x[k],y[k]) for k in x if x[k]!=y[k]}) for i,(x,y) in enumerate(zip(calculated,after['state']['agents'],strict=True)) if x!=y])
                        assert after['state']['events']==admitted['state']['events'] and after['scenes']==admitted['scenes']
                        learning=next(r for r in snap['learning']['records'] if r['inquiry']==q['id']);details.append(dict(inquiry=q['id'],owner=v['proposer'],source=v['recipient'],start=now+2,end=after['state']['tick'],outcome=learning['outcome'],selected=q['decision']['selected'],hunger_at_selection=preview['input']['base']['hunger'],hunger_at_admission=q['decision']['input']['hunger']))
                        event=events[next(c['event'] for c in q['decision']['input']['concerns'] if c['id']==ask['concern'])]
                        assert event['decision']['selected']=='Refuse'
                        truth=event['decision']['food']<=1 and event['decision']['hunger']>=60
                        outcome=learning['outcome']
                        if outcome is not None:
                            changed=(outcome['support_before'],outcome['status_before'])!=(outcome['support_after'],outcome['status_after'])
                            wrong=outcome['support_after']!=0 and (outcome['support_after']>0)!=truth
                            metrics['retained_changes']+=changed
                            metrics['receipt_without_change']+=outcome['receipt_acquired'] and not changed
                            metrics['incorrect_assessment_after']+=wrong
                            metrics['new_misleading_assimilation']+=outcome['receipt_acquired'] and changed and wrong
                            metrics['confident_wrong_after']+=wrong and abs(outcome['support_after'])>=60
                            details[-1]['observer_historical_scarcity']=truth
                        metrics['queries']+=1
            assert now<after['state']['tick']<=p['end'] and wrapper['need_after']==index and cursor==after['state']['tick']
            metrics['world_ticks']+=after['state']['tick']-now
            metrics['handshake_ticks']+=2*bool(s['responses'])
            metrics['resource_ticks']+=after['state']['events']-before['state']['events']
            metrics['query_ticks']+=(queries[s['query']]['time_spent'] if s['query'] is not None else 0)
            if not s['responses'] and s['active_before'] is None:
                metrics['consumption_only_ticks' if eating else 'idle_ticks']+=after['state']['tick']-now
            timeline.append(dict(start=now,end=after['state']['tick'],scene=None if active is None else active['scene'],available=[x['available'] for x in people],selected=selected,consume=eating,started=s['started'],finished=s['finished'],query=s['query']))
            active=s['active_after'];previous=after
        assert previous['state']['tick']==p['end'] and active is None
        assert points[p['after']]==previous
        period_states.append(dict(tick=previous['state']['tick'],food=[a['food'] for a in previous['state']['agents']],hunger=[a['hunger'] for a in previous['state']['agents']],trust=[a['trust'] for a in previous['state']['agents']]))
    assert new_events==set(range(t['initial']['state']['events'],len(events)))
    assert new_queries==set(range(t['initial']['state']['queries'],len(queries)))
    assert consumptions==[c for c in base['cognition']['consumption'] if c['tick']>t['initial']['state']['tick']]
    assert previous['state']['agents']==base['agents'] and previous['state']['concerns']==len(transitions)
    assert [base['intentional']['profiles'][str(i)] for i in range(3)]==t['profiles']
    metrics['consumed']=metrics['voluntary_consumed']
    assert index==len(t['needs']) and cursor==clock['last']
    assert metrics['world_ticks']==sum(metrics[k] for k in ('handshake_ticks','resource_ticks','query_ticks','consumption_only_ticks','idle_ticks'))
    metrics['hunger_reduced']=sum(c['hunger_before']-c['hunger_after'] for c in consumptions)
    assert sum(a['hunger'] for a in previous['state']['agents'])==sum(a['hunger'] for a in t['initial']['state']['agents'])+metrics['actual_hunger_added']-metrics['hunger_reduced']
    assert sum(a['food'] for a in t['initial']['state']['agents'])==sum(a['food'] for a in previous['state']['agents'])+metrics['consumed']
    per_actor=[]
    for owner in range(3):
        cs=[c for c in consumptions if c['actor']==owner]
        added=sum(n['after'][owner]-n['before'][owner] for n in t['needs']);reduced=sum(c['hunger_before']-c['hunger_after'] for c in cs)
        assert previous['state']['agents'][owner]['hunger']==t['initial']['state']['agents'][owner]['hunger']+added-reduced
        resource_contacts=sum(owner in (m['invitation']['proposer'],m['invitation']['recipient']) for m in meetings)
        query_contacts=sum(owner in (q['owner'],q['source']) for q in details)
        per_actor.append(dict(actor=owner,consumed=len(cs),hunger_added=added,hunger_reduced=reduced,resource_contacts=resource_contacts,query_contacts=query_contacts))
    metrics['concern_transitions']=len(transitions)-t['initial']['state']['concerns']
    metrics['resolved_transitions']=sum(c['after'] is not None and c['after']['status']=='Resolved' and (c['before'] is None or c['before']['status']!='Resolved') for c in transitions[t['initial']['state']['concerns']:])
    actions=[dict(tick=e['tick'],participants=e['participants'],actor=e['decision']['actor'],action=e['decision']['selected'],transferred=e['transferred'],outcome=e['outcome']) for e in events[t['initial']['state']['events']:]]
    return dict(seed=t['seed'],case=t['case'],mode=t['mode'],config=t['config'],metrics=dict(metrics),meetings=meetings,inquiries=details,timeline=timeline,final=previous['state']['agents'],points=len(points),period_states=period_states,per_actor=per_actor,resource_actions=actions,consumption_hunger_histogram=dict(sorted(collections.Counter(c['hunger_before'] for c in consumptions).items())))

def fingerprint(x):return hashlib.sha256(json.dumps(x,sort_keys=True,separators=(',',':')).encode()).hexdigest()
def signature(row):return [(a['food'],a['hunger']) for a in row['final']]
def compare(a,b):
    activity=lambda r:[(s['start'],i,x) for s in r['timeline'] for i,x in enumerate(s['selected']) if x not in (None,'Wait')]
    inquiry=lambda r:[(q['start'],q['owner'],q['selected']) for q in r['inquiries']]
    periods=list(zip(a['period_states'],b['period_states'],strict=True))
    material=[(x['food'],x['hunger'])!=(y['food'],y['hunger']) for x,y in periods]
    return dict(case=a['case'],config=a['config'],left=a['mode'],right=b['mode'],activity_trace_changed=activity(a)!=activity(b),inquiry_allocation_changed=inquiry(a)!=inquiry(b),inquiry_selection_changed=[(q['owner'],q['selected']) for q in a['inquiries']]!=[(q['owner'],q['selected']) for q in b['inquiries']],inquiry_outcome_changed=[(q['owner'],q['selected'],q['outcome']) for q in a['inquiries']]!=[(q['owner'],q['selected'],q['outcome']) for q in b['inquiries']],resource_execution_changed=a['meetings']!=b['meetings'],resource_action_changed=[{k:v for k,v in e.items() if k!='tick'} for e in a['resource_actions']]!=[{k:v for k,v in e.items() if k!='tick'} for e in b['resource_actions']],trust_changed=[x['trust'] for x in a['final']]!=[x['trust'] for x in b['final']],material_changed=signature(a)!=signature(b),material_periods=sum(material),successive_material_periods=any(x and y for x,y in zip(material,material[1:])),reconverged=any(material) and not material[-1],consumed_left=a['metrics'].get('consumed',0),consumed_right=b['metrics'].get('consumed',0))
def corruptions(sample):
    rejected=[]
    for category in ('need_progression','elapsed_time','food','occupied_identity','release','actor_observation','selected_activity','consent','resource_transfer','retained_cognition'):
        bad=copy.deepcopy(sample);steps=[s['activity'] for p in bad['periods'] for s in p['steps']];busy=next(s for s in steps if s['active_before']);wave=next(s for s in steps if s['responses'])
        if category=='need_progression':next(n for n in bad['needs'] if n['pulses'])['after'][0]+=1
        elif category=='elapsed_time':bad['points'][busy['after']]['state']['tick']+=1
        elif category=='food':bad['points'][bad['periods'][-1]['after']]['state']['agents'][0]['food']+=1
        elif category=='occupied_identity':busy['active_before']['invitation']['recipient']=2
        elif category=='release':next(s for s in steps if s['finished'])['finished']['start']+=1
        elif category=='actor_observation':busy['people'][0]['available']=True
        elif category=='selected_activity':next(d for d in busy['decisions'] if d)['selected']='Invite'
        elif category=='consent':next(r for r in wave['responses'] if r['selected']!='Wait')['selected']='Wait'
        elif category=='resource_transfer':next(e for e in bad['final_state']['base']['base']['base']['base']['base']['events'] if e['transferred'])['transferred']+=1
        else:wave['decisions'][0]['contact']['input']['query']['input']['knowledge'].append(dict(event=9999999))
        try:verify(bad)
        except (AssertionError,KeyError,StopIteration,IndexError):rejected.append(category)
        else:raise AssertionError('accepted corruption '+category)
    return rejected

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--directory',type=Path,default=ROOT/'experiments/018');ap.add_argument('--self-test',action='store_true');args=ap.parse_args()
    rows=[];groups=collections.defaultdict(dict);openings={};initial=None;count=0;rejected=[];nofood=None
    for name in ('seed-42','held-out-seed-42'):
        for t in read(args.directory,name):
            if count%7==0:initial=(t['initial'],t['profiles'])
            else:assert initial==(t['initial'],t['profiles'])
            row=verify(t);rows.append(row);key=(t['case'],json.dumps(t['config'],sort_keys=True));assert t['mode'] not in groups[key];groups[key][t['mode']]=row
            first=t['periods'][0]['steps'][0]['activity'];openings[key+(t['mode'],)]=(fingerprint(t['initial']),fingerprint(first['decisions']),fingerprint(first['decisions'][0]))
            if t['case']=='no_food' and t['mode']=='Recurring':nofood=t['final_state']
            if t['case']=='no_food' and t['mode']=='Serial':assert t['final_state']==nofood
            if args.self_test and t['case']=='ongoing' and t['mode']=='Recurring' and t['config']['order']=='Ascending' and t['config']['interval']==4 and t['config']['stock']==12 and t['config']['budget']==24:rejected=corruptions(t)
            count+=1
    assert count==266 and len(groups)==38
    order_controls=0
    for (case,config),modes in groups.items():
        assert set(modes)==set(MODES);c=json.loads(config)
        if c['order']!='Ascending':
            baseline=json.dumps(dict(c,order='Ascending'),sort_keys=True)
            for mode in MODES:assert openings[(case,config,mode)][:2]==openings[(case,baseline,mode)][:2];order_controls+=1
        if case=='private_mirror':
            for mode in MODES:assert openings[(case,config,mode)][2]==openings[('information',config,mode)][2]
    assert order_controls==56
    pairs=[compare(modes[a],modes[b]) for modes in groups.values() for a,b in itertools.combinations(MODES,2)]
    assert len(pairs)==798
    invalid=next(t for t in read(args.directory/'development/initial-clock-hook','held-out-seed-42') if t['case']=='information' and t['mode']=='EatFirst' and t['config']['interval']==2)
    try:verify(invalid)
    except AssertionError:development_initial_rejected=True
    else:raise AssertionError('accepted preserved invalid hook candidate')
    print(json.dumps(dict(histories=count,matched_groups=len(groups),order_controls=order_controls,private_twins=7,corruptions=rejected,development_initial_rejected=development_initial_rejected,trials=rows,comparisons=pairs),separators=(',',':')))
if __name__=='__main__':main()
