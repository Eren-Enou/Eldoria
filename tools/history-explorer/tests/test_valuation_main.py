"""Actual024 originals: explicit schema, local terms, comparison and source guards."""
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
spec = importlib.util.spec_from_file_location('main024_tests', HERE / 'explorer.py')
e = importlib.util.module_from_spec(spec)
spec.loader.exec_module(e)


class ValuationMain(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix='woi-main024-')
        cls.repo = e.Repository(cache=cls.temp.name, experiments=(),
                                valuations=(e.ROOT / 'experiments/024',)).catalog()

    @classmethod
    def tearDownClass(cls):
        cls.temp.cleanup()

    def data(self, ordinal):
        return self.repo.get(next(k for k, v in self.repo.entries.items()
                                 if v['record'] == ordinal))

    def test_all_originals_have_lossless_explicit_schema_and_references(self):
        self.assertEqual(len(self.repo.entries), 56)
        for ordinal in range(56):
            data = self.data(ordinal)
            self.assertEqual(data['extractor_version'], 8)
            self.assertEqual(data['source']['source_schema'], 'experiment-024-trial/v1')
            self.assertEqual(data['trial']['schema'], 'experiment-024-physical-history/v1')
            self.assertNotIn('periods', data['trial'])
            self.assertEqual(e.digest(e.valuation_history.rebuild(data)),
                             data['source']['trial_fingerprint'])
            self.assertEqual([m['id'] for m in data['timeline']], list(range(len(data['timeline']))))
            for moment in data['timeline']:
                root = data['continuous'] if moment['kind'] == 'checkpoint' else data['trial']
                e.concurrent_history.resolve(root, moment['ref'])
            for audit in data['valuation']['valuations']:
                decision = data['trial']['boundaries'][audit['boundary']]['choices'][audit['choice']]
                self.assertEqual(audit['selection']['selected'], decision['selected'])

    def test_all_matched_comparisons_reproduce_separate_science(self):
        scientific = json.loads((e.ROOT / 'experiments/024/comparisons.json').read_bytes())
        for pair in scientific['pairs']:
            left, right = map(self.data, pair['ordinals'])
            result = e.valuation_history.comparison(left, right)
            self.assertTrue(result['matched_except_valuation'])
            self.assertTrue(result['valuation_contrast'])
            self.assertTrue(result['initial_state_and_profiles_match'])
            self.assertEqual(result['terminal_food_equal'], not pair['terminal_food_difference'])
            self.assertEqual(result['terminal_hunger_equal'], not pair['terminal_hunger_difference'])
            self.assertEqual(result['semantic_trust_equal'], not pair['terminal_trust_difference'])
            self.assertEqual(result['hunger_area_right_minus_left'], pair['hunger_area_delta'])
            self.assertEqual(result['full_state_trace_null'], pair['full_history_body_equal'])
            first = result['first_common_selected_choice']
            self.assertEqual(None if first is None else first['tick'],
                             None if pair['first_selected'] is None else pair['first_selected']['tick'])
        self.assertLessEqual(len(self.repo.loaded), 2)
        self.assertLessEqual(len(self.repo.valuations[0].loaded), 2)

    def test_deferral_requires_eligible_meal_owned_food_and_actual_time(self):
        data = self.data(18)
        self.assertTrue(data['valuation_deferrals'])
        changed_hunger = False
        for row in data['valuation_deferrals']:
            decision = e.concurrent_history.resolve(data['trial'], row['ref'])
            own = decision['input']['legacy']['own']
            self.assertGreater(own['food'], 0)
            self.assertNotEqual(decision['selected'], 'Consume')
            self.assertTrue(next(c for c in decision['candidates'] if c['action'] == 'Consume')['eligible'])
            start = e.concurrent_history.resolve(data['trial'], row['execution_ref'])
            end = e.concurrent_history.resolve(data['trial'], row['release_ref'])
            self.assertIn(row['owner'], start['commitment']['participants'])
            self.assertGreater(end['tick'], start['tick'])
            changed_hunger |= row['hunger_after_completion_tick'] != row['hunger']
        self.assertTrue(changed_hunger)
        self.assertEqual(self.data(20)['valuation_deferrals'], [])

    def test_import_rebuilds_presentation_and_quarantines_changed_original(self):
        data = copy.deepcopy(self.data(1))
        data['valuation_deferrals'] = [dict(invented_future=999)]
        data['actors'] = [dict(private_partner=99)]
        data['native_concurrency'] = False
        fresh = self.repo.imported(data)
        self.assertTrue(fresh['native_concurrency'])
        self.assertNotIn('actors', fresh)
        self.assertNotEqual(fresh['valuation_deferrals'], data['valuation_deferrals'])
        data['valuation']['valuations'][0]['selection']['terms'][1]['meal_relief'] += 1
        with self.assertRaisesRegex(ValueError, 'quarantined'):
            self.repo.imported(data)

    def test_wrong_schema_bonus_reference_and_missing_audit_rejected(self):
        data = self.data(1)
        original = e.valuation_history.rebuild(data)
        for mutate in (lambda x: x.update(schema='experiment-023-trial/v1'),
                       lambda x: x['valuations'][0].update(boundary=99999),
                       lambda x: x['valuations'].pop(),
                       lambda x: x['valuations'][0]['selection']['terms'][1].update(meal_relief=999)):
            bad = copy.deepcopy(original)
            mutate(bad)
            with self.assertRaises(ValueError):
                e.valuation_history.validate(bad)

    def test_actual_javascript_owned_terms_privacy_and_original_paths(self):
        if not shutil.which('node'):
            self.skipTest('Node unavailable')
        with tempfile.TemporaryDirectory(prefix='woi-main024-js-') as directory:
            for ordinal in (0, 1, 12, 18, 19, 20):
                (Path(directory) / f'{ordinal}.json').write_bytes(e.encoded(self.data(ordinal)))
            out = subprocess.run(['node', str(HERE / 'tests/valuation-native.test.cjs'), directory],
                                 capture_output=True, text=True)
            self.assertEqual(out.returncode, 0, out.stdout + out.stderr)

    def test_http_comparison_and_write_rejection(self):
        server = e.ThreadingHTTPServer(('127.0.0.1', 0), e.Handler)
        server.repo = self.repo
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        left, right = self.data(0), self.data(1)
        address = f'http://127.0.0.1:{server.server_port}'
        try:
            with urlopen(address + '/api/compare?left=' + left['source']['id'] + '&right=' + right['source']['id']) as response:
                result = json.load(response)
            self.assertTrue(result['valuation_treatment'])
            self.assertEqual(result['first_executed']['tick'], 19)
            with self.assertRaises(HTTPError) as error:
                urlopen(Request(address + '/api/run', data=b'{}'))
            self.assertEqual(error.exception.code, 403)
        finally:
            server.shutdown()
            server.server_close()
            thread.join(timeout=5)

    def test_changed_source_lock_is_rejected_in_scratch(self):
        with tempfile.TemporaryDirectory(prefix='woi-024-source-') as directory:
            root = Path(directory)
            paths = set(json.loads((e.ROOT / 'experiments/024/source-freeze.json').read_bytes()))
            paths.update('experiments/024/' + name for name in
                         ('source-freeze.json', 'archive-manifest.json', 'trials.jsonl.gz', 'registered-matrix.json'))
            for name in paths:
                target = root / name
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(e.ROOT / name, target)
            changed = root / 'prototypes/activity-valuation/src/model.rs'
            changed.write_bytes(changed.read_bytes() + b'\n// scratch corruption\n')
            with self.assertRaisesRegex(ValueError, 'source lock changed'):
                e.valuation_history.Source(root, root / 'experiments/024').catalog('scratch')


if __name__ == '__main__':
    unittest.main()
