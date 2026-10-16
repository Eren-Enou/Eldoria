"""Read-only, post-evaluation observation checks. No simulator/viewer imports.

From repository root: python -B tools/history-explorer/validation/observe019.py
Prints a compact original-reference evidence packet to stdout; writes no archives.
This selected sample is not a new experiment or a prevalence estimate.
"""
import collections
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
REVISION = 'bc15b4551e19455cf8545019213a04cc2ee9df2b'


def layers(t):
    learning = t['final_state']
    assessment = learning['base']['base']['assessment']
    compact = learning['base']['base']['base']
    return compact['base']['base'], compact['base']['inquiry'], compact['provenance'], assessment


def steps(t):
    for pn, p in enumerate(t['periods']):
        for sn, w in enumerate(p['steps']):
            yield pn, sn, w['activity']


def step_slice(t, pn, sn):
    s = t['periods'][pn]['steps'][sn]['activity']
    decisions = []
    for owner, d in enumerate(s['decisions']):
        if d is None:
            continue
        i = d['contact']['input']
        decisions.append(dict(owner=owner, own_food=i['own']['food'], hunger=i['own']['hunger'],
                              trust=i['own']['trust'], recent_episodes=i['own']['memories'][-4:],
                              available=i['people'], candidates=d['candidates'], selected=d['selected'],
                              contact_candidates=d['contact']['candidates'], contact=d['contact']['selected']))
    return dict(ref=f'periods[{pn}].steps[{sn}].activity',
                tick=t['points'][s['observed']]['state']['tick'],
                points={k: s[k] for k in ('before', 'observed', 'sent', 'admitted', 'after')},
                decisions=decisions,
                responses=[dict(owner=r['input']['own']['own']['id'],
                                tick=r['input']['own']['now'],
                                food=r['input']['own']['own']['food'],
                                hunger=r['input']['own']['own']['hunger'],
                                candidates=r['candidates'], selected=r['selected']) for r in s['responses']],
                allocations=s['allocations'], started=s['started'], query=s['query'], meals=s['consumption'])


def event_slice(e):
    d = e['decision']
    return dict(ref=f'final_state.base.base.base.base.base.events[{e["id"]}]',
                **{k: e[k] for k in ('id', 'tick', 'scene', 'participants', 'balances_before', 'balances_after', 'transferred', 'outcome')},
                decision=d, interpretations=e['interpretations'])


def state_slice(t, point):
    s = t['points'][point]['state']
    return dict(point=point, tick=s['tick'], agents=[{k: a[k] for k in ('id', 'food', 'hunger', 'trust')} for a in s['agents']])


def pressure_time(t):
    """Discrete pressure after progression/meals on [epoch,end), not welfare."""
    meals = collections.defaultdict(list)
    for _, _, s in steps(t):
        for m in s['consumption']:
            meals[m['tick']].append(m)
    h = [a['hunger'] for a in t['initial']['state']['agents']]
    area = [0, 0, 0]
    epoch, end = t['clock']['epoch'], t['clock']['last']
    for tick in range(epoch, end + 1):
        if tick > epoch and (tick - epoch) % t['config']['interval'] == 0:
            h = [min(100, v + 5) for v in h]
        for m in meals[tick]:
            i = m['actor']
            assert h[i] == m['hunger_before']
            h[i] = max(0, h[i] - 40)
            assert h[i] == m['hunger_after']
        if tick < end:
            area = [a + v for a, v in zip(area, h)]
    assert h == [a['hunger'] for a in t['points'][-1]['state']['agents']]
    return area


def main():
    trials = {}
    identities = {}
    for name, wanted in [('seed-42.jsonl.gz', {1, 7, 10, 85, 103, 107, 109, 113}),
                         ('held-out-seed-42.jsonl.gz', {7, 10, 49, 139, 182, 183})]:
        path = ROOT / 'experiments/019' / name
        archive_hash = hashlib.sha256(path.read_bytes()).hexdigest()
        with gzip.open(path, 'rb') as f:
            for ordinal, line in enumerate(f, 1):
                if ordinal not in wanted:
                    continue
                t = json.loads(line)
                key = f'{name}:{ordinal}'
                trials[key] = t
                identities[key] = dict(archive='experiments/019/' + name, record=ordinal,
                                       archive_sha256=archive_hash,
                                       record_sha256=hashlib.sha256(line.rstrip(b'\r\n')).hexdigest(),
                                       case=t['case'], seed=t['seed'], mode=t['mode'], config=t['config'])
    for t in trials.values():
        consumed = sum(m['consumed'] for _, _, s in steps(t) for m in s['consumption'])
        assert sum(a['food'] for a in t['initial']['state']['agents']) == consumed + sum(a['food'] for a in t['points'][-1]['state']['agents'])
    core = lambda n: trials[f'seed-42.jsonl.gz:{n}']
    held = lambda n: trials[f'held-out-seed-42.jsonl.gz:{n}']
    a, b, x, y = [core(n) for n in (103, 107, 109, 113)]
    first = lambda t: t['periods'][0]['steps'][0]['activity']
    own_input = lambda t: first(t)['decisions'][0]['contact']['input']
    assert a['initial'] == b['initial'] and a['profiles'] == b['profiles']
    assert x['initial'] == y['initial'] and x['profiles'] == y['profiles']
    assert own_input(a) == own_input(b) == own_input(x) == own_input(y)
    assert [c['score'] for c in first(a)['decisions'][0]['candidates']] == [0, 80, 78]
    assert [c['score'] for c in first(b)['decisions'][0]['candidates']] == [0, 80, 85]
    assert layers(core(7))[0] == layers(core(10))[0]
    assert core(7)['final_state'] == core(10)['final_state']
    contacts = {}
    for n in (103, 107, 109, 113):
        t = core(n); base, _, _, _ = layers(t)
        es = base['events'][t['initial']['state']['events']:]
        meals = [m for _, _, s in steps(t) for m in s['consumption']]
        contacts[str(n)] = dict(first=step_slice(t, 0, 0),
                                pressure_time=pressure_time(t),
                                first_events=[event_slice(e) for e in es[:5]],
                                period_states=[state_slice(t, p['after']) for p in t['periods']],
                                transfers=sum(e['transferred'] > 0 for e in es),
                                meals_by_actor=dict(collections.Counter(m['actor'] for m in meals)),
                                resource_acts_by_actor={str(i): dict(collections.Counter(e['decision']['selected'] for e in es if e['decision']['actor'] == i)) for i in range(3)})
    assert (contacts['103']['transfers'], contacts['107']['transfers']) == (12, 6)
    assert contacts['103']['pressure_time'] == [10960, 4505, 10200]
    assert contacts['107']['pressure_time'] == [8355, 6685, 10200]
    assert contacts['109']['pressure_time'] == [10640, 8260, 10200]
    assert contacts['113']['pressure_time'] == [11680, 8250, 10200]
    assert contacts['109']['transfers'] == contacts['113']['transfers'] == 0
    t = core(85); base, _, _, _ = layers(t)
    pi = next(i for i, p in enumerate(t['points']) if all(a['food'] == 0 for a in p['state']['agents']))
    assert pi == 304 and t['points'][pi]['state']['tick'] == 258
    late = [e for e in base['events'] if e['tick'] > 258]
    counts = dict(collections.Counter(e['decision']['selected'] for e in late))
    assert counts == {'Request': 65, 'Leave': 65}
    assert len({e['scene'] for e in late}) == 65 and sum(e['transferred'] for e in late) == 0
    late_steps = [(p, s, w) for p, s, w in steps(t) if t['points'][w['observed']]['state']['tick'] >= 258]
    allocation_counts = dict(collections.Counter(a['disposition'] for _, _, w in late_steps for a in w['allocations']))
    assert allocation_counts == {'Execute': 65, 'Reciprocal': 65, 'Declined': 65}
    slices = [(4, 4), (4, 8), (10, 10), (10, 13), (11, 0), (12, 9), (16, 12), (23, 9)]
    ongoing = dict(first_all_empty=state_slice(t, pi), final=state_slice(t, len(t['points']) - 1),
                   post_exhaustion_actions=counts, completed_encounters=65,
                   post_exhaustion_transfers=0, allocation_counts=allocation_counts,
                   steps=[step_slice(t, p, s) for p, s in slices],
                   events=[event_slice(base['events'][i]) for i in (43, 44, 45, 46, 47, 109, 110, 111, 112, 127, 128, 169, 170, 239, 240)])
    for pn, sn, score in [(4, 4, 133), (4, 8, 188)]:
        d = t['periods'][pn]['steps'][sn]['activity']['decisions'][0]
        assert d['candidates'][1] == dict(action='Consume', eligible=True, score=100)
        assert d['candidates'][2]['score'] == score and d['selected'] == 'Invite'
    at400 = t['periods'][16]['steps'][12]['activity']['decisions'][0]['contact']['input']['own']
    assert at400['trust'] == {'1': 100} and len(at400['memories']) == 16
    assert all(m['valence'] == 0 for m in at400['memories'])
    negatives = {}
    for n in (182, 183, 7, 10, 49):
        t = held(n); base, inquiry, prov, assessment = layers(t)
        queries = inquiry['records'][t['initial']['state']['queries']:]
        negatives[str(n)] = dict(
            steps=[step_slice(t, 0, 12 if n in (182, 183) else 3 if n in (7, 10) else 0)],
            queries=[dict(ref=f'final_state.base.base.base.base.inquiry.records[{q["id"]}]',
                          **{k: q[k] for k in ('id', 'tick', 'time_spent', 'response', 'novelty', 'information_values', 'concern_before', 'concern_after', 'cell_before', 'cell_after')},
                          owner=q['decision']['input']['owner'], selected=q['decision']['selected']) for q in queries],
            assessment_records=assessment['records'] if n in (7, 10) else [],
            provenance_queries=prov['queries'] if n in (7, 10) else [],
            period_states=[state_slice(t, p['after']) for p in t['periods'][:5]])
    assert held(7)['periods'][0]['steps'][3]['activity']['query'] is None
    assert negatives['7']['queries'][0]['owner'] == 2
    assert negatives['10']['queries'][0]['owner'] == 0
    assert negatives['10']['queries'][0]['response'] is None
    assert negatives['10']['queries'][0]['concern_after']['status'] == 'Resolved'
    assert negatives['10']['assessment_records'][-1]['support'] == 60
    assert negatives['10']['provenance_queries'][0]['receipt'] == 1
    assert negatives['7']['provenance_queries'][0]['receipt'] is None
    left, right = layers(held(182))[0], layers(held(183))[0]
    assert left['agents'] == right['agents'] and left['cognition'] != right['cognition']
    assert left['cognition']['beliefs'] == right['cognition']['beliefs']
    assert left['cognition']['information'] == right['cognition']['information']
    assert held(182)['final_state'] != held(183)['final_state']
    reconvergence = dict(final_agents_equal=True, complete_final_snapshot_equal=False,
                         unequal_cognition_fields=[k for k in left['cognition'] if left['cognition'][k] != right['cognition'][k]],
                         unequal_base_fields=[k for k in left if left[k] != right[k]])
    packet = dict(reviewed_revision=REVISION, post_evaluation_observations=True,
                  sample_not_prevalence=True, identities=identities, contacts=contacts,
                  ongoing=ongoing, negatives=negatives, deadline_reconvergence=reconvergence,
                  no_food_complete_final_snapshot_equal=True)
    print(json.dumps(packet, indent=2, sort_keys=True))


if __name__ == '__main__':
    main()
