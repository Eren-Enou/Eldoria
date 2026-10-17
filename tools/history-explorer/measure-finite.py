"""Engineering-only selected020 load costs; never writes original evidence."""
import importlib.util
import json
from pathlib import Path
import tempfile
import time

HOME=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('finite_measure',HOME/'explorer.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)

with tempfile.TemporaryDirectory(prefix='woi-finite-cost-') as temp:
    start=time.perf_counter()
    indexed=v.Repository(cache=Path(temp)/'index',experiments=('020',)).prepare()
    result=dict(note='Single engineering run; other scientific validation may run concurrently. No browser heap or statistical inference.',catalog_seconds=time.perf_counter()-start,trials=len(indexed.entries),loads=[])
    largest=max(indexed.entries.values(),key=lambda e:e['raw_bytes'])['record']
    for ordinal in dict.fromkeys((33,29,largest)):
        entry=next(e for e in indexed.entries.values() if e['record']==ordinal)
        cold=v.Repository(cache=Path(temp)/str(ordinal),experiments=('020',))
        cold.entries={entry['id']:entry}
        start=time.perf_counter();data=cold.get(entry['id']);first=time.perf_counter()-start
        start=time.perf_counter();again=cold.get(entry['id']);cached=time.perf_counter()-start
        assert data==again
        result['loads'].append(dict(original_ordinal=ordinal,source_bytes=entry['raw_bytes'],export_bytes=len(v.encoded(data)),cold_seconds=first,cached_seconds=cached,moments=len(data['timeline']),nodes=len(data['causal']['nodes'])))
    print(json.dumps(result,indent=2))
