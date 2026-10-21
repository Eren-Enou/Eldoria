"""Complementary025reconstruction: stdlib only, no evaluator/full reader/material025/viewer imports.
Independently checks physical mass/need and local contact/activity arithmetic.
Legacy nested inquiry/retention correctness is checked by the separate full reader.
"""
import argparse
import collections
import copy
import gzip
import json
from pathlib import Path

def base(trial):
    value = trial['final_state']
    while 'events' not in value:
        value = value['base']
    return value


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


def physical(trial):
    h=trial['history'];s=trial['config']['preparation'];cfg=trial['config']['world']
    actors=copy.deepcopy(h['initial']);stock=h['initial_stock'];raw=s['raw_initial'];waste=consumed=0;escrow=[0]*4
    initial=sum(a['food']for a in actors)+stock+raw
    assert initial==h['initial_total']
    ops=collections.defaultdict(list)
    b=base(h);events=b['events'][h['bootstrap_events']:];meals=b['cognition']['consumption']
    for row in h['needs']:ops[row['tick']].append((0,0,'need',row))
    for row in events:ops[row['tick']].append((1,row['scene'],'event',row))
    for n,row in enumerate(h['grants']):ops[row['tick']].append((2,n,'grant',row))
    for n,row in enumerate(meals):ops[row['tick']].append((3,n,'meal',row))
    starts={r['commitment']['id']:r['commitment']for r in h['transitions']if not r['completed']}
    ends={r['commitment']['id']:r['tick']for r in h['transitions']if r['completed']}
    processes={p['commitment']:p for p in trial['material']['processes']}
    assert set(processes)=={cid for cid,c in starts.items()if c['work']=='Prepare'}
    order=lambda p:p['owner']if cfg['order']=='Ascending'else 3-p['owner']if cfg['order']=='Descending'else(p['owner']+4-p['start']%4)%4
    expected=[]
    for p in processes.values():
        c=starts[p['commitment']]
        assert c['participants']==[p['owner']]and c['start']==p['start']and p['source']==s['source_id']==0
        assert p['extract_at']==p['start']+s['extraction_ticks']
        assert p['finish_at']==p['extract_at']+s['preparation_ticks']==c['deadline']==ends[c['id']]
        expected.extend([(p['extract_at'],0,order(p),p['commitment']),(p['finish_at'],1,order(p),p['commitment'])])
    material_ops=trial['material']['operations'];assert len(material_ops)==len(expected)
    for n,(key,row)in enumerate(zip(sorted(expected),material_ops,strict=True)):
        tick,phase,_,cid=key
        assert(row['id'],row['tick'],row['commitment'])==(n,tick,cid)
        assert row['owner']==processes[cid]['owner']and row['source']==0
        ops[tick].append((5+phase,n,'material',row))
    epoch=h['start']if cfg['phase']=='Epoch'else 0
    first=epoch+((h['start']-epoch)//4+1)*4
    assert[n['tick']for n in h['needs']]==list(range(first,h['end']+1,4))
    status={};area=[0]*4;pre=[0]*4;ledger={};holdings={};hungers={}
    captures={v['tick']:v for v in h['boundaries']};cutoff=h['start']+cfg['horizon']
    for tick in range(h['start'],h['end']+1):
        if tick>h['start']:
            for n,a in enumerate(actors):
                area[n]+=a['hunger']
                if tick<=cutoff:pre[n]+=a['hunger']
        for _,_,kind,row in sorted(ops[tick]):
            if kind=='need':
                assert row['before']==[a['hunger']for a in actors]
                for a in actors:a['hunger']=min(100,a['hunger']+5)
                assert row['after']==[a['hunger']for a in actors]
            elif kind=='event':
                ids=row['participants'];assert row['balances_before']==[actors[i]['food']for i in ids]
                assert sum(row['balances_before'])==sum(row['balances_after'])
                assert all(v>=0 for v in row['balances_after'])
                assert sum(abs(x-y)for x,y in zip(row['balances_before'],row['balances_after']))==2*row['transferred']
                for n,value in zip(ids,row['balances_after']):actors[n]['food']=value
            elif kind=='grant':
                a=actors[row['owner']];assert(row['stock_before'],row['food_before'])==(stock,a['food'])
                take=int(stock>0 and a['food']<2**32-1)
                stock-=take;a['food']+=take
                assert(row['stock_after'],row['food_after'])==(stock,a['food'])
            elif kind=='meal':
                a=actors[row['actor']];assert(row['food_before'],row['hunger_before'])==(a['food'],a['hunger'])
                assert a['food']>0 and a['hunger']>0 and row['consumed']==1
                a['food']-=1;a['hunger']=max(0,a['hunger']-40);consumed+=1
                assert(row['food_after'],row['hunger_after'])==(a['food'],a['hunger'])
            else:
                n=row['owner'];a=actors[n];cid=row['commitment'];p=processes[cid]
                assert(row['raw_before'],row['escrow_before'],row['food_before'],row['waste_before'])==(raw,escrow[n],a['food'],waste)
                if tick==p['extract_at']:
                    assert cid not in status and escrow[n]==0
                    if raw>=2:raw-=2;escrow[n]=2;outcome='Extracted'
                    else:outcome='Insufficient'
                    status[cid]=outcome
                else:
                    assert tick==p['finish_at']and cid in status
                    if status.pop(cid)=='Extracted':
                        assert escrow[n]==2;escrow[n]=0;a['food']+=1;waste+=1;outcome='Prepared'
                    else:assert escrow[n]==0;outcome='Failed'
                assert row['outcome']==outcome
                assert(row['raw_after'],row['escrow_after'],row['food_after'],row['waste_after'])==(raw,escrow[n],a['food'],waste)
            assert sum(a['food']for a in actors)+stock+raw+sum(escrow)+waste+consumed==initial
        food=[a['food']for a in actors];hunger=[a['hunger']for a in actors]
        holdings[tick]=[food,stock];ledger[tick]=[raw,escrow.copy(),stock,food,waste,consumed];hungers[tick]=hunger
        if tick in captures:
            cap=captures[tick];assert cap['stock']==stock
            assert[(a['food'],a['hunger'])for a in cap['agents']]==list(zip(food,hunger))
    assert not status and not any(escrow)
    assert h['final_stock']==stock and trial['material']['final_raw']==raw and trial['material']['final_waste']==waste
    assert[(a['food'],a['hunger'])for a in h['final_agents']]==[(a['food'],a['hunger'])for a in actors]
    return dict(ledger=ledger,holdings=holdings,hunger=hungers,area=area,pre=pre)


def decision(trial,b,d):
    i=d['input'];assert set(i)=={'legacy','retained','duration','inquiry_lane_free','raw_retained'}
    e=i['legacy'];assert set(e)=={'own','profile','now','end','people','query'}
    own=e['own'];now=e['now'];owner=own['id'];assert own==b['agents'][owner]and now==b['tick']and e['end']==now+33
    assert len(own['memories'])<=16
    free={p['id']:p['available']for p in e['people']};assert set(free)==set(range(4))and free[owner]
    contact=[dict(intent='Wait',reservation=0,eligible=True,score=0)]
    for person in e['people']:
        n=person['id']
        if n==owner:continue
        eligible=free[n]and now+8<=e['end']
        contact.append(dict(intent=dict(Invite=dict(recipient=n,kind='Resource')),reservation=8,eligible=eligible,score=resource_value(own,n)if eligible else-1))
    query=e['query']['decision']
    if query['selected']!='Pause':
        ask=query['selected']['Ask'];duration={'Direct':10,'Evidence':28}[ask['strategy']]+5
        eligible=free[ask['source']]and now+duration<=e['end']and i['inquiry_lane_free']
        score=next(c['score']for c in query['candidates']if c['action']==query['selected'])
        contact.append(dict(intent=dict(Invite=dict(recipient=ask['source'],kind='Inquiry')),reservation=duration,eligible=eligible,score=score-2 if eligible else-1))
    assert d['contact_candidates']==contact
    top=max(contact,key=lambda c:c['score']);assert d['contact']==top['intent']
    eat=own['food']>0 and own['hunger']>0
    invite=top['intent']!='Wait'and not any(p['invitation']['proposer']==owner for p in b['pending'])
    cell=i['retained'];raw=i['raw_retained'];assert cell is None or raw is None
    acquire=bool(cell and cell['owner']==owner and cell['observation']['observed_at']<=now and cell['observation']['stock']>cell['own_takes']and cell['observation']['accessible']and own['food']==0 and own['hunger']>0 and now+i['duration']<=e['end'])
    originals=[('Wait',True,0),('Consume',eat,own['hunger']if eat else-1),('Invite',invite,top['score']if invite else-1),('Acquire',acquire,own['hunger']if acquire else-1)]
    terms=[]
    for action,eligible,score in originals:
        relief=min(own['hunger'],40)if trial['config']['valuation']=='MealRelief'and action=='Consume'and eligible else 0
        terms.append(dict(action=action,eligible=eligible,base_score=score,meal_relief=relief,work_effort=0,final_score=score+relief))
    eligible=False;effort=0
    if raw:
        o=raw['observation'];effort=o['extraction_ticks']+o['preparation_ticks']
        eligible=bool(raw['owner']==owner and o['id']==0 and o['observed_at']<=now and o['stock']-raw['own_takes']>=2 and o['accessible']and(o['input_units'],o['edible_units'],o['waste_units'])==(2,1,1)and o['extraction_ticks']>0 and o['preparation_ticks']>0 and effort<=33 and own['food']==0 and own['hunger']>0 and now+effort<=e['end'])
    terms.append(dict(action='Prepare',eligible=eligible,base_score=own['hunger']if eligible else-1,meal_relief=0,work_effort=effort if eligible else 0,final_score=own['hunger']-effort if eligible else-1))
    selected=max(terms,key=lambda c:c['final_score'])['action']
    assert d['candidates']==terms and d['selected']==selected
    return dict(terms=terms,selected=selected)


def row(trial):
    assert trial['schema']=='experiment-025-trial/v1'
    h=trial['history'];assert h['schema']=='experiment-025-physical-history/v1'
    p=physical(trial);audit=[];choices={};starts=collections.defaultdict(list);effects=collections.defaultdict(list)
    for bi,b in enumerate(h['boundaries']):
        for di,d in enumerate(b['choices']):
            audit.append(dict(boundary=bi,choice=di,policy=trial['config']['valuation'],selection=decision(trial,b,d)))
            contact = None
            if d['selected']=='Invite':
                contact = [d['contact'], d['input']['legacy']['query']['decision']['selected'] if d['contact']['Invite']['kind']=='Inquiry' else None]
            choices[b['tick'],d['input']['legacy']['own']['id']]=(d['selected'],contact)
    assert audit==trial['valuations']
    for r in h['transitions']:
        if not r['completed']:starts[r['tick']].append(r['commitment'])
    for e in base(h)['events'][h['bootstrap_events']:]:effects[e['tick']].append(['resource',e])
    for g in h['grants']:effects[g['tick']].append(['claim',g])
    for m in base(h)['cognition']['consumption']:effects[m['tick']].append(['meal',m])
    for op in trial['material']['operations']:effects[op['tick']].append(['material',op])
    query_base=h['final_state']
    while 'inquiry' not in query_base:query_base=query_base['base']
    for q in query_base['inquiry']['records']:effects[q['tick']].append(['query',q])
    p.update(choices=choices,starts=dict(starts),effects=dict(effects))
    return p


def first_difference(left,right):
    return next((k for k in sorted(left.keys()|right.keys())if left.get(k)!=right.get(k)),None)


def compare(a,ar,b,br,ordinals,primary=True):
    ah,bh=a['history'],b['history']
    assert ah['start']==bh['start']and ah['end']==bh['end']
    if primary:
        ac,bc=copy.deepcopy(a['config']),copy.deepcopy(b['config'])
        assert ac['preparation'].pop('access_enabled')is False and bc['preparation'].pop('access_enabled')is True and ac==bc
    key=first_difference(ar['choices'],br['choices'])
    if primary:
        limit=key[0]if key else ah['end']
        def prefix(h):
            rows={}
            for block in h['boundaries']:
                if block['tick']>limit:continue
                for d in block['choices']:
                    i=copy.deepcopy(d['input'])
                    if i['raw_retained']is not None:i['raw_retained']['observation']['accessible']='<declared public access>'
                    rows[block['tick'],i['legacy']['own']['id']]=i
            return rows
        assert prefix(ah)==prefix(bh),'025 mismatched pre-divergence local/opportunity prefix'
    def difference(paths):
        left,right=paths;changed=[t for t in left if left[t]!=right[t]]
        return dict(first=changed[0]if changed else None,reconvergence=next((t for t in left if changed and t>changed[0]and left[t]==right[t]),None),terminal=left[ah['end']]!=right[bh['end']])
    return dict(ordinals=ordinals,primary_access_contrast=primary,first_selected=None if key is None else dict(tick=key[0],owner=key[1],left=ar['choices'].get(key),right=br['choices'].get(key)),first_work_start=first_difference(ar['starts'],br['starts']),first_completed_effect=first_difference(ar['effects'],br['effects']),full_material=difference((ar['ledger'],br['ledger'])),edible_holdings=difference((ar['holdings'],br['holdings'])),hunger_area_delta=[y-x for x,y in zip(ar['area'],br['area'])],pre_nominal_area_delta=[y-x for x,y in zip(ar['pre'],br['pre'])],terminal_hunger_difference=[a['hunger']for a in ah['final_agents']]!=[a['hunger']for a in bh['final_agents']],terminal_trust_difference=[a['trust'].get(str(n),0)for a in ah['final_agents']for n in range(4)]!=[a['trust'].get(str(n),0)for a in bh['final_agents']for n in range(4)])


def inspect(path,compatibility=False):
    worlds=[];pairs=[];cached={};previous=None;placebos=[]
    matrix=json.loads((Path(__file__).resolve().parents[2]/'experiments/025/registered-matrix.json').read_bytes())
    with(gzip.open(path,'rb')if str(path).endswith('.gz')else open(path,'rb'))as stream:
        for n,line in enumerate(stream):
            t=json.loads(line);r=row(t);h=t['history'];mat=t['material'];b=base(h)
            worlds.append(dict(ordinal=n,family=t['config']['family'],preparations=len(mat['processes']),successful=sum(o['outcome']=='Prepared'for o in mat['operations']),failed=sum(o['outcome']=='Insufficient'for o in mat['operations']),meals=len(b['cognition']['consumption']),transfers=sum(e['transferred']for e in b['events'][h['bootstrap_events']:]),area=r['area'],pre_nominal_area=r['pre'],final_full_material=r['ledger'][h['end']],final_hunger=[a['hunger']for a in h['final_agents']]))
            if compatibility:continue
            assert n<len(matrix)and t['config']==matrix[n]
            if n in(0,1,5,7,9,25):cached[n]=(t,r)
            if n>=48:
                original=cached[n-48][0];assert(t['history'],t['valuations'],t['material'])==(original['history'],original['valuations'],original['material']);placebos.append(n);continue
            if n%2==0:previous=(t,r)
            else:pairs.append(compare(*previous,t,r,[n-1,n]))
    if compatibility:
        assert len(worlds)==56
        return dict(summary=dict(compatibility_worlds=56,independent_physical_and_scalar_checks=True),worlds=worlds)
    assert len(worlds)==50 and len(pairs)==24 and placebos==[48,49]
    siblings=[compare(*cached[a],*cached[b],[a,b],primary=False)for a,b in[(5,7),(5,9),(1,25)]]
    summary=dict(worlds=50,primary_pairs=24,report_placebos=2,changed_selected_activity_or_contact=sum(p['first_selected']is not None for p in pairs),changed_work_starts=sum(p['first_work_start']is not None for p in pairs),changed_completed_effects=sum(p['first_completed_effect']is not None for p in pairs),terminal_full_material_differences=sum(p['full_material']['terminal']for p in pairs),terminal_hunger_differences=sum(p['terminal_hunger_difference']for p in pairs),terminal_trust_differences=sum(p['terminal_trust_difference']for p in pairs),pairs_with_any_higher_area=sum(any(v>0 for v in p['hunger_area_delta'])for p in pairs),pairs_with_any_lower_area=sum(any(v<0 for v in p['hunger_area_delta'])for p in pairs))
    return dict(summary=summary,pairs=pairs,sibling_sensitivities=siblings,worlds=worlds)


def main():
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('--compatibility',action='store_true');p.add_argument('--expected');p.add_argument('--output');args=p.parse_args()
    result=inspect(args.archive,args.compatibility)
    if args.expected:assert result==json.loads(Path(args.expected).read_bytes())
    if args.output:
        with Path(args.output).open('x',encoding='utf-8')as out:json.dump(result,out,indent=2);out.write('\n')
    print(json.dumps(result['summary'],sort_keys=True))


if __name__=='__main__':main()


