"""Read-only025 inventory extension; original001–024 gate runs unchanged first."""
import hashlib
import json
import subprocess
import sys
from pathlib import Path
import foundation024
import validate025_acceptance

ROOT=Path(__file__).resolve().parents[2]
CANDIDATE_SHA256='0ef885f4dd2360c5120320b6b60db19b23e7935e4bceeed266c86ce88ec4226d'
REVIEW_HASHES={'docs/experiment-025-foundation-review.md': '93331a202db82c12206f4a469d71029ef83042faef31cf4ff6e50a84bb255a2e', 'examples/support/review025_foundation.py': 'b289597713a17b8e3364c439af47f2580eb2739555ee027f0c7b07606db884cb', 'docs/development/experiment-025-foundation-review.json': '2f9cf5f284a3891fb7fd9dfab78b90320ce302507ba80018853aeb4579f31983'}

def sha(data):return hashlib.sha256(data).hexdigest()

def layout(old,new,candidate):
    assert len(old)==408 and len(new)==455
    assert all(new.get(k)==v for k,v in old.items()),'Inherited001–024 binding changed'
    additions={k:v for k,v in candidate['artifacts'].items() if k.startswith('experiments/025/')}
    assert len(additions)==47 and new.keys()-old.keys()==additions.keys(),'025 membership/count changed'
    assert all(new[k]==v['sha256'] for k,v in additions.items()),'025 binding drift'
    return additions

def identity():
    foundation024.identity()
    path=ROOT/'docs/development/experiment-025-integrity-candidate.json'
    assert sha(path.read_bytes().replace(b'\r\n',b'\n'))==CANDIDATE_SHA256
    candidate=json.loads(path.read_bytes())
    assert validate025_acceptance.verify_inventory(ROOT,candidate)==214
    old=json.loads((ROOT/'docs/foundation-001-024-sha256.json').read_bytes())
    new=json.loads((ROOT/'docs/foundation-001-025-sha256.json').read_bytes())
    added=layout(old,new,candidate)
    for name,binding in added.items():
        data=(ROOT/name).read_bytes()
        if binding['mode']=='lf':data=data.replace(b'\r\n',b'\n')
        assert sha(data)==new[name],('Original025 changed',name)
    raw=json.loads((ROOT/'docs/development/experiment-025-original-working-sha256.json').read_bytes())
    assert raw.keys()==added.keys()
    for name,expected in raw.items():assert sha((ROOT/name).read_bytes())==expected,('Raw025 changed',name)
    for name,expected in REVIEW_HASHES.items():assert sha((ROOT/name).read_bytes().replace(b'\r\n',b'\n'))==expected,('Review binding changed',name)
    print('001–025:455 portable artifacts/472 raw originals;214 candidate custody bindings; inheritance/source/review PASS')

def main():
    identity()
    subprocess.run([sys.executable,'-B','examples/support/review025_foundation.py','--expected','docs/development/experiment-025-foundation-review.json'],cwd=ROOT,check=True)

if __name__=='__main__':main()
