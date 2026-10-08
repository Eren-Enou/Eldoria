"""Freeze only010 outputs; verify001–009 without regenerating anything."""
from pathlib import Path
import gzip, hashlib, json, shutil
root=Path(__file__).resolve().parent
repo=root.parents[1]
frozen=json.loads((root/'frozen-001-009-sha256.json').read_text())
assert all(hashlib.sha256((repo/p).read_bytes()).hexdigest()==digest for p,digest in frozen.items())
manifest={}
for name in ['seed-42','held-out-seed-42','summary']:
    data=(repo/'target/experiment010'/f'{name}.json').read_bytes()
    compressed=gzip.compress(data,mtime=0)
    (root/f'{name}.json.gz').write_bytes(compressed)
    manifest[name]={'json_bytes':len(data),'gzip_bytes':len(compressed),
        'json_sha256':hashlib.sha256(data).hexdigest(),'gzip_sha256':hashlib.sha256(compressed).hexdigest()}
for name in ['benchmark.csv','performance.csv']:
    shutil.copyfile(repo/'target/experiment010'/name,root/name)
(root/'archive-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(f'{len(frozen)} frozen001–009 files unchanged;010 archived')
