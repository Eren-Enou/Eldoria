"""Read-only candidate025 custody/acceptance gate. Never adopts a foundation."""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = 'docs/development/experiment-025-integrity-candidate.json'


def verify_inventory(root, manifest):
    root = Path(root).resolve()
    assert manifest['schema'] == 'woi-025-integrity-candidate/v1'
    assert manifest['status'] == 'UNADOPTED_CANDIDATE'
    assert manifest['protected_endpoint'] == '001-024'
    entries = manifest['artifacts']
    assert entries and len(entries) == manifest['artifact_count']
    for name, binding in entries.items():
        relative = Path(name)
        assert not relative.is_absolute() and '..' not in relative.parts, name
        target = (root / relative).resolve()
        assert target.is_relative_to(root), name
        data = target.read_bytes()
        assert binding['mode'] in ('raw', 'lf'), name
        if binding['mode'] == 'lf':
            data = data.replace(b'\r\n', b'\n')
        assert hashlib.sha256(data).hexdigest() == binding['sha256'], name
    expected = {name for name in entries if name.startswith('experiments/025/')}
    actual = {path.relative_to(root).as_posix() for path in (root / 'experiments/025').rglob('*')
              if path.is_file()}
    assert expected and actual == expected, ('025 evidence membership changed', actual ^ expected)
    return len(entries)


def main():
    path = ROOT / MANIFEST
    original = path.read_bytes()
    manifest = json.loads(original)
    count = verify_inventory(ROOT, manifest)
    commands = [
        [sys.executable, '-B', '-m', 'unittest', 'discover', '-s', 'examples/support',
         '-p', 'test_validate025_acceptance.py', '-v'],
        [sys.executable, '-B', '-m', 'unittest', 'discover', '-s', 'examples/support',
         '-p', 'test_review025_food_fate.py', '-v'],
        [sys.executable, '-B', 'examples/support/review025_completion.py', '--expected',
         'docs/development/experiment-025-completion-evidence.json'],
        [sys.executable, '-B', 'examples/support/review025_food_fate.py', '--expected',
         'docs/development/experiment-025-food-fate.json'],
        ['cargo', 'run', '--release', '--locked', '-j', '1', '--example', 'validate025_evidence'],
    ]
    for command in commands:
        subprocess.run(command, cwd=ROOT, check=True)
    assert path.read_bytes() == original, 'Candidate inventory changed during validation'
    assert verify_inventory(ROOT, manifest) == count
    print(f'025 standalone acceptance candidate PASS:{count} custody bindings;'
          ' full original gate, completion controls and exact food attribution;'
          ' protected foundation remains001-024;no adoption or026')


if __name__ == '__main__':
    main()
