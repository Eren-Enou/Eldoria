"""Read-only025research integrity/reconstruction gate; no foundation extension."""
import argparse
import gzip
import hashlib
import json
import runpy
from pathlib import Path

import analyze025
import corrupt025
import review025
import review025_findings

ROOT=Path(__file__).resolve().parents[2]


def sha(blob):return hashlib.sha256(blob).hexdigest()


def sources():
    directory=ROOT/'experiments/025'
    for manifest in('source-freeze.json','validation-source-locks.json'):
        entries=json.loads((directory/manifest).read_bytes())
        assert entries
        for name,expected in entries.items():assert sha((ROOT/name).read_bytes().replace(b'\r\n',b'\n'))==expected,('025source',name)
    prospective=json.loads((directory/'prospective-validation.json').read_bytes())
    for name,expected in prospective['registered_documents_sha256'].items():assert sha((ROOT/name).read_bytes().replace(b'\r\n',b'\n'))==expected,('registration',name)
    foundation=json.loads((ROOT/'docs/development/finite-material-foundation-validation.json').read_bytes())
    assert foundation['state']=='PASS'
    for name,expected in foundation['source_sha256'].items():assert sha((ROOT/name).read_bytes().replace(b'\r\n',b'\n'))==expected,('validated001–024source',name)
    runpy.run_path(str(ROOT/'examples/support/foundation024.py'))['identity']()
    assert json.loads((directory/'runtime-gate.json').read_bytes())['status']=='PASS before first registered treatment'


def main():
    p=argparse.ArgumentParser();p.add_argument('directory',nargs='?',default='experiments/025');args=p.parse_args()
    directory=Path(args.directory);sources()
    manifest=json.loads((directory/'archive-manifest.json').read_bytes())
    matrix=json.loads((ROOT/'experiments/025/registered-matrix.json').read_bytes())
    actual=[]
    for name,expected in manifest['archives'].items():
        blob=(directory/name).read_bytes();assert sha(blob)==expected['sha256']
        raw=gzip.decompress(blob);assert sha(raw)==expected['decoded_sha256']
        lines=raw.splitlines();assert len(lines)==expected['records']==50
        for ordinal,line in enumerate(lines):
            assert sha(line)==expected['member_sha256'][ordinal]
            t=json.loads(line);assert t['config']==matrix[ordinal]
            metrics=review025.verify(t)
            assert metrics['maximum_captured_own_episode_count']<=16 and metrics['peak_live_commitments']<=4 and metrics['peak_pending_tokens']<=4
            actual.append(metrics)
    assert actual==json.loads((directory/'independent-metrics.json').read_bytes())
    archive=directory/'trials.jsonl.gz'
    findings=review025_findings.inspect(archive)
    assert findings==json.loads((directory/'independent-findings.json').read_bytes())
    analyses=analyze025.inspect(archive,actual,findings)
    assert analyses==json.loads((directory/'causal-analysis.json').read_bytes())
    controls=corrupt025.run(archive)
    assert controls==json.loads((directory/'corruption-controls.json').read_bytes())
    print('025read-only:50originals,24matched access prefixes,2report placebos,independent physical/local/cognitive reconstructions,causal chains and scratch controls PASS;001–024unchanged;no protection/adoption')


if __name__=='__main__':main()
