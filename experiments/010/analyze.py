"""Whole-chain comparisons; no policy or fixture tuning."""
from pathlib import Path
from collections import defaultdict, Counter
import gzip,json
root=Path(__file__).resolve().parent
rows=json.loads(gzip.decompress((root/'summary.json.gz').read_bytes()))
groups=defaultdict(dict)
for r in rows:
    key=(r['seed'],r['held'],r['case'],r['mode'],json.dumps(r['config'],sort_keys=True))
    groups[key][r['retention']]=r
counts=Counter()
comparisons=[]
for (seed,held,case,mode,config),modes in groups.items():
    assert set(modes)=={'Fifo','Quality','Salient'}
    distinct=lambda k:len({json.dumps(r[k],sort_keys=True) for r in modes.values()})>1
    inquiry,action,persistent=distinct('inquiry'),distinct('actions'),distinct('persistent_food')
    before={r['diagnostic_probes'][0]['selected'] for r in modes.values()}
    after={r['diagnostic_probes'][1]['selected'] for r in modes.values()}
    # These two schedules include an independent later delivery; retain their
    # observed differences, but exclude them from a conservative mediation count.
    mediated=case not in ('hidden','late_attribution') and inquiry and action and persistent and len(before)==1 and len(after)>1 and all(r['diagnostic_probes'][1]['selected']==r['actions'][0] for r in modes.values())
    counts[f'{held}:{mode}:groups']+=1
    for key,value in [('inquiry',inquiry),('action',action),('persistent_food',persistent),('full_chain',mediated),('inquiry_only',inquiry and not action)]:
        counts[f'{held}:{mode}:{key}']+=value
    if seed==42:
        compact={}
        for method,r in modes.items():
            compact[method]={k:r[k] for k in ['basis','inquiry','actions','persistent_food','resource_partner','inquiry_id','response','inquiry_time','realized_value']}
            compact[method]['diagnostic_actions']=[p['selected'] for p in r['diagnostic_probes']]
        comparisons.append({'held':held,'case':case,'mode':mode,'config':json.loads(config),'methods':compact,
            'inquiry_differs':inquiry,'actions_differ':action,'persistent_food_differs':persistent,
            'complete_chain_candidate':mediated})
(root/'comparisons.json').write_text(json.dumps(comparisons,indent=2)+'\n')
variants=defaultdict(set)
for r in rows:
    key=(r['held'],r['case'],r['mode'],r['retention'],json.dumps(r['config'],sort_keys=True))
    variants[key].add(json.dumps({k:r[k] for k in ['basis','inquiry','actions','persistent_food']},sort_keys=True))
summary={'trials':len(rows),'replayed_trials':len(rows),'seeds':sorted({r['seed'] for r in rows}),
    'groups':dict(sorted(counts.items())),'outcome_groups':len(variants),'seed_dependent_outcome_groups':sum(len(v)>1 for v in variants.values())}
(root/'analysis.json').write_text(json.dumps(summary,indent=2)+'\n')
lines=['Experiment010: fixed public schedules, one inquiry before executed resource scene.','Offer/Accept transfers food; Leave retains food; neither is a global reward ranking.']
for c in comparisons:
    lines.append(f"\nheld={c['held']} {c['case']} {c['mode']} config={c['config']}")
    for mode,r in c['methods'].items():
        lines.append(f"  {mode}: basis={r['basis']} chooses={r['inquiry']} actions={r['actions']} partner={r['resource_partner']} persistent_food={r['persistent_food'][:3]}")
        original=groups[(42,c['held'],c['case'],c['mode'],json.dumps(c['config'],sort_keys=True))][mode]
        scores=[(v['action'],v['expected_gain'],v['benefit'],v['cost'],v['score']) for v in original['inquiry_candidates']]
        lines.append(f"    query/inquiry={r['inquiry_id']} response={r['response']} candidate(action,expected,benefit,cost,score)={scores}")
(root/'report.txt').write_text('\n'.join(lines)+'\n')
print(json.dumps(summary,indent=2))
