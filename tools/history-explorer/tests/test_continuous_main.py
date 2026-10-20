"""Actual023source, lossless projection, checkpoints and readonly main routes."""
import copy,hashlib,importlib.util,json,shutil,subprocess,tempfile,threading,unittest
from pathlib import Path
from urllib.request import urlopen,Request
from urllib.error import HTTPError
HERE=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('main023_tests',HERE/'explorer.py');e=importlib.util.module_from_spec(spec);spec.loader.exec_module(e)
class ContinuousMain(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp=tempfile.TemporaryDirectory(prefix='woi-main023-')
        cls.repo=e.Repository(cache=cls.temp.name,experiments=(),continuous=(e.ROOT/'experiments/023',)).catalog()
    @classmethod
    def tearDownClass(cls):cls.temp.cleanup()
    def data(self,n):return self.repo.get(next(k for k,v in self.repo.entries.items() if v['record']==n))
    def test_timeline_ids_follow_final_checkpoint_order(self):
        # The main event list calls go(moment.id); IDs must address the final
        # sorted timeline, including inserted observer checkpoints.
        for ordinal in range(42):
            data=self.data(ordinal)
            self.assertEqual([m.get('id') for m in data['timeline']],list(range(len(data['timeline']))),ordinal)
            for moment in data['timeline']:
                self.assertEqual(data['timeline'][moment['id']]['ref'],moment['ref'])
    def test_explicit_schema_and_lossless_original(self):
        self.assertEqual(len(self.repo.entries),42);data=self.data(1)
        self.assertEqual(data['extractor_version'],7);self.assertEqual(data['source']['source_schema'],'experiment-023-trial/v1')
        self.assertEqual(data['trial']['schema'],'experiment-023-physical-history/v1')
        self.assertNotIn('periods',data['trial']);self.assertNotIn('clock',data['trial'])
        self.assertEqual(e.digest(e.continuous_history.rebuild(data)),data['source']['trial_fingerprint'])
        for moment in data['timeline']:
            root=data['continuous'] if moment['kind']=='checkpoint' else data['trial']
            e.concurrent_history.resolve(root,moment['ref'])
        self.assertTrue(any(m['kind']=='checkpoint' for m in data['timeline']))
        for owner in range(4):
            m=next(m for m in data['timeline'] if m['kind']=='checkpoint')
            self.assertIsNone(e.concurrent_history.actor(data['trial'],m,owner))
    def test_all_registered_comparisons_match_separate_science(self):
        report=json.loads((e.ROOT/'experiments/023/comparisons.json').read_bytes())
        for item in report['pairs']:
            a,b=map(self.data,item['ordinals']);c=e.continuous_history.comparison(a,b)
            self.assertTrue(c['matched_except_mode']);self.assertTrue(c['initial_state_and_profiles_match'])
            self.assertEqual(c['terminal_food_equal'],item['terminal_food_equal'])
            self.assertEqual(c['terminal_hunger_equal'],item['terminal_hunger_equal'])
            self.assertEqual(c['semantic_trust_equal'],item['terminal_semantic_trust_equal'])
            self.assertEqual(c['full_state_trace_null'],item['selected_execution_state_null_ignoring_public_end_only'])
            self.assertEqual(c['hunger_area_right_minus_left'],item['hunger_area_continuous_minus_windowed'])
        self.assertLessEqual(len(self.repo.loaded),2);self.assertLessEqual(len(self.repo.continuous[0].loaded),2)
    def test_import_ignores_derived_poison_and_rejects_original_changes(self):
        data=copy.deepcopy(self.data(13));data['actors']=[dict(invented_future=999)];data['native_concurrency']=False
        fresh=self.repo.imported(data);self.assertTrue(fresh['native_concurrency']);self.assertNotIn('actors',fresh)
        data['continuous']['checkpoints'][0]['tick']+=1
        with self.assertRaisesRegex(ValueError,'quarantined'):self.repo.imported(data)
        with self.assertRaises(ValueError):self.repo.imported([])
    def test_javascript_actual023_owned_and_checkpoint_projection(self):
        if not shutil.which('node'):self.skipTest('Node unavailable')
        with tempfile.TemporaryDirectory(prefix='woi-main023-js-') as directory:
            for n in (1,13,19,21):(Path(directory)/f'{n}.json').write_bytes(e.encoded(self.data(n)))
            out=subprocess.run(['node',str(HERE/'tests/continuous-native.test.cjs'),directory],capture_output=True,text=True)
            self.assertEqual(out.returncode,0,out.stdout+out.stderr)
    def test_http_uses023_comparison_and_rejects_write(self):
        server=e.ThreadingHTTPServer(('127.0.0.1',0),e.Handler);server.repo=self.repo
        thread=threading.Thread(target=server.serve_forever,daemon=True);thread.start()
        address=f'http://127.0.0.1:{server.server_port}';a,b=self.data(0),self.data(1)
        try:
            with urlopen(address+'/api/compare?left='+a['source']['id']+'&right='+b['source']['id']) as f:response=json.load(f)
            self.assertTrue(response['opportunity_treatment']);self.assertFalse(response['capacity_contrast'])
            with self.assertRaises(HTTPError) as error:urlopen(Request(address+'/api/run',data=b'{}'))
            self.assertEqual(error.exception.code,403)
        finally:server.shutdown();server.server_close();thread.join(timeout=5)
    def test_changed_runtime_source_rejected_in_scratch(self):
        with tempfile.TemporaryDirectory(prefix='woi-023-source-') as directory:
            root=Path(directory);paths=set(json.loads((e.ROOT/'experiments/023/source-freeze.json').read_bytes()))
            paths.update('experiments/023/'+n for n in ('source-freeze.json','archive-manifest.json','trials.jsonl.gz'))
            for name in paths:
                target=root/name;target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(e.ROOT/name,target)
            changed=root/'prototypes/continuous-world/src/coordinator.rs';changed.write_bytes(changed.read_bytes()+b'\n// scratch-only corruption\n')
            with self.assertRaisesRegex(ValueError,'source lock changed'):e.continuous_history.Source(root,root/'experiments/023').catalog('scratch')
    def test_provenance_delivery_is_distinct_from_null_native_response(self):
        data=self.data(13);chain=next(c for c in data['causal']['chains'] if c['id']=='admissions[0]')
        refs=chain['nodes'];pref='final_state.base.base.base.provenance.receipts[1]'
        self.assertIn(pref,refs)
        self.assertIn('final_state.base.base.base.base.base.cognition.information[0]',refs)
        self.assertIsNone(e.concurrent_history.resolve(data['trial'],'final_state.base.base.base.base.inquiry.records[0]')['response'])
        self.assertEqual(e.concurrent_history.resolve(data['trial'],pref)['cognition_receipt'],0)
        for ref in refs:e.concurrent_history.resolve(data['trial'],ref)
    def test_actual_custom_demo_has_distinct_status_and_member_guard(self):
        original=e.ROOT/'target/first-continuous-world-demo.jsonl'
        if not original.exists():self.skipTest('Optional actual CLI demo not available')
        with tempfile.TemporaryDirectory(prefix='woi-023-demo-') as directory:
            source=Path(directory)/'actual-demo.jsonl';shutil.copyfile(original,source)
            repo=e.Repository(cache=Path(directory)/'cache',experiments=(),continuous=(source,)).catalog()
            self.assertEqual(len(repo.entries),1);entry=next(iter(repo.entries.values()));data=repo.get(entry['id'])
            self.assertFalse(data['source']['registered']);self.assertIn('Custom deterministic',data['source']['scientific_status'])
            source.write_bytes(source.read_bytes()+b'\n')
            with self.assertRaisesRegex(ValueError,'source changed'):repo.get(entry['id'])
if __name__=='__main__':unittest.main()
