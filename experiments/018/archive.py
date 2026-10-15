"""Authorized018 deterministic streaming archive writer. Never writes older evidence."""
import argparse,gzip,hashlib,json,shutil
from pathlib import Path
D=Path(__file__).resolve().parent

def digest(path):
    h=hashlib.sha256();size=0
    with path.open('rb') as f:
        while chunk:=f.read(1048576):h.update(chunk);size+=len(chunk)
    return size,h.hexdigest()
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--from-directory',type=Path,required=True);args=ap.parse_args();manifest={}
    for name in ('seed-42.jsonl','held-out-seed-42.jsonl','summary.json'):
        source=args.from_directory/name;target=D/(name+'.gz')
        with source.open('rb') as src,target.open('wb') as dst,gzip.GzipFile(filename='',fileobj=dst,mode='wb',compresslevel=9,mtime=0) as z:shutil.copyfileobj(src,z,1048576)
        a,b=digest(source);c,d=digest(target);manifest[target.name]=dict(json_bytes=a,json_sha256=b,gzip_bytes=c,gzip_sha256=d)
    (D/'archive-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
    shutil.copyfile(args.from_directory/'performance.csv',D/'performance.csv')
    print(json.dumps(manifest,sort_keys=True))
if __name__=='__main__':main()
