"""Candidate custody controls on disposable files; no original evidence mutation."""
import copy
import hashlib
import tempfile
import unittest
from pathlib import Path

from validate025_acceptance import verify_inventory


class CandidateInventoryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='woi-025-custody-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.raw = self.root / 'experiments/025/original.txt'
        self.text = self.root / 'docs/review.md'
        self.raw.parent.mkdir(parents=True)
        self.text.parent.mkdir(parents=True)
        self.raw.write_bytes(b'original\r\n')
        self.text.write_bytes(b'review\r\n')
        self.manifest = dict(schema='woi-025-integrity-candidate/v1', status='UNADOPTED_CANDIDATE',
                             protected_endpoint='001-024', artifact_count=2,
                             artifacts={
                                 'experiments/025/original.txt': dict(mode='raw', sha256=hashlib.sha256(b'original\r\n').hexdigest()),
                                 'docs/review.md': dict(mode='lf', sha256=hashlib.sha256(b'review\n').hexdigest())})

    def test_portable_review_accepts_newlines_but_raw_evidence_does_not(self):
        self.assertEqual(verify_inventory(self.root, self.manifest), 2)
        self.text.write_bytes(b'review\n')
        self.assertEqual(verify_inventory(self.root, self.manifest), 2)
        self.raw.write_bytes(b'original\n')
        with self.assertRaises(AssertionError):
            verify_inventory(self.root, self.manifest)

    def test_changed_or_missing_evidence_is_rejected(self):
        self.raw.write_bytes(b'changed\r\n')
        with self.assertRaises(AssertionError):
            verify_inventory(self.root, self.manifest)
        self.raw.unlink()
        with self.assertRaises(FileNotFoundError):
            verify_inventory(self.root, self.manifest)

    def test_unexpected_evidence_membership_is_rejected(self):
        (self.raw.parent / 'unreviewed.txt').write_bytes(b'extra')
        with self.assertRaises(AssertionError):
            verify_inventory(self.root, self.manifest)

    def test_parent_escape_and_unknown_normalization_are_rejected(self):
        bad = copy.deepcopy(self.manifest)
        bad['artifacts']['../outside'] = bad['artifacts'].pop('docs/review.md')
        with self.assertRaises(AssertionError):
            verify_inventory(self.root, bad)
        bad = copy.deepcopy(self.manifest)
        bad['artifacts']['docs/review.md']['mode'] = 'ignore-whitespace'
        with self.assertRaises(AssertionError):
            verify_inventory(self.root, bad)

    def test_count_and_adoption_status_are_not_silently_changed(self):
        for field, value in (('artifact_count', 1), ('status', 'PROTECTED'),
                             ('protected_endpoint', '001-025')):
            bad = copy.deepcopy(self.manifest)
            bad[field] = value
            with self.assertRaises(AssertionError):
                verify_inventory(self.root, bad)


if __name__ == '__main__':
    unittest.main()
