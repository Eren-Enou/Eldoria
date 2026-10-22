"""Independent observer-only food-fate enumeration; no simulation/review imports."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


class FoodFates:
    """All physically compatible tag assignments, not actual simulated identities."""

    def __init__(self, food):
        self.food = list(food)
        # Four owned bitsets, consumed bitset, ever-transferred bitset.
        self.states = {(0, 0, 0, 0, 0, 0)}
        self.outputs = []
        self.peak_states = 1
        self.check()

    def check(self):
        assert self.states and len(self.states) <= 100_000, 'No silent attribution pruning'
        known = (1 << len(self.outputs)) - 1
        for state in self.states:
            occupied = 0
            for owner in range(4):
                assert self.food[owner] >= state[owner].bit_count()
            for compartment in state[:5]:
                assert occupied & compartment == 0, 'Duplicate hypothetical unit'
                occupied |= compartment
            assert occupied == known and state[5] & ~known == 0
        self.peak_states = max(self.peak_states, len(self.states))

    def prepared(self, owner, ref):
        assert len(self.outputs) < 8, 'Outside registered finite-input bound'
        bit = 1 << len(self.outputs)
        self.outputs.append(dict(owner=owner, ref=ref))
        self.food[owner] += 1
        new = set()
        for state in self.states:
            row = list(state)
            row[owner] |= bit
            new.add(tuple(row))
        self.states = new
        self.check()

    def untagged(self, owner, amount):
        assert amount >= 0
        self.food[owner] += amount
        self.check()

    def debit(self, owner, recipient=None):
        assert self.food[owner] > 0 and recipient != owner, 'Debit without ownership'
        new = set()
        for state in self.states:
            # Recorded food may include original or cache units without a tag.
            if self.food[owner] > state[owner].bit_count():
                new.add(state)
            bits = state[owner]
            while bits:
                bit = bits & -bits
                bits -= bit
                row = list(state)
                row[owner] ^= bit
                if recipient is None:
                    row[4] |= bit
                else:
                    row[recipient] |= bit
                    row[5] |= bit
                new.add(tuple(row))
        self.food[owner] -= 1
        if recipient is not None:
            self.food[recipient] += 1
        self.states = new
        self.check()

    def summary(self):
        def bounds(fn):
            values = [fn(state).bit_count() for state in self.states]
            return [min(values), max(values)]
        units = []
        for n, output in enumerate(self.outputs):
            bit = 1 << n
            possibilities = {bool((state[4] | state[5]) & bit) for state in self.states}
            meals = [int(bool(state[4] & bit)) for state in self.states]
            transfers = [int(bool(state[5] & bit)) for state in self.states]
            units.append(dict(**output, status='guaranteed_used' if possibilities == {True}
                              else 'guaranteed_unused' if possibilities == {False} else 'ambiguous',
                              consumed=[min(meals), max(meals)],
                              ever_transferred=[min(transfers), max(transfers)]))
        return dict(created=len(self.outputs), consumed=bounds(lambda s: s[4]),
                    ever_transferred=bounds(lambda s: s[5]),
                    ever_used=bounds(lambda s: s[4] | s[5]),
                    terminal_assignments=len(self.states), peak_assignments=self.peak_states,
                    units=units)


def core(history):
    result = history['final_state']
    while 'events' not in result:
        result = result['base']
    return result


def reconstruct(trial):
    assert trial['schema'] == 'experiment-025-trial/v1'
    history, material = trial['history'], trial['material']
    base = core(history)
    fate = FoodFates([actor['food'] for actor in history['initial']])
    raw, cache, waste, consumed = material['initial_raw'], history['initial_stock'], 0, 0
    escrow = [0] * 4
    initial = raw + cache + sum(fate.food)
    assert initial == history['initial_total']
    rows = []
    for n, event in enumerate(base['events']):
        if n >= history['bootstrap_events']:
            rows.append((event['tick'], 1, n, 'transfer', event))
    for n, grant in enumerate(history['grants']):
        rows.append((grant['tick'], 2, n, 'grant', grant))
    for n, meal in enumerate(base['cognition']['consumption']):
        rows.append((meal['tick'], 3, n, 'meal', meal))
    for n, op in enumerate(material['operations']):
        rows.append((op['tick'], 4, n, 'preparation', op))
    for tick, _, index, kind, row in sorted(rows):
        assert history['start'] <= tick <= history['end']
        if kind == 'transfer':
            actors = row['participants']
            assert [fate.food[a] for a in actors] == row['balances_before']
            changes = [a - b for a, b in zip(row['balances_after'], row['balances_before'])]
            quantity = row['transferred']
            assert sum(changes) == 0 and sum(max(0, d) for d in changes) == quantity
            if quantity:
                donor = actors[next(n for n, d in enumerate(changes) if d < 0)]
                recipient = actors[next(n for n, d in enumerate(changes) if d > 0)]
                for _ in range(quantity):
                    fate.debit(donor, recipient)
            assert [fate.food[a] for a in actors] == row['balances_after']
        elif kind == 'grant':
            owner = row['owner']
            assert (cache, fate.food[owner]) == (row['stock_before'], row['food_before'])
            quantity = row['food_after'] - row['food_before']
            assert 0 <= quantity <= cache and row['stock_after'] == cache - quantity
            cache -= quantity
            fate.untagged(owner, quantity)
        elif kind == 'meal':
            owner = row['actor']
            assert fate.food[owner] == row['food_before']
            assert row['consumed'] == row['food_before'] - row['food_after']
            assert row['consumed'] >= 0
            for _ in range(row['consumed']):
                fate.debit(owner)
                consumed += 1
            assert fate.food[owner] == row['food_after']
        else:
            owner = row['owner']
            assert row['source'] == 0 and row['id'] == index
            assert (raw, escrow[owner], fate.food[owner], waste) == (
                row['raw_before'], row['escrow_before'], row['food_before'], row['waste_before'])
            if row['outcome'] == 'Extracted':
                assert raw >= 2 and escrow[owner] == 0
                raw -= 2
                escrow[owner] = 2
            elif row['outcome'] == 'Prepared':
                assert escrow[owner] == 2
                escrow[owner] = 0
                waste += 1
                fate.prepared(owner, f'material.operations[{index}]')
            else:
                assert row['outcome'] in ('Insufficient', 'Failed') and escrow[owner] == 0
                if row['outcome'] == 'Insufficient':
                    assert raw < 2
            assert (raw, escrow[owner], fate.food[owner], waste) == (
                row['raw_after'], row['escrow_after'], row['food_after'], row['waste_after'])
        assert min(raw, cache, waste, consumed, *escrow, *fate.food) >= 0
        assert raw + sum(escrow) + cache + sum(fate.food) + waste + consumed == initial
    assert (raw, waste, escrow) == (material['final_raw'], material['final_waste'], [0] * 4)
    assert fate.food == [actor['food'] for actor in history['final_agents']]
    return dict(**fate.summary(), physical_transitions=len(rows), final_food=fate.food,
                final_raw=raw, final_cache=cache, waste=waste, consumed_total=consumed)


def inspect():
    directory = ROOT / 'experiments/025'
    expected = json.loads((directory / 'archive-manifest.json').read_bytes())['archives']['trials.jsonl.gz']
    archive = directory / 'trials.jsonl.gz'
    assert hashlib.sha256(archive.read_bytes()).hexdigest() == expected['sha256']
    decoded = hashlib.sha256()
    worlds = []
    with gzip.open(archive, 'rb') as stream:
        for ordinal, line in enumerate(stream):
            decoded.update(line)
            assert hashlib.sha256(line.rstrip(b'\n')).hexdigest() == expected['member_sha256'][ordinal]
            worlds.append(dict(ordinal=ordinal, **reconstruct(json.loads(line))))
    assert len(worlds) == expected['records'] == 50
    assert decoded.hexdigest() == expected['decoded_sha256']
    def total(rows):
        result = dict(created=sum(row['created'] for row in rows))
        for name in ('consumed', 'ever_transferred', 'ever_used'):
            # Separate ledgers allow a Cartesian product of tag assignments;
            # this is not a claim of statistical independence between runs.
            result[name] = [sum(row[name][n] for row in rows) for n in (0, 1)]
        for status in ('guaranteed_used', 'guaranteed_unused', 'ambiguous'):
            result[status] = sum(unit['status'] == status for row in rows for unit in row['units'])
        return result
    return dict(scope='Independent all-compatible prepared-unit attribution; observer tags only, not cognitive identities',
                archive_sha256=expected['sha256'], primary_48=total(worlds[:48]),
                all_50_including_placebos=total(worlds), worlds=worlds)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path)
    parser.add_argument('--expected', type=Path)
    args = parser.parse_args()
    result = inspect()
    if args.expected:
        assert result == json.loads(args.expected.read_bytes())
    if args.output:
        with args.output.open('x', encoding='utf-8') as stream:
            json.dump(result, stream, indent=2)
            stream.write('\n')
    print(json.dumps(result['primary_48'], sort_keys=True))


if __name__ == '__main__':
    main()
