"""Explicit experiment-021-trial/v1 reader; no simulation or policy imports.

Owned projections read the exact captured Choice, never memory audit prefixes.
Audit joins below are observer-only causal navigation, not restored cognition.
"""
import copy

VERSION = 'experiment-021-trial/v1'
MODES = ('FreshReference','CurrentOnly','LastSeen','Expire12','Unobserved')


def validate(t):
    fields = {'seed','case','policy','mode','config','initial','profiles','points','periods',
              'final_state','final_point','records','clock','needs','initial_total','memory_records','final_memory'}
    if not isinstance(t,dict) or set(t) != fields or t['mode'] not in MODES or t['policy'] not in ('Need','Acquire'):
        raise ValueError('Unsupported 021 trial schema')
    if not t['points'] or not t['periods'] or len(t['final_memory']) != 3:
        raise ValueError('Empty or unsupported 021 history')
    tick = t['initial']['state']['tick']
    for p in t['points']:
        if p['state']['tick'] < tick or not {'cache','pending','acquisitions'} <= p.keys():
            raise ValueError('Invalid 021 environmental capture')
        tick = p['state']['tick']
    for p in t['periods']:
        for w in p['steps']:
            if len(w['choices']) != 3:
                raise ValueError('Unsupported 021 actor count')
            for field in ('before','observed','after','sent','admitted'):
                value = w['activity'][field]
                if value is not None and not 0 <= value < len(t['points']):
                    raise ValueError('Broken 021 point reference')
            for owner,d in enumerate(w['choices']):
                if d is None: continue
                if 'cache' in d or not {'retained','duration','inquiry_lane_free','policy','candidates','selected','acquiring_base'} == set(d):
                    raise ValueError('021 requires its retained-state schema, not a current 020 observation')
                if d['retained'] is not None and d['retained']['owner'] != owner:
                    raise ValueError('Foreign environmental record in 021 actor input')


def actor(t,w,owner,common):
    d = w['choices'][owner]
    if d is None: return None
    bd = d['acquiring_base'] if d['selected']=='Acquire' else w['activity']['decisions'][owner]
    proxy = dict(decisions=[None]*3); proxy['decisions'][owner] = bd
    view = common(t,proxy,owner)
    # Explicit allowlist; no lookup in memory_records, actual cache, attempts or
    # final_memory. None stays unavailable even if those still contain a quantity.
    cell = copy.deepcopy(d['retained'])
    age = None if cell is None else view['now']-cell['observation']['observed_at']
    view.update(environmental_knowledge=dict(retained=cell,age=age,
                    estimate=None if cell is None else max(0,cell['observation']['stock']-cell['own_takes']),
                    duration=d['duration'],availability='Unknown / unavailable' if cell is None else 'Retained dated observation; current stock unknown'),
                acquisition_policy=d['policy'],inquiry_lane_free=d['inquiry_lane_free'],
                candidates=copy.deepcopy(d['candidates']),selected=d['selected'])
    return view


def timeline(t,physical):
    result = physical(t)
    # Physical timeline is the unchanged 020 record protocol, not a 020 Trial
    # conversion. Every memory event retains its actual 021 source reference.
    ranges = []
    ordinal = 0
    for pn,p in enumerate(t['periods']):
        for sn,w in enumerate(p['steps']):
            ranges.append((w['memory_before'],w['memory_after'],ordinal,pn,sn,w))
            ordinal += 1
    for n,r in enumerate(t['memory_records']):
        context = next((v for v in ranges if v[0]<=n<v[1]),None)
        if context:
            _,_,order,pn,sn,w = context
            start = t['points'][w['activity']['observed']]['state']['tick']
            phase = 95 if r['tick']==start else 760 if w['activity']['active_before'] else 260
            order = order*1000+phase+n/100000
        else:
            order = ordinal*1000+999+n/100000
        point = r['point']
        if point is None:
            point = next((i for i,p in enumerate(t['points']) if p['state']['tick']>=r['tick']),len(t['points'])-1)
        result.append(dict(tick=r['tick'],kind='environmental_memory',ref=f'memory_records[{n}]',
                           label=f'Individual {r["owner"]}: environmental {r["kind"]} · owner-local receipt / retention audit',
                           point=point,step=None,data=r,rank=0,order=order))
    result.sort(key=lambda m:(m['tick'],m['order']))
    for n,m in enumerate(result): m['id']=n
    return result


def causal(t,identity,physical,resolve):
    graph = physical(t,identity,'retained.observation')
    receipts = {(r['owner'],r['after']['receipt']):(n,r) for n,r in enumerate(t['memory_records'])
                if r['kind'] in ('Sense','Depleted')}
    for outcome in t['records']:
        attempt = outcome['attempt']; pn,sn,owner = attempt['selection']
        root = f'periods[{pn}].steps[{sn}]'; d=t['periods'][pn]['steps'][sn]['choices'][owner]
        cell = d['retained']
        if cell is None: raise ValueError('Acquisition without retained 021 input')
        n,r = receipts[(owner,cell['receipt'])]
        chain = next(c for c in graph['chains'] if c['id']==root+'.activity')
        receipt = f'memory_records[{n}]'
        obs = receipt+'.after.observation'
        chosen = root+f'.choices[{owner}]'
        retained = chosen+'.retained'
        def add(ref,kind,tick,anchor,value):
            resolve(t,ref)
            if ref not in graph['nodes']:
                graph['nodes'][ref]=dict(id=ref,trial=identity,ref=ref,source_kind=kind,tick=tick,
                    actor=owner,recipient=None,anchor=anchor,before=None,after=None,outcome=value,original_id=None,ordinals=[])
            if ref not in chain['nodes']: chain['nodes'].append(ref)
        add(receipt,'Owned environmental receipt',r['tick'],receipt,dict(owner=owner,receipt=cell['receipt'],kind=r['kind']))
        add(obs,'Legitimate environmental observation',r['tick'],receipt,r['after']['observation'])
        add(retained,'Captured retained environmental input',attempt['start'],root+'.activity',cell)
        chain['edges'].extend([dict(source=obs,target=receipt,relation='Original owner-specific received observation'),
                               dict(source=receipt,target=retained,relation='Same owner-local receipt retained in actual choice; own-takes may change'),
                               dict(source=retained,target=chosen,relation='Captured available slot, never reconstructed from this observer graph')])
    return graph


def comparison(a,b,physical):
    result = physical(a,b)
    def trust(t):
        return [[x['trust'].get(str(j),0) for j in range(3)] for x in t['final_point']['state']['agents']]
    result['raw_final_trust_equal'] = result['final_trust_equal']
    result['final_trust_equal'] = trust(a)==trust(b)
    result['final_environmental_memory_equal'] = a['final_memory']==b['final_memory']
    result['full_final_snapshot_equal'] = result['full_final_snapshot_equal'] and result['final_environmental_memory_equal']
    result['environmental_comparison'] = dict(left_mode=a['mode'],right_mode=b['mode'],left_policy=a['policy'],right_policy=b['policy'],
        passive_slots_match=a['config']['cadence']==b['config']['cadence'],
        pure_retention_modes=all(t['mode'] in ('CurrentOnly','LastSeen','Expire12') for t in (a,b)),
        note='FreshReference changes sensing frequency; Unobserved removes the channel. Same external slots do not guarantee the same post-divergence busy states.')
    if result['first_choice']:
        result['first_choice']['kind']='First recorded 021 activity/schedule difference'
    result['note']=result['note'].replace(' 020:', ' Shared physical protocol:')
    result['note']+=' 021 retains one dated own slot; unknown is not known zero. Trust equality normalizes absent entries to zero; raw-map shape is reported separately. Environmental-memory equality is separate from physical outcomes.'
    return result
