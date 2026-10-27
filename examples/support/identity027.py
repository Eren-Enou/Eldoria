"""Read-only runtime/registration publication gate; no treatment transitions."""
import hashlib,json,subprocess,sys
from pathlib import Path
from review027 import ROOT,identity
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT)
def main():
    source=identity();assert sys.argv[1]==source
    a=json.loads((ROOT/'experiments/027/activation.json').read_bytes())
    assert a['source']==source and a['baseline_exact']==50 and a['inherited_exact']==100
    for rev in (a['registration_revision'],a['runtime_revision']):
        subprocess.run(['git','merge-base','--is-ancestor',rev,'origin/main'],cwd=ROOT,check=True)
    subprocess.run(['git','merge-base','--is-ancestor',a['registration_revision'],a['runtime_revision']],cwd=ROOT,check=True)
    for name,digest in json.loads((ROOT/'experiments/027/source-freeze.json').read_bytes()).items():
        assert hashlib.sha256(git('show',a['runtime_revision']+':'+name).replace(b'\r\n',b'\n')).hexdigest()==digest
    assert not git('ls-tree','-r','--name-only',a['runtime_revision'],'experiments/027/trials.jsonl.gz')
    print('027 published registration/runtime +100 inherited/50 Fixed activation PASS',source)
if __name__=='__main__':main()
