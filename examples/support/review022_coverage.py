"""Post-evaluation requirement coverage from unchanged original022 primitives.

This complements the frozen full reader; it creates no new simulation outcomes.
In particular, a masked local question is distinct from a rejected admission.
"""
import collections
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import review022 as original

ROOT=Path(__file__).resolve().parents[2]
DIRECTORY=ROOT/'experiments/022'


def inspect():
    expected=json.loads((DIRECTORY/'archive-manifest.json').read_text())['trials.jsonl.gz']['gzip_sha256']
    assert hashlib.sha256((DIRECTORY/'trials.jsonl.gz').read_bytes()).hexdigest()==expected
    counts=collections.Counter();examples={};previous=None;worlds=0;equivalent=collections.defaultdict(list)
    def witness(name,value):
        counts[name]+=1;examples.setdefault(name,value)
    for ordinal,t in enumerate(original.read(DIRECTORY/'trials.jsonl.gz')):
        worlds+=1;starts={};ends={};start_refs={};end_refs={}
        for n,r in enumerate(t['transitions']):
            cid=r['commitment']['id']
            if r['completed']:ends[cid]=r['tick'];end_refs[cid]=f'transitions[{n}]'
            else:starts[cid]=r['commitment'];start_refs[cid]=f'transitions[{n}]'
        def interval(c):return dict(commitment=c['id'],participants=c['participants'],start=c['start'],end=ends[c['id']],start_ref=start_refs[c['id']],release_ref=end_refs[c['id']])
        resources=[c for c in starts.values() if original.kind(c)=='Resource']
        base=original.core(t);scenes=base['scenes']
        bootstrap_ids=sorted({e['scene'] for e in base['events'][:t['bootstrap_events']]})
        prior_pairs=[scenes[sid]['participants'] for sid in bootstrap_ids]
        real_partition={owner:group for group,pair in enumerate(prior_pairs) for owner in pair} if len(prior_pairs)==2 and len(set(prior_pairs[0]+prior_pairs[1]))==4 else {}
        for n,event in enumerate(base['events'][t['bootstrap_events']:],t['bootstrap_events']):
            if not event['transferred']:continue
            if event['participants'][0]//2 != event['participants'][1]//2:counts['transfers_across_fixed_id_partition']+=event['transferred']
            if real_partition and real_partition[event['participants'][0]]!=real_partition[event['participants'][1]]:
                witness('transfer_across_actual_prior_pairs',dict(ordinal=ordinal,event_ref=f'final_state.base.base.base.base.base.events[{n}]',tick=event['tick'],participants=event['participants'],before=event['balances_before'],after=event['balances_after']))
        clone=dict(t);clone['config']=dict(t['config']);clone['config'].pop('case')
        fingerprint=hashlib.sha256(json.dumps(clone,sort_keys=True,separators=(',',':')).encode()).hexdigest()
        equivalent[fingerprint].append(dict(ordinal=ordinal,case=t['config']['case'],capacity=t['config']['capacity']))
        # Narrow material usefulness: the received unit is the actor's only food,
        # and the next owned-food change is its actual meal. This is not welfare.
        food_ops=collections.defaultdict(list)
        for n,event in enumerate(base['events'][t['bootstrap_events']:],t['bootstrap_events']):
            for at,owner in enumerate(event['participants']):
                if event['balances_before'][at]!=event['balances_after'][at]:food_ops[owner].append((event['tick'],0,event['id'],'event',n,event))
        for n,grant in enumerate(t['grants']):
            if grant['food_before']!=grant['food_after']:food_ops[grant['owner']].append((grant['tick'],1,n,'grant',n,grant))
        for n,meal in enumerate(base['cognition']['consumption']):food_ops[meal['actor']].append((meal['tick'],2,n,'meal',n,meal))
        for owner,ops in food_ops.items():
            ops.sort(key=lambda op:op[:3])
            for current,following in zip(ops,ops[1:]):
                event=current[-1]
                if current[3]!='event' or following[3]!='meal' or event['decision']['actor']!=owner or not event['transferred']:continue
                at=event['participants'].index(owner);meal=following[-1]
                if event['balances_before'][at]==0 and event['balances_after'][at]==meal['food_before']==1:
                    witness('sole_received_unit_enables_actual_meal',dict(ordinal=ordinal,owner=owner,transfer_ref=f'final_state.base.base.base.base.base.events[{current[4]}]',meal_ref=f'final_state.base.base.base.base.base.cognition.consumption[{following[4]}]',received_at=current[0],consumed_at=following[0],interpretation='material availability only; no global utility ranking'))
        for n,a in enumerate(resources):
            for b in resources[n+1:]:
                if max(a['start'],b['start'])>=min(ends[a['id']],ends[b['id']]):continue
                assert not set(a['participants']).intersection(b['participants'])
                name='same_tick_resource_completions' if ends[a['id']]==ends[b['id']] else 'different_finish_resource_overlap'
                data=dict(ordinal=ordinal,a=interval(a),b=interval(b));witness(name,data)
                oa=scenes[a['work']['Resource']['scene']]['outcome'];ob=scenes[b['work']['Resource']['scene']]['outcome']
                if (oa=='Agreement') != (ob=='Agreement'):witness('overlapping_agreement_and_failure',dict(data,outcomes=[oa,ob]))
        for resource in resources:
            for other in starts.values():
                k=original.kind(other)
                if k not in ('Meal','Claim') or max(resource['start'],other['start'])>=min(ends[resource['id']],ends[other['id']]):continue
                assert not set(resource['participants']).intersection(other['participants'])
                witness('resource_overlaps_'+k,dict(ordinal=ordinal,resource=interval(resource),other=interval(other)))
        for tick in range(t['start'],t['end']):
            active=[c for c in starts.values() if c['start']<=tick<ends[c['id']]]
            kinds={original.kind(c) for c in active}
            if {'Resource','Claim','Meal'}<=kinds:
                assert len({a for c in active for a in c['participants']})==4
                witness('resource_claim_meal_same_clock',dict(ordinal=ordinal,tick=tick,activities=[interval(c) for c in active]))
        for bn,b in enumerate(t['boundaries']):
            for rn,response in enumerate(b['responses']):
                if len(response['input']['incoming'])>1:
                    witness('competing_recipient',dict(ordinal=ordinal,tick=b['tick'],response_ref=f'boundaries[{bn}].responses[{rn}]',selected=response['selected'],incoming=len(response['input']['incoming'])))
            for dn,d in enumerate(b['choices']):
                if d['input']['legacy']['query']['decision']['selected']=='Pause' or d['input']['inquiry_lane_free']:continue
                assert any(c['intent']!='Wait' and c['intent']['Invite']['kind']=='Inquiry' and not c['eligible'] and c['score']==-1 for c in d['contact_candidates'])
                counts['masked_local_questions']+=1
                for k in sorted({original.kind(c) for c in b['active']}):
                    witness('masked_query_during_'+k,dict(ordinal=ordinal,tick=b['tick'],choice_ref=f'boundaries[{bn}].choices[{dn}]',question=d['input']['legacy']['query']['decision']['selected'],blockers=[interval(c) for c in b['active'] if original.kind(c)==k]))
        for an,a in enumerate(t['admissions']):
            if a['reason']=='AtomicBlocked':
                tick=a['tick'];active=[c for c in starts.values() if c['start']<tick<ends[c['id']] or c['start']==tick and original.kind(c)=='Reply']
                earlier=[other['commitment'] for other in t['admissions'][:an] if other['tick']==tick and other['reason']=='Execute']
                active.extend(starts[cid] for cid in earlier if starts[cid] not in active)
                label='blocked_admission_during_'+','.join(sorted({original.kind(c) for c in active})) if active else 'blocked_admission_due_protocol'
                witness(label,dict(ordinal=ordinal,admission_ref=f'admissions[{an}]',tick=tick,invitation=a['invitation'],physical_blockers=[interval(c) for c in active],later_same_tick_admissions=[f'admissions[{n}]' for n,other in enumerate(t['admissions']) if n>an and other['tick']==tick]))
            if a['reason']=='Unavailable':
                recipient=a['invitation']['recipient'];tick=a['tick']
                blockers=[c for c in starts.values() if recipient in c['participants'] and c['start']<tick<ends[c['id']]]
                if blockers:witness('busy_recipient_rejected',dict(ordinal=ordinal,admission_ref=f'admissions[{an}]',tick=tick,blockers=[interval(c) for c in blockers]))
        if t['config']['horizon']==192:
            assert len(t['needs'])==48 and len(t['boundaries'])==192
            witness('continuous_192_tick_world',dict(ordinal=ordinal,start=t['start'],end=t['end'],passive_observation_windows=24,closed_social_window_reopening=False))
        if ordinal%2:
            food_a=([a['food'] for a in previous['final_agents']],previous['final_stock'],len(original.core(previous)['cognition']['consumption']))
            food_b=([a['food'] for a in t['final_agents']],t['final_stock'],len(original.core(t)['cognition']['consumption']))
            hunger_a=[a['hunger'] for a in previous['final_agents']];hunger_b=[a['hunger'] for a in t['final_agents']]
            counts['terminal_food_ledger_unequal_pairs']+=food_a!=food_b
            counts['terminal_hunger_unequal_pairs']+=hunger_a!=hunger_b
            if food_a==food_b and hunger_a!=hunger_b:
                witness('food_ledger_reconverges_hunger_does_not',dict(serial=ordinal-1,concurrent=ordinal,serial_hunger=hunger_a,concurrent_hunger=hunger_b,interpretation='resource ledger equality is not physiological or full-state equality'))
            a=dict(previous);b=dict(t);a['config']=dict(a['config'],capacity=0);b['config']=dict(b['config'],capacity=0)
            if a==b:witness('complete_null_pair',dict(serial=ordinal-1,concurrent=ordinal))
        else:previous=t
    assert worlds==100
    for required in ('same_tick_resource_completions','different_finish_resource_overlap','overlapping_agreement_and_failure','resource_overlaps_Claim','resource_overlaps_Meal','resource_claim_meal_same_clock','competing_recipient','masked_query_during_Claim','masked_query_during_Resource','blocked_admission_due_protocol','busy_recipient_rejected','continuous_192_tick_world','complete_null_pair'):
        assert counts[required]>0,required
    assert counts['masked_query_during_Claim']==1 and counts['complete_null_pair']==9
    assert counts['transfer_across_actual_prior_pairs']==100 and counts['transfers_across_fixed_id_partition']==104
    assert counts['terminal_food_ledger_unequal_pairs']==15 and counts['terminal_hunger_unequal_pairs']==20 and counts['food_ledger_reconverges_hunger_does_not']==5
    duplicates=[rows for rows in equivalent.values() if len(rows)>1]
    assert len(equivalent)==84
    return dict(original_worlds=worlds,archive_sha256=expected,classification='post-evaluation primitive coverage; no new outcomes',counts=dict(counts),examples=examples,
                equivalent_case_labels=duplicates,unique_histories_after_ignoring_case_label=len(equivalent),
                interpretation_notes=dict(original_cross_bootstrap_pair_transfers_key='The frozen reports key uses the fixed ID partition0/1 versus2/3, including worlds without two bootstrap pairs.104is that diagnostic;100transfers actually cross two executed prior pairs. No original archive was changed.',
                                          repetitions='The100registered worlds include case-label equivalents; frequencies describe this fixed grid, not independent statistical samples.'))


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--write-new',action='store_true');parser.add_argument('--verify',action='store_true');args=parser.parse_args()
    result=inspect();path=DIRECTORY/'coverage.json'
    if args.write_new:
        with path.open('x',encoding='utf-8',newline='\n') as out:json.dump(result,out,indent=2);out.write('\n')
    if args.verify:assert json.loads(path.read_text(encoding='utf-8'))==result
    print(json.dumps(result['counts']))
