"""Scratch-only corruption controls; original evidence is never written."""
import argparse
import copy
import json
from pathlib import Path
import review022 as reader


def controls(t):
    def boundary(x): return next(b for b in x['boundaries'] if b['active'])
    def first_work(x, work): return next(r['commitment']['id'] for r in x['transitions'] if not r['completed'] and reader.kind(r['commitment']) == work)
    def change_commit(x, cid, field, value):
        for r in x['transitions']:
            if r['commitment']['id'] == cid: r['commitment'][field] = copy.deepcopy(value)
    def double(x):
        starts = [r for r in x['transitions'] if not r['completed'] and reader.kind(r['commitment']) == 'Resource']
        a, b = next((a,b) for n,a in enumerate(starts) for b in starts[n+1:] if a['tick'] == b['tick'])
        change_commit(x, b['commitment']['id'], 'participants', a['commitment']['participants'])
    def deadline(x):
        cid = first_work(x, 'Claim'); c = next(r['commitment'] for r in x['transitions'] if r['commitment']['id'] == cid)
        change_commit(x, cid, 'deadline', c['deadline']+1)
    def wrong_source(x):
        cid = first_work(x, 'Resource'); change_commit(x, cid, 'source', len(x['boundaries'])-1)
    def cross_scene(x):
        starts = [r['commitment'] for r in x['transitions'] if not r['completed'] and reader.kind(r['commitment']) == 'Resource']
        change_commit(x, starts[0]['id'], 'work', starts[1]['work'])
    def remove_release(x): x['transitions'].pop(next(n for n,r in enumerate(x['transitions']) if r['completed']))
    def release_id(x): next(r for r in x['transitions'] if r['completed'])['commitment']['id'] += 10000
    def false_consent(x): next(r for b in x['boundaries'] for r in b['responses'] if isinstance(r['selected'], dict))['selected'] = 'Wait'
    def observer_leak(x): x['boundaries'][0]['choices'][0]['input']['legacy']['objective_future'] = dict(stock=100, partner_food=100)
    def ordering(x):
        a = next(n for n in range(len(x['admissions'])-1) if x['admissions'][n]['tick'] == x['admissions'][n+1]['tick'])
        x['admissions'][a], x['admissions'][a+1] = x['admissions'][a+1], x['admissions'][a]
    def completion(x):
        cid = first_work(x, 'Resource'); r = next(r for r in x['transitions'] if r['completed'] and r['commitment']['id'] == cid); r['tick'] -= 1
    def missing_event(x): reader.core(x)['events'].pop(x['bootstrap_events'])
    def actor(x): reader.core(x)['events'][x['bootstrap_events']]['decision']['actor'] = 99
    def transfer(x): reader.core(x)['events'][x['bootstrap_events']]['transferred'] += 1
    def duplicate_grant(x): x['grants'].append(copy.deepcopy(x['grants'][0]))
    def memory_owner(x): next(m for m in x['memories'] if m['after'])['after']['owner'] = 99
    def missing_choice(x): x['boundaries'][0]['choices'].pop()
    return dict(double_booked=double, duplicate_transfer=transfer, wrong_actor=actor,
                wrong_commitment=release_id, missing_release=remove_release,
                incorrect_deadline=deadline,
                skipped_tick=lambda x:x['boundaries'].pop(1),
                double_hunger=lambda x:x['needs'][0]['after'].__setitem__(0, x['needs'][0]['after'][0]+5),
                false_consent=false_consent,
                illegal_participant=lambda x:boundary(x)['active'][0]['participants'].append(99),
                unrecorded_grant=duplicate_grant, observer_leak=observer_leak,
                cross_scene=cross_scene, simultaneous_order=ordering,
                wrong_completion=completion, missing_event=missing_event,
                inconsistent_ownership=lambda x:x['boundaries'][1]['agents'][0].__setitem__('food', 999),
                impossible_final=lambda x:x['final_agents'][0].__setitem__('food', 999),
                foreign_memory=memory_owner, wrong_source=wrong_source,
                missing_local_choice=missing_choice,
                wrong_response_ref=lambda x:next(p for b in x['boundaries'] for p in b['pending'] if p['response'])['response'].__setitem__('boundary',99999),
                fourth_actor_clock=lambda x:x['needs'][0]['after'].__setitem__(3, 99))


def run(sample):
    reader.verify(sample)
    rejected = []
    for label, mutate in controls(sample).items():
        trial = copy.deepcopy(sample); mutate(trial)
        try: reader.verify(trial)
        except (AssertionError, KeyError, IndexError, StopIteration, TypeError): rejected.append(label)
        else: raise AssertionError('accepted corruption: '+label)
    return rejected


if __name__ == '__main__':
    p = argparse.ArgumentParser(); p.add_argument('archive'); p.add_argument('--output'); args = p.parse_args()
    sample = next(t for t in reader.read(args.archive) if t['config']['case'] == 'paired' and t['config']['capacity'] == 2)
    result = dict(rejected=run(sample))
    if args.output: Path(args.output).write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps(result))
