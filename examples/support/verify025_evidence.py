"""Strict JSON-value comparison around frozen025read-only report producers.

Does not import a simulator or alter evidence. Original validators remain intact.
Every projected report must first match the saved report's complete canonical
JSON bytes. This retains all fields, array ordering, numbers and boolean types.
"""
import hashlib
import json
import sys
from pathlib import Path

import validate025_observer2
import validate025

ROOT = Path(__file__).resolve().parents[2]


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False).encode('utf-8')


def project(actual, expected):
    encoded = canonical(actual)
    assert encoded == canonical(expected), '025complete JSON report values differ'
    return json.loads(encoded)


def self_test():
    assert project({'x': (1, 'a')}, {'x': [1, 'a']}) == {'x': [1, 'a']}
    assert project({'b': 2, 'a': 1}, {'a': 1, 'b': 2}) == {'a': 1, 'b': 2}
    for changed in ({'x': [2, 'a']}, {}, {'x': ['a', 1]}, {'x': [True, 'a']}, {'x': [1.0, 'a']}):
        try:
            project(changed, {'x': [1, 'a']})
        except AssertionError:
            pass
        else:
            raise AssertionError('JSON bridge accepted changed report values')
    print('025JSON bridge:tuple/array and key-order controls PASS;5semantic corruptions rejected')


def main():
    if sys.argv[1:] == ['--self-test']:
        self_test()
        return
    bindings = json.loads((ROOT / 'experiments/025/validation-json-bridge-sha256.json').read_bytes())
    for name, expected in bindings.items():
        blob = (ROOT / name).read_bytes().replace(b'\r\n', b'\n')
        assert hashlib.sha256(blob).hexdigest() == expected, name
    replacements = []
    for module, function, report in (
        (validate025.review025_findings, 'inspect', 'independent-findings.json'),
        (validate025.analyze025, 'inspect', 'causal-analysis.json'),
        (validate025.corrupt025, 'run', 'corruption-controls.json'),
    ):
        original = getattr(module, function)

        def checked(path, *args, _original=original, _report=report, **kwargs):
            actual = _original(path, *args, **kwargs)
            expected = json.loads((Path(path).parent / _report).read_bytes())
            return project(actual, expected)

        replacements.append((module, function, original))
        setattr(module, function, checked)
    try:
        validate025_observer2.main()
    finally:
        for module, function, original in replacements:
            setattr(module, function, original)
    print('025complete canonical JSON values match;original frozen producers/outputs unchanged')


if __name__ == '__main__':
    main()
