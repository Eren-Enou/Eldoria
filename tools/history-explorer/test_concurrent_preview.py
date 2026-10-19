"""Original-source and read-only HTTP checks for the separate 022 preview."""
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from urllib.error import HTTPError
from urllib.request import Request, urlopen

HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('concurrent_preview',HERE/'concurrent_preview.py')
preview=importlib.util.module_from_spec(spec);spec.loader.exec_module(preview)


class ConcurrentPreview(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.archive=preview.Archive(preview.ROOT/'experiments/022')

    def test_original_identity_and_native_schema(self):
        self.assertEqual(len(self.archive.rows),100)
        for ordinal in (0,1,5,15,18,20,31,33,35,99):
            exported=self.archive.trial(ordinal)
            self.assertEqual(exported['source']['original_ordinal'],ordinal)
            self.assertEqual(exported['trial']['config'],self.archive.rows[ordinal])
            self.assertEqual(exported['trial']['schema'],'experiment-022-trial/v1')
            self.assertEqual([a['id'] for a in exported['trial']['initial']],[0,1,2,3])
            with gzip.open(self.archive.path,'rb') as stream:
                member=next(line for n,line in enumerate(stream) if n==ordinal)
            self.assertEqual(exported['source']['member_sha256'],hashlib.sha256(member.rstrip(b'\n')).hexdigest())
            self.assertEqual(exported['trial'],json.loads(member))
        self.assertLessEqual(self.archive.trial.cache_info().currsize,2)

    def test_invalid_ordinal_and_source_freeze_failure(self):
        for ordinal in (-1,100):
            with self.assertRaises(AssertionError):self.archive.trial(ordinal)
        with tempfile.TemporaryDirectory(prefix='eldoria-022-preview-') as folder:
            directory=Path(folder).resolve()
            self.assertEqual(directory.parent,Path(tempfile.gettempdir()).resolve())
            for path in self.archive.directory.glob('*.gz'):shutil.copyfile(path,directory/path.name)
            for name in ('archive-manifest.json','registered-matrix.json'):shutil.copyfile(self.archive.directory/name,directory/name)
            (directory/'source-freeze.json').write_text(json.dumps({'src/experiment022.rs':'0'*64}),encoding='utf-8')
            with self.assertRaises(AssertionError):preview.Archive(directory)

    def test_actual_javascript_owned_projection(self):
        with tempfile.TemporaryDirectory(prefix='eldoria-022-model-') as folder:
            directory=Path(folder).resolve()
            self.assertEqual(directory.parent,Path(tempfile.gettempdir()).resolve())
            path=directory/'trial.json'
            path.write_text(json.dumps(self.archive.trial(1)['trial']),encoding='utf-8')
            result=subprocess.run(['node',str(HERE/'concurrent.test.cjs'),str(path)],capture_output=True,text=True,check=False)
            self.assertEqual(result.returncode,0,result.stdout+'\n'+result.stderr)

    def test_running_preview_is_read_only_and_uses_original_member(self):
        host='http://127.0.0.1:8768'
        with urlopen(host+'/api/catalog') as response:
            self.assertEqual(json.load(response),self.archive.rows)
            self.assertEqual(response.headers['Cache-Control'],'no-store')
        with urlopen(host+'/api/trial?ordinal=1') as response:
            self.assertEqual(json.load(response),self.archive.trial(1))
        for path,status in (('/api/trial?ordinal=-1',400),('/api/trial?ordinal=100',400),('/api/run',404),('/../AGENTS.md',404)):
            with self.assertRaises(HTTPError) as failure:urlopen(host+path)
            self.assertEqual(failure.exception.code,status)
        with self.assertRaises(HTTPError) as failure:
            urlopen(Request(host+'/api/trial',data=b'{"action":"run"}',method='POST'))
        self.assertEqual(failure.exception.code,501)


if __name__=='__main__':unittest.main()
