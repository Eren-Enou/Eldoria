"""Archive authorized017 outputs only; older evidence is never a writer target."""
import gzip,hashlib,json,shutil,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];out=ROOT/'experiments/017';source=Path(sys.argv[1])
for name in ('seed-42','held-out-seed-42','summary'):
    (out/(name+'.json.gz')).write_bytes(gzip.compress((source/(name+'.json')).read_bytes(),mtime=0))
for name in ('analysis.json','performance.csv'):
    shutil.copyfile(source/name,out/name)
manifest={}
for p in sorted(out.glob('*.json.gz')):
    data=p.read_bytes();raw=gzip.decompress(data)
    manifest[p.name[:-8]]=dict(gzip_bytes=len(data),gzip_sha256=hashlib.sha256(data).hexdigest(),json_bytes=len(raw),json_sha256=hashlib.sha256(raw).hexdigest())
(out/'archive-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
