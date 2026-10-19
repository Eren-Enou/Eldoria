"""One-time NEW023archive writer; never regenerates a historical file in place."""
from pathlib import Path
import argparse,gzip,hashlib,json
ROOT=Path(__file__).resolve().parents[2]
def sha(b):return hashlib.sha256(b).hexdigest()
def main():
    p=argparse.ArgumentParser();p.add_argument('jsonl');p.add_argument('directory');a=p.parse_args()
    directory=Path(a.directory).resolve();directory.mkdir(parents=True,exist_ok=True)
    assert directory==ROOT/'experiments/023' or 'experiments' not in directory.relative_to(ROOT).parts, 'Only023or isolated output'
    source=Path(a.jsonl).read_bytes();lines=source.splitlines();matrix=json.loads((ROOT/'experiments/023/registered-matrix.json').read_bytes())
    assert len(lines)==len(matrix)==42
    for line,config in zip(lines,matrix,strict=True):assert json.loads(line)['config']==config
    path=directory/'trials.jsonl.gz'
    with path.open('xb') as out:
        with gzip.GzipFile(filename='',mode='wb',fileobj=out,mtime=0) as f:f.write(source)
    manifest=dict(schema='experiment-023-archive/v1',archives={'trials.jsonl.gz':dict(sha256=sha(path.read_bytes()),decoded_sha256=sha(source),records=len(lines),member_sha256=[sha(l) for l in lines])})
    with (directory/'archive-manifest.json').open('x',encoding='utf-8') as f:json.dump(manifest,f,indent=2);f.write('\n')
    print('023 NEW originals archived:',len(lines),len(source),'decoded bytes;',path.stat().st_size,'gzip bytes')
if __name__=='__main__':main()
