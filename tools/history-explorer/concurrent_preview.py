"""Explicit read-only 022 research preview. Does not run simulation or select policy.

This separate four-lane application is not the main Explorer's 018–021 adapter.
Original JSONL member bytes and their hashes remain the source of every view.
"""
import argparse
import functools
import gzip
import hashlib
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import urlsplit, parse_qs

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent


class Archive:
    def __init__(self, directory):
        self.directory = Path(directory)
        self.manifest = json.loads((self.directory/'archive-manifest.json').read_text())
        for name, expected in self.manifest.items():
            assert hashlib.sha256((self.directory/name).read_bytes()).hexdigest() == expected['gzip_sha256'], name
        self.freeze = self.directory/'source-freeze.json'
        for name, expected in json.loads(self.freeze.read_text()).items():
            assert hashlib.sha256((ROOT/name).read_bytes().replace(b'\r\n',b'\n')).hexdigest() == expected, name
        self.rows = json.loads((self.directory/'registered-matrix.json').read_text())
        assert len(self.rows) == self.manifest['trials.jsonl.gz']['records'] == 100
        self.path = self.directory/'trials.jsonl.gz'
        self.archive_hash = hashlib.sha256(self.path.read_bytes()).hexdigest()

    @functools.lru_cache(maxsize=2)
    def trial(self, ordinal):
        assert 0 <= ordinal < len(self.rows)
        with gzip.open(self.path,'rb') as stream:
            for n,line in enumerate(stream):
                if n != ordinal: continue
                trial = json.loads(line)
                assert trial['schema'] == 'experiment-022-trial/v1' and trial['config'] == self.rows[n]
                return dict(trial=trial, source=dict(schema=trial['schema'], original_ordinal=n,
                    archive=str(self.path.resolve()), archive_sha256=self.archive_hash,
                    member_sha256=hashlib.sha256(line.rstrip(b'\n')).hexdigest(),
                    freeze_sha256=hashlib.sha256(self.freeze.read_bytes()).hexdigest()))
        raise ValueError('missing original member')


def serve(archive, port):
    class Handler(BaseHTTPRequestHandler):
        def do_GET(self):
            parsed = urlsplit(self.path)
            try:
                if parsed.path == '/api/catalog':
                    data = json.dumps(archive.rows).encode(); mime = 'application/json'
                elif parsed.path == '/api/trial':
                    query = parse_qs(parsed.query); ordinal = int(query['ordinal'][0])
                    data = json.dumps(archive.trial(ordinal),separators=(',',':')).encode(); mime = 'application/json'
                elif parsed.path in ('/', '/concurrent.js', '/concurrent-model.js', '/concurrent.css'):
                    name = 'concurrent.html' if parsed.path == '/' else parsed.path[1:]
                    data = (HERE/name).read_bytes(); mime = 'text/html' if name.endswith('.html') else 'text/javascript' if name.endswith('.js') else 'text/css'
                else: self.send_error(404); return
                self.send_response(200); self.send_header('Content-Type',mime+'; charset=utf-8')
                self.send_header('Cache-Control','no-store'); self.send_header('X-Content-Type-Options','nosniff')
                self.send_header('Content-Security-Policy',"default-src 'self'; script-src 'self'; style-src 'self'; object-src 'none'; frame-ancestors 'none'")
                self.send_header('Content-Length',str(len(data))); self.end_headers(); self.wfile.write(data)
            except (KeyError, ValueError, AssertionError): self.send_error(400,'Invalid original ordinal')
    ThreadingHTTPServer(('127.0.0.1',port),Handler).serve_forever()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--port',type=int,default=8768)
    parser.add_argument('--archive',default=str(ROOT/'experiments/022')); args = parser.parse_args()
    serve(Archive(args.archive),args.port)
