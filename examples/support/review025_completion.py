"""Read-only post-registration coverage for the subsequently supplied I3–M text.

No outcome writer or policy change. Corruptions exist only in temporary copies;
the frozen independent reader, rather than the viewer, must reject each one.
"""
import argparse
import copy
import gzip
import hashlib
import json
from pathlib import Path

import review025

ROOT = Path(__file__).resolve().parents[2]


def inspect():
    archive = ROOT / 'experiments/025/trials.jsonl.gz'
    before = hashlib.sha256(archive.read_bytes()).hexdigest()
    selected = {}
    totals = dict(attempts=0, completed=0, failed=0, food_created=0, waste_created=0)
    with gzip.open(archive, 'rb') as stream:
        for ordinal, line in enumerate(stream):
            trial = json.loads(line)
            if ordinal < 48:
                totals['attempts'] += len(trial['material']['processes'])
                for op in trial['material']['operations']:
                    totals['completed'] += op['outcome'] == 'Prepared'
                    totals['failed'] += op['outcome'] == 'Insufficient'
                    totals['food_created'] += op['food_after'] - op['food_before']
                    totals['waste_created'] += op['waste_after'] - op['waste_before']
            if ordinal in (1, 5, 7, 9, 17, 27, 31, 43):
                selected[ordinal] = trial
    original = selected[1]
    review025.verify(original)
    mutations = {}

    def op_change(name, index, field, value):
        bad = copy.deepcopy(original)
        bad['material']['operations'][index][field] = value
        mutations[name] = bad

    op_change('negative_stock', 0, 'raw_after', -1)
    op_change('duplicate_raw_debit', 0, 'raw_after', 4)
    op_change('duplicate_escrow_credit', 0, 'escrow_after', 4)
    op_change('missing_escrow_debit', 4, 'escrow_after', 2)
    op_change('premature_finished_food', 0, 'food_after', 1)
    op_change('missing_waste', 4, 'waste_after', 0)
    op_change('duplicate_final_output', 4, 'food_after', 2)
    op_change('inventory_overflow', 4, 'food_after', 2**32)
    bad = copy.deepcopy(original)
    review025.core(bad['history'])['cognition']['consumption'][0]['food_after'] = -1
    mutations['impossible_consumption'] = bad
    bad = copy.deepcopy(original)
    bad['history']['transitions'] = [x for x in bad['history']['transitions']
                                   if not (x['completed'] and x['commitment']['id'] == 0)]
    mutations['unaccounted_cancellation'] = bad
    bad = copy.deepcopy(original)
    bad['material']['operations'] = [x for x in bad['material']['operations']
                                     if not (x['commitment'] == 0 and x['outcome'] == 'Prepared')]
    mutations['unaccounted_termination'] = bad
    bad = copy.deepcopy(selected[17])
    event = review025.core(bad['history'])['events'][13]
    event['balances_before'][0] = 0
    mutations['transfer_without_ownership'] = bad
    bad = copy.deepcopy(original)
    bad['material']['final_raw'] += 1
    mutations['incorrect_final_total'] = bad
    controls = []
    for name, bad in mutations.items():
        try:
            review025.verify(bad)
        except AssertionError:
            controls.append(dict(category=name, rejected=True))
        else:
            raise AssertionError('Undetected completion corruption: ' + name)

    contention = []
    for ordinal in (5, 7, 9):
        trial = selected[ordinal]
        operations = trial['material']['operations']
        extraction = [o for o in operations if o['outcome'] in ('Extracted', 'Insufficient')]
        assert len(extraction) == 2
        assert sum(o['outcome'] == 'Extracted' for o in extraction) == 1
        assert trial['material']['final_raw'] == 0
        contention.append(dict(ordinal=ordinal, order=trial['config']['world']['order'],
                               source_initial=trial['material']['initial_raw'],
                               extraction=[dict(owner=o['owner'], tick=o['tick'],
                                                outcome=o['outcome'], raw_after=o['raw_after'])
                                           for o in extraction]))
    failure = next(p for p in original['material']['processes'] if p['commitment'] == 8)
    failure_operations = [dict(ref=f'material.operations[{n}]', **o)
                          for n, o in enumerate(original['material']['operations'])
                          if o['commitment'] == 8]
    assert (failure['start'], failure['extract_at'], failure['finish_at']) == (7, 9, 13)
    assert [o['outcome'] for o in failure_operations] == ['Insufficient', 'Failed']
    assert all(o['escrow_after'] == o['food_after'] == 0 for o in failure_operations)
    analysis = json.loads((ROOT / 'experiments/025/causal-analysis.json').read_bytes())
    unused = [dict(ordinal=w['ordinal'], **c) for w in analysis['process_chains']
              for c in w['chains'] if c['unused_through_end']]
    assert len(unused) == 5 and not any(c['unused_with_later_ordinary_meal_option'] for c in unused)
    assert hashlib.sha256(archive.read_bytes()).hexdigest() == before
    return dict(scope='Post-evaluation I3–M read-only checks; no new worlds or changed frozen readers',
                archive_sha256=before, primary_48=totals, material_corruptions=controls,
                contention=contention, failure=dict(process=failure, operations=failure_operations),
                unused=[dict(ordinal=c['ordinal'], owner=c['owner'], commitment=c['commitment'],
                             finish_at=c['finish_at'], first_later_opportunity=c['first_later_opportunity'],
                             unused_with_meal_option=c['unused_with_later_ordinary_meal_option']) for c in unused],
                long_world=dict(ordinal=43, end=selected[43]['history']['end'],
                                attempts=len(selected[43]['material']['processes']),
                                final_raw=selected[43]['material']['final_raw'],
                                final_waste=selected[43]['material']['final_waste']))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--expected', type=Path)
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    result = inspect()
    if args.expected:
        assert result == json.loads(args.expected.read_bytes())
    if args.output:
        with args.output.open('x', encoding='utf-8') as stream:
            json.dump(result, stream, indent=2)
            stream.write('\n')
    print('025 completion:13 material corruptions rejected;3 contention orders;5 qualified non-use cases; original archive unchanged')


if __name__ == '__main__':
    main()
