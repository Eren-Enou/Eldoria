"""Read-only observer2 provenance extension; unchanged025readers handle outcomes."""
import hashlib
import json
from pathlib import Path

import diagnose025_scene
import validate025

ROOT = Path(__file__).resolve().parents[2]


def sha(blob):
    return hashlib.sha256(blob).hexdigest()


def corrected_sources():
    directory = ROOT / 'experiments/025'
    gate = json.loads((directory / 'runtime-gate-observer2.json').read_bytes())
    assert gate['status'] == 'PASS before corrected registered treatment'
    freeze = directory / 'source-freeze-observer2.json'
    bindings = json.loads(freeze.read_bytes())
    assert 'examples/support/review025_findings.py' in bindings
    assert 'prototypes/material-preparation-observer2/src/observer_validation.rs' in bindings
    for name, expected in bindings.items():
        assert sha((ROOT / name).read_bytes().replace(b'\r\n', b'\n')) == expected, name
    for folder in ('aborted-run', 'diagnostics/scene-namespace'):
        location = directory / folder
        manifest = json.loads((location / 'manifest.json').read_bytes())
        for name, expected in manifest['files_sha256'].items():
            assert sha((location / name).read_bytes()) == expected, (folder, name)
    diagnostic = directory / 'diagnostics/scene-namespace'
    blob = (diagnostic / 'failed-snapshot.json').read_bytes()
    expected = json.loads((diagnostic / 'independent-diagnosis-v2.json').read_bytes())
    actual, _ = diagnose025_scene.inspect(json.loads(blob))
    actual['snapshot_sha256'] = sha(blob)
    actual['corruptions_rejected'] = diagnose025_scene.controls(json.loads(blob))
    assert actual == expected
    revision = json.loads((directory / 'execution-revision.json').read_bytes())
    assert revision['runtime'] == 'material-preparation-observer2'
    assert revision['source_freeze_sha256'] == sha(freeze.read_bytes())
    assert revision['original_aborted_manifest_sha256'] == sha(
        (directory / 'aborted-run/manifest.json').read_bytes())


def main():
    corrected_sources()
    validate025.main()
    print('025observer2: checked namespace bridge and original failed evidence bound; no behavioral retuning')


if __name__ == '__main__':
    main()
