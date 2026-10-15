"""Independent read-only 016 review. No evaluator or original016 reader imports.

Complement full cognitive reconstruction with separately derived retry arithmetic,
local schemas, proposal/recipient ties, consent, matching and material accounting.
"""
import collections
import copy
import gzip
import json
from pathlib import Path
from review015_findings import resource

ROOT = Path(__file__).resolve().parents[2]


def read(name):
    return json.loads(gzip.decompress((ROOT / 'experiments/016' / (name + '.json.gz')).read_bytes()))


def update(cells, mode, s):
    assert set(s) == {'owner', 'partner', 'kind', 'sent', 'received', 'outcome'}
    assert 0 <= s['owner'] < 3 and 0 <= s['partner'] < 3 and s['owner'] != s['partner']
    assert s['received'] >= s['sent'] + 2
    live = [copy.deepcopy(c) for c in cells if c['until'] > s['received']]
    removed = None
    if mode in ('Control', 'Unchanged'):
        live = []
    elif mode == 'Recent':
        live = []
    elif s['outcome'] == 'Met':
        live = [c for c in live if (c['partner'], c['kind']) != (s['partner'], s['kind'])]
    if mode not in ('Control', 'Unchanged') and s['outcome'] != 'Met':
        duration = 6 if mode == 'Recent' else 12 if mode == 'Untyped' or s['outcome'] == 'Declined' else 2
        c = {k: s[k] for k in ('partner', 'kind', 'outcome', 'received')}
        c['until'] = s['received'] + duration
        old = next((n for n, x in enumerate(live) if (x['partner'], x['kind']) == (c['partner'], c['kind'])), None)
        if old is not None:
            live[old] = c
        else:
            if len(live) == (1 if mode == 'Recent' else 2):
                removed = live.pop(0)
            live.append(c)
    return dict(signal=s, before=cells, after=live, evicted=removed)


def local(i, state, now, end, busy):
    assert set(i) == {'own', 'profile', 'now', 'end', 'people', 'query'}
    assert set(i['own']) == {'id', 'food', 'hunger', 'generosity', 'caution', 'expectation', 'trust', 'memories'}
    assert set(i['profile']) == {'privacy', 'relationship_goal', 'honesty', 'request_cost'}
    assert i['own'] == state['agents'][i['own']['id']]
    assert (i['now'], i['end']) == (now, end)
    assert i['people'] == [dict(id=n, available=now >= b) for n, b in enumerate(busy)]
    assert all(set(p) == {'id', 'available'} for p in i['people'])


def verify(t):
    memory = [[], [], []]
    total = collections.Counter()
    events = t['final_state']['base']['base']['base']['base']['base']['events']
    for p in t['periods']:
        busy = p['busy_until'][:]
        for capture in p['waves']:
            w = capture['wave']; now = w['before']['tick']
            assert capture['memory_before'] == memory
            live = [[c for c in cells if c['until'] > now] for cells in memory]
            assert capture['available'] == live
            memory = copy.deepcopy(live)
            invitations = []
            for owner, d in enumerate(w['proposals']):
                i = d['input']; local(i, w['before'], now, p['end'], busy)
                assert i['own']['id'] == owner
                expected = [dict(intent='Wait', reservation=0, eligible=True, score=0)]
                for partner in range(3):
                    if partner == owner:
                        continue
                    eligible = i['people'][owner]['available'] and i['people'][partner]['available'] and now + 8 <= p['end']
                    expected.append(dict(intent=dict(Invite=dict(recipient=partner, kind='Resource')), reservation=8, eligible=eligible, score=resource(i['own'], partner) if eligible else -1))
                query = i['query']['decision']; action = query['selected']
                if action != 'Pause':
                    ask = action['Ask']; partner = ask['source']; cost = 15 if ask['strategy'] == 'Direct' else 33
                    eligible = i['people'][owner]['available'] and i['people'][partner]['available'] and now + cost <= p['end']
                    score = next(c['score'] for c in query['candidates'] if c['action'] == action) - 2
                    expected.append(dict(intent=dict(Invite=dict(recipient=partner, kind='Inquiry')), reservation=cost, eligible=eligible, score=score if eligible else -1))
                if t['mode'] in ('Recent', 'Typed', 'Untyped'):
                    for c in expected[1:]:
                        v = c['intent']['Invite']
                        if any(x['partner'] == v['recipient'] and x['kind'] == v['kind'] for x in live[owner]):
                            c.update(eligible=False, score=-1)
                assert d['candidates'] == expected
                assert d['selected'] == max(expected, key=lambda c: c['score'])['intent']
                if d['selected'] != 'Wait':
                    v = d['selected']['Invite']
                    invitations.append(dict(proposer=owner, recipient=v['recipient'], kind=v['kind'], expires=now+2))
            if not invitations:
                assert w['after']['tick'] == p['end'] and not capture['updates']
                continue
            for owner, r in enumerate(w['responses']):
                i = r['input']; local(i['own'], w['before'], now+1, p['end'], busy)
                assert i['incoming'] == [v for v in invitations if v['recipient'] == owner]
                assert i['outgoing'] == w['proposals'][owner]
                cs = [dict(reply='Wait', score=0)]
                for v in i['incoming']:
                    own = i['own']
                    score = resource(own['own'], v['proposer']) if v['kind'] == 'Resource' else own['profile']['relationship_goal'] - own['profile']['privacy'] - 2
                    cs.append(dict(reply=dict(Accept=v['proposer']), score=score if own['people'][owner]['available'] else -1))
                d = i['outgoing']
                if d['selected'] != 'Wait':
                    cs.append(dict(reply='Continue', score=next(c['score'] for c in d['candidates'] if c['intent'] == d['selected']) if t['mode'] == 'Unchanged' else 0))
                assert r['candidates'] == cs and r['selected'] == max(cs, key=lambda c: c['score'])['reply']
            key = lambda v: v['proposer'] if t['config']['order'] == 'Ascending' else 2-v['proposer'] if t['config']['order'] == 'Descending' else (v['proposer']+3-w['id']%3)%3
            assert [a['invitation'] for a in w['allocations']] == sorted(invitations, key=key)
            occupied = set(); executions = []; outcomes = {}
            for a in w['allocations']:
                v = a['invitation']; pair = {v['proposer'], v['recipient']}
                consent = w['responses'][v['recipient']]['selected'] == dict(Accept=v['proposer'])
                reverse = any(x['proposer'] == v['recipient'] and x['recipient'] == v['proposer'] and x['kind'] == v['kind'] for x in executions)
                reason = 'Unavailable' if any(now+2 < busy[n] for n in pair) else 'Declined' if not consent else 'Reciprocal' if reverse else 'Contention' if pair & occupied else 'Execute'
                assert a['disposition'] == reason
                outcomes[v['proposer']] = 'Met' if reason in ('Execute', 'Reciprocal') else 'AcceptedUnmet' if reason == 'Contention' else reason
                total[reason] += 1
                if reason == 'Execute':
                    executions.append(v); occupied |= pair
            assert len(executions) <= 1
            x = w['execution']
            if executions:
                assert x['invitation'] == executions[0] and x['before']['tick'] == now+2
                assert x['after'] == w['after'] and now+2 < w['after']['tick'] <= p['end']
                for n in occupied:
                    busy[n] = w['after']['tick']
                for e in events[x['before']['events']:x['after']['events']]:
                    assert sum(e['balances_before']) == sum(e['balances_after'])
                    assert e['transferred'] == sum(max(0, a-b) for a, b in zip(e['balances_before'], e['balances_after']))
            else:
                assert x is None and w['after'] == dict(w['before'], tick=now+2)
            updates = []
            for owner, d in enumerate(w['proposals']):
                if d['selected'] == 'Wait':
                    continue
                v = d['selected']['Invite']
                s = dict(owner=owner, partner=v['recipient'], kind=v['kind'], sent=now, received=w['after']['tick'], outcome=outcomes[owner])
                u = update(memory[owner], t['mode'], s); updates.append(u); memory[owner] = u['after']
            assert capture['updates'] == updates
        for c in p['consumption']:
            assert c['food_after'] == c['food_before']-c['consumed']
            total['consumed'] += c['consumed']
    assert memory == t['final_cells']
    assert sum(a['food'] for a in t['initial']['agents']) == sum(a['food'] for a in t['periods'][-1]['after']['agents'])+total['consumed']
    return total


def main():
    trials = read('seed-42') + read('held-out-seed-42') + read('seed-1-decline')
    totals = [verify(t) for t in trials]
    sample = next(t for t in trials if t['case'] == 'late_willing' and t['mode'] == 'Typed')
    mutual = next(t for t in trials if t['case'] == 'mutual' and t['mode'] == 'Control')
    rejected = []
    for category in ('feedback_visibility', 'retry_deadline', 'expiration', 'candidate_legality', 'consent', 'arbitration', 'time', 'resource', 'private_state'):
        bad = copy.deepcopy(mutual if category in ('consent', 'arbitration', 'resource') else sample)
        capture = bad['periods'][0]['waves'][0]; w = capture['wave']
        if category == 'feedback_visibility': capture['updates'][0]['signal']['private_reason'] = 'other invitation'
        elif category == 'retry_deadline': capture['updates'][0]['after'][0]['until'] += 1
        elif category == 'expiration': bad['periods'][0]['waves'][1]['available'] = [[], [], []]
        elif category == 'candidate_legality': w['proposals'][0]['selected'] = 'Wait'
        elif category == 'consent': w['responses'][1]['selected'] = 'Wait'
        elif category == 'arbitration': w['allocations'][0]['disposition'] = 'Contention'
        elif category == 'time': w['after']['tick'] += 1
        elif category == 'resource': bad['final_state']['base']['base']['base']['base']['base']['events'][2]['transferred'] += 1
        else: w['proposals'][0]['input']['people'][1]['private_food'] = 4
        try: verify(bad)
        except (AssertionError, KeyError, StopIteration): rejected.append(category)
        else: raise AssertionError('accepted corruption: '+category)
    assert len(trials) == 260
    assert sum(r['Unavailable'] for r in totals) == 0
    core = lambda case, mode: next(t for t in trials[:70] if t['case'] == case and t['mode'] == mode)
    at = lambda t, tick: next(c for c in t['periods'][0]['waves'] if c['wave']['before']['tick'] == tick)
    typed, control, recent = [core('late_willing', m) for m in ('Typed', 'Control', 'Recent')]
    assert at(typed, 49)['available'][0][0]['until'] == 57
    for tick in (49, 52, 55):
        x = at(typed, tick)['wave']['execution']
        es = typed['final_state']['base']['base']['base']['base']['base']['events'][x['before']['events']:x['after']['events']]
        assert len(es) == 1 and es[0]['decision']['selected'] == 'Leave'
    assert at(typed, 58)['wave']['proposals'][0]['selected'] == 'Wait' and 58+15 > typed['periods'][0]['end']
    for t in (control, recent):
        x = at(t, 49)['wave']['execution']
        assert x['invitation']['kind'] == 'Inquiry' and x['after']['tick'] == 64
    positive = {t['mode']: t for t in trials[250:] if t['config']['order'] == 'Ascending'}
    a, b = positive['Control'], positive['Recent']
    assert at(a, 53)['wave']['before'] == at(b, 53)['wave']['before']
    assert [d['input'] for d in at(a, 53)['wave']['proposals']] == [d['input'] for d in at(b, 53)['wave']['proposals']]
    old = lambda t: t['final_state']['base']['base']['base']['base']['base']
    point = at(a, 53)['wave']['before']
    assert old(a)['events'][:point['events']] == old(b)['events'][:point['events']]
    assert old(a)['concerns']['transitions'][:point['concerns']] == old(b)['concerns']['transitions'][:point['concerns']]
    assert a['final_state']['base']['base']['assessment']['records'][:point['assessment']] == b['final_state']['base']['base']['assessment']['records'][:point['assessment']]
    assert at(a, 53)['wave']['responses'][1]['selected'] == dict(Accept=0)
    assert at(b, 53)['wave']['responses'][1]['selected'] == dict(Accept=2)
    assert (a['periods'][0]['after']['agents'][1]['trust']['2'], b['periods'][0]['after']['agents'][1]['trust']['2']) == (-28, -56)
    assert [sum(c['consumed'] for p in t['periods'] for c in p['consumption']) for t in (a, b)] == [9, 11]
    orders = {}
    for order in ('Ascending', 'Descending', 'Rotating'):
        orders[order] = [sum(sum(e['transferred'] for e in t['final_state']['base']['base']['base']['base']['base']['events'][t['initial']['events']:]) for t in trials[:250] if t['mode'] == m and t['config']['order'] == order and t['config']['budget'] == 24) for m in ('Control', 'Typed')]
    assert orders == dict(Ascending=[95, 105], Descending=[166, 162], Rotating=[147, 170])
    summaries = read('summary'); groups = collections.defaultdict(dict)
    for s in summaries:
        groups[(s['seed'], s['case'], json.dumps(s['config'], sort_keys=True))][s['mode']] = s
    counts = {}
    for mode in ('Recent', 'Typed', 'Untyped'):
        pairs = [(g['Control'], g[mode]) for g in groups.values()]
        counts[mode] = [sum(b['declined'] < a['declined'] for a, b in pairs), sum(b['declined'] > a['declined'] for a, b in pairs), sum(b['consumed'] > a['consumed'] for a, b in pairs), sum(b['consumed'] < a['consumed'] for a, b in pairs)]
    assert len(groups) == 200 and counts == dict(Recent=[60,77,2,14], Typed=[180,8,0,20], Untyped=[180,8,0,20])
    print(json.dumps(dict(histories=len(trials), corruption_categories=rejected, unavailable_primary=0, order_transfers=orders, all_seed_counts=counts, positive_consumed=[9, 11], harmful_retry_until=57, verdict='ACCEPT WITH QUALIFICATION; Typed adoption NO JUSTIFICATION'), sort_keys=True))


if __name__ == '__main__':
    main()
