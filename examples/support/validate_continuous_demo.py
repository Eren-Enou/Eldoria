"""Independent read-only check for an actual bounded prototype JSONL or gzip."""
import argparse,gzip,json
from pathlib import Path
import review023
p=argparse.ArgumentParser();p.add_argument('source');a=p.parse_args();path=Path(a.source)
rows=[]
with (gzip.open(path,'rt',encoding='utf-8') if path.suffix=='.gz' else path.open(encoding='utf-8')) as f:
    for n,line in enumerate(f):
        trial=json.loads(line);metrics=review023.verify(trial)
        rows.append(dict(ordinal=n,case=trial['config']['case'],mode=trial['config']['mode'],start=trial['history']['start'],nominal_stop=trial['history']['start']+trial['config']['horizon'],final=trial['history']['end'],strict_crossings=len(metrics['strict_crossings']),queries=metrics['queries'],social=metrics['social'],final_food=metrics['final_food'],final_hunger=metrics['final_hunger']))
print(json.dumps(dict(independent_prototype_reconstruction='PASS',rows=rows),indent=2))
