"""C1 prospective/runtime guard;original frozen-v1 guard stays unchanged."""
import hashlib,json,subprocess,sys
from pathlib import Path
from identity026 import ROOT,identity as original_identity,sha
DECLARATION='6f44686c1d3b7a85656a1ce9244c9a37bc6799ab'
DIRECTORY=ROOT/'experiments/026/continuation-c1'
def identity(treatment=False,compiled=None):
    matrix=original_identity()
    for mapping in (json.loads((DIRECTORY/'prospective-validation.json').read_bytes())['source_bound_design'],json.loads((DIRECTORY/'source-bindings.json').read_bytes())):
        for name,digest in mapping.items():assert sha(ROOT/name)==digest,('C1 declared source',name)
    failure=ROOT/'experiments/026/failure'
    for name,binding in json.loads((failure/'manifest.json').read_bytes())['members'].items():
        assert Path(name).name==name
        data=(failure/name).read_bytes()
        if binding['mode']=='lf':data=data.replace(b'\r\n',b'\n')
        else:assert binding['mode']=='raw'
        assert hashlib.sha256(data).hexdigest()==binding['sha256'],('v1 failed evidence changed',name)
    subprocess.run(['git','merge-base','--is-ancestor',DECLARATION,'HEAD'],cwd=ROOT,check=True)
    freeze=DIRECTORY/'source-freeze.json'
    if freeze.exists():
        for name,digest in json.loads(freeze.read_bytes()).items():assert sha(ROOT/name)==digest,('C1 source',name)
        if compiled is not None:assert compiled==sha(freeze)
    elif compiled is not None:assert compiled=='UNFROZEN'
    if treatment:
        assert freeze.exists() and compiled!='UNFROZEN'
        g=json.loads((DIRECTORY/'runtime-gate.json').read_bytes())
        assert g['status']=='PASS before unknown corrected treatment'and g['declaration_revision']==DECLARATION
        assert g['source_freeze_sha256']==sha(freeze) and g['baseline_worlds_exact']==50 and g['v1_prefix_worlds_exact']==43
        subprocess.run(['git','merge-base','--is-ancestor',g['runtime_publication_revision'],'HEAD'],cwd=ROOT,check=True)
        pub=subprocess.check_output(['git','show',g['runtime_publication_revision']+':experiments/026/continuation-c1/source-freeze.json'],cwd=ROOT)
        assert hashlib.sha256(pub.replace(b'\r\n',b'\n')).hexdigest()==sha(freeze)
        pub=subprocess.check_output(['git','show','HEAD:experiments/026/continuation-c1/runtime-gate.json'],cwd=ROOT)
        assert hashlib.sha256(pub.replace(b'\r\n',b'\n')).hexdigest()==sha(DIRECTORY/'runtime-gate.json')
    return matrix
if __name__=='__main__':identity(sys.argv[1]=='treatment',sys.argv[2]);print('C1 declared/source identity PASS;v1 unchanged')
