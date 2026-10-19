"""Measure the full unchanged100-world runner, including the largest archive row."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import time
from measure022 import ROOT, peak_counter


def main():
    parser=argparse.ArgumentParser();parser.add_argument('output');args=parser.parse_args()
    output=Path(args.output).resolve();output.mkdir(parents=True,exist_ok=False)
    raw=output/'replayed-trials.jsonl';begin=time.perf_counter();peak=None;samples=0
    with (output/'runner.log').open('w',encoding='utf-8') as log:
        process=subprocess.Popen([str(ROOT/'target/release/examples/experiment022.exe'),str(raw)],cwd=ROOT,stdout=log,stderr=log)
        while process.poll() is None:
            value=peak_counter(process);samples+=1
            if value is not None:peak=max(peak or 0,value)
            time.sleep(.002)
        status=process.wait()
    wall=time.perf_counter()-begin;assert status==0
    digest=hashlib.sha256();size=0
    with raw.open('rb') as stream:
        while chunk:=stream.read(1024*1024):digest.update(chunk);size+=len(chunk)
    expected=json.loads((ROOT/'experiments/022/archive-manifest.json').read_text())['trials.jsonl.gz']
    assert digest.hexdigest()==expected['json_sha256'] and size==expected['json_bytes']
    result=dict(scope='all100registered worlds, simulation+validation+JSONL disk writing; Windows process peak; no matched baseline',exit=status,
                wall_seconds=wall,peak_working_set_bytes=peak,peak_poll_samples=samples,raw_bytes=size,raw_sha256=digest.hexdigest(),exact_original_bytes=True)
    (output/'performance-suite.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8');print(json.dumps(result))


if __name__=='__main__':main()
