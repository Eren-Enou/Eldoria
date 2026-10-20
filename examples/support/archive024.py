"""Archive NEW024originals exactly once. Never replaces a historical file."""
from pathlib import Path
import argparse,gzip,hashlib,json
ROOT=Path(__file__).resolve().parents[2]
def sha(b):return hashlib.sha256(b).hexdigest()
def main():
 p=argparse.ArgumentParser();p.add_argument('jsonl');p.add_argument('directory');a=p.parse_args();directory=Path(a.directory).resolve()
 assert directory==ROOT/'experiments/024' or not directory.is_relative_to(ROOT/'experiments')
 directory.mkdir(parents=True,exist_ok=True);source=Path(a.jsonl).read_bytes();lines=source.splitlines();matrix=json.loads((ROOT/'experiments/024/registered-matrix.json').read_bytes());assert len(lines)==len(matrix)==56
 for line,cfg in zip(lines,matrix,strict=True):assert json.loads(line)['config']==cfg
 assert not(directory/'trials.jsonl.gz').exists()and not(directory/'archive-manifest.json').exists()
 with(directory/'trials.jsonl.gz').open('xb')as out:
  with gzip.GzipFile(filename='',mode='wb',fileobj=out,mtime=0)as zipped:zipped.write(source)
 manifest=dict(schema='experiment-024-archive/v1',archives={'trials.jsonl.gz':dict(sha256=sha((directory/'trials.jsonl.gz').read_bytes()),decoded_sha256=sha(source),records=56,member_sha256=[sha(line)for line in lines])})
 with(directory/'archive-manifest.json').open('x',encoding='utf-8')as out:json.dump(manifest,out,indent=2);out.write('\n')
 print('NEW024originals archived:56members;',len(source),'decoded;', (directory/'trials.jsonl.gz').stat().st_size,'gzipbytes')
if __name__=='__main__':main()
