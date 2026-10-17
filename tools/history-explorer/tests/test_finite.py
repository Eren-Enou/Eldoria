"""Explicit original020 integration: locality, references, contention and comparison."""
import copy
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

HOME = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('finite_explorer', HOME/'explorer.py')
v = importlib.util.module_from_spec(spec); spec.loader.exec_module(v)


class FiniteTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp=tempfile.TemporaryDirectory(prefix='woi020-main-')
        cls.repo=v.Repository(cache=Path(cls.temp.name),experiments=('020',)).prepare()
        cls.original={p:hashlib.sha256(p.read_bytes()).hexdigest() for p in (v.ROOT/'experiments/020').iterdir() if p.is_file()}

    @classmethod
    def tearDownClass(cls):
        assert all(hashlib.sha256(p.read_bytes()).hexdigest()==h for p,h in cls.original.items())
        cls.temp.cleanup()

    def data(self, ordinal):
        e=next(e for e in self.repo.entries.values() if e['record']==ordinal)
        return self.repo.get(e['id'])

    def test_all_original_schema_and_typed_references(self):
        self.assertEqual(len(self.repo.entries),164)
        with gzip.open(v.ROOT/'experiments/020/trials.jsonl.gz','rt',encoding='utf-8') as f:
            for ordinal,line in enumerate(f):
                t=json.loads(line); v.validate_trial(t,'020')
                e=next(e for e in self.repo.entries.values() if e['record']==ordinal)
                graph=v.finite.causal(t,e['id'],v.causal.build,v.causal.resolve)
                moments=v.finite.timeline(t,v.timeline); anchors={m['ref'] for m in moments}
                for n in graph['nodes'].values():
                    v.causal.resolve(t,n['ref'])
                    self.assertTrue(n['anchor'] in anchors or n['tick'] < t['clock']['epoch'],n)
                for c in graph['chains']:
                    for edge in c['edges']:
                        self.assertIn(edge['source'],graph['nodes']); self.assertIn(edge['target'],graph['nodes'])
                self.assertEqual([m['tick'] for m in moments],sorted(m['tick'] for m in moments))

    def test_actor_acquisition_input_has_no_future_or_private_join(self):
        d=self.data(33); a=d['actors'][0][1]
        self.assertEqual(a['selected'],'Acquire'); self.assertEqual(a['cache']['stock'],1)
        self.assertEqual(a['cache']['observed_at'],0)
        t=copy.deepcopy(d['trial'])
        t['records'][1]['outcome']='Granted';t['final_point']['cache']['stock']=99
        t['final_state']['base']['base']['base']['base']['base']['agents'][0]['food']=999
        self.assertEqual(a,v.finite.actor(t,t['periods'][0]['steps'][0],1,v.actor_view))
        self.assertNotIn('pending',a); self.assertNotIn('records',a)
        self.assertEqual(a['own']['id'],1)

    def test_admission_completion_and_lost_response(self):
        d=self.data(29);t=d['trial'];graph=d['causal']
        self.assertEqual(t['records'][1]['outcome'],'Depleted')
        self.assertTrue(any(n['source_kind']=='Busy response omission' and n['actor']==0 for n in graph['nodes'].values()))
        self.assertEqual(t['periods'][0]['steps'][0]['activity']['allocations'][0]['disposition'],'Unavailable')
        self.assertEqual(t['records'][1]['attempt']['end']-t['records'][1]['attempt']['start'],4)

    def test_020_comparison_includes_acquisition_and_placement(self):
        a=self.data(31)['trial'];b=self.data(33)['trial']
        result=v.finite.comparison(a,b,v.comparison)
        self.assertEqual(result['first_choice']['tick'],0)
        self.assertEqual(result['first_executed_difference']['right']['kind'],'claim')
        self.assertEqual(result['first_executed_difference']['right']['tick'],0)
        self.assertTrue(result['potential_supply_equal']); self.assertTrue(result['owned_placement_equal'])
        self.assertTrue(result['owned_final_material_equal'])
        self.assertFalse(result['final_cache_stock_equal'])
        self.assertFalse(result['final_material_equal'])
        owned=self.data(30)['trial']
        self.assertFalse(v.finite.comparison(owned,b,v.comparison)['owned_placement_equal'])
        ignored=self.data(32)['trial']
        control=v.finite.comparison(a,ignored,v.comparison)
        self.assertTrue(control['legacy_final_snapshot_equal'])
        self.assertFalse(control['full_final_snapshot_equal'])
        reconvergent=v.finite.comparison(self.data(8)['trial'],self.data(9)['trial'],v.comparison)
        self.assertTrue(reconvergent['material_reconvergence'])
        lost=v.finite.comparison(self.data(26)['trial'],self.data(29)['trial'],v.comparison)
        self.assertEqual(lost['model_hunger_area']['left'],[13600,11680,14200])
        self.assertEqual(lost['model_hunger_area']['right'],[14200,10260,13040])

    def test_zero_based_membership_cold_reopen_and_import(self):
        d=self.data(0)
        self.assertEqual(d['source']['source_schema'],v.finite.VERSION)
        self.assertEqual(d['source']['record'],0)
        cold=v.Repository(cache=Path(self.temp.name)/'cold',experiments=('020',))
        cold.entries=copy.deepcopy(self.repo.entries)
        self.assertEqual(cold.get(d['source']['id'])['trial'],d['trial'])
        bad=copy.deepcopy(d);bad['actors']=[{'invented':'knowledge'}]
        self.assertEqual(self.repo.imported(bad)['actors'],d['actors'])
        bad['trial']['initial_total']+=1
        with self.assertRaises(ValueError):self.repo.imported(bad)

    def test_prepare_publishes_reusable_020_catalog(self):
        catalog=Path(self.temp.name)/'catalog-020.json'
        package=json.loads(catalog.read_bytes())
        self.assertEqual(len(package['entries']),164)
        self.assertEqual(min(e['record'] for e in package['entries']),0)
        cold=v.Repository(cache=Path(self.temp.name),experiments=('020',)).catalog()
        self.assertEqual(len(cold.entries),164)
        self.assertFalse(cold.member_verified)
        self.assertTrue(all('not yet verified' in e['integrity'] for e in cold.entries.values()))

    def test_protocol_same_tick_order(self):
        d=self.data(43)
        at=[m for m in d['timeline'] if m['tick']==52 and m['kind'] in ('resource','acquisition','decision')]
        self.assertEqual([m['kind'] for m in at],['resource','acquisition','decision'])

    def test_single_unit_chain_links_transfer_and_recipient_meal(self):
        d=self.data(33)
        chain=d['causal']['chains'][0]
        edges=chain['edges']
        transfer='final_state.base.base.base.base.base.events[5]'
        meal='periods[0].steps[13].activity.consumption[0]'
        self.assertTrue(any(e['source']=='records[0]' and e['target']==transfer for e in edges))
        self.assertTrue(any(e['source']==transfer and e['target']==meal for e in edges))
        self.assertEqual(d['causal']['nodes'][meal]['actor'],1)


if __name__=='__main__':unittest.main()
