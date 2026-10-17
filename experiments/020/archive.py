"""Archive newly authorized020 evidence only. Existing destinations are rejected."""
import argparse,gzip,hashlib,json,shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def main():
    p=argparse.ArgumentParser();p.add_argument('source',type=Path);args=p.parse_args();d=Path(__file__).resolve().parent
    target=d/'trials.jsonl.gz';assert not target.exists(),'never overwrite original evidence'
    h=hashlib.sha256();size=0
    with (args.source/'trials.jsonl').open('rb') as src,target.open('wb') as raw,gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=0,compresslevel=9) as out:
        while data:=src.read(1048576):h.update(data);size+=len(data);out.write(data)
    b=target.read_bytes();manifest={target.name:dict(gzip_bytes=len(b),gzip_sha256=hashlib.sha256(b).hexdigest(),json_bytes=size,json_sha256=h.hexdigest())}
    (d/'archive-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8',newline='\n')
    shutil.copyfile(args.source/'performance.csv',d/'performance.csv')
    shutil.copyfile(ROOT/'docs/foundation-001-019-sha256.json',d/'frozen-001-019-sha256.json')
    files=['src/acquisition.rs','src/simulation/acquisition.rs','src/experiment020.rs','experiments/020/specification.md','experiments/020/design-gate.md','examples/evaluate020.rs','examples/support/review020.py','examples/validate020.rs','examples/measure020.rs','tests/acquisition.rs','experiments/020/archive.py','experiments/020/report.py']
    freeze={n:hashlib.sha256((ROOT/n).read_bytes().replace(b'\r\n',b'\n')).hexdigest() for n in files}
    (d/'source-freeze.json').write_text(json.dumps(freeze,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(json.dumps(manifest))
if __name__=='__main__':main()
