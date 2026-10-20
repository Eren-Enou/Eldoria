"""Independent024 complementary physical/score reconstruction. Explicitly inherits the independently checked023 physical arithmetic, without importing any024reader/evaluator/Rust code.

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
    hunger_by_tick = {}
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
        hunger_by_tick[tick] = [a['hunger'] for a in actors]
        if tick in captures:
            capture = captures[tick]
            assert capture['stock'] == stock
            assert [(a['food'], a['hunger']) for a in capture['agents']] == [(a['food'], a['hunger']) for a in actors]
    assert [(a['food'], a['hunger']) for a in trial['final_agents']] == [(a['food'], a['hunger']) for a in actors]
    assert trial['final_stock'] == stock and consumed == len(meals)
    return dict(area=area, ledger=ledger, hunger=hunger_by_tick)


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
    assert now == boundary['tick'] and now < trial['history']['start'] + trial['config']['world']['horizon']
    start = trial['history']['start']; cfg = trial['config']['world']
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
    terms=[]
    for candidate in candidates:
        base_score=candidate['score']
        relief=min(own['hunger'],40) if trial['config']['valuation']=='MealRelief' and candidate['action']=='Consume' and candidate['eligible'] else 0
        candidate['score']+=relief
        terms.append(dict(action=candidate['action'],eligible=candidate['eligible'],base_score=base_score,meal_relief=relief,final_score=candidate['score']))
    assert d['candidates']==candidates, 'activity score/eligibility'
    assert d['selected']==max(candidates,key=lambda c:c['score'])['action'], 'activity stable tie'

    return dict(terms=terms,selected=d['selected'])

def body_without_meal_score(history):
    # Immutable comparison projection: keep large original state by reference.
    h=dict(history);h['boundaries']=[]
    for boundary in history['boundaries']:
        b=dict(boundary);b['choices']=[]
        for choice in boundary['choices']:
            d=dict(choice);d['candidates']=[dict(c,score='<registered consumption score>') if c['action']=='Consume' else c for c in choice['candidates']]
            b['choices'].append(d)
        h['boundaries'].append(b)
    return h


def executed(history):
    ops=collections.defaultdict(list)
    for e in base(history)['events'][history['bootstrap_events']:]:ops[e['tick']].append(('resource',e['participants'],e['decision']['selected'],e['transferred']))
    for g in history['grants']:ops[g['tick']].append(('claim',g['owner'],g['outcome']))
    for m in base(history)['cognition']['consumption']:ops[m['tick']].append(('meal',m['actor']))
    compact=history['final_state']['base']['base']['base']['base']
    for q in compact['inquiry']['records']:ops[q['tick']].append(('query',q['decision']['input']['owner'],q['decision']['selected']))
    return dict(ops)


def row(trial):
    h=trial['history'];cfg=trial['config']['world'];p=physical(h,cfg['phase'])
    assert trial['schema']=='experiment-024-trial/v1' and h['schema']=='experiment-024-physical-history/v1'
    assert cfg['mode']=='Continuous' and trial['config']['valuation']in('Baseline','MealRelief')
    choices={};audit=[];missed=[];first_score_change=None
    for bi,b in enumerate(h['boundaries']):
        for di,d in enumerate(b['choices']):
            selection=decision(trial,b,d)
            audit.append(dict(boundary=bi,choice=di,policy=trial['config']['valuation'],selection=selection))
            owner=d['input']['legacy']['own']['id'];key=b['tick'],owner
            choices[key]=(d['selected'],d['contact']if d['selected']=='Invite'else None)
            if first_score_change is None and any(t['meal_relief']for t in selection['terms']):first_score_change=dict(tick=key[0],owner=owner,ref=f'history.boundaries[{bi}].choices[{di}]')
            meal=next(c for c in d['candidates']if c['action']=='Consume')
            if meal['eligible']and d['selected']!='Consume':missed.append(dict(tick=key[0],owner=owner,hunger=d['input']['legacy']['own']['hunger'],ref=f'history.boundaries[{bi}].choices[{di}]'))
    assert trial['valuations']==audit,'independent score audit'
    nominal=h['start']+cfg['horizon']
    before=[sum(p['hunger'][tick][owner] for tick in range(h['start'],nominal))for owner in range(4)]
    drain=[sum(p['hunger'][tick][owner] for tick in range(nominal,h['end']))for owner in range(4)]
    assert [x+y for x,y in zip(before,drain,strict=True)]==p['area']
    food_while_hungry=[sum(p['ledger'][tick][0][owner]>0 and p['hunger'][tick][owner]>0 for tick in range(h['start'],h['end']))for owner in range(4)]
    return dict(area=p['area'],before_nominal_area=before,drain_area=drain,ledger=p['ledger'],nominal_hunger=p['hunger'][nominal],food_while_hungry=food_while_hungry,choices=choices,execution=executed(h),missed=missed,first_score_change=first_score_change)


def inspect(path):
    pairs=[];previous=None;cache={};placebos=[];worlds=[]
    with (gzip.open(path,'rt',encoding='utf-8')if str(path).endswith('.gz')else open(path,encoding='utf-8'))as stream:
        for n,line in enumerate(stream):
            t=json.loads(line);r=row(t);h=t['history'];cfg=t['config']
            worlds.append(dict(ordinal=n,policy=cfg['valuation'],case=cfg['world']['case'],meals=len(base(h)['cognition']['consumption']),nonselected_eligible_meals=len(r['missed']),food_owned_hunger100_nonselected=sum(x['hunger']==100 for x in r['missed']),first_score_change=r['first_score_change'],area=r['area'],before_nominal_area=r['before_nominal_area'],drain_area=r['drain_area'],nominal_food=r['ledger'][h['start']+cfg['world']['horizon']][0],nominal_hunger=r['nominal_hunger'],food_owned_while_hungry_ticks=r['food_while_hungry'],final_food=[a['food']for a in h['final_agents']],final_hunger=[a['hunger']for a in h['final_agents']],final_stock=h['final_stock']))
            if n<2:cache[cfg['valuation']]=(h,t['valuations'])
            if n>=54:
                assert cache[cfg['valuation']]==(h,t['valuations']);placebos.append(n);continue
            if n%2==0:previous=(t,r);continue
            a,ar=previous;ah=a['history'];ac=dict(a['config']);bc=dict(cfg)
            assert ac.pop('valuation')=='Baseline'and bc.pop('valuation')=='MealRelief'and ac==bc
            for field in ('initial','initial_stock','initial_total','profiles','initial_contexts','start','end','bootstrap_events'):assert ah[field]==h[field]
            common=sorted(ar['choices'].keys()&r['choices'].keys())
            first=next((dict(tick=k[0],owner=k[1],baseline=list(ar['choices'][k]),meal_relief=list(r['choices'][k]))for k in common if ar['choices'][k]!=r['choices'][k]),None)
            # Exact legitimate input/opportunity prefixes until first voluntary difference.
            limit=first['tick']if first else h['end']
            def inputs(x):return {(b['tick'],d['input']['legacy']['own']['id']):d['input']for b in x['boundaries']for d in b['choices']if b['tick']<=limit}
            assert inputs(ah)==inputs(h),'unmatched pre-choice local input/opportunity'
            times=sorted(ar['execution'].keys()|r['execution'].keys());first_exec=next((tick for tick in times if ar['execution'].get(tick)!=r['execution'].get(tick)),None)
            delta=[y-x for x,y in zip(ar['area'],r['area'],strict=True)]
            diff=[tick for tick in ar['ledger']if ar['ledger'][tick]!=r['ledger'][tick]]
            recon=next((tick for tick in ar['ledger']if diff and tick>diff[0]and ar['ledger'][tick]==r['ledger'][tick]),None)
            pairs.append(dict(ordinals=[n-1,n],case=cfg['world']['case'],family=cfg['family'],first_selected=first,first_execution=first_exec,first_material=diff[0]if diff else None,temporary_food_reconvergence=recon,terminal_food_difference=ar['ledger'][h['end']]!=r['ledger'][h['end']],terminal_hunger_difference=[a['hunger']for a in ah['final_agents']]!=[a['hunger']for a in h['final_agents']],terminal_trust_difference=trust(ah)!=trust(h),full_history_body_equal=ah==h,body_null_ignoring_only_consume_scores=body_without_meal_score(ah)==body_without_meal_score(h),hunger_area_delta=delta,before_nominal_hunger_area_delta=[y-x for x,y in zip(ar['before_nominal_area'],r['before_nominal_area'],strict=True)],drain_hunger_area_delta=[y-x for x,y in zip(ar['drain_area'],r['drain_area'],strict=True)],matched_prefix_inputs=True))
    assert len(worlds)==56 and len(pairs)==27 and placebos==[54,55]
    summary=dict(worlds=56,pairs=27,report_placebos=2,selected_differences=sum(p['first_selected']is not None for p in pairs),executed_differences=sum(p['first_execution']is not None for p in pairs),terminal_food_differences=sum(p['terminal_food_difference']for p in pairs),terminal_hunger_differences=sum(p['terminal_hunger_difference']for p in pairs),terminal_trust_differences=sum(p['terminal_trust_difference']for p in pairs),full_history_body_nulls=sum(p['full_history_body_equal']for p in pairs),body_nulls_ignoring_only_consume_scores=sum(p['body_null_ignoring_only_consume_scores']for p in pairs),temporary_food_reconvergences=sum(p['temporary_food_reconvergence']is not None for p in pairs),pairs_any_higher_hunger_area=sum(any(x>0 for x in p['hunger_area_delta'])for p in pairs),pairs_any_lower_hunger_area=sum(any(x<0 for x in p['hunger_area_delta'])for p in pairs))
    return dict(summary=summary,pairs=pairs,worlds=worlds)


def main():
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('--output');p.add_argument('--expected');a=p.parse_args();result=inspect(a.archive)
    if a.expected:assert result==json.loads(Path(a.expected).read_bytes())
    if a.output:
        with Path(a.output).open('x',encoding='utf-8')as f:json.dump(result,f,indent=2);f.write('\n')
    print(json.dumps(result['summary'],sort_keys=True))
if __name__=='__main__':main()
