"""Archive authorized 012 evidence only. Never writes under experiments001–011."""
import argparse
import gzip
import hashlib
import io
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument("--source",type=Path,default=ROOT/"target/experiment012-final")
parser.add_argument("--development",type=Path,default=ROOT/"target/experiment012/summary.json")
args=parser.parse_args()
out=ROOT/"experiments/012"
manifest={}
for name in ("seed-42","held-out-seed-42","summary","development-summary"):
    source=args.development if name=="development-summary" else args.source/(name+".json")
    raw=source.read_bytes()
    stream=io.BytesIO()
    with gzip.GzipFile(filename="",mode="wb",fileobj=stream,compresslevel=9,mtime=0) as archive:
        archive.write(raw)
    compressed=stream.getvalue()
    (out/(name+".json.gz")).write_bytes(compressed)
    manifest[name]=dict(gzip_bytes=len(compressed),gzip_sha256=hashlib.sha256(compressed).hexdigest(),json_bytes=len(raw),json_sha256=hashlib.sha256(raw).hexdigest())
(out/"archive-manifest.json").write_text(json.dumps(manifest,indent=2,sort_keys=True)+"\n",encoding="utf-8")
