"""Read-only prospective/runtime identity; never execute or replace archives."""
import hashlib,json,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
REGISTRATION='759385d3e6a4dc3e40e816ef0585e335c2640d56'
FOUNDATION='0a33e385942e9bfea8205621c83d60e9271d85fb'
def sha(path):return hashlib.sha256(path.read_bytes().replace(b'\r\n',b'\n')).hexdigest()
def identity(treatment=False,compiled=None):
    import foundation025
    foundation025.identity()
    p=ROOT/'experiments/026';reg=json.loads((p/'prospective-validation.json').read_bytes())
    for manifest in (reg['registered_documents_sha256'],json.loads((p/'inherited-source-bindings.json').read_bytes())):
        for name,digest in manifest.items():assert sha(ROOT/name)==digest,('026 registration',name)
    subprocess.run(['git','merge-base','--is-ancestor',FOUNDATION,REGISTRATION],cwd=ROOT,check=True)
    subprocess.run(['git','merge-base','--is-ancestor',REGISTRATION,'HEAD'],cwd=ROOT,check=True)
    matrix=json.loads((p/'registered-matrix.json').read_bytes());old=json.loads((ROOT/'experiments/025/registered-matrix.json').read_bytes())
    assert len(matrix)==100 and len(old)==50
    for i,c in enumerate(old):
        for off,mode in enumerate(('Baseline','ProjectedNeed')):assert matrix[2*i+off]==dict(source025=i,mode=mode,world=c)
    freeze=p/'source-freeze.json'
    if freeze.exists():
        for name,digest in json.loads(freeze.read_bytes()).items():assert sha(ROOT/name)==digest,('026 source',name)
        if compiled is not None:assert compiled==sha(freeze),'binary compiled against different freeze'
    elif compiled is not None:assert compiled=='UNFROZEN'
    if treatment:
        assert freeze.exists() and compiled!='UNFROZEN'
        gate=json.loads((p/'runtime-gate.json').read_bytes())
        assert gate['status']=='PASS before registered treatment'
        assert gate['registration_revision']==REGISTRATION and gate['source_freeze_sha256']==sha(freeze)
        assert gate['baseline_worlds_exact']==50
        subprocess.run(['git','merge-base','--is-ancestor',gate['runtime_publication_revision'],'HEAD'],cwd=ROOT,check=True)
        published=subprocess.check_output(['git','show',gate['runtime_publication_revision']+':experiments/026/source-freeze.json'],cwd=ROOT)
        assert hashlib.sha256(published.replace(b'\r\n',b'\n')).hexdigest()==sha(freeze)
        committed_gate=subprocess.check_output(['git','show','HEAD:experiments/026/runtime-gate.json'],cwd=ROOT)
        assert hashlib.sha256(committed_gate.replace(b'\r\n',b'\n')).hexdigest()==sha(p/'runtime-gate.json'),'uncommitted treatment gate'
    return matrix
if __name__=='__main__':
    identity(sys.argv[1]=='treatment',sys.argv[2]);print('026 prospective/source identity PASS')
