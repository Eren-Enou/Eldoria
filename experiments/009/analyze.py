"""Derive comparisons from009 deterministic summary; timings stay in separate CSV."""
from pathlib import Path
import json
from collections import defaultdict

root=Path(__file__).resolve().parent
summary=json.loads((root/'summary.json').read_text())
comparisons=[]
for held,key in [(False,'controlled_seed42'),(True,'held_out_seed42')]:
    groups=defaultdict(dict)
    for row in summary[key]:
        scenario=row[0]
        config=row[1] if held else None
        method=row[2] if held else row[1]
        point=row[3] if held else row[2]
        groups[(scenario,json.dumps(config,sort_keys=True))][method]=point
    for (scenario,config),methods in groups.items():
        record={'held_out':held,'scenario':scenario,'config':json.loads(config),'methods':methods}
        record['salient_action_differs_from_fifo']=methods['Salient']['action']!=methods['Fifo']['action']
        record['salient_basis_differs_from_quality']=any(methods['Salient'][k]!=methods['Quality'][k] for k in ['basis_support','target_origins','positive','negative'])
        # All controlled original refusals actually occurred under scarce donor food1/hunger90.
        # This observer-only diagnostic never enters policy or the primary relevance measure.
        record['observer_belief_direction']={m:('scarcity' if p['belief'] and p['belief']>0 else 'opposing' if p['belief'] and p['belief']<0 else 'undecided') for m,p in methods.items()}
        comparisons.append(record)
assert sum(summary['outcomes'].values())==23040
assert set(summary['outcomes'].values())=={128}
(root/'comparisons.json').write_text(json.dumps(comparisons,indent=2)+'\n',encoding='utf-8')
print('Comparison groups:',len(comparisons))
for held in [False,True]:
    rows=[r for r in comparisons if r['held_out']==held]
    print('held',held,'action differences',sum(r['salient_action_differs_from_fifo'] for r in rows),'Salient/Quality basis differences',sum(r['salient_basis_differs_from_quality'] for r in rows))
