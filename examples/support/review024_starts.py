"""Read-only supplementary024 start/completion timing clarification; no policy imports."""
import argparse
import collections
import gzip
import json
from pathlib import Path

SCOPE = ('Supplemental observer measurement clarification; original024 metrics and outcomes unchanged. '
         'The archived first_execution comparer measures completed resource/claim/meal/query effects, '
         'not commitment starts. All19 changed pairs start different committed work at their first '
         'selected difference; completed primitive effects follow.')


def starts(trial):
    rows = collections.defaultdict(list)
    for index, record in enumerate(trial['history']['transitions']):
        if not record['completed']:
            rows[record['tick']].append(dict(ref=f'history.transitions[{index}]',
                                             commitment=record['commitment']))
    return dict(rows)


def inspect(archive):
    pairs = []
    previous = None
    with gzip.open(archive, 'rb') as source:
        for ordinal, line in enumerate(source):
            if ordinal >= 54:
                break  # The two reporting placebos are outside the 27 policy pairs.
            trial = json.loads(line)
            if ordinal % 2 == 0:
                previous = trial
                continue
            left, right = starts(previous), starts(trial)
            tick = next((tick for tick in sorted(left.keys() | right.keys())
                         if [x['commitment'] for x in left.get(tick, [])] !=
                         [x['commitment'] for x in right.get(tick, [])]), None)
            pairs.append(dict(ordinals=[ordinal - 1, ordinal],
                              first_started_commitment_difference=tick,
                              left=None if tick is None else left.get(tick, []),
                              right=None if tick is None else right.get(tick, [])))
    assert len(pairs) == 27
    return dict(scope=SCOPE, pairs=pairs,
                changed_pairs=sum(p['first_started_commitment_difference'] is not None for p in pairs))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('archive')
    parser.add_argument('--expected', required=True)
    parser.add_argument('--comparisons', required=True)
    args = parser.parse_args()
    result = inspect(args.archive)
    assert result == json.loads(Path(args.expected).read_bytes())
    comparisons = json.loads(Path(args.comparisons).read_bytes())
    for pair, original in zip(result['pairs'], comparisons['pairs'], strict=True):
        assert pair['ordinals'] == original['ordinals']
        expected = original['first_selected']
        assert pair['first_started_commitment_difference'] == (expected['tick'] if expected else None)
    print('024 independent started-work references:19 changed/8 unchanged pairs PASS; original metrics preserved')


if __name__ == '__main__':
    main()
