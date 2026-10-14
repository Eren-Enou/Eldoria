"""Second read-only scientific reader: raw014 claims, not evaluator classifications.

Uses only the standard library. The broader review014 reader separately reconstructs
all cognition and scoring. This reader tests decisive causal anchors, matched setup,
category totals and the limits of reservation/counterfactual interpretation.
"""
import collections
import copy
import gzip
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DIRECTORY = ROOT / 'experiments/014'
MODES = ('ResourceFirst', 'InquiryFirst', 'Local', 'NoDeadline', 'LocalPulse')


def load(name):
    return json.loads(gzip.decompress((DIRECTORY / (name + '.json.gz')).read_bytes()))


def resource_events(t):
    return t['final_state']['base']['base']['base']['base']['base']['events']


def check(primary, held, diagnostic):
    assert len(primary) == 70 and len(held) == 140 and len(diagnostic) == 5
    assert len({t['case'] for t in primary}) == 14
    groups = collections.defaultdict(list)
    for t in primary + held + diagnostic:
        groups[(t['case'], json.dumps(t['config'], sort_keys=True))].append(t)
    assert len(groups) == 43
    for trials in groups.values():
        assert {t['mode'] for t in trials} == set(MODES)
        # Identical physical and retained input before voluntary divergence, even
        # for Pulse (whose prospective policy differs, not its starting cognition).
        assert all(t['initial'] == trials[0]['initial'] for t in trials)
        schedule = lambda t: [(p['start'] - t['periods'][0]['start'], p['end'] - p['start'], p['qclose'] - p['start'], p['rclose'] - p['start'], p['source'], p['partner']) for p in t['periods']]
        assert all(schedule(t) == schedule(trials[0]) for t in trials)
    def trial(case, mode):
        return next(t for t in primary if t['case'] == case and t['mode'] == mode)
    local = trial('delayed', 'Local'); priority = trial('delayed', 'ResourceFirst')
    p = local['periods'][0]; q, r = p['records'][:2]
    assert (p['start'], p['qclose'], p['rclose']) == (41, 54, 66)
    assert q['decision']['selected'] == 'Inquiry'
    selected = q['decision']['input']['inquiry']['decision']['selected']['Ask']
    assert selected == dict(concern=0, source=2, strategy='Direct')
    assert q['after']['tick'] - q['before']['tick'] == 13
    outcome = local['final_state']['learning']['records'][0]['outcome']
    assert (outcome['support_before'], outcome['support_after'], outcome['status_before'], outcome['status_after'], outcome['receipt_acquired']) == (20, 60, 'Partial', 'Resolved', True)
    before_offer = next(c['score'] for c in q['decision']['input']['resource']['candidates'] if c['action'] == 'Offer')
    after_offer = next(c['score'] for c in r['decision']['input']['resource']['candidates'] if c['action'] == 'Offer')
    assert (before_offer, after_offer) == (-3, 31)
    events = resource_events(local)[r['before']['events']:r['after']['events']]
    assert [e['decision']['selected'] for e in events] == ['Offer', 'Accept']
    assert sum(e['transferred'] for e in events) == 1
    assert [a['food'] for a in p['after']['agents'][:2]] == [2, 5]
    other = priority['periods'][0]
    assert other['records'][0]['decision']['selected'] == 'Resource'
    assert other['records'][0]['after']['tick'] == 42
    assert 42 + 13 > other['qclose']
    assert other['records'][0]['after']['queries'] == other['records'][0]['before']['queries']
    assert [a['food'] for a in other['after']['agents'][:2]] == [3, 4]
    assert local['periods'][1]['records'][0]['before']['agents'][0]['food'] == 2
    assert priority['periods'][1]['records'][0]['before']['agents'][0]['food'] == 3
    for mode in ('InquiryFirst', 'NoDeadline', 'LocalPulse'):
        assert trial('delayed', mode)['periods'][0]['after'] == p['after']
    a = trial('material_first', 'InquiryFirst'); b = trial('material_first', 'Local')
    assert a['initial']['agents'][0]['food'] == b['initial']['agents'][0]['food'] == 0
    assert a['initial']['agents'][0]['hunger'] == b['initial']['agents'][0]['hunger'] == 80
    assert a['periods'][0]['records'][0]['after']['tick'] > a['periods'][0]['rclose']
    assert (a['periods'][0]['after']['agents'][0]['hunger'], b['periods'][0]['after']['agents'][0]['hunger']) == (80, 40)
    assert (a['periods'][-1]['after']['agents'][0]['hunger'], b['periods'][-1]['after']['agents'][0]['hunger']) == (100, 60)
    d = {t['mode']: t for t in diagnostic}
    duration = lambda t: t['periods'][0]['records'][0]['after']['tick'] - t['periods'][0]['records'][0]['before']['tick']
    assert duration(d['ResourceFirst']) == 2 and duration(d['Local']) == 13
    assert 2 + 13 > 13
    assert d['ResourceFirst']['periods'][0]['unused'] == ['Inquiry']
    assert d['Local']['periods'][0]['unused'] == ['Resource']
    # Reservation failure is narrower: observed two-tick interactions in other
    # branches do not prove the unexecuted recipient would accept in this branch.
    reservation = trial('reservation', 'Local')['periods'][0]
    rejected = [c for r in reservation['records'] for c in r['decision']['candidates'] if c['action'] == 'Resource' and not c['eligible']]
    assert rejected
    for mode in MODES:
        mirror = trial('mirrored', mode)['periods'][0]['records'][0]['decision']
        assert mirror == trial('material_first', mode)['periods'][0]['records'][0]['decision']
    transfer = lambda mode: sum(e['transferred'] for t in primary if t['mode'] == mode for e in resource_events(t)[t['initial']['events']:])
    assert [transfer(m) for m in MODES] == [26, 12, 26, 21, 26]
    assert all(all(a['food'] == 0 and a['hunger'] == 100 for a in trial('longitudinal', mode)['periods'][-1]['after']['agents']) for mode in MODES)
    return dict(verdict='ACCEPT WITH QUALIFICATION', primary=210, diagnostic=5, primary_pairs=420, diagnostic_pairs=10, core_transfers={m: transfer(m) for m in MODES}, positive_shared=True, reservation_counterfactual_unexecuted=True)


def main():
    primary, held, diagnostic = [load(n) for n in ('seed-42', 'held-out-seed-42', 'exclusive-budget')]
    result = check(primary, held, diagnostic)
    for field in ('time', 'information', 'transfer'):
        bad = copy.deepcopy(primary)
        t = next(t for t in bad if t['case'] == 'delayed' and t['mode'] == 'Local')
        if field == 'time': t['periods'][0]['qclose'] += 1
        elif field == 'information': t['final_state']['learning']['records'][0]['outcome']['support_after'] = 59
        else: resource_events(t)[5]['transferred'] = 0
        try: check(bad, held, diagnostic)
        except AssertionError: pass
        else: raise AssertionError('accepted corruption: ' + field)
    print(json.dumps(result, sort_keys=True))


if __name__ == '__main__':
    main()
