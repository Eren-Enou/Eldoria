"""Bounded actual021 graph slice for the JavaScript ordering helper test."""
import gzip
import importlib.util
import json
from pathlib import Path

home=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('knowledge_js_fixture',home/'explorer.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
with gzip.open(v.ROOT/'experiments/021/trials.jsonl.gz','rt',encoding='utf-8') as f:
    t=json.loads(next(line for n,line in enumerate(f) if n==82))
graph=v.knowledge.causal(t,'original021-82',lambda t,i,field:v.finite.causal(t,i,v.causal.build,v.causal.resolve,field),v.causal.resolve)
chain=next(c for c in graph['chains'] if c['id']=='periods[1].steps[3].activity')
graph=dict(nodes={n:graph['nodes'][n] for n in chain['nodes']},chains=[chain])
timeline=v.knowledge.timeline(t,lambda t:v.finite.timeline(t,v.timeline))
print(json.dumps(dict(causal=graph,chain=chain,timeline=[{k:m[k] for k in ('ref','tick','order')} for m in timeline])))
