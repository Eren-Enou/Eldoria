"""Real, bounded observer fixtures for the actual browser filter helper test."""
import gzip
import importlib.util
import json
from pathlib import Path

home = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('causal', home / 'causal.py')
c = importlib.util.module_from_spec(spec)
spec.loader.exec_module(c)
result = {}
for name, wanted in [('seed-42.jsonl.gz', {85, 95, 107}), ('held-out-seed-42.jsonl.gz', {10})]:
    with gzip.open(home.parents[1] / 'experiments/019' / name, 'rt') as f:
        for n, line in enumerate(f, 1):
            if n in wanted:
                t = json.loads(line)
                result[str(n)] = c.build(t, name + ':' + str(n))
                if n == 10:
                    result['trial'] = t
                if n == 107:
                    vspec = importlib.util.spec_from_file_location('viewer', home / 'explorer.py')
                    viewer = importlib.util.module_from_spec(vspec)
                    vspec.loader.exec_module(viewer)
                    result['timeline107'] = [dict(ref=m['ref'], order=m['order']) for m in viewer.timeline(t)]
print(json.dumps(result, separators=(',', ':')))
