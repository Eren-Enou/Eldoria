# World of Individuals — History Explorer

A working local, read-only application for actual recorded simulation histories.
No accounts, API keys, package installation, cloud service or Bevy window.

From the repository root, with **Python 3.10+** (tested with 3.12):

```powershell
python -B tools/history-explorer/explorer.py
```

The command starts a loopback-only server and opens your browser at
[127.0.0.1:8765](http://127.0.0.1:8765). Leave the terminal running; Ctrl+C stops it.
Use `--no-browser` to open the URL yourself, or `--port 8766` if the port is occupied.
The authentic default example is Experiment 019, useful_contact, seed 42, Baseline,
original `seed-42.jsonl.gz` record **103**. Its paired Scale110 history is record **107**.

Normal launch uses the bundled **derived discovery catalog**, then verifies the
original compressed and complete decoded archive and selected JSONL member before
showing the trial. The catalog itself is not scientific evidence and is not marked
verified merely because it loads. No whole decoded corpus enters the browser.

Optional full verification/index preparation, creating the catalog and authentic
default pair exports under ignored `target/history-explorer`:

```powershell
python -B tools/history-explorer/explorer.py --prepare
```

It is not required before normal launch. Optional valid Experiment 018 support:

```powershell
python -B tools/history-explorer/explorer.py --include018
```

That performs the slower full 018/019 index scan; it does not include 018's invalid
development candidate. Experiments 001–017 and arbitrary unregistered archives are
not supported by this release. Neither command runs a simulation or evaluator.

Use the left-hand real-history presets, select an individual, inspect its local
candidates, switch to Observer Perspective and follow actual event/point links.
Previous/next, significant-event navigation, play/pause, speed, a slider, tick jump,
next individual decision, source inspection and matched comparison are functional.
The interface renders at most 45 timeline entries near the selected moment.

Download a current viewer export from Import / export. Importing it verifies the
trial against source membership again and rebuilds display projections; imported
claims, timelines and actor projections are not trusted. JSON is data, never code.
A local export does not grant an individual access to observer history.

Read the [practical guide](../../docs/development/history-explorer-user-guide.md),
[technical architecture](../../docs/development/history-explorer-architecture.md),
[validation record](validation/acceptance.md) and [export schema](schema-v1.json).
The original viewer [scope proposal](../../docs/development/experiment-viewer-scope.md)
remains a historical proposal; the guide distinguishes implemented coverage.

Developer checks:

```powershell
python -B -m unittest discover -s tools/history-explorer/tests -v
node --check tools/history-explorer/app.js
node tools/history-explorer/tests/perspective.test.cjs
python -B tools/history-explorer/measure.py
```

Node is only an optional development syntax check. The application needs Python and
a modern browser. `tests/browser-smoke.js` is a reproducible browser interaction
recipe for Codex's browser-control runtime, with manual equivalents in the guide.
Measurements default to ignored `target/history-explorer/performance.json` and are
wall-clock engineering data, not deterministic experiment outcomes.
