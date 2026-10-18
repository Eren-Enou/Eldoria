"""World of Individuals History Explorer. Local, read-only archive consumer.
Python 3.10+; standard library only. No simulator/evaluator imports.
"""
import argparse
import copy
import gzip
import hashlib
import importlib.util
import json
import mimetypes
from pathlib import Path
import tempfile
import threading
import time
from http.server import BaseHTTPRequestHandler, HTTPServer, ThreadingHTTPServer
from urllib.parse import parse_qs, urlparse
import webbrowser

ROOT = Path(__file__).resolve().parents[2]
HOME = Path(__file__).resolve().parent
_causal_spec = importlib.util.spec_from_file_location('viewer_causal', HOME / 'causal.py')
causal = importlib.util.module_from_spec(_causal_spec)
_causal_spec.loader.exec_module(causal)
_finite_spec = importlib.util.spec_from_file_location('viewer_finite', HOME / 'finite.py')
finite = importlib.util.module_from_spec(_finite_spec)
_finite_spec.loader.exec_module(finite)
_knowledge_spec = importlib.util.spec_from_file_location('viewer_knowledge', HOME / 'knowledge.py')
knowledge = importlib.util.module_from_spec(_knowledge_spec)
_knowledge_spec.loader.exec_module(knowledge)
SCHEMA = 'woi-history-explorer/v1'
MAX_RECORD = 64 * 1024 * 1024
DEFAULT = dict(window=24, idle_gap=1, interval=4, stock=12, hunger=20, order='Ascending')


def encoded(value):
    return json.dumps(value, ensure_ascii=True, separators=(',', ':'), sort_keys=True).encode()


def digest(value):
    return hashlib.sha256(encoded(value)).hexdigest()


def layers(trial):
    """Explicit shared 018/019 final snapshot adapter; never a policy input."""
    learning = trial['final_state']
    retention = learning['base']
    assessment = retention['base']
    compact = assessment['base']
    inquiry = compact['base']
    base = inquiry['base']
    return dict(base=base, inquiry=inquiry['inquiry'], assessment=assessment['assessment'],
                learning=learning['learning'], provenance=compact['provenance'])


def validate_trial(t, experiment):
    if experiment == '021':
        knowledge.validate(t)
        layers(t)
        return
    if experiment == '020':
        finite.validate(t)
        layers(t)
        return
    if experiment not in ('018', '019'):
        raise ValueError('Unsupported source schema: ' + str(experiment))
    required = {'seed', 'case', 'mode', 'config', 'initial', 'profiles', 'points', 'periods', 'final_state', 'clock', 'needs'}
    if not isinstance(t, dict) or not required <= t.keys() or not t['points'] or not t['periods']:
        raise ValueError('Empty or unsupported trial')
    if experiment == '019' and 'pauses' not in t:
        raise ValueError('019 requires recorded pauses, including an empty list')
    modes = ('Baseline', 'EatFirst', 'Deadline', 'Scale90', 'Scale110', 'NoHistory') if experiment == '019' else ('Fixed', 'Recurring', 'Serial', 'FixedContact', 'ContactOnly', 'EatFirst', 'NoHistory')
    if t['mode'] not in modes:
        raise ValueError('Mode is not part of this historical schema')
    # Invalid development traces are intentionally outside the allowed archive catalog.
    if t.get('classification', '').startswith('INVALID'):
        raise ValueError('Invalid source classification; quarantined')
    tick = t['initial']['state']['tick']
    for point in t['points']:
        if point['state']['tick'] < tick:
            raise ValueError('Nonchronological points')
        tick = point['state']['tick']
    for p in t['periods']:
        for key in ('before', 'after'):
            if not 0 <= p[key] < len(t['points']):
                raise ValueError('Broken period point reference')
        for w in p['steps']:
            s = w['activity']
            for key in ('before', 'observed', 'after', 'sent', 'admitted'):
                if s[key] is not None and not 0 <= s[key] < len(t['points']):
                    raise ValueError('Broken step point reference')
            for d in s['decisions']:
                if d is not None and set(d['contact']['input']) != {'own', 'profile', 'now', 'end', 'people', 'query'}:
                    raise ValueError('Unexpected actor input fields')
    layers(t)


def actor_view(t, step, owner):
    """Allowlist ONLY captured local input; never join an observer archive prefix.
    No final state, partner state, hidden roots or later receipts can enter this view.
    """
    d = step['decisions'][owner] if 0 <= owner < len(step['decisions']) else None
    if d is None:
        return None
    i = d['contact']['input']
    own = {k: copy.deepcopy(i['own'][k]) for k in ('id', 'food', 'hunger', 'generosity', 'caution', 'expectation', 'trust', 'memories')}
    q = i['query']['input']
    base = {k: copy.deepcopy(q['base'][k]) for k in ('owner', 'scene', 'hunger', 'relationship_goal', 'concerns', 'sources', 'cells', 'seen', 'settings')}
    return dict(phase='activity', own=own, profile=copy.deepcopy(i['profile']), now=i['now'], end=i['end'],
                people=copy.deepcopy(i['people']), inquiry=dict(base=base, knowledge=copy.deepcopy(q['knowledge']), hints=copy.deepcopy(q['hints']), settings=copy.deepcopy(q['settings'])),
                candidates=copy.deepcopy(d['candidates']), selected=d['selected'],
                contact_candidates=copy.deepcopy(d['contact']['candidates']), contact_selected=copy.deepcopy(d['contact']['selected']),
                query_candidates=copy.deepcopy(i['query']['decision']['candidates']), query_selected=copy.deepcopy(i['query']['decision']['selected']))


def reply_view(t, step, owner):
    response = next((r for r in step['responses'] if r['input']['own']['own']['id'] == owner), None)
    if response is None:
        return None
    contact = response['input']['outgoing']
    # Reuse the input allowlist, not observer state. Activity scores from the earlier
    # phase are deliberately replaced by the actual captured reply candidates.
    adapter_input = dict(decisions=[None] * (owner + 1))
    adapter_input['decisions'][owner] = dict(contact=dict(contact, input=response['input']['own']), candidates=[], selected=response['selected'])
    view = actor_view(t, adapter_input, owner)
    view.update(phase='recipient', candidates=copy.deepcopy(response['candidates']), incoming=copy.deepcopy(response['input']['incoming']))
    return view


def resource_view(event):
    d = event['decision']
    return dict(phase='resource', own={k: d[k] for k in ('food', 'hunger', 'generosity', 'caution', 'expectation')},
                owner=d['actor'], now=event['tick'], selected=d['selected'], candidates=copy.deepcopy(d['candidates']),
                observation=copy.deepcopy(d['observation']), trust=d['trust'], episodic_valence=d['episodic_valence'], memory_causes=copy.deepcopy(d['memory_causes']))


def timeline(t):
    """Derived navigation index. Every item retains its original namespace/path."""
    result = []
    def add(tick, kind, ref, label, point=None, step=None, data=None, rank=0):
        result.append(dict(tick=tick, kind=kind, ref=ref, label=label, point=point, step=step, data=data, rank=rank))
    for pn, p in enumerate(t['periods']):
        add(p['start'], 'window', f'periods[{pn}]', f'Window {pn + 1}: {p["start"]}–{p["end"]}', p['before'], rank=-2)
        for sn, w in enumerate(p['steps']):
            s = w['activity']; ref = f'periods[{pn}].steps[{sn}].activity'; ix = [pn, sn]
            selections = ', '.join(f'{i}: {d["selected"]}' for i, d in enumerate(s['decisions']) if d is not None)
            add(t['points'][s['observed']]['state']['tick'], 'decision', ref, selections or 'Occupied; no new activity choice', s['observed'], ix, rank=-1)
            if s['responses']:
                add(t['points'][s['sent']]['state']['tick'], 'consent', ref + '.responses', 'Recipient responses (provisional)', s['sent'], ix, s['responses'])
            if s['allocations']:
                add(t['points'][s['admitted']]['state']['tick'], 'matching', ref + '.allocations', 'Arbitration: ' + ', '.join(a['disposition'] for a in s['allocations']), s['admitted'], ix, s['allocations'])
            for k in ('started', 'finished'):
                if s[k]:
                    c = s[k]; tick = c['start'] if k == 'started' else t['points'][s['after']]['state']['tick']
                    add(tick, 'commitment', ref + '.' + k, 'Resource commitment ' + k + f' · scene {c["scene"]}', s['admitted'] if k == 'started' else s['after'], ix, c)
            for ci, c in enumerate(s['consumption']):
                add(c['tick'], 'meal', ref + f'.consumption[{ci}]', f'Individual {c["actor"]} consumes {c["consumed"]}; hunger {c["hunger_before"]} → {c["hunger_after"]}', None, ix, c)
    base = layers(t)
    prefix = 'final_state.base.base.base.base.base'
    for e in base['base']['events'][t['initial']['state']['events']:]:
        add(e['tick'], 'resource', prefix + f'.events[{e["id"]}]', f'Individual {e["decision"]["actor"]}: {e["decision"]["selected"]} · transferred {e["transferred"]}', data=e)
    for q in base['inquiry']['records'][t['initial']['state']['queries']:]:
        # Actual native query record, not the invitation preview.
        # Completion boundary of the recorded atomic activity, not an invented
        # internal deliberation point or an ambiguous native receipt timestamp.
        step = next(w['activity'] for p in t['periods'] for w in p['steps'] if w['activity']['query'] == q['id'])
        end = t['points'][step['after']]['state']['tick']
        add(end, 'inquiry', f'final_state.base.base.base.base.inquiry.records[{q["id"]}]', f'Executed inquiry {q["id"]}: ' + json.dumps(q['decision']['selected'], separators=(',', ':')), data=q)
    for i, n in enumerate(t['needs']):
        add(n['to'], 'need', f'needs[{i}]', f'Elapsed needs {n["from"]} → {n["to"]} · pulses {n["pulses"]}', data=n)
    for i, p in enumerate(t.get('pauses', [])):
        add(p['to'], 'gap', f'pauses[{i}]', f'Observation gap {p["from"]} → {p["to"]}; no decisions captured inside', p['after'], [p['period'], p['step']], p)
    for i, point in enumerate(t['points']):
        add(point['state']['tick'], 'snapshot', f'points[{i}]', f'Recorded state · point {i}', i, rank=2)
    # Same-tick causal order follows the original step protocol and point/counter
    # references. A prior resource action must precede the next decision at its
    # completion tick; sorting every decision before every event would be false.
    orders = {}; point_orders = {}; ordinal = 0
    for pn, p in enumerate(t['periods']):
        orders[f'periods[{pn}]'] = ordinal * 1000 - 50
        point_orders.setdefault(p['before'], ordinal * 1000 - 80)
        for sn, w in enumerate(p['steps']):
            s = w['activity']; root = f'periods[{pn}].steps[{sn}].activity'; offset = ordinal * 1000
            orders[root] = offset + 100
            for key, phase in [('before', 70), ('observed', 90), ('sent', 380), ('admitted', 480), ('after', 980)]:
                if s[key] is not None:
                    point_orders.setdefault(s[key], offset + phase)
            for key, phase in [('responses', 400), ('allocations', 500), ('started', 600), ('finished', 950)]:
                orders[root + '.' + key] = offset + phase
            for ci, _ in enumerate(s['consumption']):
                orders[root + f'.consumption[{ci}]'] = offset + (800 if s['active_before'] else 300) + ci
            for ni in range(w['need_before'], w['need_after']):
                orders[f'needs[{ni}]'] = offset + 200
            for ei in range(t['points'][s['before']]['state']['events'], t['points'][s['after']]['state']['events']):
                orders[prefix + f'.events[{ei}]'] = offset + 700
            if s['query'] is not None:
                orders[f'final_state.base.base.base.base.inquiry.records[{s["query"]}]'] = offset + 900
            for gi, gap in enumerate(t.get('pauses', [])):
                if (gap['period'], gap['step']) == (pn, sn):
                    for ni in range(gap['need_before'], gap['need_after']):
                        orders[f'needs[{ni}]'] = offset + 990
                    orders[f'pauses[{gi}]'] = offset + 995
                    point_orders.setdefault(gap['after'], offset + 997)
            ordinal += 1
    for i, _ in enumerate(t['points']):
        orders[f'points[{i}]'] = point_orders[i]
    for m in result:
        m['order'] = orders[m['ref']]
    result.sort(key=lambda x: (x['tick'], x['order']))
    for i, m in enumerate(result):
        m['id'] = i
    return result


def choice(d):
    if d is None:
        return None
    return (d['selected'], d['contact']['selected'] if d['selected'] == 'Invite' else None,
            d['contact']['input']['query']['decision']['selected'] if d['selected'] == 'Invite' and isinstance(d['contact']['selected'], dict) and d['contact']['selected']['Invite']['kind'] == 'Inquiry' else None)


def comparison(a, b):
    """Read recorded values; no policy recomputation. Time alignment uses world ticks."""
    def steps(t):
        return [(t['points'][w['activity']['observed']]['state']['tick'], pn, sn, w['activity']) for pn, p in enumerate(t['periods']) for sn, w in enumerate(p['steps'])]
    def events(t):
        return [(e['tick'], e['decision']['actor'], e['decision']['selected'], e['transferred']) for e in layers(t)['base']['events'][t['initial']['state']['events']:]]
    def material(point):
        return [(x['id'], x['food'], x['hunger']) for x in point['state']['agents']]
    initial = a['initial'] == b['initial'] and a['profiles'] == b['profiles']
    first = None
    sa, sb = steps(a), steps(b)
    for x, y in itertools_zip(sa, sb):
        if x is None or y is None or x[0] != y[0]:
            first = dict(kind='Decision schedules differ; not an isolated voluntary difference', left=x[:3] if x else None, right=y[:3] if y else None)
            break
        if [choice(d) for d in x[3]['decisions']] != [choice(d) for d in y[3]['decisions']]:
            first = dict(kind='First recorded voluntary choice difference', tick=x[0], left=x[:3], right=y[:3], matched_observed_state=a['points'][x[3]['observed']] == b['points'][y[3]['observed']], matched_actor_inputs=[None if l is None or r is None else l['contact']['input'] == r['contact']['input'] for l, r in zip(x[3]['decisions'], y[3]['decisions'])])
            break
    resource_difference = None
    for x, y in itertools_zip(events(a), events(b)):
        if x != y:
            resource_difference = dict(left=x, right=y)
            break
    def executed(t):
        out = []
        for m in timeline(t):
            if m['kind'] == 'resource':
                e = m['data']; value = [e['decision']['actor'], e['decision']['selected'], e['transferred'], e['balances_after']]
            elif m['kind'] == 'meal':
                value = m['data']
            elif m['kind'] == 'inquiry':
                q = m['data']
                value = dict(selected=q['decision']['selected'], response=q['response'], time_spent=q['time_spent'], valid=q['valid'])
            else:
                continue
            out.append(dict(tick=m['tick'], kind=m['kind'], ref=m['ref'], value=value))
        return out
    def execution_key(x):
        return None if x is None else (x['tick'], x['kind'], x['value'])
    actual = next((dict(left=x, right=y) for x, y in itertools_zip(executed(a), executed(b)) if execution_key(x) != execution_key(y)), None)
    # Union of actual snapshot ticks; compare only exact shared recorded ticks.
    pa = {p['state']['tick']: (i, p) for i, p in enumerate(a['points'])}
    pb = {p['state']['tick']: (i, p) for i, p in enumerate(b['points'])}
    differences = [dict(tick=tick, left=pa[tick][0], right=pb[tick][0]) for tick in sorted(pa.keys() & pb.keys()) if material(pa[tick][1]) != material(pb[tick][1])]
    final_material = material(a['points'][-1]) == material(b['points'][-1])
    return dict(initial_state_and_profiles_match=initial, physical_settings_equal=a['clock']['settings'] == b['clock']['settings'], configurations_equal=a['config'] == b['config'], equal_horizon=a['clock']['last'] == b['clock']['last'] and a['clock']['epoch'] == b['clock']['epoch'], first_choice=first, first_executed_difference=actual, first_resource_event_difference=resource_difference, material_different_shared_ticks=differences, final_material_equal=final_material, material_reconvergence=bool(differences) and final_material, full_final_snapshot_equal=a['final_state'] == b['final_state'], note='Derived comparison. Shared-tick checks use the last recorded point at that tick. First execution includes actual meals, resource events and completed inquiries; incomplete internal query states are not reconstructed.')


def itertools_zip(a, b):
    import itertools
    return itertools.zip_longest(a, b)


class Repository:
    def __init__(self, root=ROOT, cache=None, experiments=('019',)):
        self.root = Path(root).resolve()
        self.cache = Path(cache or self.root / 'target/history-explorer').resolve()
        if self.cache == self.root or (self.root / 'experiments') in (self.cache, *self.cache.parents):
            raise ValueError('Cache must be outside original evidence')
        self.cache.mkdir(parents=True, exist_ok=True)
        self.entries = {}; self.loaded = collections_lru(); self.measurements = []
        self.experiments = experiments
        self.member_verified = set()
        self.lock = threading.RLock()

    def catalog(self):
        """Fast discovery only. No scientific/integrity claim from a derived index.
        Opening a trial verifies its original compressed/decoded archive and member.
        """
        data = json.loads((HOME / 'demo/catalog-v1.json').read_bytes())
        if data.get('schema') != SCHEMA:
            raise ValueError('Unknown catalog schema')
        for e in data['entries']:
            if e['experiment'] != '019' or Path(e['archive']).name != e['archive']:
                raise ValueError('Unsupported catalog source')
            if e['experiment'] not in self.experiments:
                continue
            e['integrity'] = 'Derived catalog entry; source not yet verified in this session'
            e['scientific_status'] = self.status(e['experiment'])
            self.entries[e['id']] = e
        for exp in ('020','021'):
            if exp not in self.experiments: continue
            derived = self.cache / f'catalog-{exp}.json'
            if not derived.exists():
                temp = Repository(self.root, self.cache / f'index-{exp}', (exp,)).prepare()
                derived.write_bytes(encoded(dict(schema=SCHEMA, entries=list(temp.entries.values()))))
            data020 = json.loads(derived.read_bytes())
            if data020.get('schema') != SCHEMA: raise ValueError('Unknown environmental catalog schema')
            for e in data020['entries']:
                if e['experiment'] != exp or e['archive'] != 'trials.jsonl.gz': raise ValueError('Unsupported environmental catalog source')
                e['integrity']='Derived catalog entry; source not yet verified in this session'
                self.entries[e['id']]=e
        return self

    def status(self, exp):
        protected = self.root / f'docs/foundation-001-{exp}-sha256.json'
        review = self.root / f'docs/experiment-{exp}-review.md'
        audit = self.root / f'docs/integrity-audit-001-{exp}.md'
        completed = audit.exists() and any(mark in audit.read_text(encoding='utf-8') for mark in ('Gate B: PASS', 'Gate B passes.'))
        return 'Repository declares reviewed/protected with qualification; no default adoption' if protected.exists() and review.exists() and completed else 'Reviewed research / protection not established by this viewer' if review.exists() else 'Research / not protected; scientific claims require review'

    def manifest(self, exp):
        d = self.root / 'experiments' / exp
        protected = self.root / f'docs/foundation-001-{exp}-sha256.json'
        inventory = json.loads(protected.read_bytes()) if protected.exists() else {}
        freeze_name = 'source-freeze.json' if exp in ('020','021') else 'policy-freeze.json'
        for name in ('archive-manifest.json', freeze_name):
            key = f'experiments/{exp}/{name}'
            if inventory and hashlib.sha256((d / name).read_bytes().replace(b'\r\n', b'\n')).hexdigest() != inventory.get(key):
                raise ValueError('Protected manifest identity mismatch: ' + name)
        manifest = json.loads((d / 'archive-manifest.json').read_bytes())
        for name, expected in manifest.items():
            if inventory and expected['gzip_sha256'] != inventory.get(f'experiments/{exp}/{name}'):
                raise ValueError('Protected archive digest mismatch: ' + name)
        return manifest

    def verify_member(self, e):
        d = self.root / 'experiments' / e['experiment']
        manifest = self.manifest(e['experiment'])
        if e['archive'] not in manifest or not e['archive'].endswith('.jsonl.gz') or Path(e['archive']).name != e['archive']:
            raise ValueError('Unregistered source archive')
        expected = manifest[e['archive']]; source = d / e['archive']
        if hashlib.sha256(source.read_bytes()).hexdigest() != expected['gzip_sha256'] or source.stat().st_size != expected['gzip_bytes']:
            raise ValueError('Compressed archive hash mismatch')
        if expected['gzip_sha256'] != e['archive_sha256'] or expected['json_sha256'] != e['decoded_archive_sha256']:
            raise ValueError('Stale derived catalog; source manifest changed')
        h = hashlib.sha256(); size = 0; selected = None
        with gzip.open(source, 'rb') as f:
            ordinal = 0
            while line := f.readline(MAX_RECORD + 1):
                if len(line) > MAX_RECORD:
                    raise ValueError('Trial exceeds 64 MiB input bound')
                ordinal += 1; h.update(line); size += len(line)
                if ordinal == e['record'] + (1 if e['experiment'] in ('020','021') else 0):
                    selected = line
        if (size, h.hexdigest()) != (expected['json_bytes'], expected['json_sha256']):
            raise ValueError('Decoded archive hash mismatch')
        if selected is None or hashlib.sha256(selected.rstrip(b'\r\n')).hexdigest() != e['record_sha256']:
            raise ValueError('JSONL record membership mismatch')
        t = json.loads(selected); validate_trial(t, e['experiment'])
        if digest(t) != e['trial_fingerprint'] or any(e[k] != (str(t[k]) if k == 'seed' else t[k]) for k in ('seed', 'case', 'mode', 'config')):
            raise ValueError('Derived identity does not match original source')
        identity = {k:e[k] for k in ('experiment', 'archive', 'record', 'seed', 'case', 'mode', 'config')}
        if e['experiment']=='021':
            if e.get('policy') != t['policy']: raise ValueError('021 acquisition policy identity mismatch')
            identity['policy'] = e['policy']
        if e['id'] != digest(dict(identity=identity, archive_sha256=expected['gzip_sha256'], schema=SCHEMA))[:24]:
            raise ValueError('Stale trial identifier; rebuild catalog with --prepare')
        freeze = hashlib.sha256((d / ('source-freeze.json' if e['experiment'] in ('020','021') else 'policy-freeze.json')).read_bytes().replace(b'\r\n', b'\n')).hexdigest()
        if e['policy_freeze_sha256'] != freeze:
            raise ValueError('Source freeze manifest identity changed')
        e.update(source_schema=finite.VERSION if e['experiment']=='020' else 'experiment-' + e['experiment'] + '-trial', raw_bytes=len(selected), epoch=t['clock']['epoch'], end=t['clock']['last'])
        if e['experiment']=='021': e['source_schema']=knowledge.VERSION
        e['scientific_status'] = self.status(e['experiment'])
        e['integrity'] = 'Compressed and complete decoded archive hashes verified; exact JSONL membership checked'
        self.member_verified.add(e['id'])
        return t

    def prepare(self):
        started = time.perf_counter()
        for exp in self.experiments:
            d = self.root / 'experiments' / exp
            manifest = self.manifest(exp)
            freeze = hashlib.sha256((d / ('source-freeze.json' if exp in ('020','021') else 'policy-freeze.json')).read_bytes().replace(b'\r\n', b'\n')).hexdigest()
            status = self.status(exp)
            for archive, expected in manifest.items():
                if not archive.endswith('.jsonl.gz'):
                    continue
                if Path(archive).name != archive:
                    raise ValueError('Nested or invalid development archive is quarantined')
                path = d / archive
                if hashlib.sha256(path.read_bytes()).hexdigest() != expected['gzip_sha256'] or path.stat().st_size != expected['gzip_bytes']:
                    raise ValueError('Compressed archive hash mismatch: ' + archive)
                decoded = hashlib.sha256(); size = 0; pending = []; demos = []
                with gzip.open(path, 'rb') as f:
                    ordinal = 0
                    while line := f.readline(MAX_RECORD + 1):
                        if len(line) > MAX_RECORD:
                            raise ValueError('Trial exceeds 64 MiB input bound')
                        decoded.update(line); size += len(line); ordinal += 1
                        t = json.loads(line); validate_trial(t, exp)
                        identity = dict(experiment=exp, archive=archive, record=ordinal-1 if exp in ('020','021') else ordinal, seed=str(t['seed']), case=t['case'], mode=t['mode'], config=t['config'])
                        if exp=='021': identity['policy']=t['policy']
                        key = digest(dict(identity=identity, archive_sha256=expected['gzip_sha256'], schema=SCHEMA))[:24]
                        entry = dict(id=key, **identity, source_schema=finite.VERSION if exp=='020' else 'experiment-' + exp + '-trial', archive_sha256=expected['gzip_sha256'], decoded_archive_sha256=expected['json_sha256'], record_sha256=hashlib.sha256(line.rstrip(b'\r\n')).hexdigest(), trial_fingerprint=digest(t), policy_freeze_sha256=freeze, scientific_status=status, integrity='Compressed and complete decoded archive hashes verified; exact JSONL membership checked', raw_bytes=len(line), epoch=t['clock']['epoch'], end=t['clock']['last'])
                        if exp=='021': entry['source_schema']=knowledge.VERSION
                        pending.append(entry)
                        if exp == '019' and archive == 'seed-42.jsonl.gz' and t['case'] == 'useful_contact' and t['mode'] in ('Baseline', 'Scale110'):
                            demos.append((entry, line))
                if (size, decoded.hexdigest()) != (expected['json_bytes'], expected['json_sha256']):
                    raise ValueError('Decoded archive hash mismatch: ' + archive)
                for e in pending:
                    self.entries[e['id']] = e
                    self.member_verified.add(e['id'])
                # A provisional member cannot produce a "verified" demo before
                # the complete decoded envelope has passed its hash check.
                for e, line in demos:
                    self._save(e, json.loads(line))
        self.measurements.append(dict(operation='catalog_and_full_archive_verification', seconds=time.perf_counter()-started, trials=len(self.entries)))
        (self.cache / 'catalog.json').write_bytes(encoded(dict(schema=SCHEMA, entries=list(self.entries.values()))))
        for exp in ('020','021'):
            if exp not in self.experiments: continue
            # The advertised --prepare path must replace stale020 discovery too.
            # Publish atomically; a partial index is never a verified member.
            package=dict(schema=SCHEMA,entries=[e for e in self.entries.values() if e['experiment']==exp])
            with tempfile.NamedTemporaryFile(dir=self.cache,suffix='.tmp',delete=False) as f:
                f.write(encoded(package));temporary=Path(f.name)
            temporary.replace(self.cache/f'catalog-{exp}.json')
        return self

    def _save(self, entry, trial):
        is_finite = entry['experiment']=='020'
        is_knowledge = entry['experiment']=='021'
        data = dict(schema=SCHEMA, extractor_version=4 if is_knowledge else 3 if is_finite else 2, source=copy.deepcopy(entry), trial=trial)
        if is_knowledge:
            data['timeline']=knowledge.timeline(trial,lambda t:finite.timeline(t,timeline))
            data['causal']=knowledge.causal(trial,entry['id'],lambda t,i,field:finite.causal(t,i,causal.build,causal.resolve,field),causal.resolve)
        else:
            data['timeline'] = finite.timeline(trial,timeline) if is_finite else timeline(trial)
            data['causal'] = finite.causal(trial,entry['id'],causal.build,causal.resolve) if is_finite else causal.build(trial, entry['id'])
        # Actor projections are derived strictly from captured inputs; no audit joining.
        if is_knowledge:
            data['actors']=[[knowledge.actor(trial,w,owner,actor_view) for owner in range(3)] for p in trial['periods'] for w in p['steps']]
        else:
            data['actors'] = [[finite.actor(trial,w,owner,actor_view) if is_finite else None if d is None else actor_view(trial,w['activity'],owner) for owner,d in enumerate(w['activity']['decisions'])] for p in trial['periods'] for w in p['steps']]
        data['reply_actors'] = [[reply_view(trial, w['activity'], owner) for owner in range(3)] for p in trial['periods'] for w in p['steps']]
        data['resource_actors'] = {str(e['id']): resource_view(e) for e in layers(trial)['base']['events']}
        # Atomic derived-cache replacement prevents partial exports after interruption.
        with tempfile.NamedTemporaryFile(dir=self.cache, suffix='.tmp', delete=False) as f:
            f.write(encoded(data))
            temporary = Path(f.name)
        temporary.replace(self.cache / (entry['id'] + '.json'))
        return data

    def get(self, key):
        started = time.perf_counter()
        if key not in self.entries:
            raise ValueError('Unknown registered trial')
        e = self.entries[key]; path = self.cache / (key + '.json')
        self.manifest(e['experiment'])
        e['scientific_status'] = self.status(e['experiment'])
        if key not in self.member_verified:
            return self._remember(key, self._save(e, self.verify_member(e)), started)
        source = self.root / 'experiments' / e['experiment'] / e['archive']
        if hashlib.sha256(source.read_bytes()).hexdigest() != e['archive_sha256']:
            raise ValueError('Source changed after indexing')
        if path.exists():
            data = json.loads(path.read_bytes())
            if data.get('schema') == SCHEMA and digest(data.get('trial')) == e['trial_fingerprint']:
                # Rebuild presentation fields, never trust cached projections or labels.
                return self._remember(key, self._save(e, data['trial']), started)
        with gzip.open(self.root / 'experiments' / e['experiment'] / e['archive'], 'rb') as f:
            for n, line in enumerate(f, 1):
                if n == e['record'] + (1 if e['experiment'] in ('020','021') else 0):
                    if hashlib.sha256(line.rstrip(b'\r\n')).hexdigest() != e['record_sha256']:
                        raise ValueError('Source changed after indexing')
                    t = json.loads(line)
                    if digest(t) != e['trial_fingerprint']:
                        raise ValueError('Trial membership mismatch')
                    return self._remember(key, self._save(e, t), started)
        raise ValueError('Missing registered record')

    def _remember(self, key, data, started):
        # At most two full histories retained server-side, independent of corpus size.
        self.loaded[key] = data
        self.loaded.move_to_end(key)
        while len(self.loaded) > 2:
            self.loaded.popitem(last=False)
        self.measurements.append(dict(operation='selected_trial', id=key, seconds=time.perf_counter()-started, export_bytes=len(encoded(data))))
        return data

    def imported(self, data):
        if not isinstance(data, dict) or data.get('schema') != SCHEMA:
            raise ValueError('Unknown viewer export schema')
        e = self.entries.get(data.get('source', {}).get('id'))
        if e:
            self.get(e['id'])
        if not e or digest(data.get('trial')) != e['trial_fingerprint']:
            raise ValueError('Export does not match a verified registered source; quarantined')
        validate_trial(data['trial'], e['experiment'])
        return self._save(e, data['trial'])


def collections_lru():
    from collections import OrderedDict
    return OrderedDict()


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        try:
            if not self.local_request():
                return self.fail('Only loopback Host/Origin allowed', 403)
            p = urlparse(self.path); q = parse_qs(p.query)
            if p.path == '/api/catalog':
                with self.server.repo.lock:
                    return self.reply(dict(schema=SCHEMA, entries=list(self.server.repo.entries.values()), measurements=self.server.repo.measurements[-4:]))
            if p.path == '/api/trial':
                with self.server.repo.lock:
                    return self.reply(self.server.repo.get(q.get('id', [''])[0]))
            if p.path == '/api/export':
                with self.server.repo.lock:
                    data = self.server.repo.get(q.get('id', [''])[0])
                    return self.send(encoded(data), 'application/json', download='history-' + data['source']['id'] + '.json')
            if p.path == '/api/compare':
                with self.server.repo.lock:
                    left=self.server.repo.get(q.get('left',[''])[0]);right=self.server.repo.get(q.get('right',[''])[0])
                    if left['source']['experiment'] != right['source']['experiment']:
                        raise ValueError('Compare histories from the same explicit experiment schema')
                    compare=finite.comparison if left['source']['experiment']=='020' else None
                    if left['source']['experiment']=='021':
                        compare=lambda a,b,common:knowledge.comparison(a,b,lambda x,y:finite.comparison(x,y,common))
                    return self.reply(compare(left['trial'],right['trial'],comparison) if compare else comparison(left['trial'],right['trial']))
            assets = {'/': 'index.html', '/app.js': 'app.js', '/perspective.js': 'perspective.js', '/causal.js': 'causal.js', '/style.css': 'style.css'}
            if p.path not in assets:
                return self.fail('Not found', 404)
            path = HOME / assets[p.path]
            return self.send(path.read_bytes(), mimetypes.guess_type(path)[0] or 'text/plain')
        except (ValueError, KeyError, TypeError, OSError, RecursionError) as e:
            return self.fail(str(e), 400)

    def do_POST(self):
        try:
            if not self.local_request() or self.path != '/api/import':
                return self.fail('Import requires this local origin', 403)
            n = int(self.headers.get('Content-Length', '0'))
            if not 0 < n <= MAX_RECORD:
                return self.fail('Import must be 1 byte to 64 MiB', 413)
            with self.server.repo.lock:
                return self.reply(self.server.repo.imported(json.loads(self.rfile.read(n))))
        except (ValueError, KeyError, TypeError, OSError, RecursionError) as e:
            return self.fail(str(e), 400)

    def local_request(self):
        allowed = {f'127.0.0.1:{self.server.server_port}', f'localhost:{self.server.server_port}'}
        return self.headers.get('Host') in allowed and self.headers.get('Origin', 'http://' + self.headers.get('Host', '')) in {'http://' + h for h in allowed}

    def fail(self, message, status):
        return self.send(encoded(dict(error=message)), 'application/json', status)

    def reply(self, data):
        return self.send(encoded(data), 'application/json')

    def send(self, body, content_type, status=200, download=None):
        self.send_response(status)
        self.send_header('Content-Type', content_type + '; charset=utf-8')
        self.send_header('Content-Security-Policy', "default-src 'self'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self' data:; frame-ancestors 'none'; base-uri 'none'")
        self.send_header('X-Content-Type-Options', 'nosniff')
        self.send_header('Cache-Control', 'no-store')
        if download:
            self.send_header('Content-Disposition', 'attachment; filename="' + download + '"')
        if 'gzip' in self.headers.get('Accept-Encoding', '') and len(body) > 1024:
            body = gzip.compress(body, mtime=0); self.send_header('Content-Encoding', 'gzip')
        self.send_header('Content-Length', str(len(body))); self.end_headers(); self.wfile.write(body)

    def log_message(self, fmt, *args):
        print(fmt % args, flush=True)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--port', type=int, default=8765)
    ap.add_argument('--no-browser', action='store_true')
    ap.add_argument('--prepare', action='store_true', help='Verify/index and prepare authentic default pair, then exit')
    ap.add_argument('--include018', action='store_true', help='Also index the protected 018 schema (extra scan)')
    ap.add_argument('--cache', type=Path)
    args = ap.parse_args()
    print('Preparing full archive index...' if args.prepare or args.include018 else 'Opening discovery catalog; original histories are verified on selection.', flush=True)
    repo = Repository(cache=args.cache, experiments=('019','018','020','021') if args.include018 else ('019','020','021'))
    repo.prepare() if args.prepare or args.include018 else repo.catalog()
    print(f'Catalog: {len(repo.entries)} recorded trials.', flush=True)
    if repo.measurements:
        print(json.dumps(repo.measurements), flush=True)
    if args.prepare:
        print('Prepared catalog and authentic useful_contact pair in', repo.cache); return
    # Browsers pre-open idle connections. Threads keep those from blocking assets;
    # the repository lock still serializes expensive archive/import/cache work.
    server = ThreadingHTTPServer(('127.0.0.1', args.port), Handler)
    server.repo = repo
    print(f'History Explorer: http://127.0.0.1:{server.server_port}', flush=True)
    if not args.no_browser:
        webbrowser.open(f'http://127.0.0.1:{server.server_port}')
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == '__main__':
    main()
