"""Explicit experiment-020-trial/v1 observation adapter. Original values stay intact.
Common legacy protocol readers are reused only for unchanged nested record types.
No policy execution, source learning, or reconstruction of missing actor cognition.
"""
import copy

VERSION = 'experiment-020-trial/v1'


def validate(t):
    required = {'initial_total','final_point','records','points','periods','initial','profiles','final_state','clock','needs','mode','config','case','seed'}
    if not isinstance(t, dict) or set(t) != required or t['mode'] not in ('Owned','Closed','Ignored','Need','Acquire'):
        raise ValueError('Unsupported 020 trial schema')
    if not t['points'] or not t['periods']:
        raise ValueError('Empty 020 history')
    tick = t['initial']['state']['tick']
    for p in t['points']:
        if p['state']['tick'] < tick or not {'cache','pending','acquisitions'} <= p.keys():
            raise ValueError('Invalid 020 environmental capture')
        tick = p['state']['tick']
    for p in t['periods']:
        for w in p['steps']:
            if len(w['choices']) != 3: raise ValueError('Unsupported 020 actor count')
            for field in ('before','observed','after','sent','admitted'):
                value = w['activity'][field]
                if value is not None and not 0 <= value < len(t['points']):
                    raise ValueError('Broken 020 point reference')


def actor(t, w, owner, common):
    d = w['choices'][owner]
    if d is None: return None
    bd = d['acquiring_base'] if d['selected'] == 'Acquire' else w['activity']['decisions'][owner]
    proxy = dict(decisions=[None]*3); proxy['decisions'][owner] = bd
    view = common(t, proxy, owner)
    view.update(cache=copy.deepcopy(d['cache']), acquisition_policy=d['policy'],
                inquiry_lane_free=d['inquiry_lane_free'], candidates=copy.deepcopy(d['candidates']), selected=d['selected'])
    return view


def timeline(t, common):
    result = common(t); ordinal = 0
    by_ref = {m['ref']:m for m in result}
    for pn,p in enumerate(t['periods']):
        for sn,w in enumerate(p['steps']):
            root = f'periods[{pn}].steps[{sn}]'; activity = w['activity']
            decision = by_ref[root+'.activity']
            decision['label'] = ', '.join(f'{i}: {d["selected"]}' for i,d in enumerate(w['choices']) if d) or 'Occupied; no new choice'
            for i,a in enumerate(w['admissions']):
                result.append(dict(tick=a['start'],kind='claim',ref=root+f'.admissions[{i}]',
                                   label=f'Individual {a["actor"]}: claim {a["id"]}, busy until {a["end"]}; outcome unknown at admission',
                                   point=activity['observed'],step=[pn,sn],data=a,rank=0,order=ordinal*1000+150))
            for rid in w['completed']:
                r=t['records'][rid]
                phase=750 if activity['active_before'] else 250 if r['completed_at'] <= decision['tick']+1 else 450
                point=next(i for i,p in enumerate(t['points']) if p['state']['tick'] >= r['completed_at'] and p['acquisitions'] > rid)
                result.append(dict(tick=r['completed_at'],kind='acquisition',ref=f'records[{rid}]',
                                   label=f'Individual {r["attempt"]["actor"]}: {r["outcome"]}; stock {r["cache_before"]} → {r["cache_after"]}',
                                   point=point,step=None,data=r,rank=0,order=ordinal*1000+phase+rid/10000))
            ordinal += 1
    result.sort(key=lambda m:(m['tick'],m['order']))
    for i,m in enumerate(result): m['id']=i
    return result


def causal(t, identity, common, resolve, observation_field='cache'):
    graph = common(t, identity)
    for r in t['records']:
        attempt = r['attempt']; pn,sn,owner=attempt['selection']
        root=f'periods[{pn}].steps[{sn}]'; w=t['periods'][pn]['steps'][sn]
        ci=next(i for i,a in enumerate(w['admissions']) if a['id']==attempt['id'])
        chain=next(c for c in graph['chains'] if c['id']==root+'.activity')
        def add(ref, kind, tick, anchor, outcome):
            resolve(t,ref)
            if ref not in graph['nodes']:
                graph['nodes'][ref]=dict(id=ref,trial=identity,ref=ref,source_kind=kind,tick=tick,
                    actor=owner,recipient=None,anchor=anchor,before=None,after=None,outcome=outcome,original_id=None,ordinals=[])
            if ref not in chain['nodes']: chain['nodes'].append(ref)
            return ref
        def edge(a,b,label): chain['edges'].append(dict(source=a,target=b,relation=label))
        choice=add(root+f'.choices[{owner}]','Selected acquisition',attempt['start'],root+'.activity','Acquire')
        obs=add(choice+'.'+observation_field,'Local cache observation',attempt['observation']['observed_at'],root+'.activity',attempt['observation'])
        admitted=add(root+f'.admissions[{ci}]','Claim admission / busy interval',attempt['start'],root+f'.admissions[{ci}]',dict(start=attempt['start'],end=attempt['end']))
        completion=add(f'records[{r["id"]}]','Physical acquisition outcome',r['completed_at'],f'records[{r["id"]}]',dict(outcome=r['outcome'],stock=[r['cache_before'],r['cache_after']],owned=[r['food_before'],r['food_after']],rank=r['rank']))
        edge(obs,choice,'Captured observation used by this choice')
        edge(choice,admitted,'Exact recorded selection reference')
        edge(admitted,completion,'Completion embeds exact attempt; whole busy duration charged')
        for xp,p in enumerate(t['periods']):
            for xs,s in enumerate(p['steps']):
                if owner not in s['skipped_responses']: continue
                sent=s['activity']['sent']
                if sent is not None and attempt['start'] <= t['points'][sent]['state']['tick'] < attempt['end']:
                    skipped_index=s['skipped_responses'].index(owner)
                    ref=f'periods[{xp}].steps[{xs}].skipped_responses[{skipped_index}]'
                    skip=add(ref,'Busy response omission',t['points'][sent]['state']['tick'],f'periods[{xp}].steps[{xs}].activity.responses',dict(actor=owner,recorded_omission=True))
                    edge(admitted,skip,'Actor is inside exact busy interval and explicitly omitted; no reply invented')
                    for ai,allocation in enumerate(s['activity']['allocations']):
                        if allocation['invitation']['recipient']==owner and allocation['disposition']=='Unavailable':
                            ar=f'periods[{xp}].steps[{xs}].activity.allocations[{ai}]'
                            target=add(ar,'Arbitration',t['points'][s['activity']['admitted']]['state']['tick'],f'periods[{xp}].steps[{xs}].activity.allocations','Unavailable')
                            edge(skip,target,'Same actual invitation recipient; recorded Unavailable after busy response omission')
        # Show later ownership-bearing operations as chronological context, not
        # unique unit provenance or proof that acquisition alone caused selection.
        base=t['final_state']['base']['base']['base']['base']['base']
        later=[]
        for i,e in enumerate(base['events']):
            if e['tick'] > r['completed_at'] and owner in e['participants']:
                later.append((e['tick'],f'final_state.base.base.base.base.base.events[{i}]','Later social action',e['decision']['selected']))
        for pp,p in enumerate(t['periods']):
            for ss,s in enumerate(p['steps']):
                for mi,c in enumerate(s['activity']['consumption']):
                    if c['actor']==owner and c['tick']>r['completed_at']:
                        later.append((c['tick'],f'periods[{pp}].steps[{ss}].activity.consumption[{mi}]','Later meal',c))
        for tick,ref,kind,value in sorted(later)[:3]:
            target=add(ref,kind,tick,ref,value)
            edge(completion,target,'Later recorded context involving this owner; not unique food provenance or isolated causation')
        if r['outcome']=='Granted' and t['initial_total']==1 and not any(a['food'] for a in t['initial']['state']['agents']):
            transfer=next((e for e in base['events'] if e['tick']>r['completed_at'] and e['transferred'] and e['decision']['observation']['last_signal']==dict(actor=owner,action='Offer')),None)
            if transfer:
                ref=f'final_state.base.base.base.base.base.events[{transfer["id"]}]'
                tr=add(ref,'Resource action',transfer['tick'],ref,transfer['decision']['selected'])
                edge(completion,tr,'Only initial unit was unowned; conserved grant enables this recorded transfer. Not the sole cause of consent.')
                eater=transfer['decision']['actor']
                for pp,p in enumerate(t['periods']):
                    for ss,s in enumerate(p['steps']):
                        for mi,c in enumerate(s['activity']['consumption']):
                            if c['actor']==eater and c['tick']>transfer['tick']:
                                mr=f'periods[{pp}].steps[{ss}].activity.consumption[{mi}]'
                                meal=add(mr,'Executed meal',c['tick'],mr,c)
                                edge(tr,meal,'One total conserved unit: recipient later consumes it; intermediate transfers remain in original history')
    return graph


def comparison(a,b, common):
    result=common(a,b)
    def selected(t):
        return [(t['points'][w['activity']['observed']]['state']['tick'],[None if d is None else d['selected'] for d in w['choices']]) for p in t['periods'] for w in p['steps']]
    import itertools
    result['first_choice']=next((dict(kind='First recorded 020 activity/schedule difference',tick=(x or y)[0],left=x,right=y) for x,y in itertools.zip_longest(selected(a),selected(b)) if x!=y),None)
    def executed(t):
        items=[]
        for r in t['records']: items.append((r['completed_at'],'acquisition',r['attempt']['actor'],r['outcome'],r['cache_after'],r['food_after']))
        base=t['final_state']['base']['base']['base']['base']['base']
        for e in base['events'][t['initial']['state']['events']:]: items.append((e['tick'],'resource',e['decision']['actor'],e['decision']['selected'],e['transferred'],e['balances_after']))
        for p in t['periods']:
            for w in p['steps']:
                for claim in w['admissions']:
                    items.append((claim['start'],'claim',claim['actor'],claim['cache'],claim['end']))
                if w['activity']['started'] is not None:
                    c=w['activity']['started']
                    items.append((c['start'],'commitment',c['invitation']))
                for c in w['activity']['consumption']: items.append((c['tick'],'meal',c['actor'],c['food_after'],c['hunger_after']))
                if w['activity']['query'] is not None:
                    q=t['final_state']['base']['base']['base']['base']['inquiry']['records'][w['activity']['query']]
                    items.append((t['points'][w['activity']['after']]['state']['tick'],'inquiry',q))
        return sorted(items,key=lambda x:(x[0],{'resource':0,'acquisition':1,'meal':2,'inquiry':3,'claim':4,'commitment':5}[x[1]]))
    first=next(((x,y) for x,y in itertools.zip_longest(executed(a),executed(b)) if x!=y),None)
    result['first_executed_difference']=None if first is None else {k:None if x is None else dict(tick=x[0],kind=x[1],value=x[2:]) for k,x in zip(('left','right'),first)}
    result.update(potential_supply_equal=a['initial_total']==b['initial_total'],
                  owned_placement_equal=a['initial']['state']['agents']==b['initial']['state']['agents'],
                  cache_access_equal=a['initial']['cache']['accessible']==b['initial']['cache']['accessible'],
                  final_trust_equal=[x['trust'] for x in a['final_point']['state']['agents']]==[x['trust'] for x in b['final_point']['state']['agents']])
    result['owned_final_material_equal']=result['final_material_equal']
    result['final_cache_stock_equal']=a['final_point']['cache']['stock']==b['final_point']['cache']['stock']
    result['final_material_equal']=result['owned_final_material_equal'] and result['final_cache_stock_equal']
    result['legacy_final_snapshot_equal']=result['full_final_snapshot_equal']
    result['full_final_snapshot_equal']=result['legacy_final_snapshot_equal'] and a['final_point']==b['final_point']
    pa={p['state']['tick']:(i,p) for i,p in enumerate(a['points'])}
    pb={p['state']['tick']:(i,p) for i,p in enumerate(b['points'])}
    def material(p): return ([(x['food'],x['hunger']) for x in p['state']['agents']],p['cache']['stock'])
    result['material_different_shared_ticks']=[dict(tick=tick,left=pa[tick][0],right=pb[tick][0]) for tick in sorted(pa.keys()&pb.keys()) if material(pa[tick][1])!=material(pb[tick][1])]
    result['material_reconvergence']=bool(result['material_different_shared_ticks']) and result['final_material_equal']
    def pressure_area(t):
        hunger=[a['hunger'] for a in t['initial']['state']['agents']];area=[0]*len(hunger)
        meals={}
        for p in t['periods']:
            for w in p['steps']:
                for c in w['activity']['consumption']:meals.setdefault(c['tick'],[]).append(c)
        epoch=t['clock']['epoch'];settings=t['clock']['settings']
        for tick in range(epoch,t['clock']['last']):
            area=[v+h for v,h in zip(area,hunger)]
            if (tick+1-epoch)%settings['interval']==0:hunger=[min(100,h+settings['increment']) for h in hunger]
            for c in meals.get(tick+1,[]):hunger[c['actor']]=c['hunger_after']
        return area
    result['model_hunger_area']=dict(left=pressure_area(a),right=pressure_area(b),note='Observer integral of model hunger per elapsed tick, not welfare or a universal benefit ranking.')
    result['note']+=' 020: first execution also includes actual claim admissions and resource commitment starts. Initial ownership/access are separate treatments. Common-time captured states include cache stock separately. Legacy export equality includes observer audit; inequality alone does not establish retained cognition or relationship divergence. Later chronological context is not a unique causal attribution.'
    return result
