"""Cache-free read-only023identity, primitive and negative controls gate."""
from pathlib import Path
import argparse,gzip,hashlib,json,sys
import review023
ROOT=Path(__file__).resolve().parents[2]
def sha(b):return hashlib.sha256(b).hexdigest()
def verify_sources(directory):
    for manifest in ('source-freeze.json','validation-source-locks.json'):
        for name,expected in json.loads((directory/manifest).read_bytes()).items():
            assert sha((ROOT/name).read_bytes().replace(b'\r\n',b'\n'))==expected, '023source mismatch '+name
    historical=json.loads((ROOT/'docs/foundation-001-022-sha256.json').read_bytes())
    for name,expected in historical.items():
        b=(ROOT/name).read_bytes()
        if name.startswith(tuple('experiments/%03d/'%i for i in range(13,23))) and not name.endswith('.gz'):b=b.replace(b'\r\n',b'\n')
        assert sha(b)==expected, 'Protected original changed '+name
    return len(historical)
def main():
    p=argparse.ArgumentParser();p.add_argument('directory',nargs='?',default='experiments/023');p.add_argument('--no-corruptions',action='store_true');a=p.parse_args();directory=Path(a.directory)
    protected=verify_sources(directory);manifest=json.loads((directory/'archive-manifest.json').read_bytes());matrix=json.loads((directory/'registered-matrix.json').read_bytes());count=0
    for name,expected in manifest['archives'].items():
        compressed=(directory/name).read_bytes();assert sha(compressed)==expected['sha256'];decoded=gzip.decompress(compressed);assert sha(decoded)==expected['decoded_sha256'];lines=decoded.splitlines();assert len(lines)==expected['records']
        for n,line in enumerate(lines):
            assert sha(line)==expected['member_sha256'][n];trial=json.loads(line);assert trial['config']==matrix[n];review023.verify(trial);count+=1
    assert count==len(matrix)==42
    if not a.no_corruptions:
        import corrupt023
        corrupt023.run(directory/'trials.jsonl.gz')
    print(f'023 read-only: {count} source/member/primitive checks; {protected} protected originals unchanged')
if __name__=='__main__':main()
