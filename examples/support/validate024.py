"""Read-only024 source/archive/primitive/scoring/corruption verification."""
from pathlib import Path
import argparse,gzip,hashlib,json
import review024 as r
import review024_findings as f
import analyze024
ROOT=Path(__file__).resolve().parents[2]
def sha(b):return hashlib.sha256(b).hexdigest()
def verify_sources(directory):
 for manifest in ('source-freeze.json','validation-source-locks.json'):
  for name,expected in json.loads((directory/manifest).read_bytes()).items():assert sha((ROOT/name).read_bytes().replace(b'\r\n',b'\n'))==expected,('024 source',name)
 old=json.loads((ROOT/'docs/foundation-001-023-sha256.json').read_bytes());assert len(old)==386
 for name,expected in old.items():
  b=(ROOT/name).read_bytes()
  if name.startswith(tuple(f'experiments/{i:03}/'for i in range(13,24)))and not name.endswith('.gz'):b=b.replace(b'\r\n',b'\n')
  assert sha(b)==expected,('protected original',name)
 raw=json.loads((ROOT/'docs/development/activity-valuation-original-working-sha256.json').read_bytes());assert len(raw)==403
 for name,expected in raw.items():assert sha((ROOT/name).read_bytes())==expected,('raw original',name)
 prospective=json.loads((ROOT/'experiments/024/prospective-validation.json').read_bytes())
 for name,expected in prospective['registered_documents_sha256'].items():assert sha((ROOT/name).read_bytes().replace(b'\r\n',b'\n'))==expected,('fixed registration',name)
 return len(old)
def main():
 p=argparse.ArgumentParser();p.add_argument('directory',nargs='?',default='experiments/024');p.add_argument('--no-corruptions',action='store_true');a=p.parse_args();directory=Path(a.directory);protected=verify_sources(directory)
 manifest=json.loads((directory/'archive-manifest.json').read_bytes());matrix=json.loads((ROOT/'experiments/024/registered-matrix.json').read_bytes());actual=[]
 for name,expected in manifest['archives'].items():
  blob=(directory/name).read_bytes();assert sha(blob)==expected['sha256'];decoded=gzip.decompress(blob);assert sha(decoded)==expected['decoded_sha256'];lines=decoded.splitlines();assert len(lines)==expected['records']==56
  for n,line in enumerate(lines):
   assert sha(line)==expected['member_sha256'][n];t=json.loads(line);assert t['config']==matrix[n];m=r.verify(t);assert m['maximum_captured_own_episode_count']<=16 and m['peak_live_commitments']<=4 and m['peak_pending_tokens']<=4;actual.append(m)
 assert actual==json.loads((directory/'independent-metrics.json').read_bytes())
 archive=directory/'trials.jsonl.gz';findings=f.inspect(archive);assert findings==json.loads((directory/'independent-findings.json').read_bytes())
 comparisons=analyze024.analyze(archive,actual);assert comparisons==json.loads((directory/'comparisons.json').read_bytes())
 if not a.no_corruptions:
  import corrupt024
  rejected=corrupt024.run(archive);stored=json.loads((directory/'corruption-controls.json').read_bytes())
  assert [(x['category'],x['rejected'],x['error'])for x in rejected]==[(x['category'],x['rejected'],x['error'])for x in stored]
 print(f'024 read-only:56 originals,27 matched input prefixes,2 reporting placebos,independent primitive/score/physical reconstructions;{protected} protected originals unchanged')
if __name__=='__main__':main()
