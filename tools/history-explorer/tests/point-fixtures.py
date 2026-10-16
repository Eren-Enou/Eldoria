"""Read-only small real-point fixtures for the JS ownership boundary test."""
import gzip,json
from pathlib import Path
root=Path(__file__).resolve().parents[3]
result={}
with gzip.open(root/'experiments/019/seed-42.jsonl.gz','rb') as f:
 for ordinal,line in enumerate(f,1):
  if ordinal not in (103,109):continue
  t=json.loads(line)
  result['useful' if ordinal==103 else 'private']=dict(points=[t['points'][0],t['points'][-1]],source=dict(archive='seed-42.jsonl.gz',record=ordinal))
  if ordinal==109:break
print(json.dumps(result,separators=(',',':')))
