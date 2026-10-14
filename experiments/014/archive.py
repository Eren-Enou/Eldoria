"""Archive only authorized014 evaluator outputs; never access older experiment writers."""
import gzip,hashlib,json,shutil,sys
from pathlib import Path
root=Path(__file__).resolve().parents[2];source=Path(sys.argv[1]);dest=root/'experiments/014'
for name in ['seed-42','held-out-seed-42','summary']:
    (dest/(name+'.json.gz')).write_bytes(gzip.compress((source/(name+'.json')).read_bytes(),mtime=0))
shutil.copyfile(source/'performance.csv',dest/'performance.csv')
shutil.copyfile(root/'docs/foundation-001-013-sha256.json',dest/'frozen-001-013-sha256.json')
manifest={}
for p in sorted(dest.glob('*.json.gz')):
    data=p.read_bytes();raw=gzip.decompress(data)
    manifest[p.name[:-8]]=dict(gzip_bytes=len(data),gzip_sha256=hashlib.sha256(data).hexdigest(),json_bytes=len(raw),json_sha256=hashlib.sha256(raw).hexdigest())
(dest/'archive-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
