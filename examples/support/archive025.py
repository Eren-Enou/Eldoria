"""One-time025original archive creation; never overwrite historical evidence."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('input')
    parser.add_argument('directory',nargs='?',default='experiments/025')
    args=parser.parse_args()
    directory=Path(args.directory)
    output=directory/'trials.jsonl.gz'
    manifest=directory/'archive-manifest.json'
    assert directory.resolve()==Path(__file__).resolve().parents[2]/'experiments/025'
    assert not output.exists()and not manifest.exists(),'025original archive/manifest already exists'
    matrix=json.loads((directory/'registered-matrix.json').read_bytes())
    raw=Path(args.input).read_bytes();assert raw.endswith(b'\n')
    lines=raw.splitlines();assert len(lines)==len(matrix)==50
    for line,expected in zip(lines,matrix,strict=True):
        trial=json.loads(line)
        assert trial['schema']=='experiment-025-trial/v1'and trial['config']==expected
    with output.open('xb')as target:
        with gzip.GzipFile(filename='',mode='wb',fileobj=target,compresslevel=9,mtime=0)as zipped:
            zipped.write(raw)
    sha=lambda b:hashlib.sha256(b).hexdigest()
    evidence={'schema':'experiment-025-archive-manifest/v1','archives':{'trials.jsonl.gz':{
        'sha256':sha(output.read_bytes()),'decoded_sha256':sha(raw),'records':50,
        'member_sha256':[sha(line)for line in lines]}}}
    with manifest.open('x',encoding='utf-8')as target:
        json.dump(evidence,target,indent=2);target.write('\n')
    print('025one-time original archive:50fixed members;deterministic gzip;no prior evidence replaced')


if __name__=='__main__':main()
