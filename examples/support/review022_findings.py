"""Independent post-review physical/count/witness reconstruction of original022.

No imports of the022reader, evaluator, comparison tables or viewer. This does not
replace the full cognitive reader; it independently challenges its aggregate and
temporal interpretation using primitive original operations. Never writes archives.
"""
import argparse
import collections
import copy
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def base(trial):
    value = trial['final_state']
    while 'events' not in value:
        value = value['base']
    return value


def physical(trial):
    """Rebuild food/need independently, in actual need/event/claim/meal order."""
    actors = copy.deepcopy(trial['initial'])
    stock = trial['initial_stock']
    consumed = 0
    area = [0] * 4
    operations = collections.defaultdict(list)
    needs = trial['needs']
    assert [n['tick'] for n in needs] == list(range(trial['start'] + 4, trial['end'] + 1, 4))
    for n in needs:
        operations[n['tick']].append((0, 0, 'need', n))
    for e in base(trial)['events'][trial['bootstrap_events']:]:
        operations[e['tick']].append((1, e['scene'], 'event', e))
    for n, grant in enumerate(trial['grants']):
        operations[grant['tick']].append((2, n, 'claim', grant))
    meals = base(trial)['cognition']['consumption']
    for n, meal in enumerate(meals):
        operations[meal['tick']].append((3, n, 'meal', meal))
    captures = {b['tick']: b for b in trial['boundaries']}
    ledger = {}
    for tick in range(trial['start'], trial['end'] + 1):
        if tick > trial['start']:
            for owner, actor in enumerate(actors):
                area[owner] += actor['hunger']
        for _, _, kind, op in sorted(operations[tick]):
            if kind == 'need':
                assert op['before'] == [a['hunger'] for a in actors]
                for actor in actors:
                    actor['hunger'] = min(100, actor['hunger'] + 5)
                assert op['after'] == [a['hunger'] for a in actors]
            elif kind == 'event':
                ids = op['participants']
                assert op['balances_before'] == [actors[i]['food'] for i in ids]
                before, after = op['balances_before'], op['balances_after']
                assert sum(before) == sum(after)
                assert all(v >= 0 for v in after)
                assert op['transferred'] in (0, 1)
                assert sum(abs(a - b) for a, b in zip(before, after)) == 2 * op['transferred']
                for owner, value in zip(ids, after):
                    actors[owner]['food'] = value
            elif kind == 'claim':
                actor = actors[op['owner']]
                assert (stock, actor['food']) == (op['stock_before'], op['food_before'])
                take = int(stock > 0)
                assert op['outcome'] == ('Granted' if take else 'Depleted')
                stock -= take
                actor['food'] += take
                assert (stock, actor['food']) == (op['stock_after'], op['food_after'])
            else:
                actor = actors[op['actor']]
                assert actor['food'] > 0 and actor['hunger'] > 0
                assert (actor['food'], actor['hunger']) == (op['food_before'], op['hunger_before'])
                actor['food'] -= 1
                actor['hunger'] = max(0, actor['hunger'] - 40)
                consumed += 1
                assert op['consumed'] == 1
                assert (actor['food'], actor['hunger']) == (op['food_after'], op['hunger_after'])
            assert sum(a['food'] for a in actors) + stock + consumed == trial['initial_total']
        ledger[tick] = ([a['food'] for a in actors], stock, consumed)
        if tick in captures:
            capture = captures[tick]
            assert capture['stock'] == stock
            assert [(a['food'], a['hunger']) for a in capture['agents']] == [(a['food'], a['hunger']) for a in actors]
    assert [(a['food'], a['hunger']) for a in trial['final_agents']] == [(a['food'], a['hunger']) for a in actors]
    assert trial['final_stock'] == stock and consumed == len(meals)
    return dict(area=area, ledger=ledger)


def trust(trial):
    return [a['trust'].get(str(other), 0) for a in trial['final_agents'] for other in range(4) if other != a['id']]


def resource_value(own, partner):
    # Integer division truncates toward zero in the original Rust heuristic.
    div = lambda value, by: (abs(value) // by) * (-1 if value < 0 else 1)
    relationship = own['trust'].get(str(partner), 0)
    memories = [m for m in own['memories'] if m['partner'] == partner][-4:]
    valence = max(-40, min(40, sum(m['valence'] for m in memories)))
    request = own['hunger'] - 35 + div(relationship, 4) - own['caution'] // 4
    offer = own['generosity'] - own['hunger'] // 2 - own['caution'] // 2 + relationship + valence
    if own['hunger'] >= 60 and own['food'] <= 1:
        offer -= 60
    best = max(0, request, offer) if own['food'] else max(0, request)
    return best + max(0, own['hunger']) // 2 - 2


def pair(a, b, pa, pb):
    ca, cb = dict(a['config']), dict(b['config'])
    assert ca.pop('capacity') == 1 and cb.pop('capacity') == 2 and ca == cb
    for key in ('initial', 'initial_stock', 'initial_total', 'initial_contexts', 'start', 'end', 'profiles'):
        assert a[key] == b[key], ('unmatched', key)
    assert a['boundaries'][0] == b['boundaries'][0]
    assert base(a)['events'][:a['bootstrap_events']] == base(b)['events'][:b['bootstrap_events']]
    aa, bb = dict(a, config=ca), dict(b, config=cb)
    return dict(interim=any(pa['ledger'][t] != pb['ledger'][t] for t in pa['ledger']),
                terminal_food=pa['ledger'][a['end']] != pb['ledger'][b['end']],
                terminal_hunger=[x['hunger'] for x in a['final_agents']] != [x['hunger'] for x in b['final_agents']],
                terminal_trust=trust(a) != trust(b), complete_null=aa == bb,
                hunger_delta=[y-x for x, y in zip(pa['area'], pb['area'])])


def scratch_controls(sample):
    rejected = []
    def check(name, change):
        altered = copy.deepcopy(sample)
        change(altered)
        try:
            physical(altered)
        except AssertionError:
            rejected.append(name)
        else:
            raise AssertionError(('accepted scratch corruption', name))
    check('pulse_phase', lambda t: t['needs'][0].update(tick=t['needs'][0]['tick'] + 1))
    check('double_pressure', lambda t: t['needs'][0]['after'].__setitem__(3, t['needs'][0]['after'][3] + 5))
    check('duplicate_meal', lambda t: base(t)['cognition']['consumption'].append(copy.deepcopy(base(t)['cognition']['consumption'][0])))
    check('false_final_food', lambda t: t['final_agents'][0].update(food=t['final_agents'][0]['food'] + 1))
    return rejected


def inspect():
    counts = collections.Counter()
    fingerprints = set()
    witnesses = {}
    controls = []
    previous = None
    with gzip.open(ROOT / 'experiments/022/trials.jsonl.gz', 'rt', encoding='utf-8') as stream:
        for ordinal, line in enumerate(stream):
            trial = json.loads(line)
            p = physical(trial)
            counts['worlds'] += 1
            normalized = dict(trial, config=dict(trial['config']))
            normalized['config'].pop('case')
            fingerprints.add(hashlib.sha256(json.dumps(normalized, sort_keys=True, separators=(',', ':')).encode()).hexdigest())
            starts = [r['commitment'] for r in trial['transitions'] if not r['completed']]
            ends = {r['commitment']['id']: r['tick'] for r in trial['transitions'] if r['completed']}
            resources = [c for c in starts if isinstance(c['work'], dict)]
            if ordinal == 1:
                controls = scratch_controls(trial)
                first = trial['boundaries'][0]['choices']
                assert [d['contact']['Invite']['recipient'] for d in first] == [1, 0, 3, 2]
                assert [max(c['score'] for c in d['contact_candidates']) for d in first] == [96, 64, 108, 64]
                first_resources = [c for c in resources if c['start'] == 8]
                assert [(c['id'], c['participants'], ends[c['id']]) for c in first_resources] == [(8, [0, 1], 10), (9, [2, 3], 10)]
                assert [(e['tick'], e['decision']['selected']) for e in base(trial)['events'][6:10]] == [(9, 'Offer'), (9, 'Offer'), (10, 'Accept'), (10, 'Accept')]
            if ordinal == 5:
                assert ends[8] == 10 and ends[9] == 11
            if ordinal == 0:
                active = [c for c in starts if c['start'] <= 15 < ends[c['id']]]
                assert {c['id'] for c in active} == {23, 24, 26}
                assert sorted(a for c in active for a in c['participants']) == [0, 1, 2, 3]
            peak = max((sum(c['start'] <= tick < ends[c['id']] for c in resources) for tick in range(trial['start'], trial['end'])), default=0)
            if trial['config']['capacity'] == 2:
                counts['concurrent_peak_two'] += peak == 2
            counts['executed_queries'] += sum(c['work'] == 'Query' for c in starts)
            counts['protocol_blocked'] += sum(a['reason'] == 'AtomicBlocked' for a in trial['admissions'])
            counts['masked_local_query'] += sum(not d['input']['inquiry_lane_free'] and d['input']['legacy']['query']['decision']['selected'] != 'Pause' for b in trial['boundaries'] for d in b['choices'])
            counts['nonzero_start_phase'] += trial['start'] % 4 != 0
            for boundary in trial['boundaries']:
                for d in boundary['choices']:
                    assert set(d['input']) == {'legacy', 'retained', 'duration', 'inquiry_lane_free'}
                    own = d['input']['legacy']['own']
                    assert own == boundary['agents'][own['id']]
                    candidates = [c for c in d['contact_candidates'] if c['eligible']]
                    assert d['contact'] == max(candidates, key=lambda c: c['score'])['intent']
                    legacy = d['input']['legacy']
                    available = {p['id'] for p in legacy['people'] if p['available']}
                    for candidate in d['contact_candidates']:
                        if isinstance(candidate['intent'], dict) and candidate['intent']['Invite']['kind'] == 'Resource':
                            partner = candidate['intent']['Invite']['recipient']
                            eligible = own['id'] in available and partner in available and legacy['now'] + 8 <= legacy['end']
                            assert candidate['eligible'] == eligible
                            assert candidate['score'] == (resource_value(own, partner) if eligible else -1)
                for response in boundary['responses']:
                    assert response['selected'] == max(response['candidates'], key=lambda c: c['score'])['reply']
                    own = response['input']['own']
                    for candidate in response['candidates']:
                        if candidate['reply'] == 'Continue':
                            assert candidate['score'] == 0
                        elif isinstance(candidate['reply'], dict):
                            invitation = next(i for i in response['input']['incoming'] if i['proposer'] == candidate['reply']['Accept'])
                            score = resource_value(own['own'], invitation['proposer']) if invitation['kind'] == 'Resource' else own['profile']['relationship_goal'] - own['profile']['privacy'] - 2
                            assert candidate['score'] == score
            if ordinal % 2:
                a, pa = previous
                result = pair(a, trial, pa, p)
                counts['pairs'] += 1
                for key in ('interim', 'terminal_food', 'terminal_hunger', 'terminal_trust', 'complete_null'):
                    counts[key] += result[key]
                if ordinal in (1, 3, 5, 11, 15, 19, 21, 31, 33, 35):
                    witnesses[str(ordinal)] = result
            else:
                previous = trial, p
    counts['distinct_label_neutral_histories'] = len(fingerprints)
    assert {k: counts[k] for k in ('worlds', 'pairs', 'concurrent_peak_two', 'interim', 'terminal_food', 'terminal_hunger', 'terminal_trust', 'complete_null', 'distinct_label_neutral_histories', 'executed_queries', 'protocol_blocked', 'masked_local_query')} == dict(worlds=100, pairs=50, concurrent_peak_two=41, interim=24, terminal_food=15, terminal_hunger=20, terminal_trust=31, complete_null=9, distinct_label_neutral_histories=84, executed_queries=12, protocol_blocked=31, masked_local_query=52)
    return dict(counts=dict(counts), witnesses=witnesses, additional_scratch_rejected=controls,
                scope='Independent primitive physical progression, pair fairness, integer tie selection and counts; full cognition remains separately reconstructed by frozen reader')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--expected', type=Path)
    args = parser.parse_args()
    result = inspect()
    if args.expected:
        assert result == json.loads(args.expected.read_text(encoding='utf-8'))
    print(json.dumps(result, sort_keys=True))
