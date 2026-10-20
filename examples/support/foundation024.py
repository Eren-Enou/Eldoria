"""Read-only 001–024 inventory/review extension; original gate runs separately.

No archive writers, prepared caches or viewer dependency. Original frozen
validate024 remains unchanged and is copied verbatim into the Rust superset.
"""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REVIEW_HASHES = {
    'docs/experiment-024-review.md': '6ad6a5ae7d67dcf8bcb31c23ba0112b3016c0e9b7bd7fc0ee95cbda8174eb98b',
    'examples/support/review024_audit.py': '20ca411d669331d5707e9507d0f08561636dfa32248cfab1f7a90d87a1667db3',
    'docs/development/experiment-024-review-findings.json': 'ccfac71a9b8a517db0a5e0eedcc22632e6f326691f0efa2c05b38c095d31cd6d',
    'docs/development/finite-material-review-validation.json': 'a1da75755a97001b8aa1340d9c490efd09f9bf7c5abcc122527906ab8c234dd2',
}


def sha(blob):
    return hashlib.sha256(blob).hexdigest()


def identity():
    original = json.loads((ROOT/'docs/foundation-001-023-sha256.json').read_bytes())
    current = json.loads((ROOT/'docs/foundation-001-024-sha256.json').read_bytes())
    assert len(original) == 386 and len(current) == 408
    assert all(current[k] == v for k, v in original.items()), 'Inherited hash changed'
    added = {p.relative_to(ROOT).as_posix() for p in (ROOT/'experiments/024').iterdir() if p.is_file()}
    assert len(added) == 22 and current.keys() - original.keys() == added
    for name, expected in current.items():
        b = (ROOT/name).read_bytes()
        if name.startswith(tuple(f'experiments/{i:03}/' for i in range(13, 25))) and not name.endswith('.gz'):
            b = b.replace(b'\r\n', b'\n')
        assert sha(b) == expected, ('Protected artifact changed', name)
    # Preserve exact raw working bytes in addition to portable inventory hashes.
    review = json.loads((ROOT/'docs/development/finite-material-review-validation.json').read_bytes())
    historical = json.loads((ROOT/'docs/development/activity-valuation-original-working-sha256.json').read_bytes())
    assert len(historical) == 403 and len(review['original024_raw_sha256']) == 22
    for name, expected in {**historical, **review['original024_raw_sha256']}.items():
        assert sha((ROOT/name).read_bytes()) == expected, ('Raw original changed', name)
    for name, expected in REVIEW_HASHES.items():
        assert sha((ROOT/name).read_bytes().replace(b'\r\n', b'\n')) == expected, ('Review binding', name)
    standalone = (ROOT/'examples/validate024.rs').read_text(encoding='utf-8').rstrip()
    shared = (ROOT/'examples/support/contract024.rs').read_text(encoding='utf-8').split('\n// FOUNDATION024 EXTENSION:')[0]
    assert standalone == shared, 'Frozen standalone/shared logic drift'
    supplemental = json.loads((ROOT/'experiments/024/supplemental-evidence-sha256.json').read_bytes())
    for name, expected in supplemental.items():
        assert sha((ROOT/name).read_bytes().replace(b'\r\n', b'\n')) == expected, ('Supplemental binding', name)
    print('001–024:408artifacts,386inherited+22original024;425raw originals;review and standalone parity PASS')


def main():
    identity()
    subprocess.run([sys.executable, '-B', str(ROOT/'examples/support/review024_audit.py'),
                    '--expected', str(ROOT/'docs/development/experiment-024-review-findings.json'),
                    '--controls'], cwd=ROOT, check=True)


if __name__ == '__main__':
    main()
