"""Explicit025 observer adapter. No simulator, scientific-reader or cache imports."""
import collections
import copy
import gzip
import hashlib
import importlib.util
import json
import re
from pathlib import Path

_spec = importlib.util.spec_from_file_location('viewer025_valuation_primitives', Path(__file__).with_name('valuation_history.py'))
valuation = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(valuation)
native = valuation.native
VERSION = 'experiment-025-trial/v1'
PHYSICAL = 'experiment-025-physical-history/v1'
EXTRACTOR = 9


def observer_points(original):
    h = original['history']; mat = original['material']; base, path = native.core(h)
    operations = collections.defaultdict(list)
    for n, event in enumerate(base['events']):
        if n >= h['bootstrap_events']:
            operations[event['tick']].append((1, n, 'event', event, f'{path}.events[{n}]'))
    for n, grant in enumerate(h['grants']):
        operations[grant['tick']].append((2, n, 'grant', grant, f'grants[{n}]'))
    for n, meal in enumerate(base['cognition']['consumption']):
        operations[meal['tick']].append((3, n, 'meal', meal, f'{path}.cognition.consumption[{n}]'))
    for n, need in enumerate(h['needs']):
        operations[need['tick']].append((0, n, 'need', need, f'needs[{n}]'))
    for n, op in enumerate(mat['operations']):
        operations[op['tick']].append((4, n, 'preparation', op, f'material.operations[{n}]'))
    food = [a['food'] for a in h['initial']]; hunger = [a['hunger'] for a in h['initial']]
    stock = h['initial_stock']; raw = mat['initial_raw']; waste = consumed = 0
    escrow = [0] * 4; points = []
    for tick in range(h['start'], h['end'] + 1):
        refs = []
        for _, _, kind, record, ref in sorted(operations[tick]):
            refs.append(ref)
            if kind == 'need':
                hunger = list(record['after'])
            elif kind == 'event':
                for owner, value in zip(record['participants'], record['balances_after']):
                    food[owner] = value
            elif kind == 'grant':
                stock = record['stock_after']; food[record['owner']] = record['food_after']
            elif kind == 'meal':
                food[record['actor']] = record['food_after']; hunger[record['actor']] = record['hunger_after']
                consumed += record['consumed']
            else:
                owner = record['owner']
                if owner not in range(4) or record['source'] != 0:
                    raise ValueError('025unknown preparation owner/source')
                if (raw, escrow[owner], food[owner], waste) != (record['raw_before'], record['escrow_before'], record['food_before'], record['waste_before']):
                    raise ValueError('025physical operation before-state mismatch')
                delta = (record['raw_after'] - raw, record['escrow_after'] - escrow[owner],
                         record['food_after'] - food[owner], record['waste_after'] - waste)
                required = {'Extracted': (-2, 2, 0, 0), 'Insufficient': (0, 0, 0, 0),
                            'Prepared': (0, -2, 1, 1), 'Failed': (0, 0, 0, 0)}
                if record['outcome'] not in required or delta != required[record['outcome']]:
                    raise ValueError('025conversion/extraction law mismatch')
                if record['outcome'] == 'Insufficient' and raw >= 2:
                    raise ValueError('025insufficient outcome despite sufficient material')
                raw, escrow[owner], food[owner], waste = record['raw_after'], record['escrow_after'], record['food_after'], record['waste_after']
        if raw + sum(escrow) + stock + sum(food) + waste + consumed != h['initial_total']:
            raise ValueError('025common-unit conservation failed')
        points.append(dict(tick=tick, food=list(food), hunger=list(hunger), stock=stock, consumed=consumed,
                           raw=raw, escrow=list(escrow), waste=waste, refs=refs))
    if (raw, waste, food, stock) != (mat['final_raw'], mat['final_waste'], [a['food'] for a in h['final_agents']], h['final_stock']):
        raise ValueError('025terminal material mismatch')
    return points


def validate(original):
    if set(original) != {'schema', 'config', 'history', 'checkpoints', 'valuations', 'material'} or original['schema'] != VERSION:
        raise ValueError('Unsupported025original envelope')
    h = original['history']; cfg = original['config']; world = cfg['world']; settings = cfg['preparation']
    if not (1 <= world['horizon'] <= 1024 and 1 <= world['period'] <= 128 and world['capacity'] == 2
            and 0 <= settings['raw_initial'] <= 16 and settings['source_id'] == 0
            and settings['extraction_ticks'] > 0 and settings['preparation_ticks'] > 0
            and settings['extraction_ticks'] + settings['preparation_ticks'] <= 33):
        raise ValueError('025configuration exceeds supported bounded geometry')
    native.validate(h, version=PHYSICAL)
    if h['end'] != h['start'] + cfg['world']['horizon'] + 33 or cfg['valuation'] not in ('Baseline', 'MealRelief'):
        raise ValueError('Invalid025endpoint/valuation')
    decisions = {(bi, di): d for bi, b in enumerate(h['boundaries']) for di, d in enumerate(b['choices'])}
    seen = set()
    for audit in original['valuations']:
        key = audit['boundary'], audit['choice']
        if key in seen or key not in decisions or audit['policy'] != cfg['valuation']:
            raise ValueError('Invalid025captured score identity')
        seen.add(key); d = decisions[key]; terms = audit['selection']['terms']; own = d['input']['legacy']['own']
        if terms != d['candidates'] or [t['action'] for t in terms] != ['Wait', 'Consume', 'Invite', 'Acquire', 'Prepare']:
            raise ValueError('Invalid025five-way terms')
        cell = d['input']['raw_retained']
        if cell is not None and (cell['owner'] != own['id'] or d['input']['retained'] is not None):
            raise ValueError('Foreign or simultaneous025source slots')
        for term in terms:
            if term['base_score'] + term['meal_relief'] - term['work_effort'] != term['final_score']:
                raise ValueError('Invalid025score arithmetic')
        term = terms[-1]
        if term['eligible']:
            if cell is None:
                raise ValueError('025Prepare without owned source observation')
            ob = cell['observation']; duration = ob['extraction_ticks'] + ob['preparation_ticks']; legacy = d['input']['legacy']
            if not (own['food'] == 0 and own['hunger'] > 0 and ob['accessible'] and ob['id'] == 0
                    and ob['observed_at'] <= legacy['now'] and ob['stock'] - cell['own_takes'] >= 2
                    and ob['input_units'] == 2 and ob['edible_units'] == ob['waste_units'] == 1
                    and ob['extraction_ticks'] > 0 and ob['preparation_ticks'] > 0 and duration <= 33
                    and legacy['now'] + duration <= legacy['end']
                    and term['base_score'] == own['hunger'] and term['work_effort'] == duration):
                raise ValueError('025Prepare violates captured local boundary')
        if max(terms, key=lambda t: t['final_score'])['action'] != d['selected'] or d['selected'] != audit['selection']['selected']:
            raise ValueError('Invalid025stable selected action')
    if seen != set(decisions):
        raise ValueError('Missing025score capture')
    observer_points(original)


class Source:
    experiment = '025'

    def __init__(self, root, path):
        self.root = Path(root).resolve(); self.path = Path(path).resolve()
        if self.path.is_dir(): self.path = self.path / 'trials.jsonl.gz'
        self.registered = self.path == self.root / 'experiments/025/trials.jsonl.gz'
        self.entries = {}; self.loaded = collections.OrderedDict(); self.expected = None

    def guard(self):
        freeze = self.root / 'experiments/025/source-freeze-observer2.json'
        for name, expected in json.loads(freeze.read_bytes()).items():
            if hashlib.sha256((self.root / name).read_bytes().replace(b'\r\n', b'\n')).hexdigest() != expected:
                raise ValueError('025runtime source binding changed: ' + name)
        value = hashlib.sha256(self.path.read_bytes()).hexdigest()
        if self.expected is not None and value != self.expected: raise ValueError('025source changed after catalog')
        if self.registered:
            expected = json.loads((self.path.parent / 'archive-manifest.json').read_bytes())['archives']['trials.jsonl.gz']
            if value != expected['sha256']: raise ValueError('025registered archive changed')
        return hashlib.sha256(freeze.read_bytes().replace(b'\r\n', b'\n')).hexdigest(), value

    def lines(self):
        with (gzip.open(self.path, 'rb') if self.path.suffix == '.gz' else self.path.open('rb')) as stream:
            for line in stream:
                if len(line) > 64 * 1024 * 1024: raise ValueError('025member exceeds64MiB bound')
                yield line

    def catalog(self, status):
        freeze, archive = self.guard(); self.expected = archive; decoded = hashlib.sha256(); entries = {}
        matrix = json.loads((self.root / 'experiments/025/registered-matrix.json').read_bytes()) if self.registered else None
        for ordinal, line in enumerate(self.lines()):
            decoded.update(line); original = json.loads(line); validate(original); cfg = original['config']; h = original['history']
            if matrix is not None and (ordinal >= len(matrix) or cfg != matrix[ordinal]): raise ValueError('025registration mismatch')
            identity = dict(experiment='025', archive=self.path.name, record=ordinal, seed=str(cfg['world']['seed']),
                            case=cfg['world']['case'], mode='Access enabled' if cfg['preparation']['access_enabled'] else 'Access closed', config=cfg)
            key = native.digest(dict(identity=identity, archive_sha256=archive, schema=VERSION))[:24]
            entries[key] = dict(id=key, **identity, source_schema=VERSION, physical_schema=PHYSICAL,
                archive_sha256=archive, record_sha256=hashlib.sha256(line.rstrip(b'\r\n')).hexdigest(),
                trial_fingerprint=native.digest(original), policy_freeze_sha256=freeze, epoch=h['start'], end=h['end'],
                nominal_stop=h['start'] + cfg['world']['horizon'], raw_bytes=len(line), individual_ids=[0, 1, 2, 3],
                original_reference_root='history; material is envelope-level', registered=self.registered,
                scientific_status=status if self.registered else 'Custom025prototype; not registered/protected evidence',
                integrity='Original025observer2source and archive/member hashes verified; scientific gate separate')
        if not entries: raise ValueError('Empty025source')
        if matrix is not None:
            expected = json.loads((self.path.parent / 'archive-manifest.json').read_bytes())['archives']['trials.jsonl.gz']
            if len(entries) != 50 or decoded.hexdigest() != expected['decoded_sha256']: raise ValueError('Incomplete025archive')
        for entry in entries.values(): entry['decoded_archive_sha256'] = decoded.hexdigest()
        self.entries = entries
        return entries

    def get(self, entry):
        self.guard(); key = entry['id']
        if key not in self.entries or entry['trial_fingerprint'] != self.entries[key]['trial_fingerprint']: raise ValueError('Unregistered025member')
        if key in self.loaded: self.loaded.move_to_end(key); return self.loaded[key]
        for n, line in enumerate(self.lines()):
            if n != entry['record']: continue
            if hashlib.sha256(line.rstrip(b'\r\n')).hexdigest() != entry['record_sha256']: raise ValueError('025member changed')
            original = json.loads(line); validate(original)
            if native.digest(original) != entry['trial_fingerprint']: raise ValueError('025original changed')
            self.loaded[key] = original
            while len(self.loaded) > 2: self.loaded.popitem(last=False)
            return original
        raise ValueError('Missing025member')


def rebuild(data):
    return dict(schema=VERSION, config=data['valuation']['config'], history=data['trial'],
                checkpoints=data['continuous']['checkpoints'], valuations=data['valuation']['valuations'],
                material=data['preparation']['material'])


def order_timeline(data):
    """025presentation phases preserve physical order and linked slot replacement.

    This is a projection of original records, not additional simulated time.
    Old adapters keep their existing presentation. Original IDs remain untouched.
    """
    material = data['preparation']['material']; ready = data['trial']['memories']
    for moment in data['timeline']:
        indexes = re.findall(r'\[(\d+)\]', moment['ref'])
        index = int(indexes[-1]) if indexes else 0
        moment['sequence'] = index
        if moment['kind'] == 'preparation':
            moment['phase'] = 7.1; moment['sequence'] = index * 2
        elif moment['kind'] == 'raw_memory':
            receipt = material['memories'][index]
            linked = ready[receipt['cache_record']] if receipt['cache_record'] is not None else None
            if receipt['physical'] is not None:
                moment['phase'] = 7.1; moment['sequence'] = receipt['physical'] * 2 + 1
            elif linked is not None and linked['physical'] is not None:
                moment['phase'] = 3; moment['sequence'] = linked['physical'] * 3 + 1
            else:
                moment['sequence'] = receipt['owner'] * 4 + (0 if receipt['kind'] == 'OverwrittenByReady' else 3)
        elif moment['kind'] == 'environmental_memory':
            receipt = ready[index]
            if receipt['physical'] is not None:
                moment['phase'] = 3; moment['sequence'] = receipt['physical'] * 3 + 2
            else:
                moment['sequence'] = receipt['owner'] * 4 + (2 if receipt['kind'] == 'Discarded' else 1)
        elif moment['kind'] == 'acquisition':
            moment['sequence'] = index * 3
        elif moment['kind'] == 'release':
            if data['trial']['transitions'][index]['commitment']['work'] == 'Prepare':
                moment['phase'] = 7.2
    data['timeline'].sort(key=lambda m: (m['tick'], m.get('phase', 0), m['sequence'], m['ref']))
    for n, moment in enumerate(data['timeline']): moment['id'] = n


def export(entry, original):
    data = valuation.export(entry, original, validator=validate, physical_points=observer_points, extractor=EXTRACTOR, experiment='025')
    data['preparation'] = dict(schema=VERSION, material=copy.deepcopy(original['material']),
                               note='Observer material is not actor knowledge; raw and ready receipts share one owned slot.')
    h = original['history']; material = original['material']; nodes = data['causal']['nodes']; edges = data['causal']['edges']
    def node(ref, tick, kind, owner, label, anchor=None):
        nodes[ref] = dict(ref=ref, anchor=anchor or ref, tick=tick, kind=kind, owners=[owner], label=label)
    for n, op in enumerate(material['operations']):
        ref = f'material.operations[{n}]'; label = f"Raw source{op['source']} · {op['outcome']} · work#{op['commitment']}"
        moment = dict(tick=op['tick'], kind='preparation', ref=ref, label=label, owners=[op['owner']], phase=6.1)
        data['timeline'].append(moment); node(ref, op['tick'], 'preparation', op['owner'], label)
    for n, receipt in enumerate(material['memories']):
        ref = f'material.memories[{n}]'; label = f"Own raw-source slot · individual{receipt['owner']} · {receipt['kind']}"
        data['timeline'].append(dict(tick=receipt['tick'], kind='raw_memory', ref=ref, label=label, owners=[receipt['owner']], phase=12))
        node(ref, receipt['tick'], 'raw_memory', receipt['owner'], label)
        if receipt['physical'] is not None:
            edges.append(dict(source=f'material.operations[{receipt["physical"]}]', target=ref, kind='explicit_owned_raw_update_from_physical_outcome'))
        if receipt['cache_record'] is not None:
            ready = f'memories[{receipt["cache_record"]}]'
            source, target = (ref, ready) if receipt['kind'] == 'OverwrittenByReady' else (ready, ref)
            edges.append(dict(source=source, target=target, kind='explicit_one_slot_raw_ready_replacement'))
    spans = {span['id']: span for span in data['intervals']}; base, base_ref = native.core(h)
    for process in material['processes']:
        span = spans[process['commitment']]; bi = span['source']; owner = process['owner']; boundary = h['boundaries'][bi]
        di, choice = next((n, d) for n, d in enumerate(boundary['choices']) if d['input']['legacy']['own']['id'] == owner)
        decision = f'boundaries[{bi}].choices[{di}]'; node(decision, process['start'], 'choice', owner, 'Actual voluntary Prepare choice', f'boundaries[{bi}]')
        vi = next(n for n, v in enumerate(original['valuations']) if (v['boundary'], v['choice']) == (bi, di))
        cohort = []
        receipts = [n for n, m in enumerate(material['memories']) if m['owner'] == owner and m['tick'] <= process['start'] and m['after'] == choice['input']['raw_retained']]
        if receipts: cohort.append(f'material.memories[{receipts[-1]}]')
        cohort += [decision, f'valuations[{vi}]', span['start_ref']]
        ops = [(n, op) for n, op in enumerate(material['operations']) if op['commitment'] == process['commitment']]
        if len(ops) != 2: raise ValueError('025process lacks extraction/conversion outcome pair')
        for n, _ in ops:
            cohort.append(f'material.operations[{n}]')
            cohort += [f'material.memories[{j}]' for j, receipt in enumerate(material['memories']) if receipt['physical'] == n]
        cohort.append(span['release_ref'])
        def owned_choice_after(tick, inclusive):
            later = next(((n, j, b) for n, b in enumerate(h['boundaries'])
                          if b['tick'] >= tick if inclusive or b['tick'] > tick
                          for j, d in enumerate(b['choices']) if d['input']['legacy']['own']['id'] == owner), None)
            if later:
                n, j, b = later; ref = f'boundaries[{n}].choices[{j}]'
                node(ref, b['tick'], 'choice', owner, 'Subsequent owned captured input; no state interpolation', f'boundaries[{n}]')
                if ref not in cohort: cohort.append(ref)
        # Actual ordinary inputs can follow a physical release/debit in the same
        # tick. Include those and a strictly later tick as distinct evidence.
        owned_choice_after(process['finish_at'], True)
        debits = []
        if ops[-1][1]['outcome'] == 'Prepared':
            for n, meal in enumerate(base['cognition']['consumption']):
                if meal['actor'] == owner and meal['tick'] > process['finish_at']: debits.append((meal['tick'], f'{base_ref}.cognition.consumption[{n}]'))
            for event in base['events'][h['bootstrap_events']:]:
                if owner in event['participants'] and event['tick'] > process['finish_at']:
                    position = event['participants'].index(owner)
                    if event['balances_after'][position] < event['balances_before'][position]: debits.append((event['tick'], f'{base_ref}.events[{event["id"]}]'))
        after = process['finish_at']
        if debits:
            after, ref = min(debits); cohort.append(ref)
            owned_choice_after(after, True)
        owned_choice_after(after, False)
        for source, target in zip(cohort, cohort[1:]): edges.append(dict(source=source, target=target, kind='explicit_prepare_process_and_later_owned_consequence'))
        data['causal']['chains'].append(dict(id=f'prepare:{process["commitment"]}', label=f'Individual{owner} · finite preparation#{process["commitment"]}', nodes=cohort))
    data['causal']['note'] += '025joins use exact process/commitment IDs and captured owned receipts. A later debit is not always an identifiable food unit or proof of benefit.'
    order_timeline(data)
    for ref in nodes:
        native.resolve(data['preparation'] if ref.startswith('material.') else data['valuation'] if ref.startswith('valuations[') else data['continuous'] if ref.startswith('checkpoints[') else h, ref)
    return data


def comparison(left, right):
    result = native.comparison(left, right); a, b = left['observer_physical'], right['observer_physical']
    aa, bb = {p['tick']: p for p in a}, {p['tick']: p for p in b}; common = sorted(aa.keys() & bb.keys())
    # Available edible holdings exclude already consumed material. Consumed units
    # remain in the distinct full conservation ledger below.
    food = lambda p: (p['food'], p['stock'])
    full = lambda p: (p['raw'], p['escrow'], p['stock'], p['food'], p['waste'], p['consumed'])
    def differences(projection):
        ticks = [tick for tick in common if projection(aa[tick]) != projection(bb[tick])]
        return (ticks[0] if ticks else None, next((tick for tick in common if ticks and tick > ticks[0] and projection(aa[tick]) == projection(bb[tick])), None))
    ca, cb = copy.deepcopy(left['valuation']['config']), copy.deepcopy(right['valuation']['config'])
    access_a, access_b = ca['preparation'].pop('access_enabled'), cb['preparation'].pop('access_enabled')
    first, reconvergence = differences(food); full_first, full_reconvergence = differences(full)
    result.update(preparation_treatment=True, matched_except_access=ca == cb, access_contrast=access_a != access_b,
                  left_mode='Access enabled' if access_a else 'Access closed', right_mode='Access enabled' if access_b else 'Access closed',
                  first_material=first, material_reconvergence=reconvergence, first_full_material=full_first,
                  full_material_reconvergence=full_reconvergence, terminal_food_equal=food(a[-1]) == food(b[-1]),
                  terminal_full_material_equal=full(a[-1]) == full(b[-1]),
                  full_state_trace_null=left['trial'] == right['trial'] and left['preparation']['material'] == right['preparation']['material'],
                  left_physical=a, right_physical=b,
                  note='Observer-only025access comparison. Waste makes full-material persistence partly structural; food, hunger and trust are separate. More output or sharing is not universal benefit. Cognitive accessibility flags may differ even in execution-null controls.')
    def first_different_tick(rows_a, rows_b):
        for tick in sorted({row['tick'] for row in rows_a + rows_b}):
            if [row for row in rows_a if row['tick'] == tick] != [row for row in rows_b if row['tick'] == tick]:
                return tick
        return None
    core_a, _ = native.core(left['trial']); core_b, _ = native.core(right['trial'])
    result['same_valuation_policy'] = ca['valuation'] == cb['valuation']
    result['first_conversion_difference'] = first_different_tick(
        [op for op in left['preparation']['material']['operations'] if op['outcome'] == 'Prepared'],
        [op for op in right['preparation']['material']['operations'] if op['outcome'] == 'Prepared'])
    result['first_meal_difference'] = first_different_tick(core_a['cognition']['consumption'], core_b['cognition']['consumption'])
    result['first_social_event_difference'] = first_different_tick(
        core_a['events'][left['trial']['bootstrap_events']:], core_b['events'][right['trial']['bootstrap_events']:])
    def starting_knowledge(data):
        raw = copy.deepcopy([m for m in data['preparation']['material']['memories'] if m['tick'] == data['trial']['start']])
        for memory in raw:
            for field in ('before', 'after'):
                if memory[field] is not None:
                    memory[field]['observation'].pop('accessible')
        return raw, [m for m in data['trial']['memories'] if m['tick'] == data['trial']['start']]
    result['same_initial_knowledge_except_access'] = starting_knowledge(left) == starting_knowledge(right)
    for side, data in [('left', left), ('right', right)]:
        mat = data['preparation']['material']; result[side + '_metrics'].update(preparations=len(mat['processes']), successful_preparations=sum(op['outcome'] == 'Prepared' for op in mat['operations']), failed_preparations=sum(op['outcome'] == 'Insufficient' for op in mat['operations']), queries=sum(moment['kind'] == 'inquiry' for moment in data['timeline']))
    op_a, op_b = left['preparation']['material']['operations'], right['preparation']['material']['operations']
    for tick in sorted({op['tick'] for op in op_a + op_b}):
        la = [op for op in op_a if op['tick'] == tick]; lb = [op for op in op_b if op['tick'] == tick]
        if la != lb:
            if result['first_executed'] is None or tick < result['first_executed']['tick']:
                rows = lambda values: [dict(ref=f'material.operations[{n}]', value=op) for n, op in enumerate(values) if op['tick'] == tick]
                ra, rb = rows(op_a), rows(op_b)
                index = next(n for n in range(max(len(ra), len(rb))) if n >= len(ra) or n >= len(rb) or ra[n]['value'] != rb[n]['value'])
                result['first_executed'] = dict(tick=tick, left=ra, right=rb, kind='preparation_operation',
                    left_difference=ra[index] if index < len(ra) else None, right_difference=rb[index] if index < len(rb) else None)
            break
    return result
