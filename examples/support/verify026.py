"""026 cache-free scientific archive gate; no archive writers or simulation imports."""
import argparse,gzip,hashlib,json,subprocess
from pathlib import Path
import analyze026,corrupt026,review026
from identity026 import ROOT,REGISTRATION,identity,sha
def inventory(directory):
    manifest=json.loads((directory/'archive-manifest.json').read_bytes())
    assert manifest['schema']=='experiment-026-archive/v1' and manifest['records']==100
    assert manifest['registration_revision']==REGISTRATION
    assert manifest['source_freeze_sha256']==sha(ROOT/'experiments/026/source-freeze.json')
    assert set(manifest['members'])=={'trials.jsonl.gz','findings.json','comparisons.json','corruptions.json'}
    for name,binding in manifest['members'].items():
        data=(directory/name).read_bytes()
        if binding['mode']=='lf':data=data.replace(b'\r\n',b'\n')
        else:assert binding['mode']=='raw'
        assert hashlib.sha256(data).hexdigest()==binding['sha256'],('026 member',name)
    digest=hashlib.sha256();size=lines=0
    with gzip.open(directory/'trials.jsonl.gz','rb')as stream:
        for line in stream:digest.update(line);size+=len(line);lines+=1
    assert (digest.hexdigest(),size,lines)==(manifest['raw_sha256'],manifest['raw_bytes'],100)
    return manifest
def main():
    p=argparse.ArgumentParser();p.add_argument('directory',nargs='?',default='experiments/026');a=p.parse_args();d=Path(a.directory)
    matrix=identity();before=inventory(d);expected=json.loads((d/'findings.json').read_bytes());assert len(expected)==100
    count=0
    for i,t in enumerate(review026.read(d/'trials.jsonl.gz')):
        assert i<100
        review026.verify(t,matrix[i],expected[i]);count+=1
    assert count==100
    result=analyze026.analyze(d/'trials.jsonl.gz',expected)
    assert result==json.loads((d/'comparisons.json').read_bytes())
    anchor=next(t for t in review026.read(d/'trials.jsonl.gz')if t['config']['source025']==1 and t['config']['mode']=='Baseline')
    rejected=corrupt026.corruptions(anchor)
    assert json.loads((d/'corruptions.json').read_bytes())==dict(anchor='source0251/Baseline',rejected=rejected)
    command=['cargo','run','--release','--locked','-j','1','--manifest-path','prototypes/delayed-work-valuation/Cargo.toml','--target-dir','target/delayed-work-valuation','--bin','delayed-work-valuation','--','replay',str(d/'trials.jsonl.gz')]
    subprocess.run(command,cwd=ROOT,check=True)
    identity();assert inventory(d)==before
    print('026 scientific gate PASS:100 independently reconstructed worlds/50 policy pairs/14 corruptions/exact complete replay;source and original001–025 custody unchanged. No adoption or foundation extension.')
if __name__=='__main__':main()
