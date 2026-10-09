"""Read-only reconstruction; --archive-only needs no generated target files.

Prints review evidence to stdout. Does not rewrite experiment archives or policies.
Classification distinguishes inquiry mediation from same-inquiry assimilation.
Without --archive-only, run evaluate010 first to compare generated target outputs.
"""
import collections
import argparse
import copy
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ARCHIVE = ROOT / "experiments/010"
manifest = json.loads((ARCHIVE / "archive-manifest.json").read_text())
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--archive-only", action="store_true")
archive_only = parser.parse_args().archive_only


def frozen(name):
    compressed = (ARCHIVE / f"{name}.json.gz").read_bytes()
    raw = gzip.decompress(compressed)
    expected = manifest[name]
    for data, prefix in ((compressed, "gzip"), (raw, "json")):
        assert len(data) == expected[f"{prefix}_bytes"]
        assert hashlib.sha256(data).hexdigest() == expected[f"{prefix}_sha256"]
    if not archive_only:
        assert raw == (ROOT / f"target/experiment010/{name}.json").read_bytes(), name
    return json.loads(raw)


def token(value):
    return json.dumps(value, sort_keys=True)


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


def parts(trial):
    assessment = trial["final_state"]["base"]["base"]["assessment"]
    compact = trial["final_state"]["base"]["base"]["base"]
    return assessment, compact["base"]["inquiry"], compact["base"]["base"]


def resource_readiness(agent, partner, amount):
    trust = agent["trust"].get(str(partner), 0)
    episodes = [m for m in agent["memories"] if m["partner"] == partner]
    valence = max(-40, min(40, sum(m["valence"] for m in episodes[-4:])))
    scores = {}
    if agent["food"] >= amount:
        scores["Offer"] = agent["generosity"] - agent["hunger"] // 2 - agent["caution"] // 2 + trust + valence
        if agent["hunger"] >= 60 and agent["food"] <= amount:
            scores["Offer"] -= 60
    scores["Request"] = agent["hunger"] - 35 + int(trust / 4) - agent["caution"] // 4
    scores["Leave"] = 0
    return max(scores, key=scores.get)


def reconstruct(trial):
    assessment, inquiry, base = parts(trial)
    retained = {}
    prefixes = [{}]
    for record in assessment["records"]:
        owner = record["owner"]
        live = retained.setdefault(owner, [])
        incoming = copy.deepcopy(record["incoming"])
        for item in live:
            if token(item["receipt"]) in {token(r) for r in record["revised_attribution"]}:
                item["origin"] = incoming["origin"]
        live.append(incoming)
        keep = {token(r) for r in record["retained"]}
        live[:] = [i for i in live if token(i["receipt"]) in keep]
        assert len(live) <= 32
        assert support(live, incoming["event"]) == record["support"]
        prefixes.append(copy.deepcopy(retained))
    for point in trial["points"]:
        local = prefixes[point["assessment_end"]].get(0, [])
        expected = [{"concern": c["id"], "event": c["event"], "support": support(local, c["event"])}
                    for c in point["concerns"] if c["status"] in ("Open", "Partial")]
        assert point["basis"] == expected
    for attention in trial["final_state"]["attention"]["records"]:
        record = inquiry["records"][attention["inquiry"]]
        decision = record["decision"]
        owner = decision["input"]["owner"]
        local = prefixes[attention["assessment_end"]].get(owner, [])
        projected = [{"concern": c["id"], "event": c["event"], "support": support(local, c["event"])}
                     for c in decision["input"]["concerns"] if c["status"] in ("Open", "Partial")]
        assert projected == attention["basis"] and len(projected) <= 8
        best, best_score = "Pause", 0
        for candidate in decision["candidates"]:
            if candidate["action"] != "Pause":
                benefit = candidate["importance"] * candidate["expected_gain"] // 100
                if trial["mode"] == "CurrentNeed":
                    concern = candidate["action"]["Ask"]["concern"]
                    current = next(b["support"] for b in projected if b["concern"] == concern)
                    benefit = benefit * (100 - abs(current)) // 100
                assert candidate["benefit"] == benefit
                assert candidate["score"] == benefit - candidate["cost"]
            if candidate["score"] > best_score:
                best, best_score = candidate["action"], candidate["score"]
        assert decision["selected"] == best
    for event in base["events"]:
        assert sum(event["balances_before"]) == sum(event["balances_after"])
        action, view = event["decision"]["selected"], event["decision"]["observation"]
        if action == "Offer":
            assert event["decision"]["food"] >= view["amount"]
        if action == "Accept":
            assert view["last_signal"]["action"] == "Offer"
            assert event["transferred"] == view["amount"]
    before, after = trial["points"][6:8]
    assert sum(a["food"] for a in before["agents"]) - trial["consumption"]["consumed"] == sum(a["food"] for a in after["agents"])
    assert [a["food"] for a in before["agents"][1:]] == [a["food"] for a in after["agents"][1:]]
    first = next(r for r in inquiry["records"][trial["points"][3]["inquiry_end"]:trial["points"][4]["inquiry_end"]]
                 if r["decision"]["input"]["owner"] == 0)
    return first


rows = frozen("summary")
summary = {(r["held"], r["case"], r["mode"], token(r["config"]), r["retention"]): r for r in rows if r["seed"] == 42}
findings = []
for held, name in ((False, "seed-42"), (True, "held-out-seed-42")):
    trials = frozen(name)
    groups = collections.defaultdict(list)
    for trial in trials:
        reconstruct(trial)
        groups[(trial["case"], trial["mode"], token(trial["config"]))].append(trial)
    for (case, mode, config), group in groups.items():
        if case == "protected":
            mirrored = groups[("importance", mode, config)]
            for a, b in zip(group, mirrored):
                assert reconstruct(a)["decision"] == reconstruct(b)["decision"]
                assert a["resource_partner"] != b["resource_partner"]
    for (case, mode, config), group in groups.items():
        assert len(group) == 3
        first = group[0]
        assessment, _, _ = parts(first)
        records = assessment["records"]
        deliveries = [r["incoming"] for r in records[:first["points"][3]["assessment_end"]]]
        for trial in group:
            assessment, _, _ = parts(trial)
            records = assessment["records"]
            assert trial["points"][0] == first["points"][0]
            assert [r["incoming"] for r in records[:trial["points"][3]["assessment_end"]]] == deliveries
            assert trial["config"] == first["config"] and trial["resource_partner"] == first["resource_partner"]
        measured = [summary[(held, case, mode, config, t["method"])] for t in group]
        for trial, row in zip(group, measured):
            inquiry_record = reconstruct(trial)
            _, _, base = parts(trial)
            assert row["inquiry"] == inquiry_record["decision"]["selected"]
            assert row["actions"] == [e["decision"]["selected"] for e in base["events"][trial["points"][5]["event_end"]:]]
            assert row["persistent_food"] == [a["food"] for a in trial["points"][7]["agents"]]
            for stage, probe in zip((3, 4, 5), row["diagnostic_probes"]):
                assert probe["selected"] == resource_readiness(trial["points"][stage]["agents"][0], trial["resource_partner"], trial["config"]["amount"])
        differs = lambda field: len({token(r[field]) for r in measured}) > 1
        inquiry, action, persistent = (differs(k) for k in ("inquiry", "actions", "persistent_food"))
        pre = {r["diagnostic_probes"][0]["selected"] for r in measured}
        post = {r["diagnostic_probes"][1]["selected"] for r in measured}
        mediated = (case not in ("hidden", "late_attribution") and inquiry and action and persistent
                    and len(pre) == 1 and len(post) > 1
                    and all(r["diagnostic_probes"][1]["selected"] == r["actions"][0] for r in measured))
        state = len({token([(p["concerns"], p["agents"]) for p in t["points"][3:]]) for t in group}) > 1
        basis = any(len({token(t["points"][stage]["basis"]) for t in group}) > 1 for stage in (2, 3))
        if mediated:
            category, route = "E", "full inquiry chain"
        elif action and (inquiry or len(pre) > 1 or case in ("hidden", "late_attribution")):
            category, route = "F", "mediation not isolated"
        elif action and persistent:
            category, route = "E", "same inquiry; different assimilation (not an allocation chain)"
        elif inquiry:
            category, route = "C", "inquiry only"
        elif state:
            category, route = "B", "derived state only"
        elif basis:
            category, route = "A", "retained basis only"
        else:
            category, route = "Control", "equivalent measured outcome"
        methods = {}
        for trial, row in zip(group, measured):
            methods[trial["method"]] = {
                "basis_before": row["basis"], "inquiry": row["inquiry"], "actions": row["actions"],
                "persistent_food": row["persistent_food"][:3],
                "concerns_before": [[c["id"], c["status"]] for c in trial["points"][3]["concerns"]],
                "diagnostic_actions": [p["selected"] for p in row["diagnostic_probes"]],
            }
        findings.append({"held": held, "case": case, "mode": mode, "config": json.loads(config),
                         "class": category, "route": route, "full_inquiry_chain": mediated,
                         "causal_class": "E_Q" if mediated else "E_A" if category == "E" else category,
                         "inquiry_differs": inquiry, "action_differs": action, "persistent_differs": persistent,
                         "methods": methods})

counts = collections.Counter((f["held"], f["mode"]) for f in findings if f["full_inquiry_chain"])
assert counts == {(False, "CurrentNeed"): 4, (True, "CurrentNeed"): 12, (True, "Unchanged"): 1}
unchanged = [f for f in findings if f["mode"] == "Unchanged" and f["full_inquiry_chain"]]
assert len(unchanged) == 1 and unchanged[0]["case"] == "redelivery"
assert unchanged[0]["config"]["goal"] == 80 and unchanged[0]["config"]["weak"] == 55
assert unchanged[0]["config"]["count"] == 48
for f in findings:
    if not f["held"] and f["case"] == "conflict":
        assert f["causal_class"] == "E_A" and not f["full_inquiry_chain"] and f["route"].startswith("same inquiry")
    if not f["held"] and f["case"] in ("hidden", "late_attribution"):
        assert f["causal_class"] == "F" and not f["full_inquiry_chain"]
print(json.dumps({"archive_integrity_verified": True,
                  "archive_and_target_bytes_match": None if archive_only else True, "archived_trial_rows": len(rows),
                  "seed42_groups_independently_reconstructed": len(findings), "findings": findings}, indent=2))
