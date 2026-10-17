"""Complementary read-only 020 review. No evaluator or other reader imports.

Rebuild material operations and intermediate balances directly from original
records; recompute acquisition eligibility/ranking, ties and descriptive contrasts.
Legacy contact/cognitive scoring is deliberately delegated to the separate gate.
"""
import collections
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def base(t):
    node = t['final_state']
    while 'events' not in node:
        node = node['base']
    return node


def verify(t):
    initial = t['initial']; epoch = initial['state']['tick']
    food = [a['food'] for a in initial['state']['agents']]
    stock = initial['cache']['stock']; consumed = 0
    total = sum(food) + stock
    assert total == t['initial_total']
    b = base(t); operations = []; claims = []; selections = 0
    meals = [c for p in t['periods'] for w in p['steps']
             for c in w['activity']['consumption']]
    events = b['events'][initial['state']['events']:]
    for e in events:
        operations.append((e['tick'], 0, e['id'], 'event', e))
    for r in t['records']:
        operations.append((r['completed_at'], 1, r['id'], 'claim', r))
    for n, c in enumerate(meals):
        operations.append((c['tick'], 2, n, 'meal', c))
    operations.sort(key=lambda op: op[:3])
    cursor = 0
    for point in t['points']:
        tick = point['state']['tick']
        while cursor < len(operations) and operations[cursor][0] <= tick:
            _, _, _, kind, op = operations[cursor]; cursor += 1
            if kind == 'event':
                ids = op['participants']
                assert op['balances_before'] == [food[i] for i in ids]
                delta = [v-u for u, v in zip(op['balances_before'], op['balances_after'])]
                assert sum(delta) == 0
                assert sum(max(0, d) for d in delta) == op['transferred']
                assert (op['decision']['selected'] == 'Accept') == bool(op['transferred'])
                for i, value in zip(ids, op['balances_after']):
                    assert 0 <= value <= 2**32-1; food[i] = value
            elif kind == 'claim':
                a = op['attempt']; owner = a['actor']
                assert op['completed_at'] == a['end'] == a['start'] + t['config']['duration']
                assert a['cache'] == initial['cache']['id'] == a['observation']['id']
                assert (op['cache_before'], op['food_before']) == (stock, food[owner])
                expected = 'Depleted' if stock == 0 else 'Overflow' if food[owner] == 2**32-1 else 'Granted'
                assert op['outcome'] == expected
                if expected == 'Granted': stock -= 1; food[owner] += 1
                assert (op['cache_after'], op['food_after']) == (stock, food[owner])
                order = t['config']['order']
                rank = owner if order == 'Ascending' else 2-owner if order == 'Descending' else (owner+3-a['wave']%3)%3
                assert op['rank'] == rank
                claims.append((a['end'], rank))
            else:
                owner = op['actor']; assert op['food_before'] == food[owner] > 0
                assert op['consumed'] == 1; food[owner] -= 1; consumed += 1
                assert op['food_after'] == food[owner]
            assert total == sum(food) + stock + consumed
        assert point['cache']['stock'] == stock
        assert [a['food'] for a in point['state']['agents']] == food
        assert point['state']['events'] == initial['state']['events'] + sum(e['tick'] <= tick for e in events)
        assert point['acquisitions'] == sum(r['completed_at'] <= tick for r in t['records'])
    assert cursor == len(operations) and claims == sorted(claims)
    assert len(meals) == consumed
    # Reconstruct physical hunger over each elapsed tick, independently of needs records.
    hunger = [a['hunger'] for a in initial['state']['agents']]
    area = [0, 0, 0]; by_tick = collections.defaultdict(list)
    for c in meals: by_tick[c['tick']].append(c)
    for tick in range(epoch, t['final_point']['state']['tick']):
        area = [v+h for v, h in zip(area, hunger)]
        if (tick+1-epoch) % 4 == 0: hunger = [min(100, h+5) for h in hunger]
        for c in by_tick[tick+1]:
            owner = c['actor']; assert c['hunger_before'] == hunger[owner]
            hunger[owner] = max(0, hunger[owner]-40)
            assert c['hunger_after'] == hunger[owner]
    assert hunger == [a['hunger'] for a in t['final_point']['state']['agents']]
    for pno, p in enumerate(t['periods']):
        for sno, w in enumerate(p['steps']):
            point = t['points'][w['activity']['observed']]
            for owner, d in enumerate(w['choices']):
                if d is None: continue
                selections += 1
                bd = d['acquiring_base'] if d['selected'] == 'Acquire' else w['activity']['decisions'][owner]
                i = bd['contact']['input']; obs = d['cache']; now = i['now']
                assert set(i) == {'own', 'profile', 'now', 'end', 'people', 'query'}
                assert i['own'] == point['state']['agents'][owner]
                assert obs == dict(id=point['cache']['id'], observed_at=now, stock=point['cache']['stock'], accessible=point['cache']['accessible'], duration=t['config']['duration'])
                can = (next(v['available'] for v in i['people'] if v['id'] == owner)
                       and i['own']['food'] == 0 and i['own']['hunger'] > 0
                       and obs['stock'] > 0 and obs['accessible'] and now+obs['duration'] <= i['end'])
                cs = bd['candidates'] + [dict(action='Acquire', eligible=bool(can), score=i['own']['hunger'] if can else -1)]
                assert cs == d['candidates']
                selected = bd['selected'] if d['policy'] == 'Ignored' else 'Acquire' if d['policy'] == 'Acquire' and can else max(cs, key=lambda c:c['score'])['action']
                assert selected == d['selected']
                if selected == 'Acquire':
                    assert sum(a['selection'] == [pno, sno, owner] for a in w['admissions']) == 1
    exhaustion = None; late_empty = None
    if t['case'] in ('ongoing', 'stranded'):
        positive = [i for i,p in enumerate(t['points']) if any(a['food'] for a in p['state']['agents'])]
        exhaustion = t['points'][max(positive,default=-1)+1]['state']['tick']
        terminals = [e for e in events if e['tick'] > exhaustion and e['outcome'] is not None]
        late_empty = sum(not any(x['transferred'] for x in events if x['scene']==e['scene']) for e in terminals)
    return dict(area=area, selections=selections, meals=[sum(c['actor']==i for c in meals) for i in range(3)],
                attempts=len(claims), grants=sum(r['outcome']=='Granted' for r in t['records']),
                transfers=sum(e['transferred'] for e in events),
                exhaustion=exhaustion, late_empty=late_empty,
                choice_hash=hashlib.sha256(json.dumps([(t['points'][w['activity']['observed']]['state']['tick'],[None if d is None else d['selected'] for d in w['choices']]) for p in t['periods'] for w in p['steps']]).encode()).hexdigest(),
                material_periods=[[(a['food'],a['hunger']) for a in t['points'][p['after']]['state']['agents']] for p in t['periods']],
                material_final=[(a['food'],a['hunger']) for a in b['agents']],
                trust_final=[a['trust'] for a in b['agents']],
                final=t['final_state'], initial=initial, profiles=t['profiles'])


def main():
    groups = collections.defaultdict(dict); totals = collections.Counter(); anchors = {}
    with gzip.open(ROOT/'experiments/020/trials.jsonl.gz', 'rt', encoding='utf-8') as f:
        for ordinal, line in enumerate(f):
            t = json.loads(line)
            try: r = verify(t)
            except Exception:
                print('FAIL original ordinal', ordinal, flush=True); raise
            for k in ('attempts','grants','transfers','selections'): totals[k] += r[k]
            totals['meals'] += sum(r['meals']); totals['worlds'] += 1
            key = (t['seed'], t['case'], json.dumps(t['config'], sort_keys=True))
            # Keep hashes, not full histories, between trials.
            compact = {k:v for k,v in r.items() if k not in ('final','initial','profiles')}
            compact['snapshot'] = hashlib.sha256(json.dumps(r['final'], sort_keys=True).encode()).hexdigest()
            compact['start'] = r['initial']['state']; compact['total'] = t['initial_total']
            groups[key][t['mode']] = compact
            if ordinal in (26,28,29,31,33,40,41,42,43,44):
                anchors[str(ordinal)] = {k:v for k,v in compact.items() if k not in ('snapshot','start','choice_hash','material_periods','material_final','trust_final')}
            if ordinal == 33:
                assert [(x['completed_at'],x['outcome']) for x in t['records'][:2]] == [(4,'Granted'),(4,'Depleted')]
                e = base(t)['events'][5]
                assert (e['tick'],e['scene'],e['decision']['selected'],e['transferred']) == (16,2,'Accept',1)
                c = t['periods'][0]['steps'][13]['activity']['consumption'][0]
                assert (c['tick'],c['actor'],c['hunger_before'],c['hunger_after']) == (17,1,100,60)
    assert totals['worlds'] == 164 and len(groups) == 40
    assert (totals['attempts'],totals['grants'],totals['meals'],totals['transfers']) == (309,242,1095,1045)
    nulls = 0; primary_equal = 0; primary_counts=collections.Counter()
    for (seed, case, conf), modes in groups.items():
        assert len({x['total'] for x in modes.values()}) == 1
        if 'Closed' in modes and 'Ignored' in modes:
            assert modes['Closed']['snapshot'] == modes['Ignored']['snapshot']; nulls += 1
        if seed==42 and json.loads(conf)==dict(duration=4,window=4 if case=='last_window' else 24,order='Descending' if case=='lost_contact' else 'Ascending'):
            x,y=modes['Closed'],modes['Need']
            assert x['start']==y['start']
            primary_counts['pairs']+=1
            primary_counts['choice']+=x['choice_hash']!=y['choice_hash']
            primary_counts['period_material']+=x['material_periods']!=y['material_periods']
            primary_counts['final_material']+=x['material_final']!=y['material_final']
            primary_counts['final_trust']+=x['trust_final']!=y['trust_final']
        if seed == 42 and case != 'last_window' and json.loads(conf) == dict(duration=4,window=24,order='Descending' if case=='lost_contact' else 'Ascending'):
            primary_equal += modes['Need']['snapshot'] == modes['Acquire']['snapshot']
    # Include last_window (its primary window is four).
    primary_equal += groups[(42,'last_window',json.dumps(dict(duration=4,window=4,order='Ascending'),sort_keys=True))]['Need']['snapshot'] == groups[(42,'last_window',json.dumps(dict(duration=4,window=4,order='Ascending'),sort_keys=True))]['Acquire']['snapshot']
    assert primary_equal == 12
    assert dict(primary_counts)==dict(pairs=14,choice=12,period_material=12,final_material=1,final_trust=6)
    for key,value in {'26':([13600,11680,14200],None,None),'29':([14200,10260,13040],None,None),'33':([11680,13520,10200],None,None),'40':([46050,33060,49205],258,65),'41':([49370,39370,50755],165,85),'43':([42915,35110,43525],213,74)}.items():
        a=anchors[key];assert (a['area'],a['exhaustion'],a['late_empty'])==value
    print(json.dumps(dict(totals=dict(totals), groups=len(groups), closed_ignored_full_nulls=nulls, primary_need_acquire_full_nulls=primary_equal, primary_closed_need=dict(primary_counts), anchors=anchors), sort_keys=True))


if __name__ == '__main__': main()
