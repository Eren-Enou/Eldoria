"""Archive only authorized016 outputs; never touch protected evidence."""
import gzip,hashlib,json,shutil,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];output=ROOT/'experiments/016';source=Path(sys.argv[1])
for name in ['seed-42','held-out-seed-42','summary']:
    (output/(name+'.json.gz')).write_bytes(gzip.compress((source/(name+'.json')).read_bytes(),mtime=0))
shutil.copyfile(source/'performance.csv',output/'performance.csv')
shutil.copyfile(ROOT/'docs/foundation-001-015-sha256.json',output/'frozen-001-015-sha256.json')
manifest={}
for p in sorted(output.glob('*.json.gz')):
    b=p.read_bytes();raw=gzip.decompress(b)
    manifest[p.name[:-8]]=dict(gzip_bytes=len(b),gzip_sha256=hashlib.sha256(b).hexdigest(),json_bytes=len(raw),json_sha256=hashlib.sha256(raw).hexdigest())
(output/'archive-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
