"""Additive qualified026 custody. All inherited gates/sources stay unchanged."""
import copy,hashlib,json,subprocess,sys
from pathlib import Path
import foundation025
from identity026_c1 import identity as c1_identity
ROOT=Path(__file__).resolve().parents[2]
CUSTODY_SHA256='e53d5f9ccb00214e12a33a0aae4f69a42cef25bd7bfea5bc893c1742ad58f4ba'
RAW_SHA256='12d757d4a73b7ae21f98c03483e57bc72c00a4efc4d4272e08eea5e7ecf97baf'
def sha(b):return hashlib.sha256(b).hexdigest()
def binding(data,b):
 assert b['mode']in('raw','lf')
 if b['mode']=='lf':data=data.replace(b'\r\n',b'\n')
 assert sha(data)==b['sha256'],'protected content or qualification drift'
def layout(old,new,m):
 assert len(old)==455 and len(new)==495
 assert all(new.get(k)==v for k,v in old.items()),'inherited binding drift'
 added=set(m['original026_paths']);assert len(added)==40 and new.keys()-old.keys()==added
 assert all(new[n]==m['bindings'][n]['sha256']for n in added),'original026 binding drift'
def identity():
 foundation025.identity();c1_identity()
 p=ROOT/'docs/development/experiment-026-foundation-custody.json'
 assert sha(p.read_bytes().replace(b'\r\n',b'\n'))==CUSTODY_SHA256,'026 custody definition changed'
 m=json.loads(p.read_bytes())
 old=json.loads((ROOT/'docs/foundation-001-025-sha256.json').read_bytes());new=json.loads((ROOT/'docs/foundation-001-026-sha256.json').read_bytes());layout(old,new,m)
 for n,b in m['bindings'].items():
  try:binding((ROOT/n).read_bytes(),b)
  except AssertionError as e:raise AssertionError(('026 protected binding changed',n))from e
 p=ROOT/'docs/development/experiment-026-original-working-sha256.json';assert sha(p.read_bytes().replace(b'\r\n',b'\n'))==RAW_SHA256
 raw=json.loads(p.read_bytes());assert set(raw)==set(m['original026_paths'])
 for n,s in raw.items():assert sha((ROOT/n).read_bytes())==s,('raw026 changed',n)
 # Definition-only scratch faults never write into protected evidence.
 faults=[]
 for kind in ('missing_original','inherited_drift','new_digest_drift'):
  mutated=copy.deepcopy(new)
  if kind=='missing_original':del mutated[m['original026_paths'][0]]
  elif kind=='inherited_drift':mutated[next(iter(old))]='0'*64
  else:mutated[m['original026_paths'][0]]='0'*64
  try:layout(old,mutated,m)
  except AssertionError:faults.append(kind)
  else:raise AssertionError(('accepted layout corruption',kind))
 print('001â€“026:495 portable/512 raw originals;',len(m['bindings']),'source/review/evidence bindings;3 layout corruptions rejected;identity PASS')
 return dict(portable=495,raw_originals=512,bindings=len(m['bindings']),layout_faults=faults)
def main():
 identity()
 subprocess.run([sys.executable,'-B','examples/support/review026_foundation.py','--expected','docs/development/experiment-026-foundation-review.json'],cwd=ROOT,check=True)
if __name__=='__main__':main()
