"""Bounded observer-only causal references, supplementary to frozen024 readers."""
import argparse
import json
from pathlib import Path

import analyze024 as analysis
import review024 as reader


def inspect(archive, comparisons):
    witnesses = []
    selected = {0, 1, 6, 7, 12, 13, 18, 19, 20, 21, 52, 53}
    for ordinal, trial in enumerate(reader.read(archive)):
        if ordinal not in selected:
            continue
        pair = comparisons['pairs'][ordinal // 2]
        first = pair['first_selected']
        tick = first['tick'] if first else trial['history']['start']
        execution = analysis.executions(trial)
        later = [dict(tick=t, records=records) for t, records in sorted(execution.items())
                 if tick <= t <= tick + 12]
        missed = comparisons['worlds'][ordinal]['missed_eligible_meals']
        max_hunger = next((x for x in missed if x['hunger'] == 100), None)
        witnesses.append(dict(
            ordinal=ordinal, config=trial['config'], first_selected=first,
            first_local_reference=None if first is None else {
                key: pair['first_selected_local'][key]['ref']
                for key in ('baseline', 'meal_relief')},
            subsequent_executed_records=later,
            first_missed_meal_at_maximum_hunger=max_hunger,
            inquiries=comparisons['worlds'][ordinal]['inquiries'],
            persistent_state=comparisons['worlds'][ordinal]['independent']))
    return dict(
        scope='Observer-only original references; later outcomes never enter selection. '
              'Ordinals are zero-based. Full local inputs and score terms are in comparisons.json.',
        pairs=[{k: p[k] for k in (
            'ordinals', 'first_selected', 'first_execution', 'first_material',
            'temporary_food_reconvergence', 'terminal_food_difference',
            'terminal_hunger_difference', 'terminal_trust_difference',
            'full_history_body_equal', 'body_null_ignoring_only_consume_scores')}
            for p in comparisons['pairs']],
        witnesses=witnesses)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('archive')
    parser.add_argument('--comparisons', required=True)
    parser.add_argument('--output')
    parser.add_argument('--expected')
    args = parser.parse_args()
    result = inspect(args.archive, json.loads(Path(args.comparisons).read_bytes()))
    if args.expected:
        assert result == json.loads(Path(args.expected).read_bytes())
    if args.output:
        with Path(args.output).open('x', encoding='utf-8') as out:
            json.dump(result, out, indent=2)
            out.write('\n')
    print('024 causal references:27 pairs,12 bounded witness worlds PASS')


if __name__ == '__main__':
    main()
