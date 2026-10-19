"""Explicit native022 adapter for the main Explorer. Read-only observation only.

No experiment runner, policy, scientific reader or prepared comparison is imported.
Actor views refer exclusively to captured owned inputs. Graph joins are observer
navigation over original IDs, never new cognition or timestamp-inferred causation.
"""
import collections
import copy
import gzip
import hashlib
import json
import re
from pathlib import Path

VERSION = 'experiment-022-trial/v1'
EXTRACTOR = 5
SCHEMA = 'woi-history-explorer/v1'


def encoded(value):
    return json.dumps(value, ensure_ascii=True, separators=(',', ':'), sort_keys=True).encode()


def digest(value):
    return hashlib.sha256(encoded(value)).hexdigest()


def core(trial):
    value, path = trial['final_state'], 'final_state'
    while 'events' not in value:
        value, path = value['base'], path + '.base'
    return value, path


def resolve(trial, path):
    parts = re.findall(r'([A-Za-z_][A-Za-z_0-9]*)|\[(\d+)\]', path)
    rebuilt = ''.join(('.' if n else '') + field if field else '[' + index + ']'
                      for n, (field, index) in enumerate(parts))
    if rebuilt != path:
        raise ValueError('Unsupported native original reference')
    value = trial
    for field, index in parts:
        value = value[field] if field else value[int(index)]
    return value


def work(commitment):
    return 'Resource' if isinstance(commitment['work'], dict) else commitment['work']


def validate(trial):
    if trial.get('schema') != VERSION or len(trial.get('initial', [])) != 4:
        raise ValueError('Unsupported native022 schema; no021 conversion allowed')
    if [a['id'] for a in trial['initial']] != [0, 1, 2, 3]:
        raise ValueError('Invalid native022 individual identities')
    if trial['config']['capacity'] not in (1, 2):
        raise ValueError('Invalid native022 social capacity')
    previous = trial['start'] - 1
    for boundary in trial['boundaries']:
        if not previous < boundary['tick'] <= trial['end']:
            raise ValueError('Invalid native022 boundary chronology')
        previous = boundary['tick']
        if [a['id'] for a in boundary['agents']] != [0, 1, 2, 3]:
            raise ValueError('Invalid captured four-person state')
        for choice in boundary['choices']:
            owner = choice['input']['legacy']['own']['id']
            if not 0 <= owner < 4 or choice['input']['legacy']['now'] != boundary['tick']:
                raise ValueError('Invalid captured native022 input')
            if choice['input']['retained'] is not None and choice['input']['retained']['owner'] != owner:
                raise ValueError('Foreign retained environmental slot')
    intervals(trial)


def intervals(trial):
    starts, ends, result = {}, {}, []
    for index, record in enumerate(trial['transitions']):
        cid = record['commitment']['id']
        target = ends if record['completed'] else starts
        if cid in target:
            raise ValueError('Duplicate native022 commitment transition')
        target[cid] = index, record
    if starts.keys() != ends.keys():
        raise ValueError('Original022 commitment lacks actual release')
    for cid, (index, record) in starts.items():
        end_index, end = ends[cid]
        c = record['commitment']
        if end['commitment'] != c or not c['start'] < end['tick'] <= c['deadline']:
            raise ValueError('Invalid native022 interval identity')
        result.append(dict(id=cid, kind=work(c), owners=c['participants'], start=c['start'],
                           end=end['tick'], deadline=c['deadline'], source=c['source'],
                           start_ref=f'transitions[{index}]', release_ref=f'transitions[{end_index}]',
                           scene=c['work']['Resource']['scene'] if work(c) == 'Resource' else None))
    return result


def timeline(trial):
    moments = []
    def add(tick, kind, ref, label, owners=(), **extra):
        moments.append(dict(tick=tick, kind=kind, ref=ref, label=label, owners=list(owners), **extra))
    add(trial['start'], 'snapshot', 'initial', 'Declared initial state after bootstrap', phase=-1)
    for index, boundary in enumerate(trial['boundaries']):
        add(boundary['tick'], 'decision', f'boundaries[{index}]', 'Captured boundary choices',
            [d['input']['legacy']['own']['id'] for d in boundary['choices']], boundary=index, phase=13)
        for number, response in enumerate(boundary['responses']):
            own = response['input']['own']
            add(own['now'], 'reply', f'boundaries[{index}].responses[{number}]',
                f"Independent reply by individual {own['own']['id']}", [own['own']['id']], phase=8)
    for index, record in enumerate(trial['transitions']):
        c = record['commitment']; kind = work(c)
        phase = (2 if kind == 'Resource' else 4 if kind == 'Claim' else 6 if kind == 'Meal' else 7) if record['completed'] else (9 if kind == 'Reply' else 11 if kind in ('Resource', 'Query') else 14)
        add(record['tick'], 'release' if record['completed'] else 'commitment', f'transitions[{index}]',
            f"{kind} #{c['id']} · {'released' if record['completed'] else 'started'}", c['participants'], phase=phase)
    for index, admission in enumerate(trial['admissions']):
        i = admission['invitation']
        add(admission['tick'], 'admission', f'admissions[{index}]',
            f"{i['proposer']} → {i['recipient']} · {i['kind']}: {admission['reason']}", [i['proposer'], i['recipient']], phase=10)
    b, path = core(trial)
    for index, event in enumerate(b['events']):
        if index < trial['bootstrap_events']:
            continue
        add(event['tick'], 'resource', f'{path}.events[{index}]',
            f"Scene {event['scene']} · individual {event['decision']['actor']}: {event['decision']['selected']}",
            event['participants'], phase=1)
    for index, grant in enumerate(trial['grants']):
        add(grant['tick'], 'acquisition', f'grants[{index}]', f"Claim #{grant['commitment']}: {grant['outcome']}", [grant['owner']], phase=3)
    for index, meal in enumerate(b['cognition']['consumption']):
        add(meal['tick'], 'meal', f'{path}.cognition.consumption[{index}]', f"Individual {meal['actor']} eats", [meal['actor']], phase=5)
    for index, need in enumerate(trial['needs']):
        add(need['tick'], 'need', f'needs[{index}]', 'One shared four-person need pulse', range(4), phase=0)
    for index, record in enumerate(trial['memories']):
        add(record['tick'], 'environmental_memory', f'memories[{index}]',
            f"Individual {record['owner']} · {record['kind']}", [record['owner']], phase=12)
    for index, receipt in enumerate(b['cognition']['information']):
        if receipt['tick'] >= trial['start']:
            add(receipt['tick'], 'information', f'{path}.cognition.information[{index}]',
                f"Information receipt {receipt['id']} · listener {receipt['listener']}", [receipt['listener']], phase=7)
    queries = trial['final_state']['base']['base']['base']['base']['inquiry']['records']
    for index, query in enumerate(queries):
        add(query['tick'], 'inquiry', f'final_state.base.base.base.base.inquiry.records[{index}]',
            f"Executed inquiry {query['id']} · individual {query['decision']['input']['owner']}",
            [query['decision']['input']['owner']], input_tick=query['tick'] - query['time_spent'], phase=7)
    add(trial['end'], 'snapshot', 'final_agents', 'Final recorded owned states', range(4), phase=20)
    moments.sort(key=lambda m: (m['tick'], m['phase'], m['ref']))
    for number, moment in enumerate(moments):
        moment['id'] = number
        resolve(trial, moment['ref'])
    return moments


def actor(trial, moment, owner):
    """Never backfill a missing input from a boundary, audit or nearest snapshot."""
    if moment['kind'] == 'decision':
        boundary = resolve(trial, moment['ref'])
        for number, choice in enumerate(boundary['choices']):
            if choice['input']['legacy']['own']['id'] == owner:
                return dict(phase='activity', ref=moment['ref'] + f'.choices[{number}]',
                            input=choice['input'], selected=choice['selected'], candidates=choice['candidates'],
                            contact=choice['contact'], contact_candidates=choice['contact_candidates'])
    elif moment['kind'] == 'reply':
        response = resolve(trial, moment['ref'])
        if response['input']['own']['own']['id'] == owner:
            return dict(phase='recipient', ref=moment['ref'], **response)
    elif moment['kind'] == 'resource':
        decision = resolve(trial, moment['ref'])['decision']
        if decision['actor'] == owner:
            return dict(phase='resource', ref=moment['ref'] + '.decision', **decision)
    elif moment['kind'] == 'inquiry':
        decision = resolve(trial, moment['ref'])['decision']
        if decision['input']['owner'] == owner:
            # The full response contains partner-private decision context. Only
            # the actor's original selection input is an owned policy projection.
            return dict(phase='inquiry selection', ref=moment['ref'] + '.decision',
                        input_tick=moment['input_tick'], **decision)
    elif moment['kind'] == 'snapshot':
        own = resolve(trial, moment['ref'])[owner]
        result = dict(phase='owned snapshot', ref=moment['ref'] + f'[{owner}]', own=own)
        if moment['ref'] == 'final_agents':
            result['retained'] = trial['final_memory'][owner]
        return result
    return None


def causal(trial, moments):
    nodes = {m['ref']: dict(ref=m['ref'], anchor=m['ref'], tick=m['tick'], kind=m['kind'],
                            owners=m['owners'], label=m['label']) for m in moments}
    edges, chains = [], []
    spans = {c['id']: c for c in intervals(trial)}
    b, base_path = core(trial)
    reply_refs = {}
    invitation_refs = {}
    for boundary_index, boundary in enumerate(trial['boundaries']):
        for pending_index, pending in enumerate(boundary['pending']):
            i = pending['invitation']
            key = pending['source'], i['proposer'], i['recipient'], i['expires']
            invite_ref = f'boundaries[{boundary_index}].pending[{pending_index}].invitation'
            invitation_refs.setdefault(key, invite_ref)
            nodes[invite_ref] = dict(ref=invite_ref, anchor=f'boundaries[{boundary_index}]', tick=boundary['tick'],
                                     kind='invitation', owners=[i['proposer'], i['recipient']],
                                     label='Original proposal captured in pending protocol')
            if pending['response'] is not None:
                r = pending['response']; i = pending['invitation']
                reply_refs[pending['source'], i['proposer'], i['recipient'], i['expires']] = f"boundaries[{r['boundary']}].responses[{r['response']}]"

    def choice_node(boundary_index, owner):
        boundary = trial['boundaries'][boundary_index]
        for index, choice in enumerate(boundary['choices']):
            if choice['input']['legacy']['own']['id'] == owner:
                ref = f'boundaries[{boundary_index}].choices[{index}]'
                nodes[ref] = dict(ref=ref, anchor=f'boundaries[{boundary_index}]', tick=boundary['tick'],
                                  kind='choice', owners=[owner], label=f'Individual {owner}: {choice["selected"]}')
                return ref
        return None

    for number, admission in enumerate(trial['admissions']):
        i = admission['invitation']; ref = f'admissions[{number}]'
        choice = choice_node(admission['source'], i['proposer'])
        if choice is None:
            raise ValueError('Native admission has no original sending choice')
        cohort = [choice]
        send = next((c for c in spans.values() if c['kind'] == 'Send' and c['source'] == admission['source'] and c['owners'] == [i['proposer']]), None)
        if send:
            cohort.append(send['start_ref'])
            edges.append(dict(source=choice, target=send['start_ref'], kind='selected_send'))
        proposal = invitation_refs.get((admission['source'], i['proposer'], i['recipient'], i['expires']))
        if proposal:
            cohort.append(proposal)
            edges.append(dict(source=choice, target=proposal, kind='same_invitation_source'))
        response = reply_refs.get((admission['source'], i['proposer'], i['recipient'], i['expires']))
        if response:
            cohort.append(response)
            if proposal:
                edges.append(dict(source=proposal, target=response, kind='recorded_recipient_opportunity'))
            edges.append(dict(source=response, target=ref, kind='recipient_record_considered'))
        cohort.append(ref)
        edges.append(dict(source=choice, target=ref, kind='invitation_source'))
        if admission['commitment'] is not None:
            c = spans[admission['commitment']]
            cohort.append(c['start_ref'])
            edges.append(dict(source=ref, target=c['start_ref'], kind='admitted_commitment'))
            if c['kind'] == 'Resource':
                for index, event in enumerate(b['events']):
                    if event['scene'] == c['scene']:
                        event_ref = f'{base_path}.events[{index}]'
                        cohort.append(event_ref)
                        edges.append(dict(source=c['start_ref'], target=event_ref, kind='scene_identity'))
            cohort.append(c['release_ref'])
            edges.append(dict(source=c['start_ref'], target=c['release_ref'], kind='same_commitment_release'))
            for owner in c['owners']:
                later = next((index for index, boundary in enumerate(trial['boundaries'])
                              if boundary['tick'] >= c['end'] and any(d['input']['legacy']['own']['id'] == owner for d in boundary['choices'])), None)
                if later is not None:
                    target = choice_node(later, owner)
                    cohort.append(target)
                    edges.append(dict(source=c['release_ref'], target=target, kind='availability_context_not_unique_motive'))
        chains.append(dict(id=ref, label=f"{i['proposer']} → {i['recipient']} · {i['kind']} · {admission['reason']} @ {admission['tick']}", nodes=cohort))
    for c in spans.values():
        if c['kind'] not in ('Claim', 'Meal'):
            continue
        owner = c['owners'][0]
        choice = choice_node(c['source'], owner)
        cohort = [choice, c['start_ref']]
        edges.append(dict(source=choice, target=c['start_ref'], kind='selected_physical_work'))
        if c['kind'] == 'Claim':
            results = [f'grants[{index}]' for index, grant in enumerate(trial['grants']) if grant['commitment'] == c['id']]
        else:
            results = [f'{base_path}.cognition.consumption[{index}]' for index, meal in enumerate(b['cognition']['consumption']) if meal['actor'] == owner and meal['tick'] == c['end']]
        for target in results:
            cohort.append(target)
            edges.append(dict(source=c['start_ref'], target=target, kind='same_commitment_result'))
        cohort.append(c['release_ref'])
        edges.append(dict(source=c['start_ref'], target=c['release_ref'], kind='same_commitment_release'))
        chains.append(dict(id=c['start_ref'], label=f"Individual {owner} · {c['kind']} #{c['id']} · {c['start']}–{c['end']}", nodes=cohort))
    for ref in nodes:
        resolve(trial, ref)
    return dict(nodes=nodes, edges=edges, chains=chains,
                note='Joins use original invitation sources, reply references, commitment and scene IDs. Later input is availability context, not proof of a unique motive.')


class Source:
    """Stream source discovery; retain at most two decoded original members."""
    def __init__(self, root):
        self.root = Path(root)
        self.directory = self.root / 'experiments/022'
        self.entries = {}
        self.loaded = collections.OrderedDict()
        self.manifest = None

    def guard(self):
        d = self.directory
        inventory_path = self.root / 'docs/foundation-001-022-sha256.json'
        inventory = json.loads(inventory_path.read_bytes()) if inventory_path.exists() else {}
        for name in ('archive-manifest.json', 'source-freeze.json'):
            data = (d / name).read_bytes().replace(b'\r\n', b'\n')
            if inventory and hashlib.sha256(data).hexdigest() != inventory.get('experiments/022/' + name):
                raise ValueError('Protected022 source manifest changed')
        manifest = json.loads((d / 'archive-manifest.json').read_bytes())
        freeze = json.loads((d / 'source-freeze.json').read_bytes())
        for name, expected in freeze.items():
            if hashlib.sha256((self.root / name).read_bytes().replace(b'\r\n', b'\n')).hexdigest() != expected:
                raise ValueError('Original022 source lock changed: ' + name)
        archive = d / 'trials.jsonl.gz'; expected = manifest['trials.jsonl.gz']
        if inventory and inventory.get('experiments/022/trials.jsonl.gz') != expected['gzip_sha256']:
            raise ValueError('Protected022 archive identity changed')
        if archive.stat().st_size != expected['gzip_bytes'] or hashlib.sha256(archive.read_bytes()).hexdigest() != expected['gzip_sha256']:
            raise ValueError('Original022 compressed archive changed')
        self.manifest = manifest
        return hashlib.sha256((d / 'source-freeze.json').read_bytes().replace(b'\r\n', b'\n')).hexdigest()

    def catalog(self, status):
        freeze = self.guard()
        matrix = json.loads((self.directory / 'registered-matrix.json').read_bytes())
        expected = self.manifest['trials.jsonl.gz']
        decoded = hashlib.sha256(); size = 0; entries = {}
        with gzip.open(self.directory / 'trials.jsonl.gz', 'rb') as stream:
            for ordinal, line in enumerate(stream):
                if len(line) > 64 * 1024 * 1024:
                    raise ValueError('Native022 original exceeds input bound')
                decoded.update(line); size += len(line)
                trial = json.loads(line); validate(trial)
                if trial['config'] != matrix[ordinal]:
                    raise ValueError('Native022 registration identity mismatch')
                config = trial['config']
                identity = dict(experiment='022', archive='trials.jsonl.gz', record=ordinal,
                                seed=str(config['seed']), case=config['case'], mode=f"Capacity{config['capacity']}", config=config)
                key = digest(dict(identity=identity, archive_sha256=expected['gzip_sha256'], schema=SCHEMA))[:24]
                entries[key] = dict(id=key, **identity, source_schema=VERSION,
                                    archive_sha256=expected['gzip_sha256'], decoded_archive_sha256=expected['json_sha256'],
                                    record_sha256=hashlib.sha256(line.rstrip(b'\r\n')).hexdigest(), trial_fingerprint=digest(trial),
                                    policy_freeze_sha256=freeze, epoch=trial['start'], end=trial['end'], raw_bytes=len(line),
                                    individual_ids=[0, 1, 2, 3], scientific_status=status,
                                    integrity='Complete gzip/raw archive, source locks and native original member verified')
        if len(entries) != len(matrix) or len(entries) != expected['records'] or (size, decoded.hexdigest()) != (expected['json_bytes'], expected['json_sha256']):
            raise ValueError('Incomplete native022 original archive')
        self.entries = entries
        return entries

    def get(self, entry):
        self.guard()
        key = entry['id']
        if key not in self.entries or any(entry[k] != self.entries[key][k] for k in ('record', 'record_sha256', 'trial_fingerprint')):
            raise ValueError('Unregistered native022 original identity')
        if key in self.loaded:
            self.loaded.move_to_end(key)
            return self.loaded[key]
        with gzip.open(self.directory / entry['archive'], 'rb') as stream:
            for number, line in enumerate(stream):
                if number != entry['record']:
                    continue
                if hashlib.sha256(line.rstrip(b'\r\n')).hexdigest() != entry['record_sha256']:
                    raise ValueError('Native022 member changed')
                trial = json.loads(line)
                if digest(trial) != entry['trial_fingerprint']:
                    raise ValueError('Native022 trial fingerprint changed')
                validate(trial)
                self.loaded[key] = trial
                while len(self.loaded) > 2:
                    self.loaded.popitem(last=False)
                return trial
        raise ValueError('Missing native022 original ordinal')


def export(entry, trial):
    moments = timeline(trial)
    return dict(schema=SCHEMA, extractor_version=EXTRACTOR, native_concurrency=True,
                source=copy.deepcopy(entry), trial=trial, timeline=moments,
                intervals=intervals(trial), causal=causal(trial, moments),
                observer_physical=observer_points(trial)[0])


def observer_points(trial):
    """Small material vectors from physical records, never an actor projection."""
    b, path = core(trial)
    ops = collections.defaultdict(list)
    for index, event in enumerate(b['events']):
        if index >= trial['bootstrap_events']:
            ops[event['tick']].append((1, index, 'event', event, f'{path}.events[{index}]'))
    for index, grant in enumerate(trial['grants']):
        ops[grant['tick']].append((2, index, 'grant', grant, f'grants[{index}]'))
    for index, meal in enumerate(b['cognition']['consumption']):
        ops[meal['tick']].append((3, index, 'meal', meal, f'{path}.cognition.consumption[{index}]'))
    for index, need in enumerate(trial['needs']):
        ops[need['tick']].append((0, index, 'need', need, f'needs[{index}]'))
    food = [a['food'] for a in trial['initial']]
    hunger = [a['hunger'] for a in trial['initial']]
    stock, consumed, points, area = trial['initial_stock'], 0, [], [0] * 4
    for tick in range(trial['start'], trial['end'] + 1):
        if tick > trial['start']:
            area = [x + y for x, y in zip(area, hunger)]
        refs = []
        for _, _, kind, record, ref in sorted(ops[tick]):
            refs.append(ref)
            if kind == 'need':
                hunger = list(record['after'])
            elif kind == 'event':
                for owner, value in zip(record['participants'], record['balances_after']):
                    food[owner] = value
            elif kind == 'grant':
                stock = record['stock_after']; food[record['owner']] = record['food_after']
            else:
                food[record['actor']] = record['food_after']
                hunger[record['actor']] = record['hunger_after']; consumed += record['consumed']
        points.append(dict(tick=tick, food=list(food), hunger=list(hunger), stock=stock, consumed=consumed, refs=refs))
    return points, area


def comparison(left, right):
    a, b = left['trial'], right['trial']
    ca, cb = dict(a['config']), dict(b['config'])
    ca.pop('capacity'); cb.pop('capacity')
    pa, area_a = observer_points(a); pb, area_b = observer_points(b)
    ta, tb = {p['tick']: p for p in pa}, {p['tick']: p for p in pb}
    common = sorted(ta.keys() & tb.keys())
    def material(p):
        return p['food'], p['stock'], p['consumed']
    differences = [tick for tick in common if material(ta[tick]) != material(tb[tick])]
    def first_lists(left_records, right_records, projection):
        aa, bb = collections.defaultdict(list), collections.defaultdict(list)
        for ref, record, tick in left_records:
            aa[tick].append(dict(ref=ref, value=projection(record)))
        for ref, record, tick in right_records:
            bb[tick].append(dict(ref=ref, value=projection(record)))
        for tick in sorted(aa.keys() | bb.keys()):
            if [x['value'] for x in aa[tick]] != [x['value'] for x in bb[tick]]:
                size = max(len(aa[tick]), len(bb[tick]))
                index = next(n for n in range(size) if n >= len(aa[tick]) or n >= len(bb[tick]) or aa[tick][n]['value'] != bb[tick][n]['value'])
                return dict(tick=tick, left=aa[tick], right=bb[tick],
                            left_difference=aa[tick][index] if index < len(aa[tick]) else None,
                            right_difference=bb[tick][index] if index < len(bb[tick]) else None)
        return None
    admissions = lambda t: [(f'admissions[{n}]', r, r['tick']) for n, r in enumerate(t['admissions'])]
    admission_projection = lambda r: dict(invitation=r['invitation'], reason=r['reason'])
    def executed(t):
        base_value, base_ref = core(t)
        records = [(f'{base_ref}.events[{n}]', dict(kind='resource', participants=e['participants'], action=e['decision']['selected'], transferred=e['transferred']), e['tick']) for n, e in enumerate(base_value['events']) if n >= t['bootstrap_events']]
        records += [(f'grants[{n}]', dict(kind='claim', owner=g['owner'], outcome=g['outcome']), g['tick']) for n, g in enumerate(t['grants'])]
        records += [(f'{base_ref}.cognition.consumption[{n}]', dict(kind='meal', owner=m['actor']), m['tick']) for n, m in enumerate(base_value['cognition']['consumption'])]
        return sorted(records, key=lambda r: (r[2], {'resource': 1, 'claim': 2, 'meal': 3}[r[1]['kind']], int(re.findall(r'\[(\d+)\]', r[0])[-1])))
    def choices(t):
        return {(boundary['tick'], d['input']['legacy']['own']['id']):
                dict(ref=f'boundaries[{n}].choices[{j}]', selected=d['selected'], contact=d['contact'] if d['selected'] == 'Invite' else None)
                for n, boundary in enumerate(t['boundaries']) for j, d in enumerate(boundary['choices'])}
    aa, bb = choices(a), choices(b)
    first_opportunity = next((dict(tick=k[0], owner=k[1], left=aa.get(k), right=bb.get(k)) for k in sorted(aa.keys() | bb.keys()) if k not in aa or k not in bb), None)
    first_choice = next((dict(tick=k[0], owner=k[1], left=aa[k], right=bb[k]) for k in sorted(aa.keys() & bb.keys()) if (aa[k]['selected'], aa[k]['contact']) != (bb[k]['selected'], bb[k]['contact'])), None)
    trust = lambda t: [actor['trust'].get(str(other), 0) for actor in t['final_agents'] for other in range(4) if other != actor['id']]
    metrics = lambda t: dict(meals=len(core(t)[0]['cognition']['consumption']), grants=sum(g['outcome'] == 'Granted' for g in t['grants']), depleted=sum(g['outcome'] == 'Depleted' for g in t['grants']), transfers=sum(e['transferred'] for e in core(t)[0]['events'][t['bootstrap_events']:]), social=len([c for c in intervals(t) if c['kind'] == 'Resource']))
    return dict(native_concurrency=True, matched_except_capacity=ca == cb,
                capacity_contrast={a['config']['capacity'], b['config']['capacity']} == {1, 2},
                initial_state_and_profiles_match=all(a[k] == b[k] for k in ('initial', 'initial_stock', 'profiles', 'initial_contexts')),
                equal_horizon=(a['start'], a['end']) == (b['start'], b['end']),
                first_admission=first_lists(admissions(a), admissions(b), admission_projection),
                first_executed=first_lists(executed(a), executed(b), lambda r: r),
                first_opportunity=first_opportunity, first_common_selected_choice=first_choice,
                first_material=differences[0] if differences else None,
                material_reconvergence=next((tick for tick in common if differences and tick > differences[0] and material(ta[tick]) == material(tb[tick])), None),
                terminal_food_equal=material(pa[-1]) == material(pb[-1]), terminal_hunger_equal=pa[-1]['hunger'] == pb[-1]['hunger'],
                semantic_trust_equal=trust(a) == trust(b), raw_trust_equal=[x['trust'] for x in a['final_agents']] == [x['trust'] for x in b['final_agents']],
                complete_null_except_capacity=dict(a, config=ca) == dict(b, config=cb),
                hunger_area_right_minus_left=[y-x for x, y in zip(area_a, area_b)],
                left_metrics=metrics(a), right_metrics=metrics(b), left_physical=pa, right_physical=pb,
                note='Observer-derived comparison. First missing decision opportunity, admission, execution and common selected choice are separate. Post-divergence inputs need not match; more contact or lower hunger area is not universal benefit.')
