"""Compress only new009 full traces, reproducibly; no archived001-008 writes."""
from pathlib import Path
import gzip
import hashlib
import json

root = Path(__file__).resolve().parents[2]
manifest = {}
for name in ("seed-42", "held-out-seed-42"):
    raw = (root / "target" / "experiment009" / (name + ".json")).read_bytes()
    json.loads(raw)  # reject incomplete evaluator output
    compressed = gzip.compress(raw, compresslevel=9, mtime=0)
    (root / "experiments" / "009" / (name + ".json.gz")).write_bytes(compressed)
    manifest[name] = {"json_bytes": len(raw), "gzip_bytes": len(compressed),
                      "json_sha256": hashlib.sha256(raw).hexdigest(),
                      "gzip_sha256": hashlib.sha256(compressed).hexdigest()}
(root / "experiments" / "009" / "archive-manifest.json").write_text(
    json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
