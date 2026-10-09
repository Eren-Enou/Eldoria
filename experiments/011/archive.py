"""Archive new011 evidence only; verify protected001–010 without regenerating it."""
import gzip
import hashlib
import json
import shutil
from pathlib import Path

folder = Path(__file__).resolve().parent
repo = folder.parents[1]
frozen = json.loads((repo / "docs/foundation-001-010-sha256.json").read_text())
assert all(hashlib.sha256((repo / p).read_bytes()).hexdigest() == h for p, h in frozen.items())
manifest = {}
for name in ["seed-42", "held-out-seed-42", "summary"]:
    raw = (repo / "target/experiment011" / f"{name}.json").read_bytes()
    compressed = gzip.compress(raw, mtime=0)
    (folder / f"{name}.json.gz").write_bytes(compressed)
    manifest[name] = {"json_bytes": len(raw), "gzip_bytes": len(compressed),
                      "json_sha256": hashlib.sha256(raw).hexdigest(),
                      "gzip_sha256": hashlib.sha256(compressed).hexdigest()}
for name in ["benchmark.csv", "performance.csv", "scoring.csv", "analysis.json", "report.txt", "throughput.txt"]:
    shutil.copyfile(repo / "target/experiment011" / name, folder / name)
(folder / "archive-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
(folder / "frozen-001-010-sha256.json").write_text(json.dumps(frozen, indent=2) + "\n", encoding="utf-8")
print(f"New011 evidence archived; {len(frozen)} protected001–010 hashes unchanged")
