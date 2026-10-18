"""Complementary scientific review of original 021. Standard library only.

No evaluator, prepared analysis, earlier reader or simulation scorer is imported.
Reconstruct physical operations, hunger area, owned environmental transitions,
acquisition eligibility/ties and matched descriptive contrasts. The original
full reader separately reconstructs legacy cognition and inquiry/consent.
"""
import collections
import copy
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True).encode()).hexdigest()


def base(t):
    b = t['final_state']
    while 'events' not in b:
        b = b['base']
    return b


def verify(t):
    b = base(t)
    initial = t['initial']; epoch = initial['state']['tick']
    food = [a['food'] for a in initial['state']['agents']]
    stock = initial['cache']['stock']; consumed = 0
    total = sum(food) + stock
    assert total == t['initial_total']
    events = b['events'][initial['state']['events']:]
    meals = [c for p in t['periods'] for w in p['steps'] for c in w['activity']['consumption']]
    ops = sorted([(e['tick'], 0, e['id'], 'event', e) for e in events]
                 + [(r['completed_at'], 1, r['id'], 'claim', r) for r in t['records']]
                 + [(c['tick'], 2, n, 'meal', c) for n, c in enumerate(meals)])
    cursor = 0; claims = []
    for point in t['points']:
        tick = point['state']['tick']
        while cursor < len(ops) and ops[cursor][0] <= tick:
            _, _, _, kind, op = ops[cursor]; cursor += 1
            if kind == 'event':
                ids = op['participants']
                assert op['balances_before'] == [food[i] for i in ids]
                delta = [v-u for u, v in zip(op['balances_before'], op['balances_after'])]
                assert sum(delta) == 0 and sum(max(0, d) for d in delta) == op['transferred']
                assert (op['decision']['selected'] == 'Accept') == bool(op['transferred'])
                for i, value in zip(ids, op['balances_after']):
                    assert 0 <= value < 2**32; food[i] = value
            elif kind == 'claim':
                a = op['attempt']; owner = a['actor']
                assert a['start'] + t['config']['duration'] == a['end'] == op['completed_at']
                assert (op['cache_before'], op['food_before']) == (stock, food[owner])
                expected = 'Depleted' if not stock else 'Overflow' if food[owner] == 2**32-1 else 'Granted'
                assert op['outcome'] == expected
                if expected == 'Granted': stock -= 1; food[owner] += 1
                assert (op['cache_after'], op['food_after']) == (stock, food[owner])
                order = t['config']['order']
                rank = owner if order == 'Ascending' else 2-owner if order == 'Descending' else (owner+3-a['wave']%3)%3
                assert op['rank'] == rank
                claims.append((a['end'], rank))
            else:
                owner = op['actor']; assert op['food_before'] == food[owner] > 0
                food[owner] -= 1; consumed += 1
                assert op['food_after'] == food[owner] and op['consumed'] == 1
            assert sum(food) + stock + consumed == total
        assert point['cache']['stock'] == stock
        assert [a['food'] for a in point['state']['agents']] == food
    assert cursor == len(ops) and claims == sorted(claims)
    hunger = [a['hunger'] for a in initial['state']['agents']]; area = [0]*3
    by_tick = collections.defaultdict(list)
    for c in meals: by_tick[c['tick']].append(c)
    for tick in range(epoch, t['final_point']['state']['tick']):
        area = [v+h for v, h in zip(area, hunger)]
        if (tick+1-epoch) % 4 == 0: hunger = [min(100, h+5) for h in hunger]
        for c in by_tick[tick+1]:
            owner = c['actor']; assert c['hunger_before'] == hunger[owner]
            hunger[owner] = max(0, hunger[owner]-40)
            assert c['hunger_after'] == hunger[owner]
    assert hunger == [a['hunger'] for a in b['agents']]

    # Validate each transition against its real sensor/physical source, retaining
    # only the owner's slot. Never copy a quantity back out of an old attempt.
    slots = [None]*3; receipts = [0]*3; states = [copy.deepcopy(slots)]
    for n, r in enumerate(t['memory_records']):
        owner = r['owner']; now = r['tick']; kind = r['kind']; prior = slots[owner]
        assert r['id'] == n and r['before'] == prior
        after = copy.deepcopy(prior)
        if kind in ('Sense', 'Depleted'):
            if kind == 'Sense':
                assert t['mode'] != 'Unobserved'
                assert t['mode'] == 'FreshReference' or (now-epoch) % t['config']['cadence'] == 0
                p = t['points'][r['point']]
                assert p['state']['tick'] == now and r['physical'] is None
                obs = dict(id=p['cache']['id'], observed_at=now, stock=p['cache']['stock'],
                           accessible=p['cache']['accessible'], duration=t['config']['duration'])
            else:
                physical = t['records'][r['physical']]
                assert physical['outcome'] == 'Depleted' and physical['attempt']['actor'] == owner
                assert physical['completed_at'] == now and r['point'] is None
                obs = dict(id=physical['attempt']['cache'], observed_at=now, stock=0,
                           accessible=True, duration=t['config']['duration'])
            after = dict(owner=owner, receipt=receipts[owner], observation=obs, own_takes=0)
            receipts[owner] += 1
        elif kind == 'Taken':
            physical = t['records'][r['physical']]
            assert prior is not None and physical['outcome'] == 'Granted'
            assert (physical['attempt']['actor'], physical['completed_at']) == (owner, now)
            assert t['mode'] != 'Unobserved'
            assert t['mode'] != 'Expire12' or now-prior['observation']['observed_at'] < 12
            after['own_takes'] += 1
        elif kind == 'Expired':
            assert prior is not None
            age = now-prior['observation']['observed_at']
            assert (t['mode'] == 'Expire12' and age >= 12) or (t['mode'] == 'CurrentOnly' and age != 0)
            after = None
        else:
            assert kind == 'Discarded' and t['mode'] == 'CurrentOnly' and prior is not None
            after = None
        assert r['after'] == after
        slots[owner] = after; states.append(copy.deepcopy(slots))
    assert slots == t['final_memory']
    selections = []; local_choices = []; choice_count = 0; overestimates = 0
    for pno, p in enumerate(t['periods']):
        for sno, w in enumerate(p['steps']):
            s = w['activity']; point = t['points'][s['observed']]; now = point['state']['tick']
            prefix = w['memory_before']
            local_wave = []
            for owner, d in enumerate(w['choices']):
                if d is None:
                    local_wave.append(None)
                    continue
                # The selection boundary is after this owner's expiry/sense but
                # before CurrentOnly discard. Other owners' same-wave receipts
                # never enter this projection.
                while prefix < w['memory_after']:
                    r = t['memory_records'][prefix]
                    if r['owner'] != owner or r['tick'] != now or r['kind'] not in ('Expired','Sense'): break
                    prefix += 1
                cell = states[prefix][owner]
                assert cell == d['retained'], 'decision must use its actual available owned slot'
                sensed = t['mode'] == 'FreshReference' or (t['mode'] != 'Unobserved' and (now-epoch) % t['config']['cadence'] == 0)
                if sensed:
                    assert prefix and t['memory_records'][prefix-1]['kind'] == 'Sense'
                    assert t['memory_records'][prefix-1]['owner'] == owner
                if t['mode'] == 'CurrentOnly' and cell is not None:
                    assert t['memory_records'][prefix]['kind'] == 'Discarded'
                    assert t['memory_records'][prefix]['owner'] == owner
                    prefix += 1
                bd = d['acquiring_base'] if d['selected'] == 'Acquire' else s['decisions'][owner]
                i = bd['contact']['input']; own = i['own']
                assert set(i) == {'own','profile','now','end','people','query'}
                assert own == point['state']['agents'][owner] and i['profile'] == t['profiles'][owner]
                assert i['people'] == s['people'] and i['now'] == now and i['end'] == p['end']
                assert d['duration'] == t['config']['duration']
                assert d['inquiry_lane_free'] == (not point['pending'])
                can = bool(cell and cell['owner'] == owner and cell['observation']['observed_at'] <= now
                           and cell['observation']['stock'] > cell['own_takes'] and cell['observation']['accessible']
                           and own['food'] == 0 and own['hunger'] > 0 and now+d['duration'] <= i['end'])
                assert next(x['available'] for x in i['people'] if x['id'] == owner)
                # Rebuild the outer allocation from captured legacy contact
                # candidates; the separate original gate checks their internals.
                contact = bd['contact']; top = max(contact['candidates'], key=lambda c:c['score'])
                assert top['intent'] == contact['selected']
                invite = top['intent'] != 'Wait'; eat = own['food'] > 0 and own['hunger'] > 0
                outer = [dict(action='Wait', eligible=True, score=0),
                         dict(action='Consume', eligible=eat, score=own['hunger'] if eat else -1),
                         dict(action='Invite', eligible=invite, score=top['score'] if invite else -1)]
                assert bd['candidates'] == outer and bd['selected'] == max(outer,key=lambda c:c['score'])['action']
                candidates = outer + [dict(action='Acquire', eligible=can, score=own['hunger'] if can else -1)]
                assert d['candidates'] == candidates
                selected = 'Acquire' if t['policy'] == 'Acquire' and can else max(candidates,key=lambda c:c['score'])['action']
                assert selected == d['selected']
                local_wave.append((selected, digest(i)))
                if selected == 'Acquire':
                    attempt = next(a for a in w['admissions'] if a['actor'] == owner)
                    assert attempt['selection'] == [pno,sno,owner] and attempt['observation'] == cell['observation']
                    assert attempt['start'] == now
                    overestimates += cell['observation']['stock']-cell['own_takes'] > point['cache']['stock']
                choice_count += 1
            selections.append((now, [None if d is None else d['selected'] for d in w['choices']]))
            local_choices.append((now, local_wave))
    return dict(initial=digest([t['initial'],t['profiles']]), choices=digest(selections), local_choices=local_choices,
                periods=[([a['food'] for a in t['points'][p['after']]['state']['agents']],
                          [a['hunger'] for a in t['points'][p['after']]['state']['agents']],
                          t['points'][p['after']]['cache']['stock']) for p in t['periods']],
                final=(food, hunger, stock), trust=[[a['trust'].get(str(j),0) for j in range(3)] for a in b['agents']],
                raw_trust=[a['trust'] for a in b['agents']], legacy=digest(t['final_state']), area=area,
                counts=dict(worlds=1,claims=len(claims),grants=sum(r['outcome']=='Granted' for r in t['records']),
                            failures=sum(r['outcome']=='Depleted' for r in t['records']),meals=len(meals),
                            transfers=sum(e['transferred'] for e in events),memory=len(t['memory_records']),
                            inquiries=len(b['inquiry']['records'])-initial['state']['queries'] if 'inquiry' in b else t['final_point']['state']['queries']-initial['state']['queries'],
                            selections=choice_count,selected_overestimates=overestimates))


def main():
    groups = collections.defaultdict(dict); totals = collections.Counter(); witnesses = {}; sample = None
    matrix = json.loads((ROOT/'experiments/021/registered-matrix.json').read_bytes())
    with gzip.open(ROOT/'experiments/021/trials.jsonl.gz','rt',encoding='utf-8') as f:
        for ordinal,line in enumerate(f):
            t = json.loads(line)
            assert {k:t[k] for k in matrix[ordinal]} == matrix[ordinal]
            try: r = verify(t)
            except Exception:
                print('FAIL original ordinal',ordinal,flush=True); raise
            totals.update(r['counts'])
            if ordinal == 82: sample = t
            key = digest([t['seed'],t['case'],t['policy'],t['config']])
            groups[key][t['mode']] = r
            if ordinal in (1,2,11,12,31,32,80,81,82,153,154,161,162,163,186,187):
                witnesses[str(ordinal)] = {k:r[k] for k in ('counts','area','final','trust')}
    assert len(matrix) == totals['worlds'] == 214 and len(groups) == 57
    assert [totals[k] for k in ('claims','grants','failures','meals','transfers','inquiries','memory')] == [599,396,203,934,671,34,25112]
    contrasts = {}
    for mode in ('LastSeen','Expire12','FreshReference','Unobserved'):
        rows = [(v['CurrentOnly'],v[mode]) for v in groups.values() if mode in v]
        assert all(l['initial']==r['initial'] for l,r in rows)
        for l,r in rows:
            found = False
            for (lt,lc),(rt,rc) in zip(l['local_choices'],r['local_choices']):
                assert lt == rt, 'time diverged before a selected activity did'
                for a,b in zip(lc,rc):
                    if a is None or b is None:
                        assert a == b
                    elif a[0] != b[0]:
                        assert a[1] == b[1], 'unmatched old local input at first treatment choice'
                        found = True
                        break
                if found: break
        contrasts[mode] = dict(pairs=len(rows),choices=sum(l['choices']!=r['choices'] for l,r in rows),
                               periods=sum(l['periods']!=r['periods'] for l,r in rows),
                               final=sum(l['final']!=r['final'] for l,r in rows),
                               trust=sum(l['trust']!=r['trust'] for l,r in rows))
    assert contrasts['LastSeen'] == dict(pairs=57,choices=43,periods=14,final=0,trust=10)
    assert contrasts['Expire12'] == dict(pairs=44,choices=33,periods=12,final=0,trust=6)
    assert contrasts['FreshReference'] == dict(pairs=28,choices=6,periods=6,final=0,trust=4)
    assert contrasts['Unobserved'] == dict(pairs=28,choices=24,periods=24,final=24,trust=15)
    assert witnesses['81']['area'] == [45230,39035,45525]
    assert witnesses['82']['area'] == [43035,35215,44115]
    assert witnesses['80']['area'] == [42915,35110,43525]
    assert witnesses['31']['area'] == witnesses['32']['area']
    assert witnesses['161']['trust'] == witnesses['163']['trust']
    rejected = []
    for category in ('foreign_receipt','invented_quantity','receipt_reuse','rejuvenated_take',
                     'unavailable_choice','false_grant','duplicate_meal','wrong_deadline'):
        bad = copy.deepcopy(sample)
        sense = next(r for r in bad['memory_records'] if r['kind']=='Sense')
        taken = next(r for r in bad['memory_records'] if r['kind']=='Taken')
        if category == 'foreign_receipt': sense['after']['owner'] = 1
        elif category == 'invented_quantity': sense['after']['observation']['stock'] += 1
        elif category == 'receipt_reuse': sense['after']['receipt'] += 1
        elif category == 'rejuvenated_take': taken['after']['observation']['observed_at'] = taken['tick']
        elif category == 'unavailable_choice': bad['periods'][1]['steps'][3]['choices'][0]['retained'] = None
        elif category == 'false_grant': bad['records'][0]['food_after'] += 1
        elif category == 'duplicate_meal':
            w = next(w for p in bad['periods'] for w in p['steps'] if w['activity']['consumption'])
            w['activity']['consumption'].append(copy.deepcopy(w['activity']['consumption'][0]))
        else: bad['records'][0]['attempt']['end'] += 1
        try: verify(bad)
        except (AssertionError,KeyError,IndexError,TypeError): rejected.append(category)
        else: raise AssertionError('accepted corruption: '+category)
    print(json.dumps(dict(totals=dict(totals),groups=len(groups),contrasts=contrasts,witnesses=witnesses,
                          complementary_corruptions=rejected),sort_keys=True))


if __name__ == '__main__':
    main()
