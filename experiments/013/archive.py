"""013-only deterministic archival writer. Never writes historical experiments."""
import argparse,gzip,hashlib,io,json
from pathlib import Path
parser=argparse.ArgumentParser()
parser.add_argument('--source',type=Path,default=Path('target/experiment013-final'))
args=parser.parse_args();root=Path(__file__).resolve().parent
manifest={}
for name in ['seed-42','held-out-seed-42','summary','seed-diagnostic']:
 raw=(args.source/(name+'.json')).read_bytes();buf=io.BytesIO()
 with gzip.GzipFile(fileobj=buf,mode='wb',filename='',mtime=0) as file:file.write(raw)
 (root/(name+'.json.gz')).write_bytes(buf.getvalue())
for name in ['seed-42','held-out-seed-42','summary','development-preview','seed-diagnostic']:
 compressed=(root/(name+'.json.gz')).read_bytes();raw=gzip.decompress(compressed)
 manifest[name]=dict(gzip_bytes=len(compressed),gzip_sha256=hashlib.sha256(compressed).hexdigest(),json_bytes=len(raw),json_sha256=hashlib.sha256(raw).hexdigest())
(root/'archive-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
