"""Full C1 scientific gate;does not weaken or replace the frozen-v1 gate."""
import gzip,hashlib,json,subprocess
from pathlib import Path
import review026_c1 as r,analyze026_c1 as a,corrupt026_c1 as c
from identity026_c1 import ROOT,DIRECTORY,DECLARATION,identity,sha
def inventory(directory):
    m=json.loads((directory/'archive-manifest.json').read_bytes())
    assert m['schema']=='experiment-026-c1-archive/v1'and m['records']==100 and m['declaration_revision']==DECLARATION
    assert m['source_freeze_sha256']==sha(DIRECTORY/'source-freeze.json')
    assert set(m['members'])=={'trials.jsonl.gz','findings.json','comparisons.json','corruptions.json'}
    for name,binding in m['members'].items():
        data=(directory/name).read_bytes()
        if binding['mode']=='lf':data=data.replace(b'\r\n',b'\n')
        else:assert binding['mode']=='raw'
        assert hashlib.sha256(data).hexdigest()==binding['sha256']
    digest=hashlib.sha256();size=rows=0
    with gzip.open(directory/'trials.jsonl.gz','rb')as stream:
        for line in stream:digest.update(line);size+=len(line);rows+=1
    assert (rows,size,digest.hexdigest())==(100,m['raw_bytes'],m['raw_sha256'])
    return m
def main():
    matrix=identity();before=inventory(DIRECTORY);findings=json.loads((DIRECTORY/'findings.json').read_bytes());assert len(findings)==100;count=0
    for i,t in enumerate(r.read(DIRECTORY/'trials.jsonl.gz')):
        assert i<100;r.verify(t,matrix[i],findings[i]);count+=1
    assert count==100
    assert a.analyze(DIRECTORY/'trials.jsonl.gz',findings)==json.loads((DIRECTORY/'comparisons.json').read_bytes())
    assert c.inspect(DIRECTORY/'trials.jsonl.gz')==json.loads((DIRECTORY/'corruptions.json').read_bytes())
    # Build both explicitly:failure verification must not depend on a cached v1 executable.
    subprocess.run(['cargo','build','--release','--locked','-j','1','--manifest-path','prototypes/delayed-work-valuation/Cargo.toml','--target-dir','target/delayed-work-valuation','--bin','delayed-work-valuation'],cwd=ROOT,check=True)
    subprocess.run(['python','-B','examples/support/verify026_failure.py'],cwd=ROOT,check=True)
    subprocess.run(['cargo','run','--release','--locked','-j','1','--manifest-path','prototypes/delayed-work-valuation-c1/Cargo.toml','--target-dir','target/delayed-work-valuation','--bin','delayed-work-valuation-c1','--','replay',str(DIRECTORY/'trials.jsonl.gz')],cwd=ROOT,check=True)
    identity();assert inventory(DIRECTORY)==before
    print('C1 scientific gate PASS:100 independent worlds/50 policy pairs/17 corruption categories/exact100 replay;old v1 exact43-prefix failure preserved;001–025 unchanged. No default adoption or foundation extension.')
if __name__=='__main__':main()
