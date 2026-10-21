"""Actual025sources: material phases, privacy, lossless references and old-main integration."""
import copy
import importlib.util
import json
import shutil
import subprocess
import tempfile
import threading
import unittest
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen

HERE = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('main025_tests', HERE / 'explorer.py')
e = importlib.util.module_from_spec(spec)
spec.loader.exec_module(e)


def resolve(data, ref):
    root = (data['preparation'] if ref.startswith('material.') else
            data['valuation'] if ref.startswith('valuations[') else
            data['continuous'] if ref.startswith('checkpoints[') else data['trial'])
    return e.concurrent_history.resolve(root, ref)


class PreparationMain(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix='woi-main025-')
        cls.repo = e.Repository(cache=cls.temp.name, experiments=(),
                               preparations=(e.ROOT / 'experiments/025',)).catalog()

    @classmethod
    def tearDownClass(cls):
        cls.temp.cleanup()

    def data(self, ordinal):
        return self.repo.get(next(key for key, value in self.repo.entries.items()
                                 if value['record'] == ordinal))

    def test_all_fifty_originals_roundtrip_and_all_links_resolve(self):
        self.assertEqual(len(self.repo.entries), 50)
        for ordinal in range(50):
            data = self.data(ordinal)
            self.assertEqual(data['extractor_version'], 9)
            self.assertEqual(data['source']['source_schema'], 'experiment-025-trial/v1')
            self.assertEqual(data['trial']['schema'], 'experiment-025-physical-history/v1')
            self.assertEqual(e.digest(e.preparation_history.rebuild(data)), data['source']['trial_fingerprint'])
            self.assertEqual([moment['id'] for moment in data['timeline']], list(range(len(data['timeline']))))
            for moment in data['timeline']:
                resolve(data, moment['ref'])
            ops = [moment['ref'] for moment in data['timeline'] if moment['kind'] == 'preparation']
            self.assertEqual(ops, [f'material.operations[{n}]' for n in range(len(data['preparation']['material']['operations']))])
            positions = {moment['ref']: n for n, moment in enumerate(data['timeline'])}
            for n, receipt in enumerate(data['preparation']['material']['memories']):
                if receipt['cache_record'] is None:
                    continue
                raw, ready = positions[f'material.memories[{n}]'], positions[f'memories[{receipt["cache_record"]}]']
                self.assertLess(raw, ready) if receipt['kind'] == 'OverwrittenByReady' else self.assertLess(ready, raw)
            for ref in data['causal']['nodes']:
                resolve(data, ref)
            for edge in data['causal']['edges']:
                resolve(data, edge['source']); resolve(data, edge['target'])
            for point in data['observer_physical']:
                self.assertEqual(point['raw'] + sum(point['escrow']) + point['stock'] + sum(point['food'])
                                 + point['waste'] + point['consumed'], data['trial']['initial_total'])
        self.assertLessEqual(len(self.repo.loaded), 2)
        self.assertLessEqual(len(self.repo.preparations[0].loaded), 2)

    def test_all_primary_contrasts_match_separate_scientific_findings(self):
        findings = json.loads((e.ROOT / 'experiments/025/independent-findings.json').read_bytes())
        for pair in findings['pairs']:
            left, right = map(self.data, pair['ordinals'])
            result = e.preparation_history.comparison(left, right)
            self.assertTrue(result['matched_except_access'])
            self.assertTrue(result['access_contrast'])
            self.assertTrue(result['initial_state_and_profiles_match'])
            self.assertEqual(result['first_full_material'], pair['full_material']['first'])
            self.assertEqual(result['full_material_reconvergence'], pair['full_material']['reconvergence'])
            self.assertEqual(result['terminal_full_material_equal'], not pair['full_material']['terminal'])
            self.assertEqual(result['first_material'], pair['edible_holdings']['first'])
            self.assertEqual(result['material_reconvergence'], pair['edible_holdings']['reconvergence'])
            self.assertEqual(result['terminal_food_equal'], not pair['edible_holdings']['terminal'])
            self.assertEqual(result['terminal_hunger_equal'], not pair['terminal_hunger_difference'])
            self.assertEqual(result['semantic_trust_equal'], not pair['terminal_trust_difference'])
            self.assertEqual(result['hunger_area_right_minus_left'], pair['hunger_area_delta'])
            self.assertEqual(None if result['first_executed'] is None else result['first_executed']['tick'], pair['first_completed_effect'])
        null = e.preparation_history.comparison(self.data(12), self.data(13))
        self.assertIsNone(null['first_executed'])
        # The received accessibility flag differs even though no action changes.
        self.assertFalse(null['full_state_trace_null'])

    def test_actual_success_failure_and_conservation_compartments(self):
        data = self.data(1)
        at = {point['tick']: point for point in data['observer_physical']}
        self.assertEqual((at[2]['raw'], at[2]['escrow'], at[2]['food']), (0, [2] * 4, [0] * 4))
        self.assertEqual((at[6]['waste'], at[6]['escrow'], at[6]['food']), (4, [0] * 4, [1] * 4))
        self.assertEqual(at[7]['consumed'], 4)
        source = e.preparation_history.rebuild(data)
        operations = source['material']['operations']
        failed = next(op for op in operations if op['outcome'] == 'Insufficient' and op['owner'] == 0)
        span = next(span for span in data['intervals'] if span['id'] == failed['commitment'])
        self.assertEqual((span['start'], failed['tick'], span['end']), (7, 9, 13))
        chain = next(chain for chain in data['causal']['chains'] if chain['id'] == 'prepare:0')
        self.assertIn('material.operations[0]', chain['nodes'])
        self.assertIn('material.operations[4]', chain['nodes'])
        self.assertTrue(any('consumption[0]' in ref for ref in chain['nodes']))
        self.assertIn('boundaries[6].choices[0]', chain['nodes'])
        self.assertIn('boundaries[7].choices[0]', chain['nodes'])
        self.assertIn('boundaries[13].choices[0]', chain['nodes'])
        self.assertEqual(resolve(data, 'material.operations[0]')['raw_after'], 6)
        self.assertEqual(at[2]['raw'], 0)  # End-of-tick is distinct from the first operation's after-state.

    def test_import_ignores_derived_poison_but_rejects_material_changes(self):
        data = copy.deepcopy(self.data(1))
        data['observer_physical'] = [{'invented': 9999}]
        data['timeline'] = []; data['causal'] = {}
        restored = self.repo.imported(data)
        self.assertNotIn('invented', restored['observer_physical'][0])
        bad = copy.deepcopy(restored)
        bad['preparation']['material']['final_raw'] = 9999
        with self.assertRaises(ValueError):
            self.repo.imported(bad)

    def test_bounded_schema_score_and_conversion_falsification(self):
        original = e.preparation_history.rebuild(self.data(1))
        for mutate in (lambda x: x.update(schema='experiment-024-trial/v1'),
                       lambda x: x['config']['world'].update(horizon=10**9),
                       lambda x: x['valuations'][0]['selection']['terms'][4].update(work_effort=0),
                       lambda x: x['material']['operations'][0].update(escrow_after=1),
                       lambda x: x['material']['operations'][4].update(waste_after=0)):
            bad = copy.deepcopy(original); mutate(bad)
            with self.assertRaises(ValueError):
                e.preparation_history.validate(bad)

    def test_actual_javascript_raw_privacy_after_forgetting_and_source_paths(self):
        if not shutil.which('node'):
            self.skipTest('Node unavailable')
        with tempfile.TemporaryDirectory(prefix='woi-main025-js-') as directory:
            for ordinal in (0, 1, 15, 17, 19, 27):
                (Path(directory) / f'{ordinal}.json').write_bytes(e.encoded(self.data(ordinal)))
            result = subprocess.run(['node', str(HERE / 'tests/preparation-native.test.cjs'), directory], capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_http_uses025_comparison_and_remains_read_only(self):
        server = e.ThreadingHTTPServer(('127.0.0.1', 0), e.Handler); server.repo = self.repo
        thread = threading.Thread(target=server.serve_forever, daemon=True); thread.start()
        try:
            left, right = self.data(0), self.data(1)
            address = f'http://127.0.0.1:{server.server_port}'
            with urlopen(address + '/api/compare?left=' + left['source']['id'] + '&right=' + right['source']['id']) as response:
                result = json.load(response)
            self.assertTrue(result['preparation_treatment'])
            self.assertEqual(result['first_full_material'], 2)
            with self.assertRaises(HTTPError) as error:
                urlopen(Request(address + '/api/run', data=b'{}'))
            self.assertEqual(error.exception.code, 403)
        finally:
            server.shutdown(); server.server_close(); thread.join(timeout=5)


if __name__ == '__main__':
    unittest.main()
