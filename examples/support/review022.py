"""Read-only 022 primitive reconstruction, independent of the Rust scheduler.

Reuses reviewed Python implementations of unchanged resource/contact/cognitive
arithmetic. Never imports the 022 runner, comparison output, or viewer. Rebuilds
the four-person clock, reservations, local observations and physical operations.
"""
import argparse
import collections
import copy
import gzip
import hashlib
import json
from bisect import bisect_right
from pathlib import Path
import knowledge021
import review015 as old

ROOT = Path(__file__).resolve().parents[2]


def core(t):
    return t['final_state']['base']['base']['base']['base']['base']


def kind(c):
    return 'Resource' if isinstance(c['work'], dict) else c['work']


def rank(owner, tick, order):
    return owner if order == 'Ascending' else 3-owner if order == 'Descending' else (owner+4-tick % 4) % 4


def reconstruct_initial(t):
    """Check generated traits, executed bootstrap cognition and declared interventions."""
    state=t['config']['seed']; mask=2**64-1
    def draw(bound):
        nonlocal state
        state=(state+0x9e3779b97f4a7c15)&mask; z=state
        z=((z^(z>>30))*0xbf58476d1ce4e5b9)&mask
        z=((z^(z>>27))*0x94d049bb133111eb)&mask; z^=z>>31
        return ((z>>32)*bound)>>32
    agents=[dict(id=n,food=draw(7),hunger=draw(101),generosity=draw(101),caution=draw(101),expectation=draw(101),trust={},memories=[]) for n in range(4)]
    case=t['config']['case']; events=core(t)['events'][:t['bootstrap_events']]
    paired=case in ('paired','mixed','durations','same_end','consumption','acquisition','ongoing','exhaustion')
    pairs=[[0,1],[2,3]] if paired else [[0,2]] if case in ('busy_recipient','missed_contact','private_helpful','private_cautious') else [[0,1]] if case.startswith('inquiry_') else []
    assert {e['scene'] for e in events} == set(range(len(pairs)))
    cursor=0
    for sid,(receiver,donor) in enumerate(pairs):
        agents[receiver].update(food=0,hunger=90,generosity=0,caution=0)
        agents[donor].update(food=1 if case.startswith('inquiry_') else 4,hunger=80 if case.startswith('inquiry_') else 0,generosity=0 if case.startswith('inquiry_') else 100,caution=100 if case.startswith('inquiry_') else 0)
        scene=[e for e in events if e['scene']==sid]
        assert scene and all(e['participants']==[receiver,donor] for e in scene)
        assert [e['tick'] for e in scene]==list(range(cursor+1,cursor+len(scene)+1))
        for turn,event in enumerate(scene):
            ids=[receiver,donor]; d=event['decision']; action=d['selected']
            signal=None if not turn else dict(actor=scene[turn-1]['decision']['actor'],action=scene[turn-1]['decision']['selected'])
            assert d['actor']==ids[turn%2]
            assert d['observation']==dict(partner=ids[1-turn%2],last_signal=signal,amount=1,turns_left=6-turn)
            expected_outcome='Agreement' if action=='Accept' else 'Refusal' if action=='Refuse' else 'Withdrawal' if action=='Leave' else 'Timeout' if turn+1==6 else None
            assert event['outcome']==expected_outcome and event['transferred']==int(action=='Accept')
            for owner,memory in zip(ids,event['interpretations'],strict=True):
                assert (memory['event'],memory['partner'],memory['observed'])==(event['id'],next(a for a in ids if a!=owner),action)
        agents=old.resources.resource(scene,agents);cursor=scene[-1]['tick']
    assert all(t['initial'][n]['memories']==a['memories'] and t['initial'][n]['trust']==a['trust'] and t['initial'][n]['expectation']==a['expectation'] for n,a in enumerate(agents))
    provenance=t['final_state']['base']['base']['base']['provenance']
    if case.startswith('inquiry_'):
        assert t['start']==cursor+2
        refusal=events[-1];assert refusal['outcome']=='Refusal'
        assert len(provenance['windows'])==len(provenance['roots'])==1
        window=provenance['windows'][0]
        assert (window['id'],window['event'],window['tick'],window['roots'],window['time_spent'])==(0,refusal['id'],t['start'],[0],2)
        assert len(window['decisions'])==2
        for n,decision in enumerate(window['decisions']):
            i=decision['input'];owner=1 if n==0 else 2
            assert i==dict(own=agents[owner],profile=t['profiles'][owner],event=refusal['id'],effort=0,quality=0 if n==0 else 50,opening=n==0)
            score=100-agents[owner]['hunger']//10
            assert decision['score']==score and decision['selected']==(score>0)
        assert provenance['roots']==[dict(id=0,event=refusal['id'],observer=2,claim=True,quality=50,tick=t['start'],sensor=dict(reliability=50,effort=0,channel='AccurateFixture',second=None))]
        initial_receipts=[r for r in provenance['receipts'] if r['tick']<=t['start']]
        assert len(initial_receipts)==1
        receipt=initial_receipts[0]
        assert (receipt['id'],receipt['tick'],receipt['communicator'],receipt['listener'],receipt['actual_root'],receipt['parent'],receipt['depth'])==(0,t['start'],2,2,0,None,0)
    else:
        assert t['start']==cursor and not provenance['windows'] and not provenance['roots']
    expected=[dict(a,food=2,hunger=20,generosity=60,caution=20) for a in agents];stock=4
    def set_actor(n,**fields): expected[n].update(fields)
    if case=='mixed':set_actor(3,food=0);set_actor(2,hunger=100)
    elif case=='durations':set_actor(2,food=0,hunger=80)
    elif case=='consumption':set_actor(2,hunger=80,generosity=0,caution=100)
    elif case=='acquisition':
        for n in (2,3):set_actor(n,food=0,generosity=0,caution=100)
        stock=1
    elif case=='scarcity':
        for n in range(4):set_actor(n,food=0,hunger=80,generosity=0,caution=100)
        stock=1
    elif case in ('busy_recipient','missed_contact','private_helpful','private_cautious'):
        values=[(0,80,0,100),(1,20,100,0),(2,20,60,20),(0,0,0,100)]
        for n,(food,hunger,generosity,caution) in enumerate(values):set_actor(n,food=food,hunger=hunger,generosity=generosity,caution=caution)
        if case=='private_cautious':set_actor(1,generosity=0,caution=100)
        stock=1
    elif case.startswith('inquiry_') or case=='null':
        for n in range(4):set_actor(n,food=0,hunger=0,generosity=0,caution=100)
        stock=0
        if case=='inquiry_conflict':
            for n in (1,3):set_actor(n,food=2,hunger=20,generosity=60,caution=20)
    elif case=='exhaustion':
        for n in range(4):set_actor(n,food=1)
        stock=1
    else:assert case in ('paired','same_end','contenders','order_bias','ongoing')
    assert t['initial']==expected and t['initial_stock']==stock
    assert t['profiles']==[dict(privacy=0,relationship_goal=100,honesty=80,request_cost=40)]*4


def reconstruct_concerns(t, resources, finishes):
    """Derive the concern ledger and each prefix's legal time from actual work."""
    b = core(t); events = b['events']; compact = t['final_state']['base']['base']['base']
    records = compact['base']['inquiry']['records']; live = collections.defaultdict(list)
    expected = []; times = []; creations = []; next_id = 0
    actions = []
    for e in events[:t['bootstrap_events']]:
        if e['outcome'] == 'Refusal': actions.append((e['tick'], 0, e['id'], 'create', e))
    for c in resources:
        actions.append((finishes[c['id']], 0, rank(c['participants'][0], c['start'], t['config']['order']), 'scene', c))
    for i in b['cognition']['information']:
        actions.append((i['tick'], 1, i['id'], 'belief', i))
    for q in records: actions.append((q['tick'], 2, q['id'], 'query', q))

    def change(tick, reason, before, after, receipt=None):
        expected.append(dict(id=len(expected), reason=reason, before=copy.deepcopy(before), after=copy.deepcopy(after), receipt=receipt, follow_up=None)); times.append(tick)

    def create(tick, event):
        nonlocal next_id
        target = event['decision']['actor']; owner = next(a for a in event['participants'] if a != target)
        assert t['profiles'][owner]['relationship_goal'] == 100
        creations.append(dict(owner=owner, target=target, event=event['id'], scene=event['scene'], importance=100, support=None, receipt=None, retained=True))
        items = live[owner]
        if len(items) == 8:
            removed = min(items, key=lambda c:(c['status'] in ('Open','Partial'), c['importance'] if c['status'] in ('Open','Partial') else 0, c['id']))
            items.remove(removed)
            if removed['status'] in ('Open','Partial'):
                abandoned = dict(removed, status='Abandoned'); change(tick, 'capacity abandonment', removed, abandoned); removed = abandoned
            change(tick, 'capacity eviction', removed, None)
        concern = dict(id=next_id, owner=owner, target=target, event=event['id'], created_scene=event['scene'], question='RefusalScarcity', importance=100, uncertainty=100, age=0, attempts=0, failures=0, status='Open', last_receipt=None)
        next_id += 1; items.append(concern); change(tick, 'consequential unresolved refusal', None, concern)

    for tick, _, _, k, op in sorted(actions, key=lambda a:a[:3]):
        if k == 'create': create(tick, op)
        elif k == 'scene':
            for owner in op['participants']:
                for at, before in enumerate(live[owner]):
                    if before['status'] in ('Open','Partial'):
                        after = dict(before, age=min(2**32-1, before['age']+1)); live[owner][at] = after
                        change(tick, '022 actual resource encounter', before, after)
            for e in events:
                if e['scene'] == op['work']['Resource']['scene'] and e['outcome'] == 'Refusal': create(tick, e)
        elif k == 'belief':
            owner = op['listener']; at = next((n for n,c in enumerate(live[owner]) if c['event'] == op['event']), None)
            if at is not None:
                before = live[owner][at]; support = op['after']['support']
                after = dict(before, uncertainty=100-abs(support), last_receipt=op['id'], status='Abandoned' if before['status']=='Abandoned' else 'Resolved' if abs(support)>=60 else 'Partial')
                live[owner][at] = after; change(tick, 'local belief update', before, after, op['id'])
        else:
            owner = op['decision']['input']['owner']; cid = op['decision']['selected']['Ask']['concern']
            at = next(n for n,c in enumerate(live[owner]) if c['id'] == cid); before = live[owner][at]
            after = dict(before, attempts=min(2**32-1,before['attempts']+1), failures=min(2**32-1,before['failures']+int(op['realized_value']==0)))
            assert op['concern_after'] == after
            q = compact['provenance']['queries'][op['id']]; receipt = None
            if q['receipt'] is not None: receipt = compact['provenance']['receipts'][q['receipt']]['cognition_receipt']
            elif op['response'] and op['response']['receipts']: receipt = op['response']['receipts'][-1]
            live[owner][at] = after; change(tick, 'provenance inquiry outcome', before, after, receipt)
    assert expected == b['concerns']['transitions']
    assert creations == b['concerns']['creations']
    assert {str(k):v for k,v in live.items() if v} == {k:v for k,v in b['concerns']['items'].items() if v}
    return times


def verify(t):
    assert t['schema'] == 'experiment-022-trial/v1'
    cfg = t['config']; start, end = t['start'], t['end']
    assert end-start == cfg['horizon'] and cfg['capacity'] in (1, 2)
    snap = t['final_state']; compact = snap['base']['base']['base']; base = core(t)
    events = base['events']; scenes = base['scenes']; queries = compact['base']['inquiry']['records']
    info = base['cognition']['information']; changes = base['concerns']['transitions']
    meals = base['cognition']['consumption']; initial = copy.deepcopy(t['initial'])
    assert [a['id'] for a in initial] == list(range(4))
    reconstruct_initial(t)
    assert initial == t['boundaries'][0]['agents']
    # Independent unchanged 006–012 scoring, retained assessment and learning.
    old.cognitive.reconstruct(dict(final_state=snap, mode='Contextual', case=cfg['case'], config=cfg, opportunities=[]))
    prefixes = old.assessment.bases(t)
    _, assessment_times = old.cognitive.assessment_prefixes(compact, snap['base']['base']['assessment'])
    assert len(events) == len({e['id'] for e in events})
    for n, e in enumerate(events):
        assert e['id'] == n and e['participants'] == scenes[e['scene']]['participants']
        assert all(0 <= a < 4 for a in e['participants'])
    transitions = t['transitions']; starts = {}; finishes = {}; live = {}; next_id = 0
    last_tick = start
    for tr in transitions:
        c = tr['commitment']; cid = c['id']; tick = tr['tick']; k = kind(c)
        assert start <= tick <= end and tick >= last_tick; last_tick = tick
        assert len(c['participants']) == (2 if k in ('Resource', 'Query') else 1)
        assert len(set(c['participants'])) == len(c['participants']) and all(0 <= a < 4 for a in c['participants'])
        assert 0 <= c['source'] < len(t['boundaries'])
        if not tr['completed']:
            assert cid == next_id and tick == c['start']; next_id += 1
            assert not set(c['participants']).intersection(a for v in live.values() for a in v['participants']), 'double booking'
            if k == 'Query': assert not live, 'atomic query overlaps work'
            assert all(kind(v) != 'Query' for v in live.values())
            assert len(live) < 4
            if k == 'Resource':
                assert sum(kind(v) == 'Resource' for v in live.values()) < cfg['capacity']
                assert c['deadline'] == tick+6
            elif k == 'Claim': assert c['deadline'] == tick+cfg['duration']
            elif k != 'Query': assert c['deadline'] == tick+1
            starts[cid] = copy.deepcopy(c); live[cid] = c
        else:
            assert cid in live and live.pop(cid) == c and cid not in finishes, 'wrong/missing release'
            assert c['start'] < tick <= c['deadline']
            if k not in ('Resource', 'Query'): assert tick == c['deadline']
            finishes[cid] = tick
    assert not live and starts.keys() == finishes.keys()
    resources = [c for c in starts.values() if kind(c) == 'Resource']
    qwork = [c for c in starts.values() if kind(c) == 'Query']
    by_scene = {c['work']['Resource']['scene']: c for c in resources}
    assert len(by_scene) == len(resources)
    actual_events = events[t['bootstrap_events']:]
    assert len(scenes) == len({e['scene'] for e in events})
    for c in resources:
        sid = c['work']['Resource']['scene']; es = [e for e in actual_events if e['scene'] == sid]
        assert [e['tick'] for e in es] == list(range(c['start']+1, finishes[c['id']]+1)), 'skipped scene progression'
        assert es and es[-1]['outcome'] == scenes[sid]['outcome'] and es[-1]['outcome'] is not None
        assert scenes[sid]['participants'] == c['participants'] and scenes[sid]['turns'] == len(es)
    assert all(e['scene'] in by_scene for e in actual_events)
    for c in qwork:
        qs = [q for q in queries if q['tick'] == finishes[c['id']] and q['tick']-q['time_spent'] == c['start']]
        assert len(qs) == 1
        a = qs[0]['decision']['selected']['Ask']; assert c['participants'] == [qs[0]['decision']['input']['owner'], a['source']]
    assert len(qwork) == len(queries)
    concern_times = reconstruct_concerns(t, resources, finishes)
    for tick, recorded in ((start,t['initial_contexts']), (end,t['final_contexts'])):
        cs = old.concern_prefix(changes, bisect_right(concern_times,tick))
        expected = dict(completed=[],created=[])
        for owner in range(4):
            expected['completed'].append(sum(owner in s['participants'] and any(e['scene']==s['id'] and e['outcome'] is not None and e['tick']<=tick for e in events) for s in scenes))
            expected['created'].append({str(c['id']):sum(s['id']<c['created_scene'] and owner in s['participants'] for s in scenes) for c in cs.get(owner,[])})
        assert recorded == expected
    # Every missing ordinary decision tick must lie inside an actual atomic query.
    ticks = [b['tick'] for b in t['boundaries']]
    assert ticks == sorted(set(ticks)) and ticks[0] == start
    assert set(range(start, end))-set(ticks) == {tick for c in qwork for tick in range(c['start'], finishes[c['id']])}
    for c in qwork:
        assert all(not (other['start'] < finishes[c['id']] and finishes[other['id']] > c['start'])
                   for other in starts.values() if other['id'] != c['id']), 'query leap over deadline'

    def at(tick, phase='choice'):
        return [c for c in starts.values() if c['start'] <= tick < finishes[c['id']]
                and (c['start'] < tick or kind(c) in (('Reply', 'Resource') if phase == 'choice' else ()))]

    def people(tick, phase='choice'):
        busy = {a for c in at(tick, phase) for a in c['participants']}
        return [dict(id=a, available=a not in busy) for a in range(4)]

    # Need, core events, claims, meals and legitimate information deliveries only.
    agents = copy.deepcopy(initial); stock = t['initial_stock']; consumed = 0
    assert t['initial_total'] == sum(a['food'] for a in agents)+stock
    needs = {n['tick']: n for n in t['needs']}
    assert list(needs) == list(range(start+4, end+1, 4))
    grants = t['grants']; claim_work = [c for c in starts.values() if kind(c) == 'Claim']
    assert len(grants) == len(claim_work) and len(meals) == sum(kind(c) == 'Meal' for c in starts.values())
    for g, c in zip(grants, sorted(claim_work, key=lambda c:(finishes[c['id']], rank(c['participants'][0], c['start'], cfg['order']), c['id'])), strict=True):
        assert (g['commitment'], g['owner'], g['tick']) == (c['id'], c['participants'][0], finishes[c['id']])
    for meal in meals:
        assert sum(kind(c) == 'Meal' and c['participants'] == [meal['actor']] and finishes[c['id']] == meal['tick'] for c in starts.values()) == 1
    ops = collections.defaultdict(list)
    for n in t['needs']: ops[n['tick']].append((0, 0, 'need', n))
    for e in actual_events: ops[e['tick']].append((1, e['scene'], 'event', e))
    for n, g in enumerate(grants): ops[g['tick']].append((2, n, 'grant', g))
    for n, m in enumerate(meals): ops[m['tick']].append((3, n, 'meal', m))
    for i in info:
        if i['tick'] > start: ops[i['tick']].append((4, i['id'], 'info', i))
    states = {start: copy.deepcopy(agents)}; stocks = {start: stock}; areas = [0]*4
    for tick in range(start+1, end+1):
        for n, a in enumerate(agents): areas[n] += a['hunger']
        for _, _, k, op in sorted(ops[tick]):
            if k == 'need':
                assert op['before'] == [a['hunger'] for a in agents]
                assert op['after'] == [min(100, h+5) for h in op['before']]
                for a, h in zip(agents, op['after'], strict=True): a['hunger'] = h
            elif k == 'event':
                c = by_scene[op['scene']]; prior = [e for e in events[:op['id']] if e['scene'] == op['scene']]
                turn = len(prior); ids = c['participants']; d = op['decision']
                signal = None if not prior else dict(actor=prior[-1]['decision']['actor'], action=prior[-1]['decision']['selected'])
                assert d['actor'] == ids[turn % 2]
                assert d['observation'] == dict(partner=ids[1-turn % 2], last_signal=signal, amount=1, turns_left=6-turn)
                assert (d['generosity'], d['caution'], d['expectation']) == tuple(agents[d['actor']][f] for f in ('generosity', 'caution', 'expectation'))
                assert d['memory_causes'] == [m['event'] for m in [m for m in agents[d['actor']]['memories'] if m['partner'] == d['observation']['partner']][-4:]][::-1]
                action = d['selected']; outcome = 'Agreement' if action == 'Accept' else 'Refusal' if action == 'Refuse' else 'Withdrawal' if action == 'Leave' else 'Timeout' if turn+1 == 6 else None
                assert op['outcome'] == outcome and op['transferred'] == int(action == 'Accept')
                for owner, memory in zip(ids, op['interpretations'], strict=True):
                    assert (memory['event'], memory['partner'], memory['observed']) == (op['id'], next(a for a in ids if a != owner), action)
                agents = old.resources.resource([op], agents)
            elif k == 'grant':
                owner = op['owner']; assert (op['stock_before'], op['food_before']) == (stock, agents[owner]['food'])
                expected = 'Depleted' if stock == 0 else 'Overflow' if agents[owner]['food'] == 2**32-1 else 'Granted'
                assert op['outcome'] == expected
                if expected == 'Granted': stock -= 1; agents[owner]['food'] += 1
                assert (op['stock_after'], op['food_after']) == (stock, agents[owner]['food'])
            elif k == 'meal':
                a = agents[op['actor']]; assert op['food_before'] == a['food'] > 0 and op['hunger_before'] == a['hunger'] > 0
                a['food'] -= 1; a['hunger'] = max(0, a['hunger']-40); consumed += 1
                assert (op['food_after'], op['hunger_after'], op['consumed']) == (a['food'], a['hunger'], 1)
            else:
                agents = old.query_agents(agents, events, [op], tick-1, tick)
            assert sum(a['food'] for a in agents)+stock+consumed == t['initial_total']
        states[tick] = copy.deepcopy(agents); stocks[tick] = stock
    assert agents == t['final_agents'] == base['agents'] and stock == t['final_stock']

    memory = knowledge021.Memory('LastSeen', start, cfg['cadence'])
    memory.slots = [None]*4; memory.receipts = [0]*4
    mem_cursor = 0
    def apply_memory(record):
        nonlocal mem_cursor
        owner, tick, k = record['owner'], record['tick'], record['kind']
        assert record['id'] == mem_cursor and 0 <= owner < 4
        if k == 'Sense':
            assert (tick-start) % cfg['cadence'] == 0 and people(tick)[owner]['available']
            memory.receive(owner, dict(id=0, observed_at=tick, stock=stocks[tick], accessible=True, duration=cfg['duration']), 'Sense')
        else:
            assert record['physical'] is not None
            g = grants[record['physical']]; assert (owner, tick) == (g['owner'], g['tick'])
            memory.result(owner, 0, tick, g['outcome'], record['physical'], cfg['duration'])
        assert memory.records[-1] == record
        mem_cursor += 1

    # Observer prefix captures are causal references, excluded from actor inputs.
    def context(i, b, public, now):
        owner = i['own']['id']; assert set(i) == {'own', 'profile', 'now', 'end', 'people', 'query'}
        assert i['own'] == states[now][owner] and i['profile'] == t['profiles'][owner]
        assert i['now'] == now and i['end'] == end and i['people'] == public
        query = i['query']; own = query['input']['base']
        assert set(query['input']) == {'base', 'knowledge', 'hints', 'settings'}
        assert set(own) == {'owner','scene','hunger','relationship_goal','concerns','sources','cells','seen','settings'}
        assert own['owner'] == owner and own['hunger'] == i['own']['hunger'] and own['relationship_goal'] == i['profile']['relationship_goal']
        completed_scenes = [s for s in scenes if s['participants'].count(owner) and any(e['scene'] == s['id'] and e['outcome'] is not None and e['tick'] <= now for e in events)]
        assert own['scene'] == len(completed_scenes), 'foreign scene count'
        cs = old.concern_prefix(changes, b['concerns']).get(owner, [])
        cs = copy.deepcopy(cs)
        for c in cs: c['created_scene'] = sum(s['id'] < c['created_scene'] and owner in s['participants'] for s in scenes)
        assert own['concerns'] == cs
        source_ids = [p['id'] for p in public if p['available'] and p['id'] != owner and not any(c['target'] == p['id'] for c in cs)]
        assert [s['id'] for s in own['sources']] == source_ids and all(not s['offers'] for s in own['sources'])
        assert query['input']['knowledge'] == old.cognitive.provenance_prefix(compact, now).get(owner, [])
        assert all(len(query['input'][name]) <= 32 for name in ('knowledge','hints'))
        assert len(own['cells']) <= 32 and len(own['seen']) <= 32 and len(cs) <= 8
        for source in own['sources']:
            credibility = 0
            for q in queries:
                if q['tick'] > now: break
                response = q['response']
                if response and q['decision']['input']['owner'] == owner and q['decision']['selected']['Ask']['source'] == source['id']:
                    credibility = response['credibility_after']
            assert source['credibility'] == credibility
        assert query['decision']['input'] == own
        items = prefixes[b['assessment']].get(str(owner), [])
        basis = [dict(concern=c['id'], event=c['event'], support=old.assessment.grouped(items, c['event']), context='EvidencePresent' if any(x['event'] == c['event'] and x['quality'] > 0 and 'Claim' not in x['origin'] for x in items) else 'ClaimFallback') for c in cs if c['status'] in ('Open', 'Partial')]
        cscores, selected, _ = old.assessment.scores(own, {k:query['input'][k] for k in ('knowledge', 'hints', 'settings')}, basis, old.cells_prefix(snap['learning']['records'], owner, b['queries']), 'Contextual')
        assert (query['decision']['candidates'], query['decision']['selected']) == (cscores, selected)
        for signature in own['seen']:
            receipt = info[signature['receipt']]; assert receipt['listener'] == owner and receipt['tick'] <= now
        for cell in own['cells']:
            r = queries[cell['last_record']]; assert r['tick'] <= now and r['decision']['input']['owner'] == owner and r['cell_after'] == cell

    proposals = []; responses = {}; response_refs = {}; choices = {}
    for n, b in enumerate(t['boundaries']):
        tick = b['tick']; assert b['agents'] == states[tick] and b['stock'] == stocks[tick]
        assert b['events'] == sum(e['tick'] <= tick for e in events)
        assert b['assessment'] == bisect_right(assessment_times, tick) and b['queries'] == sum(q['tick'] <= tick for q in queries)
        assert b['concerns'] == bisect_right(concern_times, tick)
        assert b['active'] == sorted(at(tick), key=lambda c:c['id'])
        public = people(tick)
        expected_owners = [p['id'] for p in public if p['available']] if tick < end else []
        assert [d['input']['legacy']['own']['id'] for d in b['choices']] == expected_owners
        incoming_owners = sorted({p['invitation']['recipient'] for p in proposals if p['invitation']['expires'] == tick+1 and people(tick,'response')[p['invitation']['recipient']]['available']})
        assert [r['input']['own']['own']['id'] for r in b['responses']] == incoming_owners, 'missing independent recipient opportunity'
        for response_index, response in enumerate(b['responses']):
            ri = response['input']; owner = ri['own']['own']['id']; context(ri['own'], b, people(tick, 'response'), tick)
            incoming = [p['invitation'] for p in proposals if p['invitation']['recipient'] == owner and p['invitation']['expires'] == tick+1]
            assert ri['incoming'] == incoming and incoming and people(tick, 'response')[owner]['available']
            cs, _ = old.response(ri)
            for c in cs:
                if c['reply'] == 'Continue': c['score'] = 0
            assert (response['candidates'], response['selected']) == (cs, max(cs, key=lambda c:c['score'])['reply'])
            responses[tick, owner] = response
            response_refs[tick, owner] = dict(boundary=n,response=response_index,selected=response['selected'])
        # Claims completed before this boundary update owned memory before sensing.
        while mem_cursor < len(t['memories']) and t['memories'][mem_cursor]['tick'] < tick:
            apply_memory(t['memories'][mem_cursor])
        for d in b['choices']:
            owner = d['input']['legacy']['own']['id']; context(d['input']['legacy'], b, public, tick)
            while mem_cursor < len(t['memories']) and t['memories'][mem_cursor]['tick'] == tick and (t['memories'][mem_cursor]['kind'] != 'Sense' or t['memories'][mem_cursor]['owner'] <= owner):
                apply_memory(t['memories'][mem_cursor])
            assert d['input']['retained'] == memory.slots[owner]
            expected = knowledge021.choose(d['input'], 'Need')
            pending_own = any(p['invitation']['proposer'] == owner and p['invitation']['expires'] > tick for p in proposals)
            if pending_own:
                expected['candidates'][2].update(eligible=False, score=-1)
                expected['selected'] = max(expected['candidates'], key=lambda c:c['score'])['action']
            assert d['candidates'] == expected['candidates'] and d['selected'] == expected['selected']
            assert (d['contact_candidates'], d['contact']) == (expected['base']['contact']['candidates'], expected['base']['contact']['selected'])
            assert d['input']['duration'] == cfg['duration'] and d['input']['inquiry_lane_free'] == (not b['active'] and not b['pending'])
            choices[n, owner] = d
            work = {'Invite':'Send', 'Acquire':'Claim', 'Consume':'Meal'}.get(d['selected'])
            if work:
                assert sum(c['source'] == n and c['participants'] == [owner] and kind(c) == work and c['start'] == tick for c in starts.values()) == 1
            if d['selected'] == 'Invite':
                inv = dict(proposer=owner, **d['contact']['Invite'], expires=tick+2)
                proposals.append(dict(source=n, invitation=inv, question=d['input']['legacy']['query']['decision']['selected']))
        # Pending audit contains only still-live own protocol tokens; no resurrected intent.
        expected_pending = []
        for p in proposals:
            inv = p['invitation']
            if inv['expires'] > tick and t['boundaries'][p['source']]['tick'] < tick:
                expected_pending.append(dict(p, response=response_refs.get((inv['expires']-1, inv['recipient']))))
        assert b['pending'] == expected_pending
    while mem_cursor < len(t['memories']): apply_memory(t['memories'][mem_cursor])
    assert t['final_memory'] == memory.slots
    assert len(proposals) == len(t['admissions'])
    # Admission reasons are independently derived from actual proposals/consent and capacity.
    expected_admissions = []; executed = collections.defaultdict(list)
    for p in sorted(proposals, key=lambda p:(p['invitation']['expires'], rank(p['invitation']['proposer'], p['invitation']['expires'], cfg['order']))):
        inv = p['invitation']; tick = inv['expires']; actors = [inv['proposer'], inv['recipient']]
        admitted_here = executed[tick]
        pre = [c for c in starts.values() if c['start'] < tick < finishes[c['id']] or c['start'] == tick and kind(c) == 'Reply']
        busy = {a for c in pre for a in c['participants']}
        reply = responses.get((tick-1, inv['recipient']))
        if busy.intersection(actors): reason = 'Unavailable'
        elif reply is None or reply['selected'] != dict(Accept=inv['proposer']): reason = 'Declined'
        elif any(x['proposer'] == inv['recipient'] and x['recipient'] == inv['proposer'] and x['kind'] == inv['kind'] for x in admitted_here): reason = 'Reciprocal'
        elif any(set([x['proposer'], x['recipient']]).intersection(actors) for x in admitted_here): reason = 'Contention'
        elif inv['kind'] == 'Resource' and sum(kind(c) == 'Resource' for c in pre)+sum(x['kind'] == 'Resource' for x in admitted_here) >= cfg['capacity']: reason = 'Capacity'
        elif inv['kind'] == 'Inquiry' and (pre or admitted_here or any(other['invitation']['expires'] > tick or other['invitation']['expires'] == tick and rank(other['invitation']['proposer'], tick, cfg['order']) > rank(inv['proposer'], tick, cfg['order']) for other in proposals if t['boundaries'][other['source']]['tick'] < tick)): reason = 'AtomicBlocked'
        else: reason = 'Execute'
        found = [c for c in starts.values() if kind(c) in ('Resource', 'Query') and c['start'] == tick and c['source'] == p['source'] and c['participants'] == actors]
        if reason == 'Execute':
            assert len(found) == 1; admitted_here.append(inv); cid = found[0]['id']
            if inv['kind'] == 'Inquiry':
                q = next(q for q in queries if q['tick'] == finishes[cid]); assert q['decision']['selected'] == p['question']
        else: assert not found; cid = None
        expected_admissions.append(dict(tick=tick, source=p['source'], invitation=inv, reason=reason, commitment=cid))
    assert t['admissions'] == expected_admissions
    # All starts require an actual local cause; no hidden extra action.
    for c in starts.values():
        if kind(c) in ('Resource', 'Query'): assert any(a['commitment'] == c['id'] for a in expected_admissions)
        elif kind(c) == 'Reply': assert (c['start'], c['participants'][0]) in responses
        else: assert (c['source'], c['participants'][0]) in choices
    intervals = [(c['start'], finishes[c['id']], c) for c in resources]
    peak_social = max((sum(a <= tick < b for a,b,_ in intervals) for tick in range(start,end)), default=0)
    metrics = dict(social=len(resources), peak_social=peak_social, queries=len(qwork), transfers=sum(e['transferred'] for e in actual_events), zero_transfer=sum(not any(e['transferred'] for e in actual_events if e['scene'] == sid) for sid in by_scene), meals=len(meals), grants=sum(g['outcome']=='Granted' for g in grants), failed_claims=sum(g['outcome']=='Depleted' for g in grants), hunger_area=areas, committed_ticks=[sum(finishes[c['id']]-c['start'] for c in starts.values() if owner in c['participants']) for owner in range(4)], reasons=dict(collections.Counter(a['reason'] for a in expected_admissions)))
    return metrics


def read(path):
    opener = gzip.open if str(path).endswith('.gz') else open
    with opener(path, 'rt', encoding='utf-8') as stream:
        for line in stream: yield json.loads(line)


def main():
    parser = argparse.ArgumentParser(); parser.add_argument('archive'); parser.add_argument('--output'); args = parser.parse_args()
    rows = []
    for ordinal, t in enumerate(read(args.archive)):
        try: metrics = verify(t)
        except Exception:
            print('FAILED trial', ordinal, t['config'], flush=True); raise
        rows.append(dict(ordinal=ordinal, config=t['config'], metrics=metrics))
    result = dict(worlds=len(rows), rows=rows)
    if args.output: Path(args.output).write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps(dict(verified=len(rows), social=sum(r['metrics']['social'] for r in rows), queries=sum(r['metrics']['queries'] for r in rows))))


if __name__ == '__main__': main()
