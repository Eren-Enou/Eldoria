"""Independent post-review reconstruction of original023, forked explicitly from the022 physical arithmetic checker.

No imports of any022/023reader, evaluator, comparison tables or viewer. This does not
replace the full cognitive reader; it independently challenges its aggregate and
temporal interpretation using primitive original operations. Never writes archives.
"""
import argparse
import collections
import copy
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def base(trial):
    value = trial['final_state']
    while 'events' not in value:
        value = value['base']
    return value


def physical(trial, phase):
    """Rebuild food/need independently, in actual need/event/claim/meal order."""
    actors = copy.deepcopy(trial['initial'])
    stock = trial['initial_stock']
    consumed = 0
    area = [0] * 4
    operations = collections.defaultdict(list)
    needs = trial['needs']
    epoch = trial['start'] if phase == 'Epoch' else 0
    first = epoch + ((trial['start'] - epoch) // 4 + 1) * 4
    assert [n['tick'] for n in needs] == list(range(first, trial['end'] + 1, 4)), 'need phase'
    for n in needs:
        operations[n['tick']].append((0, 0, 'need', n))
    for e in base(trial)['events'][trial['bootstrap_events']:]:
        operations[e['tick']].append((1, e['scene'], 'event', e))
    for n, grant in enumerate(trial['grants']):
        operations[grant['tick']].append((2, n, 'claim', grant))
    meals = base(trial)['cognition']['consumption']
    for n, meal in enumerate(meals):
        operations[meal['tick']].append((3, n, 'meal', meal))
    captures = {b['tick']: b for b in trial['boundaries']}
    ledger = {}
    for tick in range(trial['start'], trial['end'] + 1):
        if tick > trial['start']:
            for owner, actor in enumerate(actors):
                area[owner] += actor['hunger']
        for _, _, kind, op in sorted(operations[tick]):
            if kind == 'need':
                assert op['before'] == [a['hunger'] for a in actors]
                for actor in actors:
                    actor['hunger'] = min(100, actor['hunger'] + 5)
                assert op['after'] == [a['hunger'] for a in actors]
            elif kind == 'event':
                ids = op['participants']
                assert op['balances_before'] == [actors[i]['food'] for i in ids]
                before, after = op['balances_before'], op['balances_after']
                assert sum(before) == sum(after)
                assert all(v >= 0 for v in after)
                assert op['transferred'] in (0, 1)
                assert sum(abs(a - b) for a, b in zip(before, after)) == 2 * op['transferred']
                for owner, value in zip(ids, after):
                    actors[owner]['food'] = value
            elif kind == 'claim':
                actor = actors[op['owner']]
                assert (stock, actor['food']) == (op['stock_before'], op['food_before'])
                take = int(stock > 0)
                assert op['outcome'] == ('Granted' if take else 'Depleted')
                stock -= take
                actor['food'] += take
                assert (stock, actor['food']) == (op['stock_after'], op['food_after'])
            else:
                actor = actors[op['actor']]
                assert actor['food'] > 0 and actor['hunger'] > 0
                assert (actor['food'], actor['hunger']) == (op['food_before'], op['hunger_before'])
                actor['food'] -= 1
                actor['hunger'] = max(0, actor['hunger'] - 40)
                consumed += 1
                assert op['consumed'] == 1
                assert (actor['food'], actor['hunger']) == (op['food_after'], op['hunger_after'])
            assert sum(a['food'] for a in actors) + stock + consumed == trial['initial_total']
        ledger[tick] = ([a['food'] for a in actors], stock, consumed)
        if tick in captures:
            capture = captures[tick]
            assert capture['stock'] == stock
            assert [(a['food'], a['hunger']) for a in capture['agents']] == [(a['food'], a['hunger']) for a in actors]
    assert [(a['food'], a['hunger']) for a in trial['final_agents']] == [(a['food'], a['hunger']) for a in actors]
    assert trial['final_stock'] == stock and consumed == len(meals)
    return dict(area=area, ledger=ledger)


def trust(trial):
    return [a['trust'].get(str(other), 0) for a in trial['final_agents'] for other in range(4) if other != a['id']]


def resource_value(own, partner):
    # Integer division truncates toward zero in the original Rust heuristic.
    div = lambda value, by: (abs(value) // by) * (-1 if value < 0 else 1)
    relationship = own['trust'].get(str(partner), 0)
    memories = [m for m in own['memories'] if m['partner'] == partner][-4:]
    valence = max(-40, min(40, sum(m['valence'] for m in memories)))
    request = own['hunger'] - 35 + div(relationship, 4) - own['caution'] // 4
    offer = own['generosity'] - own['hunger'] // 2 - own['caution'] // 2 + relationship + valence
    if own['hunger'] >= 60 and own['food'] <= 1:
        offer -= 60
    best = max(0, request, offer) if own['food'] else max(0, request)
    return best + max(0, own['hunger']) // 2 - 2


def decision(trial, boundary, d):
    i = d['input']
    assert set(i) == {'legacy', 'retained', 'duration', 'inquiry_lane_free'}
    e = i['legacy']
    assert set(e) == {'own', 'profile', 'now', 'end', 'people', 'query'}
    own = e['own']; now = e['now']; owner = own['id']
    assert own == boundary['agents'][owner], 'captured owned state'
    assert now == boundary['tick'] and now < trial['history']['start'] + trial['config']['horizon']
    start = trial['history']['start']; cfg = trial['config']
    expected_end = now + 33 if cfg['mode'] == 'Continuous' else start + ((now-start)//cfg['period']+1)*cfg['period']
    assert e['end'] == expected_end, 'public end'
    assert len(own['memories']) <= 16
    free = {p['id']: p['available'] for p in e['people']}
    assert set(free) == set(range(4)) and free[owner]
    contact = [dict(intent='Wait', reservation=0, eligible=True, score=0)]
    for person in e['people']:
        partner = person['id']
        if partner == owner: continue
        eligible = free[partner] and now + 8 <= expected_end
        contact.append(dict(intent=dict(Invite=dict(recipient=partner,kind='Resource')), reservation=8,eligible=eligible,score=resource_value(own,partner) if eligible else -1))
    query = e['query']['decision']
    if query['selected'] != 'Pause':
        ask = query['selected']['Ask']; reservation = {'Direct':10,'Evidence':28}[ask['strategy']] + 5
        eligible = free[ask['source']] and now + reservation <= expected_end and i['inquiry_lane_free']
        source_score = next(c['score'] for c in query['candidates'] if c['action']==query['selected'])
        contact.append(dict(intent=dict(Invite=dict(recipient=ask['source'],kind='Inquiry')),reservation=reservation,eligible=eligible,score=source_score-2 if eligible else -1))
    assert d['contact_candidates'] == contact, 'contact arithmetic'
    top = max(contact,key=lambda c:c['score'])
    assert d['contact'] == top['intent'], 'contact tie'
    eat = own['food'] > 0 and own['hunger'] > 0 and now < expected_end
    invite = top['intent'] != 'Wait'
    # Duplicate outgoing protocol masks only the final Invite activity.
    pending = any(p['invitation']['proposer']==owner for p in boundary['pending'])
    invite = invite and not pending
    cell = i['retained']
    acquire = bool(cell and cell['owner']==owner and cell['observation']['observed_at']<=now and cell['observation']['stock']>cell['own_takes'] and cell['observation']['accessible'] and own['food']==0 and own['hunger']>0 and now+i['duration']<=expected_end)
    candidates = [dict(action='Wait',eligible=True,score=0),dict(action='Consume',eligible=eat,score=own['hunger'] if eat else -1),dict(action='Invite',eligible=invite,score=top['score'] if invite else -1),dict(action='Acquire',eligible=acquire,score=own['hunger'] if acquire else -1)]
    assert d['candidates']==candidates, 'activity score/eligibility'
    assert d['selected']==max(candidates,key=lambda c:c['score'])['action'], 'activity stable tie'


def clean_end(value):
    if isinstance(value,dict):
        return {k: ('<public end>' if k=='end' and set(value)=={'own','profile','now','end','people','query'} else clean_end(v)) for k,v in value.items()}
    if isinstance(value,list): return [clean_end(v) for v in value]
    return value


def row(trial):
    h=trial['history']; cfg=trial['config']; p=physical(h,cfg['phase'])
    assert h['end']==h['start']+cfg['horizon']+33
    choices={}
    for bi,b in enumerate(h['boundaries']):
        for di,d in enumerate(b['choices']):
            decision(trial,b,d)
            choices[b['tick'],d['input']['legacy']['own']['id']]=(d['selected'],d['contact'] if d['selected']=='Invite' else None)
    starts={r['commitment']['id']:r['commitment'] for r in h['transitions'] if not r['completed']}
    ends={r['commitment']['id']:r['tick'] for r in h['transitions'] if r['completed']}
    assert starts.keys()==ends.keys() and all(ends[c]<=h['end'] for c in starts)
    reports=range(h['start']+cfg['period'],h['end']+1,cfg['period'])
    crossing=any(c['start']<t<ends[n] for t in reports for n,c in starts.items())
    nominal=h['start']+cfg['horizon']
    unfinished=any(c['start']<=nominal<ends[n] for n,c in starts.items()) or any(t<=nominal<t+2 and choice[0]=='Invite' for (t,_),choice in choices.items())
    scenes={e['scene'] for e in base(h)['events'][h['bootstrap_events']:]}
    transfers=sum(e['transferred'] for e in base(h)['events'][h['bootstrap_events']:])
    zero=sum(not any(e['transferred'] for e in base(h)['events'] if e['scene']==s) for s in scenes)
    return dict(area=p['area'],ledger=p['ledger'],choices=choices,crossing=crossing,unfinished=unfinished,social=len(scenes),meals=len(base(h)['cognition']['consumption']),transfers=transfers,zero=zero)


def inspect():
    counts=collections.Counter(); pairs=[]; previous=None; placebos={}; witnesses={}; controls=[]; sample=None
    with gzip.open(ROOT/'experiments/023/trials.jsonl.gz','rt',encoding='utf-8') as stream:
        for n,line in enumerate(stream):
            t=json.loads(line); h=t['history']; r=row(t); counts['worlds']+=1
            counts['strict_crossing_worlds']+=r['crossing'];counts['unfinished_nominal_worlds']+=r['unfinished']
            if n in (1,13):placebos[t['config']['case']]=h
            if n in (0,1,10,11,18,19,20,21,22,23):
                witnesses[str(n)]=dict(social=r['social'],meals=r['meals'],transfers=r['transfers'],zero_transfer=r['zero'],final_food=[a['food'] for a in h['final_agents']],final_hunger=[a['hunger'] for a in h['final_agents']],area=r['area'],stock=h['final_stock'])
            if n==0:sample=t
            if n<40:
                if n%2==0:previous=(t,r)
                else:
                    a,ra=previous; ah=a['history'];ca=dict(a['config']);cb=dict(t['config']);assert ca.pop('mode')=='Windowed' and cb.pop('mode')=='Continuous' and ca==cb
                    for key in ('initial','profiles','initial_stock','initial_total','initial_contexts','start','end','bootstrap_events'):assert ah[key]==h[key],('matched',key)
                    delta=[y-x for x,y in zip(ra['area'],r['area'],strict=True)]
                    different=[tick for tick in ra['ledger'] if ra['ledger'][tick]!=r['ledger'][tick]]
                    fields=dict(choice=ra['choices']!=r['choices'],terminal_food=ra['ledger'][h['end']]!=r['ledger'][h['end']],terminal_hunger=[v['hunger'] for v in ah['final_agents']]!=[v['hunger'] for v in h['final_agents']],terminal_trust=trust(ah)!=trust(h),complete_null=clean_end(ah)==clean_end(h),reconverged=bool(different and any(tick>different[0] and ra['ledger'][tick]==r['ledger'][tick] for tick in ra['ledger'])),higher_area=any(x>0 for x in delta),lower_area=any(x<0 for x in delta),more_zero_transfer=r['zero']>ra['zero'])
                    counts.update({k:int(v) for k,v in fields.items()});counts['pairs']+=1
                    pairs.append(dict(ordinals=[n-1,n],hunger_area_delta=delta,**fields))
            else:
                assert h==placebos[t['config']['case']];counts['exact_report_placebos']+=1
    def corrupt(name,change):
        t=copy.deepcopy(sample);change(t)
        try:row(t)
        except (AssertionError,KeyError):controls.append(name)
        else:raise AssertionError(('accepted corruption',name))
    corrupt('wrong_public_end',lambda t:t['history']['boundaries'][0]['choices'][0]['input']['legacy'].update(end=999))
    corrupt('wrong_consume_score',lambda t:t['history']['boundaries'][0]['choices'][0]['candidates'][1].update(score=999))
    corrupt('partner_private_input',lambda t:t['history']['boundaries'][0]['choices'][0]['input']['legacy'].update(partner_food=4))
    corrupt('wrong_phase',lambda t:t['history']['needs'][0].update(tick=t['history']['needs'][0]['tick']+1))
    corrupt('false_final_food',lambda t:t['history']['final_agents'][0].update(food=999))
    return dict(reviewed_revision='336ab08354bda6dbdb5171e4066e28ea28344b6b',summary=dict(counts),pairs=pairs,witnesses=witnesses,rejected_controls=controls)


def main():
    parser=argparse.ArgumentParser();parser.add_argument('--output');parser.add_argument('--expected');args=parser.parse_args()
    result=inspect()
    if args.expected:assert result==json.loads(Path(args.expected).read_bytes()),'independent review anchors changed'
    if args.output:
        with Path(args.output).open('x',encoding='utf-8') as f:json.dump(result,f,indent=2);f.write('\n')
    print(json.dumps(result['summary'],sort_keys=True)); print('023 complementary independent score/physical reconstruction PASS; scratch controls',len(result['rejected_controls']))
if __name__=='__main__':main()
