"""Observer-only025causal measurements. No outcome is a policy input or utility."""
import argparse
import gzip
import json
from pathlib import Path

import review025
import review025_findings


def refs(path,n):return f'{path}[{n}]'


def chains(t):
    h=t['history'];mat=t['material'];core=review025.core(h)
    events=core['events'][h['bootstrap_events']:];meals=core['cognition']['consumption']
    starts={r['commitment']['id']:(n,r['commitment'])for n,r in enumerate(h['transitions'])if not r['completed']}
    releases={r['commitment']['id']:n for n,r in enumerate(h['transitions'])if r['completed']}
    results=[]
    for p in mat['processes']:
        si,c=starts[p['commitment']];bi=c['source'];owner=p['owner']
        di,d=next((n,d)for n,d in enumerate(h['boundaries'][bi]['choices'])if d['input']['legacy']['own']['id']==owner)
        ai=next(n for n,a in enumerate(t['valuations'])if(a['boundary'],a['choice'])==(bi,di))
        operations=[(n,o)for n,o in enumerate(mat['operations'])if o['commitment']==p['commitment']]
        assert len(operations)==2
        en,extracted=operations[0];cn,completed=operations[1]
        opportunities=[]
        for n,b in enumerate(h['boundaries']):
            if b['tick']<p['finish_at']:continue
            for k,choice in enumerate(b['choices']):
                own=choice['input']['legacy']['own']
                if own['id']==owner:opportunities.append(dict(tick=b['tick'],ref=f'history.boundaries[{n}].choices[{k}]',kind='ordinary',food=own['food'],hunger=own['hunger'],selected=choice['selected'],eligible_consume=choice['candidates'][1]['eligible']))
            for k,response in enumerate(b['responses']):
                own=response['input']['own']['own']
                if own['id']==owner:opportunities.append(dict(tick=b['tick'],ref=f'history.boundaries[{n}].responses[{k}]',kind='recipient',food=own['food'],hunger=own['hunger'],selected=response['selected']))
        debits=[];credits=[]
        if completed['outcome']=='Prepared':
            for n,meal in enumerate(meals):
                if meal['actor']==owner and meal['tick']>p['finish_at']:
                    debits.append(dict(tick=meal['tick'],kind='meal',ref=f'history.final_state.base.base.base.base.base.cognition.consumption[{n}]',before=meal['food_before'],after=meal['food_after']))
            for event in events:
                if owner not in event['participants']or event['tick']<=p['finish_at']:continue
                position=event['participants'].index(owner);before=event['balances_before'][position];after=event['balances_after'][position]
                change=dict(tick=event['tick'],kind='resource_transfer',ref=f'history.final_state.base.base.base.base.base.events[{event["id"]}]',before=before,after=after,actor=event['decision']['actor'],selected=event['decision']['selected'])
                if after<before:debits.append(change)
                elif after>before:credits.append(change)
            for n,g in enumerate(h['grants']):
                if g['owner']==owner and g['tick']>p['finish_at']and g['food_after']>g['food_before']:credits.append(dict(tick=g['tick'],kind='ready_grant',ref=refs('history.grants',n)))
        first=min(debits,key=lambda x:x['tick'])if debits else None
        isolated=bool(first and completed['food_before']==0 and completed['food_after']==1 and first['before']==1 and not any(c['tick']<=first['tick']for c in credits))
        later=[o for o in opportunities if o['tick']>(first['tick']if first else p['finish_at'])]
        crossings=[dict(ref=refs('checkpoints',n),tick=cp['tick'],material_point=refs('material.points',cp['material_point']))for n,cp in enumerate(t['checkpoints'])if 'Reporting'in cp['labels']and p['commitment']in cp['active']and p['start']<cp['tick']<p['finish_at']]
        blocked=[dict(ref=refs('history.admissions',n),tick=a['tick'],reason=a['reason'],warning='A blocked proposal is not guaranteed useful contact or consent')for n,a in enumerate(h['admissions'])if p['start']<=a['tick']<p['finish_at']and owner in(a['invitation']['proposer'],a['invitation']['recipient'])and a['reason']!='Execute']
        overlaps=[]
        for cid,(index,other)in starts.items():
            end=h['transitions'][releases[cid]]['tick']
            if cid!=p['commitment']and max(p['start'],other['start'])<min(p['finish_at'],end):
                overlaps.append(dict(ref=refs('history.transitions',index),kind='Resource'if isinstance(other['work'],dict)else other['work'],owners=other['participants'],start=other['start'],end=end))
        results.append(dict(owner=owner,commitment=p['commitment'],source=p['source'],start=p['start'],extract_at=p['extract_at'],finish_at=p['finish_at'],choice_ref=f'history.boundaries[{bi}].choices[{di}]',valuation_ref=refs('valuations',ai),raw_receipt=d['input']['raw_retained'],selection=t['valuations'][ai]['selection'],start_ref=refs('history.transitions',si),extraction_ref=refs('material.operations',en),extraction_outcome=extracted['outcome'],completion_ref=refs('material.operations',cn),completion_outcome=completed['outcome'],release_ref=refs('history.transitions',releases[p['commitment']]),first_later_opportunity=min(opportunities,key=lambda x:x['tick'])if opportunities else None,first_later_material_debit=first,isolated_new_unit_to_first_debit=isolated,unused_through_end=completed['outcome']=='Prepared'and first is None,unused_with_later_ordinary_meal_option=completed['outcome']=='Prepared'and first is None and any(o['kind']=='ordinary'and o['eligible_consume']for o in opportunities),next_opportunity_after_debit=min(later,key=lambda x:x['tick'])if later else None,report_crossings=crossings,blocked_proposals=blocked,unrelated_concurrent_work=overlaps))
    return results


def coverage(worlds,rows,comparisons):
    flat=[(n,c)for n,cs in enumerate(rows)for c in cs]
    success=[(n,c)for n,c in flat if c['completion_outcome']=='Prepared']
    failure=[(n,c)for n,c in flat if c['extraction_outcome']=='Insufficient']
    ref=lambda entry:dict(ordinal=entry[0],commitment=entry[1]['commitment'],choice_ref=entry[1]['choice_ref'])if entry else None
    first=lambda pred:next(((n,c)for n,c in success if pred(c)),None)
    return dict(total_attempts=len(flat),successful=len(success),failed=len(failure),successful_with_later_debit=sum(c['first_later_material_debit']is not None for _,c in success),isolated_new_unit_to_debit=sum(c['isolated_new_unit_to_first_debit']for _,c in success),successful_unused=sum(c['unused_through_end']for _,c in success),unused_with_genuine_meal_option=sum(c['unused_with_later_ordinary_meal_option']for _,c in success),witnesses=dict(first_success=ref(success[0]if success else None),first_failed_full_cost=ref(failure[0]if failure else None),later_meal=ref(first(lambda c:c['first_later_material_debit']is not None and c['first_later_material_debit']['kind']=='meal')),later_share=ref(first(lambda c:c['first_later_material_debit']is not None and c['first_later_material_debit']['kind']=='resource_transfer')),unused_despite_meal_option=ref(first(lambda c:c['unused_with_later_ordinary_meal_option']))),primary_pairs_with_higher_actor_area=[p['ordinals']for p in comparisons['pairs']if any(v>0 for v in p['hunger_area_delta'])],primary_pairs_with_lower_actor_area=[p['ordinals']for p in comparisons['pairs']if any(v<0 for v in p['hunger_area_delta'])],worlds_with_unselected_eligible_preparation=[n for n,m in enumerate(worlds)if m['preparation_not_selected']>0],worlds_with_executed_inquiry=[n for n,m in enumerate(worlds)if m['queries']>0])


def inspect(path,metrics=None,findings=None):
    rows=[];world_metrics=[]
    with(gzip.open(path,'rb')if str(path).endswith('.gz')else open(path,'rb'))as source:
        for line in source:
            t=json.loads(line);rows.append(chains(t))
            if metrics is None:world_metrics.append(review025.verify(t))
    if metrics is not None:world_metrics=metrics
    if findings is None:findings=review025_findings.inspect(path)
    assert len(rows)==len(world_metrics)==50
    measured=coverage(world_metrics,rows,findings)
    primary=[c for n,cs in enumerate(rows[:48])for c in cs]
    measured['counts_include_reporting_placebos']=True
    measured['primary_48_only']=dict(attempts=len(primary),successful=sum(c['completion_outcome']=='Prepared'for c in primary),failed=sum(c['extraction_outcome']=='Insufficient'for c in primary),later_debits=sum(c['first_later_material_debit']is not None for c in primary),unused_despite_meal_option=sum(c['unused_with_later_ordinary_meal_option']for c in primary))
    return dict(scope='All50original records, including2reporting placebos; primary48counts separate. Observer measurements, not utility; missing witnesses remain missing; fungible-food attribution explicitly bounded',coverage=measured,process_chains=[dict(ordinal=n,chains=c)for n,c in enumerate(rows)])


def main():
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('--metrics');p.add_argument('--findings');p.add_argument('--output');args=p.parse_args()
    metrics=json.loads(Path(args.metrics).read_bytes())if args.metrics else None
    findings=json.loads(Path(args.findings).read_bytes())if args.findings else None
    result=inspect(args.archive,metrics,findings)
    if args.output:
        with Path(args.output).open('x',encoding='utf-8')as out:json.dump(result,out,indent=2);out.write('\n')
    print(json.dumps(result['coverage'],sort_keys=True))


if __name__=='__main__':main()
