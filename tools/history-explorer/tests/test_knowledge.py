"""Real original021 adapter boundaries; never generates simulation evidence."""
import copy
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

HOME=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('knowledge_explorer',HOME/'explorer.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)


class KnowledgeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp=tempfile.TemporaryDirectory(prefix='woi021-main-')
        cls.repo=v.Repository(cache=Path(cls.temp.name),experiments=('021',)).prepare()
        cls.original={p:hashlib.sha256(p.read_bytes()).hexdigest() for p in (v.ROOT/'experiments/021').iterdir() if p.is_file()}

    @classmethod
    def tearDownClass(cls):
        assert all(hashlib.sha256(p.read_bytes()).hexdigest()==h for p,h in cls.original.items())
        cls.temp.cleanup()

    def data(self,n):
        e=next(e for e in self.repo.entries.values() if e['record']==n)
        return self.repo.get(e['id'])

    def view(self,t,p,s,owner):
        return v.knowledge.actor(t,t['periods'][p]['steps'][s],owner,v.actor_view)

    def test_original214_identity_and_all_typed_links(self):
        self.assertEqual(len(self.repo.entries),214)
        self.assertEqual(len({e['id'] for e in self.repo.entries.values()}),214)
        with gzip.open(v.ROOT/'experiments/021/trials.jsonl.gz','rt',encoding='utf-8') as f:
            for ordinal,line in enumerate(f):
                t=json.loads(line);v.validate_trial(t,'021')
                e=next(e for e in self.repo.entries.values() if e['record']==ordinal)
                self.assertEqual(e['policy'],t['policy']);self.assertEqual(e['source_schema'],v.knowledge.VERSION)
                moments=v.knowledge.timeline(t,lambda t:v.finite.timeline(t,v.timeline))
                graph=v.knowledge.causal(t,e['id'],lambda t,i,field:v.finite.causal(t,i,v.causal.build,v.causal.resolve,field),v.causal.resolve)
                anchors={m['ref'] for m in moments}
                for node in graph['nodes'].values():
                    v.causal.resolve(t,node['ref'])
                    self.assertTrue(node['anchor'] in anchors or node['tick']<t['clock']['epoch'],node)
                for chain in graph['chains']:
                    for edge in chain['edges']:
                        self.assertIn(edge['source'],graph['nodes']);self.assertIn(edge['target'],graph['nodes'])
                self.assertEqual([m['tick'] for m in moments],sorted(m['tick'] for m in moments))

    def test_actor_stale_quantity_and_current_only_unknown(self):
        left=self.data(81)['trial'];right=self.data(82)['trial']
        a=self.view(left,1,3,0);b=self.view(right,1,3,0)
        self.assertEqual(a['selected'],'Invite');self.assertIsNone(a['environmental_knowledge']['estimate'])
        self.assertEqual(b['selected'],'Acquire');self.assertEqual(b['environmental_knowledge']['age'],4)
        self.assertEqual(b['environmental_knowledge']['estimate'],12)
        changed=copy.deepcopy(right)
        changed['memory_records']=[];changed['final_memory']=[None]*3
        changed['records'][0]['outcome']='Depleted';changed['points'][0]['cache']['stock']=999
        changed['final_state']['base']['base']['base']['base']['base']['agents'][1]['food']=999
        self.assertEqual(b,self.view(changed,1,3,0))
        self.assertNotIn('cache',b);self.assertNotIn('records',b)

    def test_rival_stock_change_does_not_patch_owned_record(self):
        t=self.data(82)['trial'];a=self.view(t,2,3,0)
        self.assertEqual(a['now'],52);self.assertEqual(a['environmental_knowledge']['estimate'],9)
        point=t['points'][t['periods'][2]['steps'][3]['activity']['observed']]
        self.assertEqual(point['cache']['stock'],8)
        self.assertEqual(a['environmental_knowledge']['retained']['receipt'],6)
        self.assertEqual(t['records'][3]['attempt']['actor'],2)

    def test_stale_failure_has_exact_busy_and_result_links(self):
        d=self.data(32);t=d['trial'];a=self.view(t,0,4,2)
        self.assertEqual(a['environmental_knowledge']['estimate'],1)
        r=t['records'][2]
        self.assertEqual((r['attempt']['start'],r['completed_at'],r['outcome']),(4,8,'Depleted'))
        self.assertEqual((r['food_before'],r['food_after']),(0,0))
        nodes=d['causal']['nodes']
        self.assertIn('memory_records[2]',nodes)
        self.assertIn('periods[0].steps[4].choices[2].retained.observation',nodes)
        self.assertIn('records[2]',nodes)

    def test_foreign_memory_and_schema_coercion_rejected(self):
        t=copy.deepcopy(self.data(82)['trial']);t['periods'][1]['steps'][3]['choices'][0]['retained']['owner']=2
        with self.assertRaises(ValueError):v.validate_trial(t,'021')
        t=copy.deepcopy(self.data(82)['trial']);t['periods'][1]['steps'][3]['choices'][0]['cache']={}
        with self.assertRaises(ValueError):v.validate_trial(t,'021')
        with self.assertRaises(ValueError):v.validate_trial(self.data(82)['trial'],'020')

    def test_expiry_and_success_without_cognitive_recovery(self):
        t=self.data(163)['trial']
        expired=[r for r in t['memory_records'] if r['kind']=='Expired']
        self.assertTrue(expired)
        for r in expired:
            self.assertIsNone(r['after']);self.assertGreaterEqual(r['tick']-r['before']['observation']['observed_at'],12)
        views=[self.view(t,p,s,o) for p,x in enumerate(t['periods']) for s,w in enumerate(x['steps']) for o,d in enumerate(w['choices']) if d and d['retained'] is None]
        self.assertTrue(views)
        self.assertTrue(all(x['environmental_knowledge']['estimate'] is None for x in views))
        self.assertTrue(any(r['outcome']=='Granted' for r in t['records']))

    def test_reconvergence_semantic_trust_and_complete_null(self):
        a=self.data(81)['trial'];b=self.data(82)['trial']
        c=v.knowledge.comparison(a,b,lambda x,y:v.finite.comparison(x,y,v.comparison))
        self.assertTrue(c['material_reconvergence']);self.assertTrue(c['final_material_equal'])
        self.assertFalse(c['final_trust_equal']);self.assertTrue(c['environmental_comparison']['pure_retention_modes'])
        a=self.data(161)['trial'];b=self.data(163)['trial']
        c=v.knowledge.comparison(a,b,lambda x,y:v.finite.comparison(x,y,v.comparison))
        self.assertTrue(c['final_trust_equal']);self.assertFalse(c['raw_final_trust_equal'])
        a=self.data(121)['trial'];b=self.data(122)['trial']
        c=v.knowledge.comparison(a,b,lambda x,y:v.finite.comparison(x,y,v.comparison))
        self.assertIsNone(c['first_choice']);self.assertIsNone(c['first_executed_difference'])

    def test_large_selected_export_preserves_original_and_conservation(self):
        d=self.data(82);t=d['trial']
        self.assertEqual(d['extractor_version'],4)
        self.assertEqual(d['source']['record'],82)
        consumed=0
        for p in t['periods']:
            for w in p['steps']:
                consumed+=len(w['activity']['consumption'])
                point=t['points'][w['activity']['after']]
                self.assertEqual(sum(a['food'] for a in point['state']['agents'])+point['cache']['stock']+consumed,t['initial_total'])
        for q in d['actors']:
            for a in q:
                if a and a['environmental_knowledge']['retained']:
                    self.assertEqual(a['environmental_knowledge']['retained']['owner'],a['own']['id'])


if __name__=='__main__':unittest.main()
