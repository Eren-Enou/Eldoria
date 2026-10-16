"""Ground-truth linkage tests from immutable archives, without prepared UI caches."""
import copy
import gzip
import importlib.util
import json
import sys
from pathlib import Path
import unittest

HOME = Path(__file__).resolve().parents[1]
ROOT = HOME.parents[1]
spec = importlib.util.spec_from_file_location('causal', HOME / 'causal.py')
c = importlib.util.module_from_spec(spec)
spec.loader.exec_module(c)


class CausalArchiveTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = {}
        for name, nums in [('seed-42.jsonl.gz', {7, 10, 43, 55, 85, 95, 103, 107}), ('held-out-seed-42.jsonl.gz', {7, 10, 182, 183})]:
            with gzip.open(ROOT / 'experiments/019' / name, 'rt') as f:
                for n, line in enumerate(f, 1):
                    if n in nums:
                        cls.raw[(name, n)] = json.loads(line)

    def data(self, n=103, held=False):
        t = self.raw[('held-out-seed-42.jsonl.gz' if held else 'seed-42.jsonl.gz', n)]
        return t, c.build(t, f'real:{n}:{held}')

    def test_original_members_and_all_reference_edges(self):
        for identity, t in self.raw.items():
            d = c.build(t, str(identity))
            for key, n in d['nodes'].items():
                self.assertEqual(n['trial'], str(identity))
                self.assertEqual(key, n['ref'])
                c.resolve(t, n['ref'])
            for ch in d['chains']:
                for e in ch['edges']:
                    self.assertIn(e['source'], d['nodes'])
                    self.assertIn(e['target'], d['nodes'])

    def test_meal_consent_and_actual_scene_direction(self):
        _, d = self.data()
        group = d['chains'][0]
        kinds = [d['nodes'][n]['source_kind'] for n in group['nodes']]
        self.assertIn('Executed meal', kinds)
        self.assertIn('Recipient response', kinds)
        self.assertTrue(any('meal values match' in e['relation'] for e in group['edges']))
        self.assertEqual(d['encounters'][0]['participants'], [1, 0])
        self.assertEqual(d['encounters'][0]['actions'], ['Offer', 'Accept'])
        self.assertEqual(d['encounters'][0]['transferred'], 1)
        self.assertTrue(any('memory-cause' in e['relation'] for ch in d['chains'] for e in ch['edges']))
        _, scaled = self.data(107)
        self.assertEqual(scaled['encounters'][0]['participants'], [0, 1])
        self.assertEqual(scaled['encounters'][0]['actions'], ['Request', 'Offer', 'Accept'])

    def test_count_encounters_not_actions_or_allocations(self):
        t, d = self.data(85)
        actual = [e for e in d['encounters'] if e['end'] > 258 and e['completed'] and not e['transferred']]
        self.assertEqual(len(actual), 65)
        self.assertEqual({tuple(e['actions']) for e in actual}, {('Request', 'Leave')})
        self.assertEqual({e['outcome'] for e in actual}, {'Withdrawal'})
        self.assertEqual(len([e for e in c.resolve(t, c.BASE)['events'] if e['tick'] > 258]), 130)
        self.assertEqual(len([n for n in d['nodes'].values() if n['source_kind'] == 'Arbitration' and n['tick'] > 258]), 195)

    def test_missed_query_and_foreign_actual_edge(self):
        _, d = self.data(7, True)
        group = next(ch for ch in d['chains'] if ch['id'] == 'periods[0].steps[3].activity')
        self.assertIsNone(group['query'])
        self.assertFalse(any(d['nodes'][n]['source_kind'] == 'Executed inquiry' for n in group['nodes']))
        self.assertTrue(any(d['nodes'][n]['outcome'] == 'Contention' for n in group['nodes']))
        responses = [d['nodes'][n] for n in group['nodes'] if d['nodes'][n]['source_kind'] == 'Recipient response']
        self.assertTrue(any(n['actor'] == 2 and n['recipient'] == 0 and n['outcome'] == {'Accept': 0} for n in responses))
        _, deadline = self.data(183, True)
        group = next(ch for ch in deadline['chains'] if ch['tick'] == 16)
        executions = [deadline['nodes'][n] for n in group['nodes'] if deadline['nodes'][n]['source_kind'] == 'Actual resource execution']
        self.assertEqual([(n['actor'], n['recipient']) for n in executions], [(2, 0)])

    def test_null_native_and_real_provenance_are_distinct(self):
        _, d = self.data(10, True)
        queries = [n for n in d['nodes'].values() if n['source_kind'] == 'Executed inquiry']
        self.assertIsNone(queries[0]['outcome']['native_response'])
        self.assertEqual(queries[0]['outcome']['status_after'], 'Resolved')
        receipts = [n for n in d['nodes'].values() if n['source_kind'] == 'Provenance delivery']
        self.assertEqual([n['outcome']['receipt'] for n in receipts], [1])
        assessments = [n for n in d['nodes'].values() if n['source_kind'] == 'Retained assessment update']
        self.assertEqual([(n['outcome']['before'], n['outcome']['support']) for n in assessments], [(20, 60)])
        _, empty = self.data(7, True)
        self.assertFalse(any(n['source_kind'] == 'Provenance delivery' for n in empty['nodes'].values()))

    def test_wrong_redundant_information_not_automatically_useful(self):
        _, wrong = self.data(43)
        outcomes = [n['outcome'] for n in wrong['nodes'].values() if n['source_kind'] == 'Observed learning consequence']
        self.assertTrue(any(o['support_before'] == 20 and o['support_after'] == -30 for o in outcomes))
        _, redundant = self.data(55)
        outcomes = [n['outcome'] for n in redundant['nodes'].values() if n['source_kind'] == 'Observed learning consequence']
        self.assertTrue(any(o['support_before'] == o['support_after'] == 10 for o in outcomes))

    def test_missing_and_forged_references_rejected(self):
        t, _ = self.data()
        for ref in ['points[999999]', '__proto__', 'points[-1]', 'points[0];alert(1)']:
            with self.assertRaises(ValueError):
                c.resolve(t, ref)
        bad = copy.deepcopy(t)
        bad['periods'][0]['steps'][0]['activity']['started']['scene'] = 999999
        with self.assertRaises(ValueError):
            c.build(bad, 'scratch')
        bad = copy.deepcopy(self.raw[('held-out-seed-42.jsonl.gz', 10)])
        c.resolve(bad, c.PROV)['queries'][0]['receipt'] = 0  # another listener, same numeric namespace
        with self.assertRaises(ValueError):
            c.build(bad, 'scratch')

    def test_null_and_reconvergence_do_not_get_new_meanings(self):
        _, a = self.data(7)
        _, b = self.data(10)
        self.assertEqual(a['encounters'], b['encounters'])
        a, _ = self.data(182, True)
        b, _ = self.data(183, True)
        self.assertEqual(c.resolve(a, c.BASE)['agents'], c.resolve(b, c.BASE)['agents'])
        self.assertNotEqual(a['final_state'], b['final_state'])


if __name__ == '__main__':
    if '--all-sources' in sys.argv:
        count = nodes = 0
        for experiment in ('018', '019'):
            directory = ROOT / 'experiments' / experiment
            for name in json.loads((directory / 'archive-manifest.json').read_text()):
                if not name.endswith('.jsonl.gz'):
                    continue
                with gzip.open(directory / name, 'rt') as f:
                    for ordinal, line in enumerate(f, 1):
                        d = c.build(json.loads(line), f'{experiment}/{name}/{ordinal}')
                        count += 1
                        nodes += len(d['nodes'])
        assert count == 602
        print(f'All-source causal scan PASS: {count} histories, {nodes} nodes; no files written')
    else:
        unittest.main()
