"""Independent offline 011 reconstruction; no data from here enters policies.

Default reads target evaluator outputs and writes target analysis/report.
--archive-only verifies the new 011 gzip manifest and prints results, without writes.
"""
import argparse
import collections
import copy
import gzip
import hashlib
import itertools
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ARCHIVE = ROOT / "experiments/011"
TARGET = ROOT / "target/experiment011"
MODES = ["Unchanged", "CurrentNeed", "StatusValue"]
ACTIVE = {"Open", "Partial"}


def token(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def signed_div(a, b):
    return (abs(a) // b) * (-1 if a < 0 else 1)


def origin_key(origin):
    name, value = next(iter(origin.items()))
    index = ["Claim", "Disclosure", "NativeReading", "Known", "Unknown"].index(name)
    return (index, value["speaker"], value["channel"]) if isinstance(value, dict) else (index, value)


def support(items, event):
    relevant = [i for i in items if i["event"] == event]
    evidence = any(i["quality"] > 0 and "Claim" not in i["origin"] for i in relevant)
    groups = {}
    for item in relevant:
        if evidence and "Claim" in item["origin"]:
            continue
        pair = groups.setdefault(origin_key(item["origin"]), [0, 0])
        sign = 0 if item["claim"] else 1
        pair[sign] = max(pair[sign], item["quality"])
    positive = negative = 0
    for key in sorted(groups):
        p, n = groups[key]
        positive += (100 - positive) * p // 100
        negative += (100 - negative) * n // 100
    return positive - negative


def projection(point, retained):
    return [{"concern": c["id"], "event": c["event"], "support": support(retained, c["event"])}
            for c in point["concerns"] if c["status"] in ACTIVE]


def readiness(agent, partner, amount):
    trust = agent["trust"].get(str(partner), 0)
    memories = [m for m in agent["memories"] if m["partner"] == partner][-4:]
    valence = max(-40, min(40, sum(m["valence"] for m in memories)))
    scores = {}
    if agent["food"] >= amount:
        scores["Offer"] = agent["generosity"] - agent["hunger"] // 2 - agent["caution"] // 2 + trust + valence
    scores["Request"] = agent["hunger"] - 35 + signed_div(trust, 4) - agent["caution"] // 4
    scores["Leave"] = 0
    return max(scores, key=scores.get)


def check_candidates(decision, context, basis, mode):
    own = decision["input"]
    best, score = "Pause", 0
    for c in decision["candidates"]:
        if c["action"] == "Pause":
            assert c["score"] == c["benefit"] == c["cost"] == 0
            continue
        ask = c["action"]["Ask"]
        concern = next(x for x in own["concerns"] if x["id"] == ask["concern"])
        source = next(x for x in own["sources"] if x["id"] == ask["source"])
        assert concern["owner"] == own["owner"] and concern["status"] in ACTIVE
        assert concern["created_scene"] < own["scene"] and source["id"] != own["owner"]
        cell = next((x for x in own["cells"] if x["concern"] == concern["id"] and x["source"] == source["id"]
                     and x["strategy"] == ask["strategy"]), None)
        prior = 55 if ask["strategy"] == "Direct" else 65
        learned = cell["expected"] if cell and own["settings"]["use_history"] else prior
        offer = next((x for x in reversed(source["offers"]) if x["event"] == concern["event"]), None)
        opportunity = 0
        if offer and ask["strategy"] == "Evidence":
            origin = {"Evidence": {"subject": concern["target"], "channel": offer["channel"]}}
            seen = any(x["event"] == concern["event"] and x["origin"] == origin and x["quality"] >= offer["quality"] for x in own["seen"])
            attempted = cell and cell["attempted_offer"] == [offer["channel"], offer["quality"]]
            if not seen and not attempted:
                strongest = max((x["quality"] for x in own["seen"] if x["event"] == concern["event"]), default=0)
                opportunity = max(0, min(90, offer["quality"] - strongest + 30))
        gain = max(0, min(100, max(learned, opportunity) + signed_div(source["credibility"], 10)))
        hint = next((h for h in reversed(context["hints"]) if h["source"] == source["id"] and h["event"] == concern["event"]), None)
        known = hint["known"] if hint else None
        if context["settings"]["provenance"] and known is not None and any(k["event"] == concern["event"] and k["known"] == known for k in context["knowledge"]):
            gain = 0
        cost = (10 if ask["strategy"] == "Direct" else 28) + max(0, own["hunger"]) // 10
        if offer and ask["strategy"] == "Evidence":
            cost += offer["effort"]
        assert c["importance"] == concern["importance"]
        assert (c["prior_or_learned"], c["opportunity_value"], c["expected_gain"], c["cost"]) == (learned, opportunity, gain, cost)
        current = next(x["support"] for x in basis if x["concern"] == concern["id"])
        benefit = concern["importance"] * gain // 100
        if mode == "CurrentNeed":
            benefit = benefit * (100 - abs(current)) // 100
        elif mode == "StatusValue":
            leverage = min(100, gain * 100 // max(1, 60 - abs(current)))
            benefit = concern["importance"] * ((gain + leverage) // 2) // 100
        assert (c["benefit"], c["score"]) == (benefit, benefit - cost)
        if c["score"] > score:
            best, score = c["action"], c["score"]
    assert decision["selected"] == best


def reconstruct(trial, row):
    snapshot = trial["final_state"]
    assessment = snapshot["base"]["base"]["assessment"]
    compact = snapshot["base"]["base"]["base"]
    inquiry = compact["base"]["inquiry"]["records"]
    events = compact["base"]["base"]["events"]
    information = compact["base"]["base"]["cognition"]["information"]
    retained, prefixes = {}, [{}]
    for record in assessment["records"]:
        live = retained.setdefault(record["owner"], [])
        incoming = copy.deepcopy(record["incoming"])
        # 011 uses native receipts only; hidden origins are never substituted here.
        assert "Native" in incoming["receipt"] and not record["revised_attribution"]
        live.append(incoming)
        evicted = live.pop(0)["receipt"] if len(live) > 32 else None
        assert record["evicted"] == evicted
        assert record["retained"] == [i["receipt"] for i in live]
        assert record["support"] == support(live, incoming["event"])
        prefixes.append(copy.deepcopy(retained))
    assert snapshot["base"]["retention"]["method"] == "Fifo"
    for capture in snapshot["value"]["records"]:
        query = compact["provenance"]["queries"][capture["query"]]
        assert query["inquiry"] == capture["inquiry"]
        decision = inquiry[capture["inquiry"]]["decision"]
        boundary = capture["assessment_end"]
        ticks = [information[r["incoming"]["receipt"]["Native"]]["tick"] for r in assessment["records"]]
        assert all(t <= query["observed_at"] for t in ticks[:boundary])
        assert boundary == len(ticks) or ticks[boundary] > query["observed_at"]
        own = prefixes[boundary].get(decision["input"]["owner"], [])
        basis = projection({"concerns": decision["input"]["concerns"]}, own)
        assert basis == capture["basis"] and len(basis) <= 8
        check_candidates(decision, compact["contexts"]["values"][query["context"]], basis, trial["mode"])
    assert len(snapshot["value"]["records"]) == len(compact["provenance"]["queries"]) - snapshot["value"]["first_query"]
    points = [trial["before_prelude"], trial["after_action"], trial["persistent"]]
    for opportunity, step in zip(trial["opportunities"], row["steps"]):
        points.extend([opportunity["before"], opportunity["after"]])
        r = next(r for r in inquiry[opportunity["before"]["inquiry_end"]:opportunity["after"]["inquiry_end"]] if r["decision"]["input"]["owner"] == 0)
        assert (step["inquiry"], step["selected"], step["candidates"], step["response"], step["cell_after"]) == (r["id"], r["decision"]["selected"], r["decision"]["candidates"], r["response"], r["cell_after"])
        for point, key in [(opportunity["before"], "readiness_before"), (opportunity["after"], "readiness_after")]:
            assert step[key]["selected"] == readiness(point["agents"][0], trial["resource_partner"], trial["config"]["amount"])
    for point in points:
        assert point["basis"] == projection(point, prefixes[point["assessment_end"]].get(0, []))
    for event in events:
        assert sum(event["balances_before"]) == sum(event["balances_after"])
        if event["decision"]["selected"] == "Offer":
            assert event["decision"]["food"] >= event["decision"]["observation"]["amount"]
        if event["decision"]["selected"] == "Accept":
            assert event["decision"]["observation"]["last_signal"]["action"] == "Offer"
            assert event["transferred"] == event["decision"]["observation"]["amount"]
    resource = events[trial["opportunities"][-1]["after"]["event_end"]:]
    assert row["actions"] == [e["decision"]["selected"] for e in resource]
    assert row["persistent_food"] == [a["food"] for a in trial["persistent"]["agents"]]
    before, after = trial["after_action"]["agents"], trial["persistent"]["agents"]
    assert sum(a["food"] for a in before) - trial["consumption"]["consumed"] == sum(a["food"] for a in after)
    assert before[1:] == after[1:]


def category(a, b):
    query_diff = [s["selected"] for s in a["steps"]] != [s["selected"] for s in b["steps"]]
    action_diff = a["actions"] != b["actions"]
    persistent = a["persistent_food"] != b["persistent_food"]
    if not action_diff:
        return "C" if query_diff else "Equivalent"
    if not query_diff:
        return "E_A" if persistent else "F"
    pre = a["pre_readiness"]["selected"] == b["pre_readiness"]["selected"]
    post = a["post_readiness"]["selected"] != b["post_readiness"]["selected"]
    executed = all(r["actions"][0] == r["post_readiness"]["selected"] for r in [a, b])
    return "E_Q" if pre and post and executed and persistent else "F"


def measures(row):
    queries = [s["selected"]["Ask"] for s in row["steps"] if s["selected"] != "Pause"]
    before = row["steps"][0]["concerns_before"]
    after = row["steps"][-1]["concerns_after"]
    def retained(s, name):
        return {b["event"]: b["support"] for b in s[name]}
    changed = 0
    for s in row["steps"]:
        if s["selected"] == "Pause":
            continue
        c = next(c for c in s["concerns_before"] if c["id"] == s["selected"]["Ask"]["concern"])
        old, new = retained(s, "basis_before"), retained(s, "basis_after")
        status = next(c2["status"] for c2 in s["concerns_after"] if c2["id"] == c["id"])
        changed += old.get(c["event"], 0) != new.get(c["event"], 0) or status != c["status"]
    resolved = sum(c["status"] in ACTIVE and next(x["status"] for x in after if x["id"] == c["id"]) == "Resolved" for c in before)
    return {"asks": len(queries), "pause": len(row["steps"]) - len(queries), "changed": changed,
            "unproductive": len(queries) - changed, "distinct_concerns": len({q["concern"] for q in queries}),
            "repeated_concern": len(queries) - len({q["concern"] for q in queries}),
            "resolved": resolved, "unresolved": row["unresolved_before_resource"],
            "time": sum(s["time"] for s in row["steps"]), "transfers": int(row["transferred"] > 0)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archive-only", action="store_true")
    args = parser.parse_args()
    def read(name):
        if not args.archive_only:
            return json.loads((TARGET / f"{name}.json").read_bytes())
        manifest = json.loads((ARCHIVE / "archive-manifest.json").read_text())
        compressed = (ARCHIVE / f"{name}.json.gz").read_bytes()
        raw = gzip.decompress(compressed)
        for data, prefix in [(raw, "json"), (compressed, "gzip")]:
            assert len(data) == manifest[name][f"{prefix}_bytes"]
            assert hashlib.sha256(data).hexdigest() == manifest[name][f"{prefix}_sha256"]
        return json.loads(raw)
    rows = read("summary")
    seed42 = [r for r in rows if r["seed"] == 42]
    lookup = {(r["held"], r["case"], token(r["config"]), r["mode"]): r for r in seed42}
    for held, name in [(False, "seed-42"), (True, "held-out-seed-42")]:
        trials = read(name)
        for t in trials:
            reconstruct(t, lookup[(held, t["case"], token(t["config"]), t["mode"])])
        groups = collections.defaultdict(list)
        for t in trials:
            groups[(t["case"], token(t["config"]))].append(t)
        for (case, config), group in groups.items():
            for t in group:
                assert t["before_prelude"] == group[0]["before_prelude"]
                assert t["opportunities"][0]["before"] == group[0]["opportunities"][0]["before"]
            if case == "uncertain_minor":
                pair = groups[("mirrored_future", config)]
                assert all(a["opportunities"] == b["opportunities"] and a["resource_partner"] != b["resource_partner"] for a, b in zip(group, pair))
    comparisons = []
    for held in [False, True]:
        groups = collections.defaultdict(dict)
        for r in seed42:
            if r["held"] == held:
                groups[(r["case"], token(r["config"]))][r["mode"]] = r
        for (case, config), group in groups.items():
            for a, b in itertools.combinations(MODES, 2):
                comparisons.append({"held": held, "case": case, "config": json.loads(config), "modes": [a, b],
                                    "class": category(group[a], group[b]), "measures": [measures(group[a]), measures(group[b])]})
    totals = []
    for held in [False, True]:
        for mode in MODES:
            selected = [r for r in seed42 if r["held"] == held and r["mode"] == mode]
            values = [measures(r) for r in selected]
            totals.append({"held": held, "mode": mode, "trials": len(selected), **{k: sum(v[k] for v in values) for k in values[0]}})
    for mode in MODES:
        assert any(measures(group)["resolved"] < max(measures(r)["resolved"] for r in seed42 if r["held"] == group["held"] and r["case"] == group["case"] and r["config"] == group["config"])
                   for group in seed42 if group["mode"] == mode), f"missing worse allocation: {mode}"
    signature = lambda r: token({k: r[k] for k in ["actions", "persistent_food", "unresolved_before_resource"]} | {"queries": [s["selected"] for s in r["steps"]]})
    variants = collections.defaultdict(set)
    for r in rows:
        variants[(r["held"], r["case"], token(r["config"]), r["mode"])].add(signature(r))
    analysis = {"trials": len(rows), "seeds": len({r["seed"] for r in rows}), "independently_reconstructed": len(seed42),
                "measured_seed_invariant": all(len(v) == 1 for v in variants.values()), "totals": totals,
                "class_counts": dict(collections.Counter(f"{r['held']}:{','.join(r['modes'])}:{r['class']}" for r in comparisons)),
                "comparisons": comparisons}
    if args.archive_only:
        print(json.dumps({k: v for k, v in analysis.items() if k != "comparisons"}, indent=2))
    else:
        (TARGET / "analysis.json").write_text(json.dumps(analysis, indent=2) + "\n", encoding="utf-8")
        lines = ["011 seed42 sequential allocations. A/B/C = concern0/1/2; E/D = method. Transfer is not global utility."]
        for r in seed42:
            actions = []
            for s in r["steps"]:
                q = s["selected"]
                actions.append("Pause" if q == "Pause" else f"{'ABC'[q['Ask']['concern']]}/source{q['Ask']['source']}/{q['Ask']['strategy']}")
            lines.append(f"held={r['held']} {r['case']} {r['mode']} {token(r['config'])}: {' -> '.join(actions)}; {r['actions']}; food={r['persistent_food']}; {measures(r)}")
        (TARGET / "report.txt").write_text("\n".join(lines) + "\n", encoding="utf-8")
        print(json.dumps({k: v for k, v in analysis.items() if k != "comparisons"}, indent=2))


if __name__ == "__main__":
    main()
