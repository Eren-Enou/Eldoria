"""Create original021 evidence only at fresh paths; never rewrite older evidence."""
import argparse,gzip,hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def main():
    p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,required=True);p.add_argument('--analysis',type=Path,required=True);p.add_argument('--comparisons',type=Path,required=True);a=p.parse_args()
    out=ROOT/'experiments/021';manifest={}
    for name,source in [('trials.jsonl',a.directory/'trials.jsonl'),('analysis.json',a.analysis),('comparisons.json',a.comparisons)]:
        dest=out/(name+'.gz');assert not dest.exists(),dest
        raw=source.read_bytes();compressed=gzip.compress(raw,compresslevel=9,mtime=0)
        with dest.open('xb') as f:f.write(compressed)
        manifest[dest.name]=dict(gzip_bytes=len(compressed),gzip_sha256=hashlib.sha256(compressed).hexdigest(),json_bytes=len(raw),json_sha256=hashlib.sha256(raw).hexdigest())
    with (out/'archive-manifest.json').open('x') as f:json.dump(manifest,f,indent=2);f.write('\n')
    print(json.dumps(manifest))
if __name__=='__main__':main()
