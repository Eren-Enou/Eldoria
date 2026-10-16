"""Real-archive acceptance tests. Run from repo root with unittest discovery."""
import copy
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import socket
import tempfile
import threading
import urllib.request
import urllib.error
import unittest
from unittest.mock import patch

HOME = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('explorer', HOME / 'explorer.py')
v = importlib.util.module_from_spec(spec); spec.loader.exec_module(v)


class RealArchiveTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix='woi-viewer-test-')
        cls.repo = v.Repository(cache=Path(cls.temp.name)/'cache').prepare()
        cls.original = {p: hashlib.sha256(p.read_bytes()).hexdigest() for p in (v.ROOT/'experiments/019').glob('*') if p.is_file()}

    @classmethod
    def tearDownClass(cls):
        for p,h in cls.original.items():
            assert hashlib.sha256(p.read_bytes()).hexdigest() == h, p
        cls.temp.cleanup()

    def data(self, case='useful_contact', mode='Baseline', **changes):
        config = dict(v.DEFAULT, **changes)
        e = next(e for e in self.repo.entries.values() if e['case']==case and e['mode']==mode and e['config']==config and e['seed']=='42')
        return self.repo.get(e['id'])

    def test_catalog_identity_and_membership(self):
        self.assertEqual(len(self.repo.entries),336)
        d=self.data();s=d['source']
        self.assertEqual((s['record'],s['archive'],s['source_schema']), (103,'seed-42.jsonl.gz','experiment-019-trial'))
        with gzip.open(v.ROOT/'experiments/019'/s['archive'],'rb') as f:
            line=next(l for n,l in enumerate(f,1) if n==103)
        self.assertEqual(hashlib.sha256(line.rstrip(b'\r\n')).hexdigest(),s['record_sha256'])
        self.assertEqual(json.loads(line),d['trial'])

    def test_schema_dispatch_and_invalid_sources(self):
        t=self.data()['trial']
        v.validate_trial(t,'019')
        # Same physical wrapper is supported explicitly, with 018's own mode/config.
        with gzip.open(v.ROOT/'experiments/018/seed-42.jsonl.gz','rt') as f: old=json.loads(next(f))
        v.validate_trial(old,'018')
        for value,exp in [({},'019'),(t,'017'),(dict(t,classification='INVALID_PHYSICAL_INTEGRATION'),'019')]:
            with self.assertRaises(ValueError):v.validate_trial(value,exp)
        with self.assertRaises(ValueError):self.repo.imported({'schema':'unknown'})
        with self.assertRaises(ValueError):self.repo.get('../../experiments')

    def test_point_refs_chronology_and_consumption(self):
        d=self.data();t=d['trial'];moments=d['timeline']
        self.assertEqual([m['tick'] for m in moments],sorted(m['tick'] for m in moments))
        first=next(m for m in moments if m['kind']=='decision');s=t['periods'][0]['steps'][0]['activity']
        self.assertEqual(first['point'],s['observed'])
        self.assertEqual(first['ref'],'periods[0].steps[0].activity')
        meal=next(m for m in moments if m['kind']=='meal')
        self.assertEqual((meal['tick'],meal['data']['hunger_before'],meal['data']['hunger_after']), (1,80,40))
        self.assertEqual(meal['data'],s['consumption'][0])
        need=next(m for m in moments if m['kind']=='need')
        self.assertEqual(need['data'],t['needs'][0])
        for decision in (m for m in moments if m['kind']=='decision'):
            captured=t['points'][decision['point']]['state']
            for event in (m for m in moments if m['kind']=='resource' and m['tick']==decision['tick']):
                if event['data']['id']<captured['events']:
                    self.assertLess(event['id'],decision['id'])

    def test_resource_linkage_and_provisional_consent(self):
        d=self.data();t=d['trial'];s=t['periods'][0]['steps'][0]['activity']
        self.assertEqual(s['decisions'][0]['selected'],'Consume')
        self.assertEqual(s['started']['invitation']['proposer'],1)
        self.assertTrue(s['responses'])
        self.assertEqual(s['allocations'][0]['disposition'],'Execute')
        es=[m for m in d['timeline'] if m['kind']=='resource']
        self.assertEqual([(m['tick'],m['data']['decision']['selected']) for m in es[:2]],[(3,'Offer'),(4,'Accept')])
        self.assertEqual(es[1]['data']['transferred'],1)
        self.assertTrue(es[1]['ref'].endswith('events[1]'))

    def test_actor_allowlist_private_twins_and_no_history_restore(self):
        a=self.data()['trial'];b=self.data('unproductive_contact')['trial']
        sa=a['periods'][0]['steps'][0]['activity'];sb=b['periods'][0]['steps'][0]['activity']
        self.assertEqual(v.actor_view(a,sa,0),v.actor_view(b,sb,0))
        self.assertNotEqual(a['initial']['state']['agents'][1],b['initial']['state']['agents'][1])
        view=v.actor_view(a,sa,0)
        self.assertEqual(view['own']['memories'],[])
        mutated=copy.deepcopy(a);v.layers(mutated)['base']['events'].append({'secret':'FORGOTTEN_CONTENT'})
        self.assertEqual(v.actor_view(mutated,sa,0),view)
        # A separately established trust consequence is retained, not erased by empty episodes.
        local=copy.deepcopy(sa);local['decisions'][0]['contact']['input']['own']['trust']={'1':7}
        self.assertEqual(v.actor_view(a,local,0)['own']['trust'],{'1':7})
        absent=copy.deepcopy(sa);absent['decisions'][0]=None
        self.assertIsNone(v.actor_view(a,absent,0))

    def test_real_evicted_evidence_is_not_recovered(self):
        t=self.data('formerly_useful')['trial']
        # The archive retains acquisition history; the display uses only each input's knowledge.
        for p in t['periods']:
            for w in p['steps']:
                s=w['activity']
                for owner,d in enumerate(s['decisions']):
                    if d is not None:
                        self.assertEqual(v.actor_view(t,s,owner)['inquiry']['knowledge'],d['contact']['input']['query']['input']['knowledge'])
        provenance=v.layers(t)['provenance']
        self.assertIn({'owner':2,'collection':'knowledge','receipt':0},provenance['evictions'])
        first=t['periods'][0]['steps'][0]['activity']
        retained=v.actor_view(t,first,2)['inquiry']['knowledge']
        self.assertEqual(len(retained),32)
        self.assertNotIn(0,[k['receipt'] for k in retained])
        self.assertEqual(provenance['receipts'][0]['id'],0)

    def test_missed_inquiry_is_not_executed(self):
        d=self.data('information',order='Descending');t=d['trial']
        s=t['periods'][0]['steps'][3]['activity']
        self.assertEqual(t['points'][s['observed']]['state']['tick'],45)
        self.assertEqual(s['decisions'][0]['selected'],'Invite')
        self.assertEqual(s['decisions'][0]['contact']['selected']['Invite']['kind'],'Inquiry')
        self.assertIsNone(s['query'])
        self.assertIsNotNone(s['started'])
        self.assertNotEqual(s['started']['invitation']['kind'],'Inquiry')
        queries=[m for m in d['timeline'] if m['kind']=='inquiry']
        self.assertTrue(queries)
        native=v.layers(t)['inquiry']['records']
        for m in queries:self.assertEqual(m['data'],native[m['data']['id']])

    def test_observation_gap_is_elapsed_not_choice(self):
        d=self.data('recurring',idle_gap=5);t=d['trial'];g=next(m for m in d['timeline'] if m['kind']=='gap')
        self.assertEqual(g['data'],t['pauses'][0])
        self.assertGreater(g['data']['to'],g['data']['from'])
        self.assertFalse(any(m['kind']=='decision' and g['data']['from']<m['tick']<g['data']['to'] for m in d['timeline']))

    def test_matched_divergence_and_executed_difference(self):
        a=self.data()['trial'];b=self.data(mode='Scale110')['trial'];c=v.comparison(a,b)
        self.assertTrue(c['initial_state_and_profiles_match'])
        self.assertEqual(c['first_choice']['tick'],0)
        self.assertTrue(c['first_choice']['matched_observed_state'])
        self.assertEqual(c['first_choice']['matched_actor_inputs'],[True]*3)
        self.assertEqual(c['first_executed_difference']['left']['kind'],'meal')
        self.assertEqual(c['first_executed_difference']['left']['tick'],1)
        self.assertEqual(c['first_resource_event_difference']['left'],(3,1,'Offer',0))
        self.assertFalse(c['final_material_equal']);self.assertFalse(c['full_final_snapshot_equal'])

    def test_reconvergence_and_full_null_are_distinct(self):
        a=self.data('ongoing','EatFirst',stock=4)['trial'];b=self.data('ongoing','Deadline',stock=4)['trial'];c=v.comparison(a,b)
        self.assertTrue(c['material_reconvergence']);self.assertTrue(c['final_material_equal'])
        self.assertFalse(c['full_final_snapshot_equal'])
        self.assertEqual(c['first_choice']['tick'],16)
        a=self.data('no_food')['trial'];b=self.data('no_food','Scale90')['trial'];c=v.comparison(a,b)
        self.assertIsNone(c['first_choice']);self.assertIsNone(c['first_executed_difference']);self.assertTrue(c['full_final_snapshot_equal'])

    def test_scheduling_comparison_is_qualified(self):
        a=self.data()['trial'];b=self.data(window=6)['trial'];c=v.comparison(a,b)
        self.assertTrue(c['initial_state_and_profiles_match']);self.assertFalse(c['configurations_equal']);self.assertTrue(c['equal_horizon'])
        self.assertTrue(all(w['activity']['started'] is None for p in b['periods'] for w in p['steps']))

    def test_cache_and_import_cannot_forge_projection_or_source(self):
        d=self.data();d['actors'][0][0]['own']['food']=999
        d['causal']={'observer_only':False,'nodes':{'forged':'<script>invented receipt</script>'}}
        d['source']['scientific_status']='FORGED'
        imported=self.repo.imported(d)
        self.assertEqual(imported['actors'][0][0]['own']['food'],2)
        self.assertTrue(imported['causal']['observer_only'])
        self.assertNotIn('forged',imported['causal']['nodes'])
        self.assertNotEqual(imported['source']['scientific_status'],'FORGED')
        bad=copy.deepcopy(d);bad['trial']['initial']['state']['agents'][0]['food']=999
        with self.assertRaises(ValueError):self.repo.imported(bad)
        d=self.data();p=self.repo.cache/(d['source']['id']+'.json');bad=copy.deepcopy(d);bad['trial']['case']='tampered';p.write_bytes(v.encoded(bad))
        self.assertEqual(self.repo.get(d['source']['id'])['trial']['case'],'useful_contact')

    def test_archive_hash_rejection_and_size_bounds(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp);d=root/'experiments/019';d.mkdir(parents=True)
            manifest=json.loads((v.ROOT/'experiments/019/archive-manifest.json').read_bytes());name='seed-42.jsonl.gz'
            (d/'archive-manifest.json').write_bytes(v.encoded({name:manifest[name]}));(d/'policy-freeze.json').write_text('{}')
            (d/name).write_bytes(b'corrupt')
            with self.assertRaisesRegex(ValueError,'Compressed archive hash mismatch'):v.Repository(root,root/'cache').prepare()
            raw=b'{}\n';gz=gzip.compress(raw,mtime=0);(d/name).write_bytes(gz)
            manifest[name]=dict(gzip_sha256=hashlib.sha256(gz).hexdigest(),gzip_bytes=len(gz),json_sha256='wrong',json_bytes=len(raw))
            (d/'archive-manifest.json').write_bytes(v.encoded({name:manifest[name]}))
            with patch.object(v,'MAX_RECORD',1):
                with self.assertRaisesRegex(ValueError,'64 MiB'):v.Repository(root,root/'cache').prepare()
            # A valid trial with a forged decoded digest must be rejected too.
            raw=v.encoded(self.data()['trial'])+b'\n';gz=gzip.compress(raw,mtime=0);(d/name).write_bytes(gz)
            manifest[name].update(gzip_sha256=hashlib.sha256(gz).hexdigest(),gzip_bytes=len(gz),json_bytes=len(raw))
            (d/'archive-manifest.json').write_bytes(v.encoded({name:manifest[name]}))
            with self.assertRaisesRegex(ValueError,'Decoded archive hash mismatch'):v.Repository(root,root/'cache').prepare()
            self.assertEqual(list((root/'cache').glob('*.json')),[])
        with self.assertRaises(ValueError):v.Repository(cache=v.ROOT/'experiments/019/cache')

    def test_fast_catalog_verifies_source_before_display(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo=v.Repository(cache=Path(tmp)).catalog()
            e=next(e for e in repo.entries.values() if e['archive']=='seed-42.jsonl.gz' and e['record']==103)
            self.assertIn('not yet verified',e['integrity'])
            d=repo.get(e['id'])
            self.assertIn('hashes verified',d['source']['integrity'])
            self.assertEqual(d['trial']['case'],'useful_contact')
            e['case']='forged label'
            repo.member_verified.clear()
            with self.assertRaisesRegex(ValueError,'identity does not match'):repo.get(e['id'])

    def test_recipient_and_resource_local_views(self):
        t=self.data()['trial'];s=t['periods'][0]['steps'][0]['activity']
        for r in s['responses']:
            owner=r['input']['own']['own']['id'];a=v.reply_view(t,s,owner)
            self.assertEqual(a['own'],r['input']['own']['own'])
            self.assertEqual(a['now'],r['input']['own']['now'])
            self.assertEqual(a['candidates'],r['candidates'])
        event=v.layers(t)['base']['events'][1];a=v.resource_view(event)
        self.assertEqual(a['owner'],0);self.assertEqual(a['own']['food'],event['decision']['food'])
        self.assertNotIn('balances_before',a);self.assertNotIn('participants',a)
        self.assertEqual(a['memory_causes'],event['decision']['memory_causes'])

    def test_local_http_boundaries(self):
        server=v.ThreadingHTTPServer(('127.0.0.1',0),v.Handler);server.repo=self.repo
        thread=threading.Thread(target=server.serve_forever,daemon=True);thread.start()
        url=f'http://127.0.0.1:{server.server_port}'
        try:
            # Idle browser preconnections must not monopolize a single-thread server.
            idle=socket.create_connection(('127.0.0.1',server.server_port))
            with urllib.request.urlopen(url+'/api/catalog',timeout=3) as response:
                self.assertIn("default-src 'self'",response.headers['Content-Security-Policy'])
                self.assertEqual(len(json.load(response)['entries']),336)
            idle.close()
            source=self.data()['source']
            with urllib.request.urlopen(url+'/api/export?id='+source['id']) as response:
                self.assertIn('attachment;',response.headers['Content-Disposition'])
                self.assertEqual(json.load(response)['source']['record'],103)
            cases=[('/../../AGENTS.md',None,{},404),('/api/trial?id=missing',None,{},400),('/api/import',b'{}',{'Origin':'https://example.invalid'},403),('/api/import',b'{}',{'Content-Type':'application/json'},400),('/api/import',b'['*2000+b']'*2000,{'Content-Type':'application/json'},400),('/api/import',b'x',{'Content-Length':str(v.MAX_RECORD+1)},413)]
            for path,body,headers,code in cases:
                with self.assertRaises(urllib.error.HTTPError) as caught:
                    urllib.request.urlopen(urllib.request.Request(url+path,data=body,headers=headers))
                self.assertEqual(caught.exception.code,code)
        finally:
            server.shutdown();server.server_close();thread.join()


if __name__=='__main__': unittest.main()
