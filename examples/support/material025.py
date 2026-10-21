"""Independent025 primitive material/slot reconstruction, Python stdlib only.

Uses registered physical laws and original operations, never the Rust evaluator,
viewer, saved metrics or outcome labels. Full cognitive checks remain separate.
"""
import collections
import copy


def rank(owner, tick, order):
    return owner if order == 'Ascending' else 3-owner if order == 'Descending' else (owner+4-tick % 4) % 4


class Physical:
    def __init__(self, trial, starts, finishes):
        self.trial = trial
        self.history = trial['history']
        self.settings = trial['config']['preparation']
        self.recorded = trial['material']
        self.starts = starts
        self.finishes = finishes
        self.raw = self.settings['raw_initial']
        self.waste = 0
        self.active = {}
        self.points = [dict(tick=self.history['start'], raw=self.raw, waste=0, active=[])]
        self.point_before_choice = {}
        self.raw_at_tick = {}
        self.by_tick = collections.defaultdict(list)
        self.admit = collections.defaultdict(list)
        self.processes = []
        self.op_cursor = 0
        assert self.recorded['initial_raw'] == self.raw
        for c in starts.values():
            if c['work'] != 'Prepare':
                continue
            assert self.settings['feature_enabled'] and self.settings['access_enabled']
            p = dict(commitment=c['id'], owner=c['participants'][0], source=self.settings['source_id'],
                     start=c['start'], extract_at=c['start']+self.settings['extraction_ticks'],
                     finish_at=c['start']+self.settings['extraction_ticks']+self.settings['preparation_ticks'])
            assert c['deadline'] == p['finish_at'] == finishes[c['id']]
            self.processes.append(p)
            self.admit[p['start']].append(p)
        assert self.processes == self.recorded['processes']
        expected_order = []
        for p in self.processes:
            expected_order.append((p['extract_at'], 0, rank(p['owner'], p['start'], self.history['config']['order']), p['commitment']))
            expected_order.append((p['finish_at'], 1, rank(p['owner'], p['start'], self.history['config']['order']), p['commitment']))
        operations = self.recorded['operations']
        assert len(operations) == 2*len(self.processes)
        process_by_id = {p['commitment']: p for p in self.processes}
        for n, (expected, op) in enumerate(zip(sorted(expected_order), operations, strict=True)):
            tick, phase, _, work = expected
            p = process_by_id[work]
            assert (op['id'], op['tick'], op['commitment'], op['owner'], op['source']) == (n, tick, work, p['owner'], p['source'])
            assert op['outcome'] in (('Extracted', 'Insufficient') if phase == 0 else ('Prepared', 'Failed'))
            self.by_tick[tick].append((5+phase, n, 'material', op))

    def point(self, tick):
        self.points.append(dict(tick=tick, raw=self.raw, waste=self.waste,
                                active=copy.deepcopy(list(self.active.values()))))

    def finish_tick(self, tick):
        # Boundary input is captured before new ordinary work is admitted.
        self.point_before_choice[tick] = len(self.points)-1
        self.raw_at_tick[tick] = self.raw
        for p in sorted(self.admit[tick], key=lambda p: p['commitment']):
            assert len(self.active) < 4
            assert not any(a['process']['owner'] == p['owner'] for a in self.active.values())
            self.active[p['commitment']] = dict(process=p, phase='Extracting', escrow=0)
            self.point(tick)

    def remaining(self):
        return self.raw+self.waste+sum(a['escrow'] for a in self.active.values())

    def apply(self, op, agents):
        assert op['id'] == self.op_cursor
        self.op_cursor += 1
        active = self.active[op['commitment']]
        owner = op['owner']
        assert (op['raw_before'], op['escrow_before'], op['food_before'], op['waste_before']) == (self.raw, active['escrow'], agents[owner]['food'], self.waste)
        if active['phase'] == 'Extracting':
            assert op['tick'] == active['process']['extract_at'] and active['escrow'] == 0
            if self.raw >= 2:
                expected = 'Extracted'
                self.raw -= 2
                active.update(phase='Preparing', escrow=2)
            else:
                expected = 'Insufficient'
                active.update(phase='Failed', escrow=0)
        else:
            assert op['tick'] == active['process']['finish_at']
            if active['phase'] == 'Preparing':
                assert active['escrow'] == 2
                expected = 'Prepared'
                agents[owner]['food'] += 1
                self.waste += 1
                active['escrow'] = 0
            else:
                assert active['phase'] == 'Failed' and active['escrow'] == 0
                expected = 'Failed'
        assert op['outcome'] == expected
        assert agents[owner]['food'] < 2**32 and self.waste < 2**32
        assert (op['raw_after'], op['escrow_after'], op['food_after'], op['waste_after']) == (self.raw, active['escrow'], agents[owner]['food'], self.waste)
        if expected in ('Prepared', 'Failed'):
            del self.active[op['commitment']]
        self.point(op['tick'])

    def finish(self):
        assert not self.active
        assert self.op_cursor == len(self.recorded['operations'])
        assert self.recorded['points'] == self.points, '025 material point chronology/escrow'
        assert (self.recorded['final_raw'], self.recorded['final_waste'], self.recorded['final_active']) == (self.raw, self.waste, [])
        for b in self.history['boundaries']:
            assert b['material_point'] == self.point_before_choice[b['tick']], '025 boundary before-admission material ref'


class Slots:
    def __init__(self, trial, physical, stocks, people):
        self.trial, self.physical = trial, physical
        self.t = trial['history']
        self.s = trial['config']['preparation']
        self.epoch = self.t['start'] if trial['config']['world']['phase'] == 'Epoch' else 0
        self.cache, self.raw = [None]*4, [None]*4
        self.receipts = [0]*4
        self.old_records, self.raw_records = [], []
        self.inputs = {}
        self.stocks, self.people = stocks, people
        self.grants = collections.defaultdict(list)
        for n, grant in enumerate(self.t['grants']):
            self.grants[grant['tick']].append((n, grant))
        self.ops = collections.defaultdict(list)
        for op in trial['material']['operations']:
            self.ops[op['tick']].append(op)
        self.boundaries = {b['tick']: (bi, b) for bi, b in enumerate(self.t['boundaries'])}
        nominal = self.t['start']+trial['config']['world']['horizon']
        for tick in range(self.t['start'], self.t['end']+1):
            for n, grant in self.grants[tick]:
                owner = grant['owner']
                if grant['outcome'] == 'Granted' and self.cache[owner] is not None:
                    cell = copy.deepcopy(self.cache[owner])
                    cell['own_takes'] += 1
                    self.old_change(owner, tick, 'Taken', cell, n)
                elif grant['outcome'] == 'Depleted':
                    self.ready(owner, tick, 'Depleted', n)
            for op in self.ops[tick]:
                owner = op['owner']
                if op['outcome'] == 'Extracted' and self.raw[owner] is not None:
                    cell = copy.deepcopy(self.raw[owner])
                    cell['own_takes'] += 2
                    assert cell['own_takes'] <= cell['observation']['stock']
                    self.raw_change(owner, tick, 'Taken', cell, op['id'], None)
                elif op['outcome'] == 'Insufficient':
                    self.observe_raw(owner, tick, 'Insufficient', op['id'], op['raw_after'])
            if tick not in self.boundaries:
                continue
            bi, boundary = self.boundaries[tick]
            for di, choice in enumerate(boundary['choices']):
                owner = choice['input']['legacy']['own']['id']
                assert tick < nominal and people(tick)[owner]['available']
                plan = self.s['observation']
                offset = (tick-self.epoch) % plan['period']
                if plan['ready_offsets'][owner] == offset:
                    self.ready(owner, tick, 'Sense', None)
                if plan['raw_offsets'][owner] == offset:
                    assert self.s['feature_enabled']
                    self.observe_raw(owner, tick, 'Sense', None, physical.raw_at_tick[tick])
                self.inputs[bi, di] = (copy.deepcopy(self.cache[owner]), copy.deepcopy(self.raw[owner]))
                assert self.cache[owner] is None or self.raw[owner] is None
        assert self.old_records == self.t['memories'], '025 ready knowledge/overwrite/receipt chronology'
        assert self.raw_records == trial['material']['memories'], '025 raw knowledge/overwrite/receipt chronology'
        assert self.cache == self.t['final_memory']
        assert self.raw == trial['material']['final_slots']

    def old_change(self, owner, tick, kind, after, physical):
        before = copy.deepcopy(self.cache[owner])
        record = dict(id=len(self.old_records), owner=owner, tick=tick, kind=kind,
                      before=before, after=copy.deepcopy(after), physical=physical, point=None)
        self.old_records.append(record)
        self.cache[owner] = copy.deepcopy(after)

    def raw_change(self, owner, tick, kind, after, physical, cache_record):
        record = dict(id=len(self.raw_records), owner=owner, tick=tick, kind=kind,
                      before=copy.deepcopy(self.raw[owner]), after=copy.deepcopy(after),
                      physical=physical, cache_record=cache_record)
        self.raw_records.append(record)
        self.raw[owner] = copy.deepcopy(after)

    def ready(self, owner, tick, kind, physical):
        if self.raw[owner] is not None:
            self.raw_change(owner, tick, 'OverwrittenByReady', None, None, len(self.old_records))
        cell = dict(owner=owner, receipt=self.receipts[owner], observation=dict(id=0, observed_at=tick,
                    stock=self.stocks[tick], accessible=True, duration=self.t['config']['duration']), own_takes=0)
        self.receipts[owner] += 1
        self.old_change(owner, tick, kind, cell, physical)

    def observe_raw(self, owner, tick, kind, physical, stock):
        cache_record = None
        if self.cache[owner] is not None:
            cache_record = len(self.old_records)
            self.old_change(owner, tick, 'Discarded', None, None)
        cell = dict(owner=owner, receipt=self.receipts[owner], observation=dict(id=self.s['source_id'],
                    observed_at=tick, stock=stock, accessible=self.s['access_enabled'],
                    extraction_ticks=self.s['extraction_ticks'], preparation_ticks=self.s['preparation_ticks'],
                    input_units=2, edible_units=1, waste_units=1), own_takes=0)
        self.receipts[owner] += 1
        self.raw_change(owner, tick, kind, cell, physical, cache_record)


def selection(i, original, valuation):
    """Independent integer terms; no actual source or future outcome parameter."""
    assert set(i) == {'legacy', 'retained', 'duration', 'inquiry_lane_free', 'raw_retained'}
    assert i['retained'] is None or i['raw_retained'] is None
    own, now, end = i['legacy']['own'], i['legacy']['now'], i['legacy']['end']
    terms = []
    for c in original:
        bonus = min(own['hunger'], 40) if valuation == 'MealRelief' and c['action'] == 'Consume' and c['eligible'] else 0
        terms.append(dict(action=c['action'], eligible=c['eligible'], base_score=c['score'], meal_relief=bonus, work_effort=0, final_score=c['score']+bonus))
    raw = i['raw_retained']
    eligible, effort = False, 0
    if raw is not None:
        o = raw['observation']
        assert set(raw) == {'owner', 'receipt', 'observation', 'own_takes'}
        assert set(o) == {'id','observed_at','stock','accessible','extraction_ticks','preparation_ticks','input_units','edible_units','waste_units'}
        effort = o['extraction_ticks']+o['preparation_ticks']
        eligible = bool(raw['owner'] == own['id'] and o['id'] == 0 and o['observed_at'] <= now
                        and o['stock']-raw['own_takes'] >= 2 and o['accessible']
                        and (o['input_units'], o['edible_units'], o['waste_units']) == (2, 1, 1)
                        and o['extraction_ticks'] > 0 and o['preparation_ticks'] > 0
                        and 0 < effort <= 33 and own['food'] == 0 and own['hunger'] > 0
                        and any(p['id'] == own['id'] and p['available'] for p in i['legacy']['people'])
                        and now+effort <= end)
    terms.append(dict(action='Prepare', eligible=eligible, base_score=own['hunger'] if eligible else -1,
                      meal_relief=0, work_effort=effort if eligible else 0,
                      final_score=own['hunger']-effort if eligible else -1))
    assert [c['action'] for c in terms] == ['Wait','Consume','Invite','Acquire','Prepare']
    return dict(terms=terms, selected=max(terms, key=lambda c: c['final_score'])['action'])
