"""Complementary read-only scientific review017: derive scope from original data.
No evaluator, archived classification flags, or review017 imports.
"""
import collections,gzip,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def read(name):return json.loads(gzip.decompress((ROOT/'experiments/017'/(name+'.json.gz')).read_bytes()))
def material(t):return (t['final_food'],t['final_hunger'])
def main():
    rows=read('summary');assert len(rows)==940
    groups=collections.defaultdict(dict)
    for r in rows:
        k=(r['seed'],r['case'],json.dumps(r['config'],sort_keys=True));assert r['mode'] not in groups[k];groups[k][r['mode']]=r
    assert len(groups)==188 and len({r['case'] for r in rows})==13
    changes=[];history=[]
    for key,g in groups.items():
        assert set(g)=={'Serial','Committed','SerialActive','Active','NoHistory'}
        a,b=g['Serial'],g['Committed']
        for field in ('invitations','declined','unmet','meetings','queries','transfers','voluntary_consumed','consumed','final_food','final_hunger'):assert a[field]==b[field],(key,field)
        a,b=g['SerialActive'],g['Active']
        if material(a)!=material(b):
            changes.append(dict(seed=key[0],case=key[1],config=b['config']))
            assert b['consumed']==a['consumed']+1 and b['transfers']==a['transfers']-1 and b['queries']==a['queries']
            assert all(x<=y for x,y in zip(b['final_hunger'],a['final_hunger'],strict=True))
        a,b=g['Active'],g['NoHistory']
        if material(a)!=material(b):history.append((a,b))
    assert len(changes)==7 and all(c['case']=='competing_eater' for c in changes)
    assert collections.Counter((c['config']['order'],c['config']['budget']) for c in changes)=={('Descending',24):4,('Ascending',48):3}
    assert len(history)==34
    assert sum(b['consumed']>a['consumed'] for a,b in history)==7
    assert sum(b['consumed']<a['consumed'] for a,b in history)==3
    assert sum(any(v>u for u,v in zip(a['final_hunger'],b['final_hunger'],strict=True)) for a,b in history)==3
    order_totals={}
    for mode,wanted in [('Active',[(29,6,17,21),(48,8,32,21),(32,6,22,21)]),('NoHistory',[(0,0,15,0),(0,0,20,0),(0,0,40,0)])]:
        for order,target in zip(('Ascending','Descending','Rotating'),wanted,strict=True):
            rs=[r for r in rows if r['seed']==42 and r['mode']==mode and r['config']['order']==order and r['config']['budget']==24 and r['config']['periods']==6]
            assert len(rs)==13
            totals=tuple(sum(r[k] for r in rs) for k in ('meetings','queries','transfers','consumed'))
            assert (totals==target if mode=='Active' else totals[2]==target[2])
            order_totals[mode+'/'+order]=totals
    # Inspect original full held-out histories, not the derived analysis.json.
    ts=read('held-out-seed-42')
    chosen={t['mode']:t for t in ts if t['case']=='competing_eater' and t['config']['order']=='Descending' and t['config']['budget']==24}
    a,b=chosen['SerialActive'],chosen['Active']
    def snap(t):return t['final_state']['base']['base']['base']['base']['base']
    def step(t,tick):return next(s for s in t['periods'][0]['steps'] if t['points'][s['observed']]['state']['tick']==tick)
    assert a['initial']==b['initial'] and a['profiles']==b['profiles']
    # At the first subsequent common boundary, no evidence/learning/event/concern
    # prefix counter differs. Only independently executed own food/hunger differs.
    x,y=[t['points'][step(t,4)['observed']] for t in (a,b)]
    assert {k:v for k,v in x.items() if k!='state'}=={k:v for k,v in y.items() if k!='state'}
    assert {k:v for k,v in x['state'].items() if k!='agents'}=={k:v for k,v in y['state'].items() if k!='agents'}
    assert x['state']['agents'][:2]==y['state']['agents'][:2]
    assert dict(x['state']['agents'][2],food=0,hunger=0)==y['state']['agents'][2]
    assert snap(a)['events'][:2]==snap(b)['events'][:2]
    for m in ('Serial','Committed'):
        assert chosen[m]['final_state']==chosen['Serial']['final_state']
    assert any(s['active_before'] for s in chosen['Committed']['periods'][0]['steps'])
    assert not any(s['active_before'] for s in chosen['Serial']['periods'][0]['steps'])
    assert all(not s['consumption'] for p in chosen['Committed']['periods'] for s in p['steps'])
    # Current-local twins include a genuinely different private response, not just
    # a vacuous same-local/same-future fixture.
    twins=0
    for order in ('Descending','Rotating'):
        for mode in chosen:
            p,q=[next(t for t in ts if t['case']==case and t['config']['budget']==24 and t['config']['order']==order and t['mode']==mode) for case in ('information','private_mirror')]
            ps,qs=p['periods'][0]['steps'][0],q['periods'][0]['steps'][0]
            assert ps['decisions'][0]==qs['decisions'][0]
            assert ps['responses'][2]['selected']!=qs['responses'][2]['selected'];twins+=1
    print(json.dumps(dict(summary_worlds=len(rows),matched_groups=len(groups),serial_committed_material_nulls=188,serialactive_active_material_changes=changes,active_nohistory_material_changes=len(history),common_boundary_tick=4,private_response_twins=twins,order_totals=order_totals,review='ACCEPT WITH QUALIFICATION'),sort_keys=True))
if __name__=='__main__':main()
