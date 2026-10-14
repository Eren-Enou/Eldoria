"""Second, read-only scientific review of original015 execution.

No evaluator summaries/classifications, original015 reader, or Rust policies are
imported. This complements, rather than replaces, full cognitive reconstruction.
"""
import argparse
import collections
import copy
import gzip
import itertools
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def load(name):
    return json.loads(gzip.decompress((ROOT / 'experiments/015' / (name + '.json.gz')).read_bytes()))


def resource(a, partner):
    trust = a['trust'].get(str(partner), 0)
    recent = [m for m in a['memories'] if m['partner'] == partner][-4:]
    valence = max(-40, min(40, sum(m['valence'] for m in recent)))
    trunc4 = (abs(trust) // 4) * (-1 if trust < 0 else 1)
    h, g, c = a['hunger'], a['generosity'], a['caution']
    values = [0, h - 35 + trunc4 - c // 4]
    if a['food']:
        values.append(g - h // 2 - c // 2 + trust + valence - (60 if h >= 60 and a['food'] <= 1 else 0))
    return max(values) + max(h, 0) // 2 - 2


def verify(t):
    events = t['final_state']['base']['base']['base']['base']['base']['events']
    result = collections.Counter()
    for period in t['periods']:
        for w in period['waves']:
            assert len(w['proposals']) == 3
            invites = []
            for d in w['proposals']:
                i = d['input']; a = copy.deepcopy(i['own'])
                assert i['now'] == w['before']['tick'] and a == w['before']['agents'][a['id']]
                if t['mode'] == 'NoHistory':
                    a['trust'] = {}; a['memories'] = []
                for c in d['candidates']:
                    if c['intent'] == 'Wait':
                        assert c['score'] == 0
                        continue
                    v = c['intent']['Invite']
                    if not c['eligible']: expected = -1
                    elif t['mode'] == 'Simple': expected = 1
                    elif v['kind'] == 'Resource': expected = resource(a, v['recipient'])
                    else:
                        q = i['query']['decision']
                        expected = next(c['score'] for c in q['candidates'] if c['action'] == q['selected']) - 2
                    assert c['score'] == expected
                assert d['selected'] == max(d['candidates'], key=lambda c: c['score'])['intent']
                if d['selected'] != 'Wait':
                    v = d['selected']['Invite']
                    invites.append(dict(proposer=a['id'], recipient=v['recipient'], kind=v['kind'], expires=i['now']+2))
            if not invites:
                assert not w['responses'] and not w['allocations'] and w['execution'] is None
                continue
            result['invitations'] += len(invites)
            occupied = set(); executed = []
            for owner, r in enumerate(w['responses']):
                i = r['input']; assert i['outgoing'] == w['proposals'][owner]
                assert i['own']['now'] == w['before']['tick'] + 1
                assert i['incoming'] == [v for v in invites if v['recipient'] == owner]
                own = i['own']; cs = [dict(reply='Wait', score=0)]
                for inv in i['incoming']:
                    score = resource(own['own'], inv['proposer']) if inv['kind'] == 'Resource' else own['profile']['relationship_goal'] - own['profile']['privacy'] - 2
                    if not own['people'][owner]['available']: score = -1
                    cs.append(dict(reply=dict(Accept=inv['proposer']), score=score))
                d = i['outgoing']
                if d['selected'] != 'Wait':
                    cs.append(dict(reply='Continue', score=next(c['score'] for c in d['candidates'] if c['intent'] == d['selected'])))
                assert r['candidates'] == cs and r['selected'] == max(cs, key=lambda c:c['score'])['reply']
                # Read-only counterfactual: replace Continue scale with priority1.
                counter = [dict(c, score=1) if c['reply'] == 'Continue' else c for c in cs]
                result['continue_scale_sensitive'] += max(counter, key=lambda c:c['score'])['reply'] != r['selected']
            order = t['config']['order']
            key = lambda i: i['proposer'] if order == 'Ascending' else 2-i['proposer'] if order == 'Descending' else (i['proposer']+3-w['id']%3)%3
            assert [a['invitation'] for a in w['allocations']] == sorted(invites, key=key)
            for allocation in w['allocations']:
                v = allocation['invitation']
                assert v['expires'] == w['before']['tick']+2
                accepted = w['responses'][v['recipient']]['selected'] == dict(Accept=v['proposer'])
                reverse = any(x['proposer'] == v['recipient'] and x['recipient'] == v['proposer'] and x['kind'] == v['kind'] for x in executed)
                reason = 'Declined' if not accepted else 'Reciprocal' if reverse else 'Contention' if occupied.intersection([v['proposer'],v['recipient']]) else 'Execute'
                assert allocation['disposition'] == reason
                result[reason] += 1
                if reason == 'Execute':
                    executed.append(v); occupied.update([v['proposer'], v['recipient']])
            assert len(executed) <= 1
            if not executed:
                assert w['execution'] is None
                assert w['after'] == dict(w['before'], tick=w['before']['tick']+2)
            else:
                x = w['execution']; assert x['invitation'] == executed[0]
                assert x['before']['tick'] == w['before']['tick']+2
                assert x['after']['tick'] <= period['end']
                result['inquiries'] += x['invitation']['kind'] == 'Inquiry'
                es = events[x['before']['events']:x['after']['events']]
                for event in es:
                    before, after = event['balances_before'], event['balances_after']
                    assert sum(before) == sum(after)
                    assert event['transferred'] == sum(max(0, b-a) for a,b in zip(before,after))
                result['transfers'] += sum(e['transferred'] for e in es)
    return result


def main():
    ap = argparse.ArgumentParser(); ap.add_argument('--self-test', action='store_true'); args = ap.parse_args()
    ts = load('seed-42') + load('held-out-seed-42')
    assert len(ts) == 195 and len({t['case'] for t in ts}) == 13
    results = [verify(t) for t in ts]
    core = lambda case, mode='History': next(t for t in ts[:39] if t['case']==case and t['mode']==mode)
    groups = collections.defaultdict(list)
    for t in ts: groups[(t['case'],json.dumps(t['config'],sort_keys=True))].append(t)
    assert len(groups)==65
    for group in groups.values():
        assert len(group)==3 and all(t['initial']==group[0]['initial'] for t in group)
    ordering = 0
    for t in ts[39:]:
        if t['config']['budget'] == 24:
            baseline = core(t['case'],t['mode'])
            assert t['initial']==baseline['initial']
            for field in ['proposals','responses']:
                assert t['periods'][0]['waves'][0][field]==baseline['periods'][0]['waves'][0][field]
            ordering += 1
    assert ordering==78
    totals = {}
    for order in ['Ascending','Descending','Rotating']:
        totals[order] = sum(r['transfers'] for t,r in zip(ts,results) if t['mode']=='History' and t['config']['budget']==24 and t['config']['order']==order)
    assert totals==dict(Ascending=48,Descending=107,Rotating=131)
    mutual=core('mutual'); w=mutual['periods'][0]['waves'][0]
    assert [d['selected'] for d in w['proposals']]==[dict(Invite=dict(recipient=1,kind='Resource')),dict(Invite=dict(recipient=0,kind='Resource')),'Wait']
    events=mutual['final_state']['base']['base']['base']['base']['base']['events']
    assert [e['decision']['selected'] for e in events[:3]]==['Request','Offer','Accept']
    assert [e['tick'] for e in events[:3]]==[3,4,5] and events[2]['transferred']==1
    assert [(a['food'],a['hunger']) for a in mutual['periods'][0]['after']['agents']][:2]==[(3,50),(0,0)]
    for mode in ['Simple','NoHistory']:
        assert core('mutual',mode)['periods'][0]['after']['agents']==mutual['periods'][0]['after']['agents']
    contention=core('contended'); xs=[w['execution'] for p in contention['periods'] for w in p['waves'] if w['execution']]
    assert len(xs)==27 and all({x['invitation']['proposer'],x['invitation']['recipient']}=={0,1} for x in xs)
    assert contention['periods'][-1]['after']['agents'][2]['hunger']==100
    private=core('private_mirror'); counts=verify(private)
    assert (counts['invitations'],counts['Declined'],counts['inquiries'])==(144,108,0)
    # Complete failed waves leave food, trust, episodes and query/assessment counters unchanged.
    for p in private['periods']:
        for w in p['waves']:
            if w['allocations'] and not w['execution']:
                assert w['after']==dict(w['before'],tick=w['before']['tick']+2)
    h,n=core('history'),core('history','NoHistory')
    assert [c['score'] for c in h['periods'][0]['waves'][0]['proposals'][0]['candidates']][1:3]==[13,78]
    assert [c['score'] for c in n['periods'][0]['waves'][0]['proposals'][0]['candidates']][1:3]==[78,78]
    assert [a['food'] for a in h['periods'][-1]['after']['agents']]==[0,0,2]
    assert [a['food'] for a in n['periods'][-1]['after']['agents']]==[0,2,0]
    if args.self_test:
        for field in ['consent','time','transfer']:
            bad=copy.deepcopy(mutual)
            if field=='consent': bad['periods'][0]['waves'][0]['responses'][1]['selected']='Wait'
            elif field=='time': bad['periods'][0]['waves'][0]['execution']['before']['tick']+=1
            else: bad['final_state']['base']['base']['base']['base']['base']['events'][2]['transferred']=0
            try:
                verify(bad)
            except AssertionError: pass
            else: raise AssertionError('accepted '+field+' corruption')
    print(json.dumps(dict(histories=len(ts),matched_groups=len(groups),ordering_contrasts=ordering,history_transfers=totals,continue_scale_sensitive=sum(r['continue_scale_sensitive'] for r in results),verdict='ACCEPT WITH QUALIFICATION'),sort_keys=True))


if __name__=='__main__': main()
