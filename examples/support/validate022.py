"""Read-only original archive/source verification; no regeneration or prepared cache."""
import gzip
import hashlib
import json
import sys
from pathlib import Path
import review022
import corrupt022

root=Path(__file__).resolve().parents[2]
directory=Path(sys.argv[1])
manifest=json.loads((directory/'archive-manifest.json').read_text())
for name,expected in manifest.items():
    compressed=(directory/name).read_bytes()
    assert len(compressed)==expected['gzip_bytes'] and hashlib.sha256(compressed).hexdigest()==expected['gzip_sha256'],name
    digest=hashlib.sha256();size=0
    with gzip.open(directory/name,'rb') as stream:
        while chunk:=stream.read(1024*1024):digest.update(chunk);size+=len(chunk)
    assert size==expected['json_bytes'] and digest.hexdigest()==expected['json_sha256'],name
freeze=json.loads((directory/'source-freeze.json').read_text())
for name,expected in freeze.items():
    assert hashlib.sha256((root/name).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==expected, name
matrix=json.loads((directory/'registered-matrix.json').read_text())
count=0;sample=None
for n,trial in enumerate(review022.read(directory/'trials.jsonl.gz')):
    assert trial['config']==matrix[n]
    review022.verify(trial);count+=1
    if n==1: sample=trial
assert count==len(matrix)==manifest['trials.jsonl.gz']['records']==100
rejected=corrupt022.run(sample)
assert len(rejected)==23
print(json.dumps(dict(primitive_worlds=count,source_locks=len(freeze),artifacts=len(manifest),scratch_corruptions_rejected=len(rejected))))
