"""Narrow compatibility entrypoint for the unchanged Python History Explorer.

The Rust workbench supplies one verified path and SHA256, never browser paths.
All configuration, execution, native reading and comparison logic stays in Rust.
"""
import argparse
import hashlib
from pathlib import Path
import sys

HOME = Path(__file__).resolve().parent
sys.path.insert(0, str(HOME.parent / 'history-explorer'))
import explorer  # noqa: E402


START = b"""'use strict';
// This catalog contains exactly the selected run. No filesystem URL parameters.
let opened = false;
function openSelectedRun() {
 const select = document.getElementById('trial');
 const status = document.getElementById('status');
 if (!opened && select.options.length === 1 && status.textContent.startsWith('Catalog ready.')) {
  opened = true;
  document.getElementById('load').click();
 }
}
const startupObserver = new MutationObserver(openSelectedRun);
startupObserver.observe(document.getElementById('status'), {childList:true,subtree:true});
openSelectedRun();
"""


class Handler(explorer.Handler):
    def do_GET(self):
        if not self.local_request():
            return self.fail('Only loopback Host/Origin allowed', 403)
        if self.path == '/workbench-start.js':
            return self.send(START, 'text/javascript')
        if self.path == '/':
            original = (explorer.HOME / 'index.html').read_bytes()
            if original.count(b'</body>') != 1:
                return self.fail('Explorer startup interface changed', 409)
            return self.send(original.replace(b'</body>', b'<script src="/workbench-start.js" defer></script></body>'), 'text/html')
        return super().do_GET()


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    source = ap.add_mutually_exclusive_group(required=True)
    for flag in ('continuous', 'valuation', 'preparation'):
        source.add_argument('--' + flag, type=Path)
    ap.add_argument('--expected-sha256', required=True)
    ap.add_argument('--cache', type=Path, required=True)
    ap.add_argument('--port', type=int, required=True)
    ap.add_argument('--no-browser', action='store_true')
    args = ap.parse_args()
    paths = [p for p in (args.continuous, args.valuation, args.preparation) if p]
    if len(paths) != 1 or paths[0].stat().st_size > 64 * 1024 * 1024:
        raise ValueError('Exactly one bounded selected history is required')
    if hashlib.sha256(paths[0].read_bytes()).hexdigest() != args.expected_sha256:
        raise ValueError('Selected history differs from completed run hash')
    repo = explorer.Repository(cache=args.cache, experiments=(),
        continuous=[args.continuous] if args.continuous else (),
        valuations=[args.valuation] if args.valuation else (),
        preparations=[args.preparation] if args.preparation else ())
    repo.catalog()
    if len(repo.entries) != 1:
        raise ValueError('Selected history must contain exactly one trial')
    # Verify membership and native schema before announcing a usable URL.
    repo.get(next(iter(repo.entries)))
    server = explorer.ThreadingHTTPServer(('127.0.0.1', args.port), Handler)
    server.repo = repo
    print(f'History Explorer: http://127.0.0.1:{server.server_port}', flush=True)
    try:
        server.serve_forever()
    finally:
        server.server_close()


if __name__ == '__main__':
    main()
