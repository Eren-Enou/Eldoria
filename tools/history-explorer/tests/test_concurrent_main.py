"""Main Explorer native022 source, ownership, comparison and HTTP acceptance."""
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import threading
import unittest
from urllib.error import HTTPError
from urllib.request import Request, urlopen

HERE = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('explorer_native022_tests', HERE / 'explorer.py')
explorer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(explorer)
native = explorer.concurrent_history


class NativeConcurrentMain(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.cache = tempfile.TemporaryDirectory(prefix='woi-main022-tests-')
        cls.repo = explorer.Repository(cache=cls.cache.name, experiments=('022',)).catalog()

    @classmethod
    def tearDownClass(cls):
        cls.cache.cleanup()

    def data(self, ordinal):
        entry = next(e for e in self.repo.entries.values() if e['record'] == ordinal)
        return self.repo.get(entry['id'])

    def test_native_original_identity_and_typed_chains(self):
        self.assertEqual(len(self.repo.entries), 100)
        data = self.data(1)
        self.assertEqual(data['extractor_version'], 5)
        self.assertEqual(data['trial']['schema'], native.VERSION)
        self.assertNotIn('periods', data['trial'])
        self.assertNotIn('clock', data['trial'])
        self.assertEqual(data['source']['individual_ids'], [0, 1, 2, 3])
        for node in data['causal']['nodes'].values():
            native.resolve(data['trial'], node['ref'])
        chain = next(c for c in data['causal']['chains'] if c['id'] == 'admissions[0]')
        kinds = [data['causal']['nodes'][ref]['kind'] for ref in chain['nodes']]
        for kind in ('choice', 'invitation', 'reply', 'admission', 'commitment', 'resource', 'release'):
            self.assertIn(kind, kinds)
        self.assertTrue(any(e['kind'] == 'availability_context_not_unique_motive' for e in data['causal']['edges']))

    def test_matched_capacity_comparisons_and_nulls(self):
        totals = dict(food=0, hunger=0, trust=0, nulls=0)
        for ordinal in range(0, 100, 2):
            comparison = native.comparison(self.data(ordinal), self.data(ordinal + 1))
            self.assertTrue(comparison['matched_except_capacity'])
            self.assertTrue(comparison['initial_state_and_profiles_match'])
            self.assertTrue(comparison['equal_horizon'])
            totals['food'] += not comparison['terminal_food_equal']
            totals['hunger'] += not comparison['terminal_hunger_equal']
            totals['trust'] += not comparison['semantic_trust_equal']
            totals['nulls'] += comparison['complete_null_except_capacity']
        self.assertEqual(totals, dict(food=15, hunger=20, trust=31, nulls=9))
        c = native.comparison(self.data(0), self.data(1))
        self.assertEqual(c['first_admission']['tick'], 8)
        self.assertEqual(c['first_executed']['tick'], 9)
        self.assertEqual(c['first_admission']['left_difference']['value']['reason'], 'Capacity')
        self.assertEqual(c['first_admission']['right_difference']['value']['reason'], 'Execute')
        self.assertIsNone(c['first_executed']['left_difference'])
        self.assertEqual(c['first_executed']['right_difference']['value']['participants'], [2, 3])
        self.assertEqual(c['first_opportunity']['tick'], 8)
        self.assertEqual(c['first_common_selected_choice']['tick'], 16)
        self.assertEqual(c['hunger_area_right_minus_left'], [-840, 45, -1350, 100])
        self.assertLessEqual(len(self.repo.loaded), 2)
        self.assertLessEqual(len(self.repo.concurrent.loaded), 2)

    def test_import_rebuilds_views_and_quarantines_changed_trial(self):
        data = copy.deepcopy(self.data(13))
        data['actors'] = [{'invented_future_stock': 999}]
        data['native_concurrency'] = False
        rebuilt = self.repo.imported(data)
        self.assertTrue(rebuilt['native_concurrency'])
        self.assertNotIn('actors', rebuilt)
        data['trial']['initial'][0]['food'] += 1
        with self.assertRaisesRegex(ValueError, 'verified registered source'):
            self.repo.imported(data)

    def test_captured_owner_absence_and_query_response_privacy(self):
        data = self.data(13)
        moment = next(m for m in data['timeline'] if m['kind'] == 'decision' and m['tick'] == 14)
        self.assertIsNone(native.actor(data['trial'], moment, 2))
        self.assertEqual(native.actor(data['trial'], moment, 3)['selected'], 'Acquire')
        data = self.data(18)
        moment = next(m for m in data['timeline'] if m['kind'] == 'need' and m['tick'] == 8)
        self.assertIsNone(native.actor(data['trial'], moment, 0))
        moment = next(m for m in data['timeline'] if m['kind'] == 'inquiry' and m['tick'] == 19)
        owned = native.actor(data['trial'], moment, 0)
        self.assertEqual(owned['input_tick'], 6)
        self.assertNotIn('response', owned)
        self.assertNotIn('concern_after', owned)

    def test_actual_javascript_ownership_projection(self):
        if not shutil.which('node'):
            self.skipTest('Node unavailable')
        with tempfile.TemporaryDirectory(prefix='woi-native022-js-') as directory:
            for ordinal in (0, 1, 13, 18, 24, 26):
                (Path(directory) / f'{ordinal}.json').write_bytes(explorer.encoded(self.data(ordinal)))
            result = subprocess.run(['node', str(HERE / 'tests/native-concurrent.test.cjs'), directory], capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_local_read_only_native_http_and_assets(self):
        server = explorer.ThreadingHTTPServer(('127.0.0.1', 0), explorer.Handler)
        server.repo = self.repo
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        address = f'http://127.0.0.1:{server.server_port}'
        try:
            entry = next(e for e in self.repo.entries.values() if e['record'] == 1)
            with urlopen(address + '/api/trial?id=' + entry['id']) as response:
                data = json.load(response)
            self.assertTrue(data['native_concurrency'])
            with urlopen(address + '/native-concurrent.js') as response:
                self.assertIn(b'const NativeConcurrent', response.read())
            with self.assertRaises(HTTPError) as error:
                urlopen(Request(address + '/api/run', data=b'{}', headers={'Content-Type': 'application/json'}))
            self.assertEqual(error.exception.code, 403)
            with self.assertRaises(HTTPError) as error:
                urlopen(Request(address + '/api/catalog', headers={'Origin': 'https://foreign.invalid'}))
            self.assertEqual(error.exception.code, 403)
        finally:
            server.shutdown(); server.server_close(); thread.join(timeout=5)

    def test_changed_source_rejected_without_touching_originals(self):
        with tempfile.TemporaryDirectory(prefix='woi-native022-source-') as directory:
            root = Path(directory).resolve()
            self.assertEqual(root.parent, Path(tempfile.gettempdir()).resolve())
            source = explorer.ROOT
            paths = set(json.loads((source / 'experiments/022/source-freeze.json').read_bytes()))
            paths.update(p.relative_to(source).as_posix() for p in (source / 'experiments/022').iterdir() if p.is_file())
            paths.add('docs/foundation-001-022-sha256.json')
            for name in paths:
                original = source / name
                if not original.exists():
                    continue
                target = root / name; target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(original, target)
            target = root / 'src/model.rs'
            target.write_bytes(target.read_bytes() + b'\n// scratch source corruption\n')
            with self.assertRaisesRegex(ValueError, 'source lock changed'):
                native.Source(root).catalog('Scratch only')


if __name__ == '__main__':
    unittest.main()
