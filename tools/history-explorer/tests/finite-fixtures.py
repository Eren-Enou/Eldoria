"""Original020 witnesses for actual JavaScript helper tests; read-only."""
import gzip
import importlib.util
import json
from pathlib import Path
home=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('finite_fixture',home/'explorer.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
result={}
with gzip.open(v.ROOT/'experiments/020/trials.jsonl.gz','rt',encoding='utf-8') as f:
    for ordinal,line in enumerate(f):
        if ordinal not in (29,33,43):continue
        t=json.loads(line)
        result[str(ordinal)]=dict(trial=t,causal=v.finite.causal(t,str(ordinal),v.causal.build,v.causal.resolve),timeline=v.finite.timeline(t,v.timeline))
        if ordinal==43:break
print(json.dumps(result,separators=(',',':')))
