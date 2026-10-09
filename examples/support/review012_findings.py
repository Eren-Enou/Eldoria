"""Read-only scientific anchors for 012; no runtime or evaluator imports.

Extends review012's independently implemented reconstruction with immediate
resource-readiness mediation, archived reporting consistency, negative anchors,
and a quantified comparison with the preserved defective development summary.
Run without -O. Output is a review diagnostic, not replacement experiment evidence.
"""
import collections
import csv
import itertools
import json
from pathlib import Path
import review012 as r

ROOT = Path(__file__).resolve().parents[2]
DIRECTORY = ROOT / "experiments/012"


def main():
    archived = json.loads((DIRECTORY / "analysis.json").read_bytes())
    core_trials = r.read(DIRECTORY, "seed-42")
    held_trials = r.read(DIRECTORY, "held-out-seed-42")
    trials = core_trials + held_trials
    rows = [dict(held=held, **r.reconstruct(t)) for held, suite in [(False, core_trials), (True, held_trials)] for t in suite]
    assert len(core_trials) == 45 and len(held_trials) == 135
    assert len({t["case"] for t in trials}) == 15
    assert len({r.old.key(t["config"]) for t in core_trials}) == 1
    assert len({r.old.key(t["config"]) for t in held_trials}) == 3
    assert rows == archived["trials"] and len(rows) == 180
    groups = collections.defaultdict(dict)
    for t, row in zip(trials, rows, strict=True):
        groups[(row["held"], t["case"], r.old.key(t["config"]))][t["mode"]] = (t, row)
    classes = collections.Counter()
    material_anchors = []
    expected_comparisons = []
    for modes in groups.values():
        for left, right in itertools.combinations(r.MODES, 2):
            a, x = modes[left]
            b, y = modes[right]
            queries = lambda row: [s["selected"] for s in row["steps"]]
            material_same = (x["actions"], x["food"]) == (y["actions"], y["food"])
            cls = "Equivalent" if material_same and queries(x) == queries(y) else "C" if material_same else "D" if queries(x) != queries(y) else "Ambiguous"
            classes[f'{x["held"]}:{left}/{right}:{cls}'] += 1
            expected_comparisons.append(dict(held=str(x["held"]), case=a["case"], support=str(a["config"]["support"]),
                left=left, right=right, **{"class": cls}, left_actions=json.dumps(x["actions"]), right_actions=json.dumps(y["actions"]),
                left_food=json.dumps(x["food"]), right_food=json.dumps(y["food"])))
            if cls == "D":
                # Check cognition has changed immediate legal resource readiness,
                # before resource execution or its automatic follow-up inquiries.
                ready = lambda t, p: r.old.readiness(p["agents"][0], t["resource_partner"], t["config"]["amount"])
                assert ready(a, a["opportunities"][0]["before"]) == ready(b, b["opportunities"][0]["before"]) == "Leave"
                assert ready(a, a["opportunities"][-1]["after"]) == x["actions"][0]
                assert ready(b, b["opportunities"][-1]["after"]) == y["actions"][0]
                assert x["actions"][0] != y["actions"][0] and x["food"] != y["food"]
                material_anchors.append(dict(case=a["case"], support=a["config"]["support"], pair=[left, right]))
    assert dict(classes) == archived["classifications"]
    with (DIRECTORY / "comparisons.csv").open(newline="") as handle:
        comparisons = list(csv.DictReader(handle))
    assert comparisons == expected_comparisons and len(comparisons) == 180
    core = {t["case"]: {} for t in trials if t["config"]["support"] == 20}
    for t, row in zip(trials, rows, strict=True):
        if t["config"]["support"] == 20:
            core[t["case"]][t["mode"]] = (t, row)
    action = lambda concern, source, strategy: {"Ask": dict(concern=concern, source=source, strategy=strategy)}
    for mode, (t, row) in core["changed_circumstances"].items():
        first, second = row["steps"][:2]
        assert first["selected"] == action(0, 1, "Direct")
        assert first["outcome"]["novelty_value"] == 50 and first["outcome"]["support_before"] == first["outcome"]["support_after"] == 20
        assert first["after"]["expected_change"] == 27
        inquiry = t["final_state"]["base"]["base"]["base"]["base"]["inquiry"]["records"][0]
        assert inquiry["cell_after"]["expected"] == 52
        if mode != "Unchanged":
            assert second["selected"] == action(0, 1, "Evidence")
            assert (second["outcome"]["support_after"], second["outcome"]["status_after"], second["after"]["expected_change"]) == (60, "Resolved", 82)
            assert row["actions"] == ["Offer", "Accept"] and row["food"][:2] == [2, 5]
        else:
            assert second["selected"] == action(0, 1, "Direct")
            assert row["actions"] == ["Leave"] and row["food"][:2] == [3, 4]
    global_row = core["context_specific"]["GlobalProgress"][1]
    contextual = core["context_specific"]["Contextual"][1]
    assert [(s["selected"], s["outcome"]) for s in global_row["steps"][:2]] == [(s["selected"], s["outcome"]) for s in contextual["steps"][:2]]
    assert [s["after"]["expected_change"] for s in contextual["steps"][:2]] == [27, 32]
    assert contextual["steps"][2]["selected"] == action(1, 1, "Direct")
    assert contextual["steps"][2]["outcome"]["support_after"] == 50
    assert global_row["steps"][2]["selected"] == action(0, 1, "Direct") and global_row["steps"][3]["selected"] == "Pause"
    assert core["context_specific"]["Unchanged"][1]["steps"][3]["outcome"]["support_after"] == 50
    assert all(row["actions"] == ["Leave"] for _, row in core["context_specific"].values())
    for mode in r.MODES:
        assert core["ignorance"][mode][1]["steps"] == core["withholding"][mode][1]["steps"]
        assert core["context_specific"][mode][1]["steps"] == core["mirrored_future"][mode][1]["steps"]
    for t, row in zip(trials, rows, strict=True):
        if t["case"] == "misleading" and t["mode"] == "Contextual" and t["config"]["support"] in [20, 40]:
            step = next(s for s in row["steps"] if s["outcome"] and s["outcome"]["support_after"] == -t["config"]["quality"])
            assert step["after"]["expected_change"] == 77
        if t["case"] == "later_informed" and t["config"]["support"] in [20, 40]:
            transfer = (t["config"]["support"] == 20) == (t["mode"] == "GlobalProgress")
            assert row["actions"] == (["Offer", "Accept"] if transfer else ["Leave"])
        if t["case"] == "forgetting" and t["config"]["support"] == 20 and t["mode"] == "Contextual":
            point = next(o["before"] for o in t["opportunities"] if o["phase"] == "limited")
            basis = next(b for b in point["basis"] if b["concern"] == 0)
            assert (basis["support"], basis["context"]) == (0, "ClaimFallback")
            assert any(c["context"] == "EvidencePresent" for c in point["cells"])
            step = next(s for s in row["steps"] if s["phase"] == "limited")
            assert (step["outcome"]["support_after"], step["outcome"]["novelty_value"]) == (50, 0)
    phases = collections.defaultdict(collections.Counter)
    automatic = collections.defaultdict(collections.Counter)
    totals = collections.defaultdict(collections.Counter)
    for row in rows:
        total = totals[(row["held"], row["mode"])]
        total.update(row["metrics"])
        total["trials"] += 1
        for step in row["steps"]:
            counter = phases[(row["held"], row["mode"], step["phase"])]
            counter["asks"] += step["selected"] != "Pause"
            counter["pause"] += step["selected"] == "Pause"
            counter["time"] += step["time"]
            total["asks"] += step["selected"] != "Pause"
            total["pause"] += step["selected"] == "Pause"
            total["time"] += step["time"]
            total["invalid_reply"] += step["selected"] != "Pause" and not step["valid_learning"]
        for metric in ["updates", "changed", "positive_novelty_no_change", "no_receipt"]:
            automatic[(row["held"], row["mode"])][metric] += row["history_metrics"].get(metric, 0) - row["metrics"].get(metric, 0)
    assert [dict(held=h, mode=m, **v) for (h, m), v in totals.items()] == archived["totals"]
    development = r.read(DIRECTORY, "development-summary")
    final = r.read(DIRECTORY, "summary")
    changes = collections.defaultdict(collections.Counter)
    for a, b in zip(development, final, strict=True):
        assert all(a[k] == b[k] for k in ["seed", "held", "case", "config", "mode"])
        assert (a["actions"], a["persistent_food"]) == (b["actions"], b["persistent_food"])
        if a["seed"] != 42:
            continue
        counter = changes[(a["held"], a["mode"])]
        counter["changed_rows"] += a != b
        counter["changed_queries"] += [s["selected"] for s in a["steps"]] != [s["selected"] for s in b["steps"]]
        counter["changed_actions"] += a["actions"] != b["actions"]
        counter["changed_persistent_food"] += a["persistent_food"] != b["persistent_food"]
        counter["development_untrained_asks"] += sum(s["selected"] != "Pause" and s["outcome"] is None for s in a["steps"])
        assert all(s["outcome"] is not None for s in b["steps"] if s["selected"] != "Pause")
    assert all(c["changed_actions"] == c["changed_persistent_food"] == 0 for c in changes.values())
    assert len(final) == len(development) == 1800
    for mode in r.MODES:
        pick = lambda suite, case: next(t for t in suite if t["seed"] == 42 and not t["held"] and t["case"] == case and t["mode"] == mode)
        # The material progress anchor already existed before the fallback repair.
        assert pick(development, "changed_circumstances")["steps"] == pick(final, "changed_circumstances")["steps"]
        # The uniquely contextual core example needs valid no-progress feedback:
        # in development the immutable-channel failures prevented that learning.
        old_context = pick(development, "context_specific")
        assert old_context["steps"][-1]["selected"] == action(0, 1, "Evidence")
        assert old_context["steps"][-1]["outcome"] is None
    cells = [len(v) for t in trials for v in t["final_state"]["learning"]["cells"].values()]
    assert max(cells) < 32 and not any(s["evicted"] for t in trials for s in t["final_state"]["learning"]["records"])
    print(json.dumps(dict(independent_trials=180, material_anchors=material_anchors,
        phase_counts=[dict(held=h, mode=m, phase=p, **v) for (h, m, p), v in phases.items()],
        automatic_updates=[dict(held=h, mode=m, **v) for (h, m), v in automatic.items()],
        repair_effects=[dict(held=h, mode=m, **v) for (h, m), v in changes.items()],
        maximum_observed_learning_cells=max(cells), actual_learning_evictions=0), indent=2))


if __name__ == "__main__":
    main()
