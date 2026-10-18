"""Read-only021 primitive reconstruction. No Rust selector or prepared classification.
The physical/old-cognitive verifier is extended from reviewed020;new information
logic comes from independently written knowledge021,not simulation source calls.
"""
import argparse,collections,copy,gzip,json,time
from bisect import bisect_right
from pathlib import Path
import review020 as legacy
from knowledge021 import Memory,choose
old=legacy.old
parent=legacy.parent
ROOT=Path(__file__).resolve().parents[2]
read=legacy.read
rank=legacy.rank
allocate=legacy.allocate
def verify(t):
    assert set(t)=={'seed','case','mode','config','initial','profiles','points','periods','final_state','final_point','records','clock','needs','initial_total','policy','memory_records','final_memory'}
    points=t['points'];snap=t['final_state'];compact=snap['base']['base']['base'];base=compact['base']['base']
    events=base['events'];queries=compact['base']['inquiry']['records'];transitions=base['concerns']['transitions'];info=base['cognition']['information']
    old.cognitive.reconstruct(dict(final_state=snap,mode='Contextual',case=t['case'],config=t['config'],opportunities=[]))
    prefixes=old.assessment.bases(t)
    _,assessment_times=old.cognitive.assessment_prefixes(compact,snap['base']['base']['assessment'])
    previous=t['initial'];agents=copy.deepcopy(previous['state']['agents']);cache=copy.deepcopy(previous['cache']);pending=[];record_cursor=0;attempt_cursor=0;need_cursor=0;event_cursor=previous['state']['events'];query_cursor=previous['state']['queries'];scene_cursor=previous['scenes'];cursor=previous['state']['tick'];active=None;wave=0;consumptions=[];metrics=collections.Counter();timeline=[];details=[];area=[0,0,0]
    clock=t['clock'];epoch=cursor
    assert clock['epoch']==epoch and clock['settings']==dict(interval=4,increment=5)
    assert t['config']['duration'] in (2,4,8) and t['config']['window'] in (4,24)
    assert cache['id']==0 and 0<=cache['stock']<=cache['capacity'] and not previous['pending'] and previous['acquisitions']==0
    initial_total=sum(a['food'] for a in agents)+cache['stock'];assert initial_total==t['initial_total']
    policy=t['policy'];memory=Memory(t['mode'],epoch,t['config']['cadence'])
    assert policy in ('Need','Acquire') and t['mode'] in ('FreshReference','CurrentOnly','LastSeen','Expire12','Unobserved')
    assert t['config']['cadence'] in (4,8,16)
    def sync(to):
        nonlocal cursor,need_cursor
        while cursor<to:
            n=t['needs'][need_cursor];assert n['from']==cursor and cursor<n['to']<=to
            assert n['before']==[a['hunger'] for a in agents]
            pulses=(n['to']-epoch)//4-(cursor-epoch)//4
            assert n['pulses']==pulses and n['after']==[min(100,h+5*pulses) for h in n['before']]
            for tick in range(cursor,n['to']):
                for k,a in enumerate(agents):area[k]+=min(100,a['hunger']+5*((tick-epoch)//4-(cursor-epoch)//4))
            for a,h in zip(agents,n['after'],strict=True):a['hunger']=h
            cursor=n['to'];need_cursor+=1
    def busy():
        return {p['actor'] for p in pending}|(set() if active is None else {active['invitation']['proposer'],active['invitation']['recipient']})
    def public():return [dict(id=n,available=n not in busy()) for n in range(3)]
    def point(p):
        assert p['state']['tick']==cursor and p['state']['agents']==agents
        assert p['state']['assessment']==bisect_right(assessment_times,cursor),'assessment prefix contains unavailable cognition or omits a receipt'
        assert p['cache']==cache and p['pending']==pending and p['acquisitions']==record_cursor
        assert p['state']['events']==event_cursor and p['state']['queries']==query_cursor and p['scenes']==scene_cursor
        assert sum(a['food'] for a in agents)+cache['stock']+len(consumptions)==initial_total
    def tick():
        nonlocal event_cursor,record_cursor,agents
        sync(cursor+1)
        if active is not None:
            ev=events[event_cursor];assert ev['id']==event_cursor and ev['tick']==cursor and ev['scene']==active['scene']
            scene=base['scenes'][ev['scene']];prior=[x for x in events[:event_cursor] if x['scene']==ev['scene']];turn=len(prior);ids=scene['participants']
            assert ids==[active['invitation']['proposer'],active['invitation']['recipient']] and ev['participants']==ids and ev['decision']['actor']==ids[turn%2]
            signal=None if not prior else dict(actor=prior[-1]['decision']['actor'],action=prior[-1]['decision']['selected'])
            assert ev['decision']['observation']==dict(partner=ids[1-turn%2],last_signal=signal,amount=1,turns_left=6-turn)
            action=ev['decision']['selected'];outcome='Agreement' if action=='Accept' else 'Refusal' if action=='Refuse' else 'Withdrawal' if action=='Leave' else 'Timeout' if turn+1==6 else None
            assert ev['outcome']==outcome and ev['transferred']==int(action=='Accept')
            assert not {p['actor'] for p in pending}.intersection(ids)
            agents=old.resources.resource([ev],agents);event_cursor+=1
            metrics['resource_ticks']+=1;metrics['transfers']+=ev['transferred'];metrics['offers']+=action=='Offer';metrics['requests']+=action=='Request'
        due=sorted([p for p in pending if p['end']<=cursor],key=lambda p:(p['end'],rank(p['actor'],p['wave'],t['config']['order'])))
        for attempt in due:
            assert attempt['end']==cursor
            r=t['records'][record_cursor];a=agents[attempt['actor']]
            before=(cache['stock'],a['food']);outcome='Depleted' if cache['stock']==0 else 'Overflow' if a['food']==2**32-1 else 'Granted'
            if outcome=='Granted':cache['stock']-=1;a['food']+=1
            assert r==dict(id=record_cursor,attempt=attempt,completed_at=cursor,rank=rank(attempt['actor'],attempt['wave'],t['config']['order']),cache_before=before[0],cache_after=cache['stock'],food_before=before[1],food_after=a['food'],hunger=a['hunger'],outcome=outcome)
            prior=memory.slots[attempt['actor']]
            if outcome=='Granted' and t['mode']=='Expire12' and prior is not None and cursor-prior['observation']['observed_at']>=12:metrics['grant_after_expiry']+=1
            memory.result(attempt['actor'],attempt['cache'],cursor,outcome,record_cursor,t['config']['duration'])
            pending.remove(attempt);record_cursor+=1;metrics[outcome]+=1
    def meal(c):
        assert c['actor'] not in busy();parent.consume(agents,c,cursor);consumptions.append(c);metrics['consumed']+=1
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

    for pno,p in enumerate(t['periods']):
        assert points[p['before']]==previous and p['start']==cursor and p['end']==min(cursor+t['config']['window'],epoch+(576 if t['case'] in ('ongoing','stranded') else 144))
        for sno,w in enumerate(p['steps']):
            s=w['activity'];before=points[s['before']];observed=points[s['observed']];after=points[s['after']];now=cursor
            assert w['need_before']==need_cursor and before==previous and s['wave']==wave and s['active_before']==active
            assert w['memory_before']==len(memory.records)
            point(before);people=public();assert s['people']==people
            if active is None and any(x['available'] for x in people):
                assert observed==dict(before,state=dict(before['state'],concerns=observed['state']['concerns']))
                changes=transitions[before['state']['concerns']:observed['state']['concerns']];cs=old.concern_prefix(transitions,before['state']['concerns']);expected=[c for owner in sorted(cs) for c in cs[owner] if c['status'] in ('Open','Partial')]
                assert len(changes)==len(expected)
                for r,c in zip(changes,expected,strict=True):assert r['reason']=='015 observed encounter wave' and r['before']==c and r['after']==dict(c,age=min(2**32-1,c['age']+1))
            else:assert observed==before
            assert len(w['choices'])==len(s['decisions'])==3
            expected_admissions=[];outgoing=[];eating=[];selections=[];completed_before=record_cursor
            for owner,d in enumerate(w['choices']):
                if not people[owner]['available']:
                    assert d is None and s['decisions'][owner] is None;outgoing.append(None);selections.append(None);continue
                assert set(d)=={'retained','duration','inquiry_lane_free','policy','candidates','selected','acquiring_base'}
                base_decision=d['acquiring_base'] if d['selected']=='Acquire' else s['decisions'][owner]
                assert base_decision is not None
                if d['selected']!='Acquire':assert d['acquiring_base'] is None
                retained=memory.prepare(owner,now,cache,t['config']['duration'],s['observed'])
                assert d['retained']==retained
                i=dict(legacy=base_decision['contact']['input'],retained=d['retained'],duration=d['duration'],inquiry_lane_free=d['inquiry_lane_free'])
                full=dict(input=i,policy=d['policy'],base=base_decision,candidates=d['candidates'],selected=d['selected'])
                assert i['duration']==t['config']['duration'] and i['inquiry_lane_free']==(not pending)
                if retained is None:metrics['choice_unknown']+=1
                else:
                    upper=max(0,retained['observation']['stock']-retained['own_takes'])
                    metrics['choice_dated']+=retained['observation']['observed_at']<now
                    metrics['choice_overestimate']+=upper>cache['stock']
                    if d['selected']=='Acquire':metrics['acquire_overestimate']+=upper>cache['stock']
                context(i['legacy'],observed,people,now,p['end'])
                assert i['legacy']['own']['id']==owner and full==choose(i,policy)
                selections.append(d['selected']);metrics['choice_'+d['selected']]+=1
                if d['selected']=='Acquire':
                    assert s['decisions'][owner] is None
                    expected_admissions.append(dict(id=attempt_cursor,cache=cache['id'],actor=owner,start=now,end=now+t['config']['duration'],wave=wave,selection=[pno,sno,owner],observation=i['retained']['observation']))
                    attempt_cursor+=1;outgoing.append(None)
                else:
                    assert s['decisions'][owner]==base_decision
                    contact=copy.deepcopy(base_decision['contact'])
                    if d['selected']!='Invite':contact['selected']='Wait'
                    outgoing.append(contact)
                    if d['selected']=='Consume':eating.append(owner)
                memory.discard(owner,now)
            assert w['admissions']==expected_admissions;pending.extend(copy.deepcopy(expected_admissions));assert len(pending)<=3
            assert [c['actor'] for c in s['consumption']]==eating
            if active is not None:
                assert not w['provisional'] and not w['skipped_responses'] and not w['atomic_blocked']
                assert not s['responses'] and not s['allocations'] and s['sent'] is None and s['admitted'] is None and s['query'] is None and s['started'] is None
                tick()
                for c in s['consumption']:meal(c)
                finished=events[event_cursor-1]['outcome'] is not None
                assert s['finished']==(active if finished else None) and s['active_after']==(None if finished else active)
                if finished:active=None;metrics['meetings']+=1
            else:
                wave+=any(x['available'] for x in people)
                inv=[dict(proposer=n,recipient=d['selected']['Invite']['recipient'],kind=d['selected']['Invite']['kind'],expires=now+2) for n,d in enumerate(outgoing) if d and d['selected']!='Wait']
                tick()
                for c in s['consumption']:meal(c)
                if not inv:
                    assert not s['responses'] and not s['allocations'] and not w['provisional'] and not w['skipped_responses'] and not w['atomic_blocked']
                    assert s['sent'] is None and s['admitted'] is None and s['query'] is None and s['started'] is None and s['finished'] is None and s['active_after'] is None
                else:
                    sent=points[s['sent']];point(sent);response_people=public()
                    assert w['skipped_responses']==[x['id'] for x in response_people if not x['available']]
                    assert [r['input']['own']['own']['id'] for r in s['responses']]==[x['id'] for x in response_people if x['available']]
                    for response in s['responses']:
                        i=response['input'];owner=i['own']['own']['id'];context(i['own'],sent,response_people,now+1,p['end'])
                        if outgoing[owner] is not None:assert i['outgoing']==outgoing[owner]
                        else:assert i['outgoing']['selected']=='Wait'
                        assert i['incoming']==[v for v in inv if v['recipient']==owner]
                        cs,_=old.response(i)
                        for c in cs:
                            if c['reply']=='Continue':c['score']=0
                        assert response['candidates']==cs and response['selected']==max(cs,key=lambda c:c['score'])['reply']
                    tick();admitted=points[s['admitted']];point(admitted)
                    allocations=allocate(cursor,t['config']['order'],s['wave'],public(),inv,s['responses']);assert w['provisional']==allocations
                    blocked=[]
                    if pending:
                        for allocation in allocations:
                            if allocation['disposition']=='Execute' and allocation['invitation']['kind']=='Inquiry':blocked.append(allocation['invitation']);allocation['disposition']='Unavailable'
                    assert w['atomic_blocked']==blocked and s['allocations']==allocations;metrics['atomic_blocked']+=len(blocked)
                    selected=[a['invitation'] for a in allocations if a['disposition']=='Execute'];assert len(selected)<=1
                    if not selected:assert s['started'] is None and s['query'] is None and s['finished'] is None and s['active_after'] is None
                    elif selected[0]['kind']=='Resource':
                        v=selected[0];active=dict(invitation=v,scene=scene_cursor,start=cursor,first_event=event_cursor)
                        assert s['started']==active and s['active_after']==active and s['finished'] is None and s['query'] is None and cursor+6<=p['end'];scene_cursor+=1
                    else:
                        assert not pending and s['query']==query_cursor and s['started'] is None and s['finished'] is None and s['active_after'] is None
                        q=queries[query_cursor];v=selected[0];preview=outgoing[v['proposer']]['input']['query'];query_cursor+=1
                        assert q['decision']['input']==dict(preview['input']['base'],hunger=agents[v['proposer']]['hunger']) and q['decision']['selected']==preview['decision']['selected']
                        ask=q['decision']['selected']['Ask'];assert ask['source']==v['recipient'];admission=cursor;response_tick=cursor+1+(10 if ask['strategy']=='Direct' else 28)
                        sync(response_tick)
                        if q['response'] is not None:ri=q['response']['decision']['input']
                        else:
                            exchange=next(x for x in compact['provenance']['exchanges'] if admission<x['tick']<=after['state']['tick']);assert exchange['tick']-exchange['time_spent']==response_tick;ri=exchange['decision']['input']
                        assert ri['own']==agents[v['recipient']] and ri['profile']==t['profiles'][v['recipient']]
                        sync(after['state']['tick']);assert q['time_spent']==cursor-admission and 0<q['time_spent']<=31
                        agents=old.query_agents(agents,events,info,admission,cursor);metrics['queries']+=1
                        details.append(dict(id=q['id'],start=admission,end=cursor,owner=v['proposer'],source=v['recipient'],selected=q['decision']['selected'],outcome=next(r['outcome'] for r in snap['learning']['records'] if r['inquiry']==q['id'])))
            point(after)
            assert w['memory_after']==len(memory.records)
            assert w['completed']==list(range(completed_before,record_cursor)) and w['need_after']==need_cursor and now<cursor<=p['end']
            timeline.append(dict(period=pno,step=sno,start=now,end=cursor,selected=selections,admissions=[p['id'] for p in expected_admissions],completed=w['completed'],started=s['started'],finished=s['finished'],query=s['query']))
            previous=after
        assert cursor==p['end'] and active is None and not pending and points[p['after']]==previous
    assert previous==t['final_point'] and cursor-epoch==(576 if t['case'] in ('ongoing','stranded') else 144)
    assert record_cursor==len(t['records']) and attempt_cursor==record_cursor and need_cursor==len(t['needs']) and clock['last']==cursor
    assert event_cursor==len(events) and query_cursor==len(queries)
    assert consumptions==[c for c in base['cognition']['consumption'] if c['tick']>epoch]
    assert previous['state']['agents']==base['agents'] and previous['state']['concerns']==len(transitions)
    assert [base['intentional']['profiles'][str(i)] for i in range(3)]==t['profiles']
    final_memory=memory.finish(cursor)
    assert t['memory_records']==memory.records and t['final_memory']==final_memory
    return dict(seed=t['seed'],case=t['case'],policy=t['policy'],mode=t['mode'],config=t['config'],memory_changes=dict(collections.Counter(r['kind'] for r in memory.records)),final_memory=final_memory,metrics=dict(metrics),hunger_area=area,final=agents,cache=cache,period_states=[dict(tick=points[p['after']]['state']['tick'],food=[a['food'] for a in points[p['after']]['state']['agents']],hunger=[a['hunger'] for a in points[p['after']]['state']['agents']],cache=points[p['after']]['cache']['stock']) for p in t['periods']],timeline=timeline,inquiries=details)

def corruptions(samples):
    rejected=[]
    categories=('cache_identity','observation_stock','future_observation','private_leak','choice','claim_reference','deadline','duplicate_grant','missing_debit','false_success','owner','completion_order','overlap','need','meal','resource_event','missing_event','missing_record','missing_completion','elapsed_time','memory_owner','memory_counter','receipt_namespace','unavailable_recovery','unknown_is_not_zero','memory_missing','capacity','truth_patch')
    for category in categories:
        sample=samples['current'] if category in ('unavailable_recovery','unknown_is_not_zero') else samples['stale'] if category=='truth_patch' else samples['sharing'] if category in ('meal','resource_event','missing_event') else samples['contested']
        bad=copy.deepcopy(sample);steps=[w for p in bad['periods'] for w in p['steps']]
        claim=next(w for w in steps if w['admissions']);d=next(d for d in claim['choices'] if d and d['selected']=='Acquire')
        if category=='cache_identity':bad['records'][0]['attempt']['cache']=9
        elif category=='observation_stock':d['retained']['observation']['stock']+=1
        elif category=='future_observation':d['retained']['observation']['observed_at']+=1
        elif category=='private_leak':d['partner_food']=99
        elif category=='choice':d['selected']='Wait'
        elif category=='claim_reference':claim['admissions'][0]['selection'][1]+=1
        elif category=='deadline':bad['records'][0]['completed_at']+=1
        elif category=='duplicate_grant':bad['records'][1]['outcome']='Granted';bad['records'][1]['food_after']+=1
        elif category=='missing_debit':bad['records'][0]['cache_after']+=1
        elif category=='false_success':bad['records'][0]['outcome']='Depleted'
        elif category=='owner':bad['records'][0]['attempt']['actor']=2
        elif category=='completion_order':bad['records'][0],bad['records'][1]=bad['records'][1],bad['records'][0]
        elif category=='overlap':next(w for w in steps if w['choices'][0] is None)['choices'][0]=copy.deepcopy(d)
        elif category=='need':bad['needs'][0]['after'][0]=0
        elif category=='meal':next(w for w in steps if w['activity']['consumption'])['activity']['consumption'][0]['food_after']+=1
        elif category=='resource_event':bad['final_state']['base']['base']['base']['base']['base']['events'][0]['transferred']+=1
        elif category=='missing_event':bad['final_state']['base']['base']['base']['base']['base']['events'].pop()
        elif category=='missing_record':bad['records'].pop()
        elif category=='missing_completion':next(w for w in steps if w['completed'])['completed'].pop()
        elif category=='elapsed_time':bad['points'][claim['activity']['after']]['state']['tick']+=1
        elif category=='memory_owner':d['retained']['owner']=(d['retained']['owner']+1)%3
        elif category=='memory_counter':next(r for r in bad['memory_records'] if r['kind']=='Taken')['after']['own_takes']+=1
        elif category=='receipt_namespace':d['retained']['receipt']+=1
        elif category in ('unavailable_recovery','unknown_is_not_zero'):
            old=next(r['after'] for r in bad['memory_records'] if r['kind']=='Sense' and r['owner']==0)
            later=next(w['choices'][0] for w in steps if w['choices'][0] and w['choices'][0]['retained'] is None)
            later['retained']=copy.deepcopy(old)
            if category=='unknown_is_not_zero':later['retained']['observation']['stock']=0
        elif category=='memory_missing':bad['memory_records'].pop()
        elif category=='capacity':bad['final_memory'].append(copy.deepcopy(d['retained']))
        elif category=='truth_patch':
            found=False
            for w in steps:
                stock=bad['points'][w['activity']['observed']]['cache']['stock']
                for c in w['choices']:
                    if c and c['retained'] and c['retained']['observation']['stock']-c['retained']['own_takes']>stock:
                        c['retained']['observation']['stock']=stock+c['retained']['own_takes'];found=True;break
                if found:break
            assert found
        try:verify(bad)
        except (AssertionError,IndexError,KeyError,StopIteration):rejected.append(category)
        else:raise AssertionError(('corruption accepted',category))
    return rejected


def main():
    parser=argparse.ArgumentParser();parser.add_argument('--directory',type=Path,default=ROOT/'experiments/021');parser.add_argument('--output',type=Path);parser.add_argument('--expected',type=Path);parser.add_argument('--corruptions',action='store_true');args=parser.parse_args()
    start=time.perf_counter();rows=[];samples={}
    matrix=json.loads((ROOT/'experiments/021/registered-matrix.json').read_bytes())
    for index,t in enumerate(read(args.directory,'trials')):
        assert {k:t[k] for k in ('seed','case','policy','mode','config')}==matrix[index]
        try:r=verify(t);rows.append(r)
        except Exception:
            print('FAIL',index,t['seed'],t['case'],t['policy'],t['mode'],t['config'],flush=True);raise
        if t['seed']==42 and t['policy']=='Need' and t['config']['duration']==4 and t['config']['cadence']==8 and t['config']['order']=='Ascending':
            if t['mode']=='LastSeen' and t['case'] in ('contested','sharing'):samples[t['case']]=t
            if t['mode']=='CurrentOnly' and t['case']=='single_start':samples['current']=t
            if 'stale' not in samples and r['metrics'].get('choice_overestimate',0)>0:samples['stale']=t
    assert len(rows)==len(matrix)==214
    rejected=corruptions(samples) if args.corruptions else []
    result=dict(trials=rows,corruptions=rejected)
    if args.expected:assert result==json.loads(gzip.decompress(args.expected.read_bytes())),'independent021 analysis changed'
    if args.output:
        assert not args.output.exists(),'independent analysis output must be fresh'
        args.output.write_text(json.dumps(result,separators=(',',':'))+'\n',encoding='utf-8',newline='\n')
    print(json.dumps(dict(verified=len(rows),corruptions=rejected,seconds=time.perf_counter()-start)))
if __name__=='__main__':main()
