"""Post-publication, read-only 024 review; no simulation/evaluator/reader imports.

Independently checks scalar intervention, work starts, all primitive food/hunger
operations, matched prefixes, nulls and paired counts. Cognitive completeness is
separately checked by the frozen reader; this program does not replace that gate.
"""
import argparse
import collections
import copy
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def core(trial):
    value = trial['history']['final_state']
    while 'events' not in value:
        value = value['base']
    return value


def reconstruct(trial):
    h = trial['history']
    state = copy.deepcopy(h['initial'])
    stock, eaten = h['initial_stock'], 0
    initial = sum(a['food'] for a in state) + stock
    assert initial == h['initial_total']
    events = core(trial)['events'][h['bootstrap_events']:]
    meals = core(trial)['cognition']['consumption']
    operations = collections.defaultdict(list)
    for kind, rows in enumerate((h['needs'], events, h['grants'], meals)):
        for index, row in enumerate(rows):
            operations[row['tick']].append((kind, index, row))
    areas = [0] * 4
    before_stop = [0] * 4
    food_path = []
    cutoff = h['start'] + trial['config']['world']['horizon']
    assert h['end'] == cutoff + 33
    phase = h['start'] if trial['config']['world']['phase'] == 'Epoch' else 0
    first_need = phase + ((h['start'] - phase) // 4 + 1) * 4
    assert [r['tick'] for r in h['needs']] == list(range(first_need, h['end'] + 1, 4))
    captured = {b['tick']: b for b in h['boundaries']}
    for tick in range(h['start'], h['end'] + 1):
        if tick > h['start']:
            for a in state:
                areas[a['id']] += a['hunger']
                if tick <= cutoff:
                    before_stop[a['id']] += a['hunger']
        # Same-tick needs, resource events, claims and meals follow physical order.
        for kind, _, row in sorted(operations[tick]):
            if kind == 0:
                assert row['before'] == [a['hunger'] for a in state]
                for a in state:
                    a['hunger'] = min(100, a['hunger'] + 5)
                assert row['after'] == [a['hunger'] for a in state]
            elif kind == 1:
                ids = row['participants']
                assert row['balances_before'] == [state[i]['food'] for i in ids]
                assert sum(row['balances_before']) == sum(row['balances_after'])
                assert all(v >= 0 for v in row['balances_after'])
                assert row['transferred'] in (0, 1)
                assert sum(abs(x-y) for x, y in zip(row['balances_before'], row['balances_after'])) == 2*row['transferred']
                for i, value in zip(ids, row['balances_after']):
                    state[i]['food'] = value
            elif kind == 2:
                a = state[row['owner']]
                assert (stock, a['food']) == (row['stock_before'], row['food_before'])
                take = int(stock > 0)
                assert row['outcome'] == ('Granted' if take else 'Depleted')
                stock -= take
                a['food'] += take
                assert (stock, a['food']) == (row['stock_after'], row['food_after'])
            else:
                a = state[row['actor']]
                assert a['food'] > 0 and a['hunger'] > 0
                assert (a['food'], a['hunger']) == (row['food_before'], row['hunger_before'])
                a['food'] -= 1
                a['hunger'] = max(0, a['hunger']-40)
                eaten += 1
                assert row['consumed'] == 1
                assert (a['food'], a['hunger']) == (row['food_after'], row['hunger_after'])
            assert stock + eaten + sum(a['food'] for a in state) == initial
        food_path.append([*[a['food'] for a in state], stock, eaten])
        if tick in captured:
            b = captured[tick]
            assert stock == b['stock']
            assert [(a['food'], a['hunger']) for a in state] == [(a['food'], a['hunger']) for a in b['agents']]
    assert stock == h['final_stock']
    assert [(a['food'], a['hunger']) for a in state] == [(a['food'], a['hunger']) for a in h['final_agents']]
    references = []
    expected_refs = []
    for bi, b in enumerate(h['boundaries']):
        for ci, d in enumerate(b['choices']):
            expected_refs.append((bi, ci))
            i = d['input']
            assert set(i) == {'legacy', 'retained', 'duration', 'inquiry_lane_free'}
            assert set(i['legacy']) == {'own', 'profile', 'now', 'end', 'people', 'query'}
            assert i['legacy']['own'] == b['agents'][i['legacy']['own']['id']]
            assert len(i['legacy']['own']['memories']) <= 16
    for audit in trial['valuations']:
        bi, ci = audit['boundary'], audit['choice']
        references.append((bi, ci))
        d = h['boundaries'][bi]['choices'][ci]
        own = d['input']['legacy']['own']
        terms = audit['selection']['terms']
        assert [r['action'] for r in terms] == ['Wait', 'Consume', 'Invite', 'Acquire']
        assert audit['policy'] == trial['config']['valuation']
        for term, candidate in zip(terms, d['candidates'], strict=True):
            assert term['action'] == candidate['action'] and term['eligible'] == candidate['eligible']
            if term['action'] == 'Consume':
                legal = own['food'] > 0 and own['hunger'] > 0
                assert legal == term['eligible']
                assert term['base_score'] == (own['hunger'] if legal else -1)
            bonus = min(own['hunger'], 40) if audit['policy'] == 'MealRelief' and term['action'] == 'Consume' and term['eligible'] else 0
            assert term['meal_relief'] == bonus
            assert term['final_score'] == term['base_score'] + bonus == candidate['score']
        selected = max(terms, key=lambda r: r['final_score'])['action']
        assert selected == audit['selection']['selected'] == d['selected']
    assert references == expected_refs
    completed = collections.defaultdict(list)
    starts = collections.defaultdict(list)
    choices = {}
    for b in h['boundaries']:
        choices[b['tick']] = [(d['input']['legacy']['own']['id'], d['selected'], d['contact']) for d in b['choices']]
    for r in h['transitions']:
        if not r['completed']:
            starts[r['tick']].append(r['commitment'])
    for kind, rows in (('event', events), ('grant', h['grants']), ('meal', meals)):
        for r in rows:
            completed[r['tick']].append([kind, r])
    trust = [a['trust'].get(str(i), 0) for a in h['final_agents'] for i in range(4) if i != a['id']]
    normalized = copy.deepcopy(h)
    for b in normalized['boundaries']:
        for d in b['choices']:
            for c in d['candidates']:
                if c['action'] == 'Consume':
                    c['score'] = 0
    return dict(area=areas, pre_stop_area=before_stop, food_path=food_path,
                starts=dict(starts), completed=dict(completed), choices=choices,
                trust=trust, meals=len(meals), normalized=normalized,
                final_food=[*[a['food'] for a in state], stock, eaten],
                final_hunger=[a['hunger'] for a in state], decisions=len(references))


def first_difference(a, b):
    return next((tick for tick in sorted(a.keys() | b.keys()) if a.get(tick) != b.get(tick)), None)


def inspect(archive):
    summaries, pairs = [], []
    pending = None
    placebo = []
    with gzip.open(archive, 'rb') as source:
        for ordinal, line in enumerate(source):
            t = json.loads(line)
            r = reconstruct(t)
            summaries.append(dict(ordinal=ordinal, meals=r['meals'], area=r['area'],
                                  pre_stop_area=r['pre_stop_area'], final_food=r['final_food'],
                                  final_hunger=r['final_hunger'], decisions=r['decisions']))
            if ordinal in (0, 1):
                placebo.append((copy.deepcopy(t['history']), copy.deepcopy(t['valuations'])))
            if ordinal >= 54:
                assert (t['history'], t['valuations']) == placebo[ordinal-54]
                continue
            if ordinal % 2 == 0:
                pending = (t, r)
                continue
            old, left = pending
            assert old['config']['world'] == t['config']['world']
            assert old['history']['initial'] == t['history']['initial']
            tick = first_difference(left['choices'], r['choices'])
            start_tick = first_difference(left['starts'], r['starts'])
            finish_tick = first_difference(left['completed'], r['completed'])
            assert tick == start_tick
            # Check actual local/opportunity inputs through the first selection difference.
            for lb, rb in zip(old['history']['boundaries'], t['history']['boundaries']):
                if tick is not None and lb['tick'] > tick:
                    break
                assert lb['tick'] == rb['tick']
                for ld, rd in zip(lb['choices'], rb['choices'], strict=True):
                    assert ld['input'] == rd['input']
                    if ld['selected'] != rd['selected'] or ld['contact'] != rd['contact']:
                        break
            fp = left['food_path']; fq = r['food_path']
            different = [i for i, (a, b) in enumerate(zip(fp, fq, strict=True)) if a != b]
            reconverged = next((i + old['history']['start'] for i in range(different[0]+1, len(fp)) if fp[i] == fq[i]), None) if different else None
            pairs.append(dict(ordinals=[ordinal-1, ordinal], first_choice=tick,
                              first_start=start_tick, first_completed_effect=finish_tick,
                              terminal_food=left['final_food'] != r['final_food'],
                              terminal_hunger=left['final_hunger'] != r['final_hunger'],
                              terminal_trust=left['trust'] != r['trust'],
                              behavior_state_null=left['normalized'] == r['normalized'],
                              meals=[left['meals'], r['meals']],
                              full_area_delta=[b-a for a, b in zip(left['area'], r['area'])],
                              pre_stop_area_delta=[b-a for a, b in zip(left['pre_stop_area'], r['pre_stop_area'])],
                              food_reconvergence=reconverged))
    assert len(summaries) == 56 and len(pairs) == 27
    counts = {key: sum(bool(p[key]) for p in pairs) for key in ('terminal_food', 'terminal_hunger', 'terminal_trust', 'behavior_state_null')}
    counts.update(changed_choices=sum(p['first_choice'] is not None for p in pairs),
                  changed_starts=sum(p['first_start'] is not None for p in pairs),
                  changed_completed_effects=sum(p['first_completed_effect'] is not None for p in pairs),
                  more_meals=sum(p['meals'][1] > p['meals'][0] for p in pairs),
                  fewer_meals=sum(p['meals'][1] < p['meals'][0] for p in pairs),
                  temporary_food_reconvergence=sum(p['food_reconvergence'] is not None for p in pairs))
    return dict(scope='Independent post-publication review, primitive operations and score projection; no evaluator imports',
                archive_sha256=hashlib.sha256(Path(archive).read_bytes()).hexdigest(),
                counts=counts, pairs=pairs, worlds=summaries, reporting_placebos=2)


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--expected')
    p.add_argument('--controls', action='store_true')
    args = p.parse_args()
    result = inspect(ROOT/'experiments/024/trials.jsonl.gz')
    if args.expected:
        assert result == json.loads(Path(args.expected).read_bytes())
        print('024 fresh independent review:56 worlds,27 pairs,2 placebos PASS', result['counts'])
    else:
        print(json.dumps(result, indent=2))
    if args.controls:
        with gzip.open(ROOT/'experiments/024/trials.jsonl.gz', 'rb') as source:
            original = json.loads(next(source))
        for category in ('score', 'bonus', 'selection', 'meal', 'need', 'grant'):
            trial = copy.deepcopy(original)
            if category == 'score':
                trial['valuations'][0]['selection']['terms'][1]['final_score'] += 1
            elif category == 'bonus':
                trial['valuations'][0]['selection']['terms'][1]['meal_relief'] = 1
            elif category == 'selection':
                trial['valuations'][0]['selection']['selected'] = 'Wait'
            elif category == 'meal':
                core(trial)['cognition']['consumption'][0]['food_after'] += 1
            elif category == 'need':
                trial['history']['needs'][0]['after'][0] += 1
            else:
                trial['history']['grants'][0]['stock_after'] += 1
            try:
                reconstruct(trial)
            except AssertionError:
                continue
            raise AssertionError('Undetected scratch corruption: '+category)
        print('024 new independent review:6 scratch arithmetic/selection/accounting controls rejected')


if __name__ == '__main__':
    main()
