"""Read-only027 native exact replay, source/chronology, independent review and faults.
Use isolated target/material-experience, never historical Cargo host targets.
"""
import gzip,hashlib,json,subprocess,tempfile
from pathlib import Path
import review027 as r
ROOT=r.ROOT
def run(*args):subprocess.run(args,cwd=ROOT,check=True)
def main():
    source=r.identity();archive=ROOT/'experiments/027/trials.jsonl.gz'
    originals={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in (ROOT/'experiments/027').rglob('*')if p.is_file()}
    run('cargo','build','--release','--locked','-j','1','--manifest-path','prototypes/material-experience/Cargo.toml','--target-dir','target/material-experience')
    native=ROOT/'target/material-experience/release/material-experience.exe'
    assert subprocess.check_output([str(native),'identity'],cwd=ROOT).decode().strip()==source
    run('python','-B','examples/support/identity027.py',source)
    run(str(native),'parity','experiments/026/continuation-c1/trials.jsonl.gz')
    with tempfile.TemporaryDirectory(prefix='027-replay-',dir=ROOT/'target')as temp:
        t=Path(temp);fresh=t/'replay.jsonl'
        run(str(native),'run',str(fresh))
        with gzip.open(archive,'rb')as old,open(fresh,'rb')as new:
            assert all(x==y for x,y in __import__('itertools').zip_longest(old,new)),'native replay byte mismatch'
        run('python','-B','examples/support/review027.py',str(fresh),'--output',str(t/'findings.json'))
        assert json.loads((t/'findings.json').read_bytes())==json.loads((ROOT/'experiments/027/findings.json').read_bytes())
        run('python','-B','examples/support/corrupt027.py',str(fresh),str(t/'corruptions.json'))
        assert json.loads((t/'corruptions.json').read_bytes())==json.loads((ROOT/'experiments/027/corruptions.json').read_bytes())
    for n,h in originals.items():assert hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h,('027 archive modified',n)
    print('027 read-only source/publication/native100/Fixed50/replay100/independent100/faults PASS')
if __name__=='__main__':main()
