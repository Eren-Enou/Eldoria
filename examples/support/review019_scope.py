"""Complementary scientific review: recorded arithmetic, endpoints and witnesses.

No evaluator, policy or earlier reconstruction imported. Read immutable archives.
"""
import collections
import gzip
import itertools
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / 'experiments/019'


def main():
    counts = collections.Counter()
    groups = collections.defaultdict(dict)
    with gzip.open(D / 'summary.json.gz', 'rt', encoding='utf-8') as f:
        summaries = json.load(f)
    assert len(summaries) == 1296
    for s in summaries:
        groups[s['seed'], s['case'], json.dumps(s['config'], sort_keys=True)][s['mode']] = s
    expected = json.loads((D / 'all-seed-comparisons.json').read_bytes())
    comparisons = {}
    for pair in expected:
        left, right = pair.split('/')
        c = collections.Counter({k: 0 for k in expected[pair]})
        for modes in groups.values():
            a, b = modes[left], modes[right]
            c['pairs'] += 1
            c['material_changed'] += (a['final_food'], a['final_hunger']) != (b['final_food'], b['final_hunger'])
            for field, key in [('transfers', 'transfers_changed'), ('queries', 'queries_changed'), ('consumed', 'consumed_changed')]:
                c[key] += a[field] != b[field]
            for field, key, op in [('hunger_area', 'someone_lower_area', lambda a, b: b < a), ('hunger_area', 'someone_higher_area', lambda a, b: b > a), ('final_hunger', 'someone_hungrier', lambda a, b: b > a), ('final_hunger', 'someone_less_hungry', lambda a, b: b < a)]:
                c[key] += any(op(x, y) for x, y in zip(a[field], b[field], strict=True))
        assert c == expected[pair], (pair, c)
        comparisons[pair] = dict(c)
    terminals = {}
    nulls = reconvergent = 0
    for name in ('seed-42', 'held-out-seed-42', 'seed-1-focus'):
        block = []
        with gzip.open(D / (name + '.jsonl.gz'), 'rt', encoding='utf-8') as f:
            for ordinal, line in enumerate(f, 1):
                t = json.loads(line)
                counts['histories'] += 1
                for p in t['periods']:
                    for w in p['steps']:
                        s = w['activity']
                        for d in s['decisions']:
                            if d is None:
                                continue
                            i = d['contact']['input']
                            cs = d['candidates']
                            best = max(cs, key=lambda c: c['score'])['action']
                            eat = cs[1]['eligible']
                            if t['mode'] == 'EatFirst' and eat:
                                best = 'Consume'
                            if t['mode'] == 'Deadline' and eat:
                                chosen = next(c for c in d['contact']['candidates'] if c['intent'] == d['contact']['selected'])
                                best = 'Invite' if cs[2]['eligible'] and i['now'] + 1 + chosen['reservation'] > i['end'] else 'Consume'
                            assert d['selected'] == best
                            counts['selections'] += 1
                        counts['provisional_responses'] += len(s['responses'])
                        counts['actual_queries'] += s['query'] is not None
                final = t['points'][t['periods'][-1]['after']]['state']
                consumed = sum(c['consumed'] for p in t['periods'] for w in p['steps'] for c in w['activity']['consumption'])
                assert sum(a['food'] for a in t['initial']['state']['agents']) == sum(a['food'] for a in final['agents']) + consumed
                material = lambda p: [(a['food'], a['hunger']) for a in p['state']['agents']]
                block.append((t['mode'], t['final_state'], [material(t['points'][p['after']]) for p in t['periods']]))
                if len(block) == 6:
                    for a, b in itertools.combinations(block, 2):
                        nulls += a[1] == b[1]
                        reconvergent += a[2][-1] == b[2][-1] and a[2] != b[2]
                    block = []
                if name == 'seed-42' and t['case'] in ('useful_contact', 'unproductive_contact') and t['mode'] in ('Baseline', 'Scale110'):
                    terminals[t['case'] + '/' + t['mode']] = dict(record=ordinal, food=[a['food'] for a in final['agents']], hunger=[a['hunger'] for a in final['agents']])
    assert counts['histories'] == 336 and counts['actual_queries'] == 152
    assert nulls >= 317 and reconvergent > 0
    print(json.dumps(dict(counts=counts, all_seed_contrasts=comparisons, full_final_nulls=nulls, material_reconvergent_pairs=reconvergent, terminals=terminals), sort_keys=True))


if __name__ == '__main__':
    main()
