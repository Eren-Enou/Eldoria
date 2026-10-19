"""Derived 022 comparisons. Original archives and the independent reader stay read-only."""
import argparse
import collections
import json
from pathlib import Path
import review022 as primitive


def semantic_trust(agents):
    return [a['trust'].get(str(partner),0) for a in agents for partner in range(4) if partner != a['id']]


def material(t):
    return [a['food'] for a in t['final_agents']], t['final_stock'], len(primitive.core(t)['cognition']['consumption'])


def metric(t):
    m=primitive.verify(t)
    m['selected_invitations']=sum(d['selected']=='Invite' for b in t['boundaries'] for d in b['choices'])
    m['accepted_replies']=sum(isinstance(r['selected'],dict) for b in t['boundaries'] for r in b['responses'])
    ends={r['commitment']['id']:r['tick'] for r in t['transitions'] if r['completed']}
    starts=[r for r in t['transitions'] if not r['completed']]
    m['peak_commitments']=max(sum(r['tick']<=tick<ends[r['commitment']['id']] for r in starts) for tick in range(t['start'],t['end']))
    b=primitive.core(t); es=b['events'][t['bootstrap_events']:]
    m['offers']=sum(e['decision']['selected']=='Offer' for e in es)
    m['refusals']=sum(e['outcome']=='Refusal' for e in es)
    m['cross_bootstrap_pair_transfers']=sum(e['transferred'] for e in es if e['participants'][0]//2 != e['participants'][1]//2)
    m['final_food']=[a['food'] for a in t['final_agents']]
    m['final_hunger']=[a['hunger'] for a in t['final_agents']]
    m['final_stock']=t['final_stock']
    m['semantic_trust']=semantic_trust(t['final_agents'])
    m['world_ticks']=t['end']-t['start']
    m['bytes']=len(json.dumps(t,separators=(',',':')).encode('utf-8'))
    return m


def compare(a,b,ma,mb,ordinal):
    ca,cb=dict(a['config']),dict(b['config']);assert ca.pop('capacity')==1 and cb.pop('capacity')==2 and ca==cb
    assert a['initial']==b['initial'] and a['initial_stock']==b['initial_stock'] and a['profiles']==b['profiles']
    assert a['initial_contexts']==b['initial_contexts'] and a['boundaries'][0]==b['boundaries'][0]
    assert primitive.core(a)['events'][:a['bootstrap_events']]==primitive.core(b)['events'][:b['bootstrap_events']]
    aa={x['tick']:x for x in a['boundaries']};bb={x['tick']:x for x in b['boundaries']}
    material_ticks=[tick for tick in sorted(aa.keys()&bb.keys()) if ([x['food'] for x in aa[tick]['agents']],aa[tick]['stock'],aa[tick]['meals'])!=([x['food'] for x in bb[tick]['agents']],bb[tick]['stock'],bb[tick]['meals'])]
    local_ticks=[tick for tick in sorted(aa.keys()&bb.keys()) if aa[tick]['choices']!=bb[tick]['choices']]
    clone_a,clone_b=dict(a),dict(b);clone_a['config']=ca;clone_b['config']=cb
    return dict(serial=ordinal,concurrent=ordinal+1,config=ca,matched_initial=True,
                first_local_record_difference=local_ticks[0] if local_ticks else None,
                first_observed_material_difference=material_ticks[0] if material_ticks else None,
                terminal_material_equal=material(a)==material(b),
                terminal_semantic_trust_equal=semantic_trust(a['final_agents'])==semantic_trust(b['final_agents']),
                complete_null_except_capacity=clone_a==clone_b,
                concurrent_minus_serial_hunger_area=[y-x for x,y in zip(ma['hunger_area'],mb['hunger_area'],strict=True)],
                serial_metrics=ma,concurrent_metrics=mb)


def main():
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('output');args=p.parse_args()
    rows=[];comparisons=[];slices=[];previous=None
    for n,t in enumerate(primitive.read(args.archive)):
        m=metric(t);rows.append(dict(ordinal=n,config=t['config'],metrics=m))
        if n%2: comparisons.append(compare(previous,t,rows[-2]['metrics'],m,n-1))
        else: previous=t
        if n in (1,3,5,11,15,18,20,30,33,35):
            resources=[(i,r) for i,r in enumerate(t['transitions']) if not r['completed'] and primitive.kind(r['commitment'])=='Resource']
            windows=[]
            for i,r in resources[:4]:
                c=r['commitment'];release=next((j,end) for j,end in enumerate(t['transitions']) if end['completed'] and end['commitment']['id']==c['id'])
                sid=c['work']['Resource']['scene'];events=[e for e in primitive.core(t)['events'] if e['scene']==sid]
                windows.append(dict(start_ref=f'transitions[{i}]',release_ref=f'transitions[{release[0]}]',admission_source=c['source'],participants=c['participants'],start=r['tick'],end=release[1]['tick'],scene=sid,event_ids=[e['id'] for e in events],actions=[e['decision']['selected'] for e in events],transferred=sum(e['transferred'] for e in events)))
            slices.append(dict(ordinal=n,config=t['config'],first_resource_windows=windows,
                               inquiry_admissions=[dict(ref=f'admissions[{i}]',record=a) for i,a in enumerate(t['admissions']) if a['invitation']['kind']=='Inquiry'],
                               first_busy_rejection=next((dict(ref=f'admissions[{i}]',record=a) for i,a in enumerate(t['admissions']) if a['reason']=='Unavailable'),None)))
    output=Path(args.output);output.mkdir(parents=True,exist_ok=True)
    summary=dict(worlds=len(rows),matched_pairs=len(comparisons),concurrent_peak_two=sum(r['metrics']['peak_social']==2 for r in rows if r['config']['capacity']==2),
                 complete_null_pairs=sum(c['complete_null_except_capacity'] for c in comparisons),
                 interim_material_difference_pairs=sum(c['first_observed_material_difference'] is not None for c in comparisons),
                 terminal_material_difference_pairs=sum(not c['terminal_material_equal'] for c in comparisons),
                 terminal_trust_difference_pairs=sum(not c['terminal_semantic_trust_equal'] for c in comparisons),
                 totals={name:sum(r['metrics'][name] for r in rows) for name in ('social','queries','selected_invitations','accepted_replies','transfers','meals','grants','failed_claims','refusals','zero_transfer','cross_bootstrap_pair_transfers')})
    for name,value in [('reports.json',rows),('comparisons.json',comparisons),('causal-slices.json',slices),('analysis.json',summary)]:
        (output/name).write_text(json.dumps(value,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(summary))


if __name__=='__main__':main()
