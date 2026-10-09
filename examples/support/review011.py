"""Read-only scientific reconstruction of frozen 011; writes only stdout.

No runtime, evaluator, or prior analyzer scoring functions are imported. Optional
--replay-dir compares all three raw evaluator files exactly with the archives.
Assertions require ordinary Python (do not use -O). Not a foundation freeze gate.
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
MODES = ("Unchanged", "CurrentNeed", "StatusValue")
ACTIVE = {"Open", "Partial"}


def key(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def trunc(value, divisor):
    return (abs(value) // divisor) * (-1 if value < 0 else 1)


def origin_key(origin):
    name, payload = next(iter(origin.items()))
    order = ("Claim", "Disclosure", "NativeReading", "Known", "Unknown").index(name)
    return (order, payload["speaker"], payload["channel"]) if isinstance(payload, dict) else (order, payload)


def grouped(items, event):
    items = [i for i in items if i["event"] == event]
    has_reading = any(i["quality"] > 0 and "Claim" not in i["origin"] for i in items)
    groups = {}
    for item in items:
        if has_reading and "Claim" in item["origin"]:
            continue
        signs = groups.setdefault(origin_key(item["origin"]), [0, 0])
        side = 0 if item["claim"] else 1
        signs[side] = max(signs[side], item["quality"])
    totals = [0, 0]
    for origin in sorted(groups):
        for side in (0, 1):
            totals[side] += (100 - totals[side]) * groups[origin][side] // 100
    return totals[0] - totals[1]


def novelty(seen, incoming):
    if not incoming["quality"]:
        return "None", 0
    prior = [s for s in seen if s["event"] == incoming["event"]]
    if any(s["origin"] == incoming["origin"] and s["claim"] == incoming["claim"]
           and s["quality"] >= incoming["quality"] for s in prior):
        return "Redundant", 0
    if any(s["claim"] != incoming["claim"] for s in prior):
        return "Conflict", min(30, incoming["quality"])
    same = [s["quality"] for s in prior if s["claim"] == incoming["claim"]]
    if same and incoming["quality"] > max(same):
        return "Stronger", min(90, incoming["quality"] - max(same) + 20)
    return "New", min(incoming["quality"], 20 if same else 40)


def scores(record, context, support, mode):
    own = record["decision"]["input"]
    assert len(own["concerns"]) <= 8 and len(own["sources"]) <= 7
    assert len(own["cells"]) <= 32 and len(own["seen"]) <= 32
    assert len(context["knowledge"]) <= 32 and len(context["hints"]) <= 32
    actions = ["Pause"]
    for concern in own["concerns"]:
        for source in own["sources"]:
            if concern["owner"] != own["owner"] or concern["status"] not in ACTIVE \
                    or concern["created_scene"] >= own["scene"] or source["id"] == own["owner"]:
                continue
            for method in ("Direct", "Evidence"):
                actions.append({"Ask": {"concern": concern["id"], "source": source["id"], "strategy": method}})
    assert actions == [c["action"] for c in record["decision"]["candidates"]]
    best, highest = "Pause", 0
    components = []
    for candidate in record["decision"]["candidates"]:
        if candidate["action"] == "Pause":
            assert candidate["score"] == candidate["benefit"] == candidate["cost"] == 0
            continue
        ask = candidate["action"]["Ask"]
        concern = next(c for c in own["concerns"] if c["id"] == ask["concern"])
        source = next(s for s in own["sources"] if s["id"] == ask["source"])
        cell = next((c for c in own["cells"] if all(c[k] == ask[k] for k in ask)), None)
        prior = 55 if ask["strategy"] == "Direct" else 65
        expected = cell["expected"] if cell and own["settings"]["use_history"] else prior
        offer = next((o for o in reversed(source["offers"]) if o["event"] == concern["event"]), None)
        opportunity = 0
        if offer and ask["strategy"] == "Evidence":
            received = any(s["event"] == concern["event"] and s["origin"] == {
                "Evidence": {"subject": concern["target"], "channel": offer["channel"]}}
                and s["quality"] >= offer["quality"] for s in own["seen"])
            tried = cell and cell["attempted_offer"] == [offer["channel"], offer["quality"]]
            if not received and not tried:
                strongest = max((s["quality"] for s in own["seen"] if s["event"] == concern["event"]), default=0)
                opportunity = max(0, min(90, offer["quality"] - strongest + 30))
        gain = max(0, min(100, max(expected, opportunity) + trunc(source["credibility"], 10)))
        hint = next((h for h in reversed(context["hints"])
                     if h["source"] == source["id"] and h["event"] == concern["event"]), None)
        if context["settings"]["provenance"] and hint and hint["known"] is not None and any(
                k["event"] == concern["event"] and k["known"] == hint["known"] for k in context["knowledge"]):
            gain = 0
        cost = (10 if ask["strategy"] == "Direct" else 28) + max(0, own["hunger"]) // 10
        if offer and ask["strategy"] == "Evidence":
            cost += offer["effort"]
        importance = concern["importance"]
        benefit = importance * gain // 100
        current = support[concern["event"]]
        leverage = min(100, gain * 100 // max(1, 60 - abs(current)))
        if mode == "CurrentNeed":
            benefit = benefit * (100 - abs(current)) // 100
        elif mode == "StatusValue":
            benefit = importance * ((gain + leverage) // 2) // 100
        assert (candidate["importance"], candidate["prior_or_learned"], candidate["opportunity_value"],
                candidate["expected_gain"], candidate["cost"], candidate["benefit"], candidate["score"]) == (
                    importance, expected, opportunity, gain, cost, benefit, benefit - cost)
        assert candidate["attempts"] == (cell["attempts"] if cell else 0)
        assert candidate["history_record"] == (cell["last_record"] if cell and own["settings"]["use_history"] else None)
        assert candidate["notice"] == (offer["notice"] if offer else None)
        components.append({"action": candidate["action"], "gain": gain, "leverage": leverage, "score": benefit - cost})
        if benefit - cost > highest:
            best, highest = candidate["action"], benefit - cost
    assert best == record["decision"]["selected"]
    return components


def learning(record, compact, information):
    own = record["decision"]["input"]
    assert record["valid"] and record["food_before"] == record["food_after"]
    selected = record["decision"]["selected"]
    if selected == "Pause":
        assert record["time_spent"] == 1 and record["response"] is None and record["cell_after"] is None
        return
    ask = selected["Ask"]
    concern = next(c for c in own["concerns"] if c["id"] == ask["concern"])
    response = record["response"]
    values, category, value = [], "None", 0
    if response:
        assert response["valid"] and response["failure"] is None
        assert response["decision"]["input"]["own"]["id"] == ask["source"]
        assert response["decision"]["input"]["partner"] == own["owner"]
        assert response["decision"]["selected"] == max(
            response["decision"]["candidates"], key=lambda c: c["score"])["action"]
        seen = copy.deepcopy(own["seen"])
        assert response["receipts"] == [s["receipt"] for s in response["signatures"]]
        for signature in response["signatures"]:
            receipt = information[signature["receipt"]]
            assert record["tick"] - record["time_spent"] < receipt["tick"] <= record["tick"]
            assert (receipt["listener"], receipt["speaker"], receipt["event"], receipt["scarce"],
                    receipt["reliability"]) == (own["owner"], ask["source"], concern["event"],
                                                signature["claim"], signature["quality"])
            kind, useful = novelty(seen, signature)
            if kind == "Conflict" or (category != "Conflict" and useful >= value):
                category = kind
            value = max(value, useful)
            values.append([signature["receipt"], kind, useful])
            seen.append(signature)
        source = next(s for s in own["sources"] if s["id"] == ask["source"])
        offer = next((o for o in reversed(source["offers"]) if o["event"] == concern["event"]), None)
        effort = offer["effort"] if offer and response["public_action"] == "ProvideEvidence" else 0
        assert response["time_spent"] == 1 + sum(information[i]["turns"] for i in response["receipts"]) + effort
        response_time = response["time_spent"]
    else:
        # These fixtures contain no relay acquisitions: an unknowing source is silent.
        exchanges = [e for e in compact["provenance"]["exchanges"]
                     if record["tick"] - record["time_spent"] < e["tick"] <= record["tick"]]
        assert len(exchanges) == 1
        exchange = exchanges[0]
        assert exchange["valid"] and exchange["receipt"] is None and exchange["time_spent"] == 1
        assert exchange["decision"]["input"]["acquired"] is None
        assert exchange["decision"]["input"]["partner"] == own["owner"]
        assert exchange["decision"]["input"]["own"]["id"] == ask["source"]
        assert exchange["decision"]["input"]["event"] == concern["event"]
        response_time = 1
    assert (record["novelty"], record["realized_value"], record["information_values"]) == (category, value, values)
    assert record["time_spent"] == 1 + (10 if ask["strategy"] == "Direct" else 28) + response_time
    before = next((c for c in own["cells"] if all(c[k] == ask[k] for k in ask)), None)
    assert before == record["cell_before"]
    source = next(s for s in own["sources"] if s["id"] == ask["source"])
    offer = next((o for o in reversed(source["offers"]) if o["event"] == concern["event"]), None)
    attempted = [offer["channel"], offer["quality"]] if offer and ask["strategy"] == "Evidence" else None
    assert record["cell_after"] == dict(ask, attempts=before["attempts"] + 1 if before else 1,
        expected=((before["expected"] if before else (55 if ask["strategy"] == "Direct" else 65)) + value) // 2,
        last_record=record["id"], attempted_offer=attempted or (before["attempted_offer"] if before else None))
    assert record["concern_before"] == concern
    assert record["concern_after"]["attempts"] == concern["attempts"] + 1
    assert record["concern_after"]["failures"] == concern["failures"] + int(value == 0)


def readiness(agent, partner, amount):
    trust = agent["trust"].get(str(partner), 0)
    valence = max(-40, min(40, sum(m["valence"] for m in
                                 [m for m in agent["memories"] if m["partner"] == partner][-4:])))
    candidates = []
    if agent["food"] >= amount:
        candidates.append(("Offer", agent["generosity"] - agent["hunger"] // 2 - agent["caution"] // 2 + trust + valence))
    candidates.extend([("Request", agent["hunger"] - 35 + trunc(trust, 4) - agent["caution"] // 4), ("Leave", 0)])
    return max(candidates, key=lambda c: c[1])[0]


def reconstruct(trial):
    snapshot = trial["final_state"]
    compact = snapshot["base"]["base"]["base"]
    assessment = snapshot["base"]["base"]["assessment"]
    information = compact["base"]["base"]["cognition"]["information"]
    inquiry = compact["base"]["inquiry"]["records"]
    assert snapshot["base"]["retention"]["method"] == "Fifo" and assessment["method"] == "Grouped"
    live, prefixes = {}, [{}]
    for record in assessment["records"]:
        items = live.setdefault(record["owner"], [])
        assert record["revised_attribution"] == [] and "Native" in record["incoming"]["receipt"]
        assert record["basis_before"] == grouped(items, record["incoming"]["event"])
        items.append(record["incoming"])
        evicted = items.pop(0)["receipt"] if len(items) > 32 else None
        assert record["evicted"] == evicted and record["retained"] == [i["receipt"] for i in items]
        assert record["support"] == grouped(items, record["incoming"]["event"])
        prefixes.append(copy.deepcopy(live))
    for r in inquiry:
        learning(r, compact, information)
    components_by_inquiry = {}
    for capture in snapshot["value"]["records"]:
        query = compact["provenance"]["queries"][capture["query"]]
        assert query["inquiry"] == capture["inquiry"]
        record = inquiry[capture["inquiry"]]
        assert query["observed_at"] == record["tick"] - record["time_spent"]
        assert query["completed_at"] == record["tick"]
        boundary = capture["assessment_end"]
        times = [information[r["incoming"]["receipt"]["Native"]]["tick"] for r in assessment["records"]]
        assert all(t <= query["observed_at"] for t in times[:boundary])
        assert boundary == len(times) or times[boundary] > query["observed_at"]
        own = prefixes[boundary].get(record["decision"]["input"]["owner"], [])
        concerns = record["decision"]["input"]["concerns"]
        support = {c["event"]: grouped(own, c["event"]) for c in concerns}
        assert capture["basis"] == [dict(concern=c["id"], event=c["event"], support=support[c["event"]])
                                    for c in concerns if c["status"] in ACTIVE]
        assert len(capture["basis"]) <= 8
        context = compact["contexts"]["values"][query["context"]]
        components_by_inquiry[record["id"]] = scores(record, context, support, trial["mode"])
    steps = []
    for slot, opportunity in enumerate(trial["opportunities"]):
        before, after = opportunity["before"], opportunity["after"]
        assert opportunity["remaining"] == len(trial["opportunities"]) - slot - 1
        record = next(r for r in inquiry[before["inquiry_end"]:after["inquiry_end"]] if r["decision"]["input"]["owner"] == 0)
        selected = record["decision"]["selected"]
        step = {"inquiry": record["id"], "selected": selected, "time": record["time_spent"]}
        if selected != "Pause":
            ask = selected["Ask"]
            old = next(c for c in before["concerns"] if c["id"] == ask["concern"])
            new = next(c for c in after["concerns"] if c["id"] == ask["concern"])
            a = prefixes[before["assessment_end"]].get(0, [])
            b = prefixes[after["assessment_end"]].get(0, [])
            step.update(support_before=grouped(a, old["event"]), support_after=grouped(b, old["event"]),
                        status_before=old["status"], status_after=new["status"],
                        novelty=record["realized_value"], new_receipts=len(record["information_values"]),
                        cell_changed=record["cell_before"] != record["cell_after"],
                        expectation_changed=record["cell_after"]["expected"] != (
                            record["cell_before"]["expected"] if record["cell_before"] else
                            (55 if ask["strategy"] == "Direct" else 65)),
                        winner=next(c for c in components_by_inquiry[record["id"]] if c["action"] == selected))
            step["changed"] = step["support_before"] != step["support_after"] or old["status"] != new["status"]
            step["positive_novelty_without_change"] = bool(record["realized_value"] and not step["changed"])
            step["suppressed_claim"] = bool(record["response"] and record["response"]["signatures"]
                and all("Claim" in s["origin"] for s in record["response"]["signatures"])
                and any(i["event"] == old["event"] and i["quality"] > 0 and "Claim" not in i["origin"] for i in b)
                and not step["changed"])
            step["positive_top_tie"] = sum(c["score"] == step["winner"]["score"] for c in components_by_inquiry[record["id"]]) > 1
            if record["information_values"]:
                assert new["uncertainty"] == 100 - abs(step["support_after"])
                assert new["status"] == ("Resolved" if abs(step["support_after"]) >= 60 else "Partial")
        steps.append(step)
    events = compact["base"]["base"]["events"]
    for event in events:
        assert sum(event["balances_before"]) == sum(event["balances_after"])
        assert event["transferred"] == (event["decision"]["observation"]["amount"] if event["decision"]["selected"] == "Accept" else 0)
        expected = event["balances_before"].copy()
        if event["transferred"]:
            receiver = event["participants"].index(event["decision"]["actor"])
            donor = event["participants"].index(event["decision"]["observation"]["last_signal"]["actor"])
            assert donor != receiver and expected[donor] >= event["transferred"]
            expected[donor] -= event["transferred"]
            expected[receiver] += event["transferred"]
        assert event["balances_after"] == expected
    start = trial["opportunities"][-1]["after"]["event_end"]
    resource = events[start:trial["after_action"]["event_end"]]
    assert resource[0]["decision"]["selected"] == readiness(trial["opportunities"][-1]["after"]["agents"][0], trial["resource_partner"], trial["config"]["amount"])
    for event in resource:
        if event["decision"]["selected"] == "Accept":
            assert event["decision"]["observation"]["last_signal"]["action"] == "Offer"
    after, persistent = trial["after_action"]["agents"], trial["persistent"]["agents"]
    assert after[0]["food"] - persistent[0]["food"] == trial["consumption"]["consumed"]
    assert after[1:] == persistent[1:]
    return {"case": trial["case"], "config": trial["config"], "mode": trial["mode"], "steps": steps,
            "actions": [e["decision"]["selected"] for e in resource], "food": [a["food"] for a in persistent],
            "pre": readiness(trial["opportunities"][0]["before"]["agents"][0], trial["resource_partner"], trial["config"]["amount"]),
            "post": readiness(trial["opportunities"][-1]["after"]["agents"][0], trial["resource_partner"], trial["config"]["amount"])}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--replay-dir", type=Path)
    args = parser.parse_args()
    manifest = json.loads((ARCHIVE / "archive-manifest.json").read_bytes())
    archives = {}
    for name, expected in manifest.items():
        compressed = (ARCHIVE / (name + ".json.gz")).read_bytes()
        raw = gzip.decompress(compressed)
        for data, prefix in ((raw, "json"), (compressed, "gzip")):
            assert len(data) == expected[prefix + "_bytes"]
            assert hashlib.sha256(data).hexdigest() == expected[prefix + "_sha256"]
        if args.replay_dir:
            assert (args.replay_dir / (name + ".json")).read_bytes() == raw
        archives[name] = json.loads(raw)
    rows, first_inputs = [], {}
    for held, name in ((False, "seed-42"), (True, "held-out-seed-42")):
        trials = archives[name]
        groups = collections.defaultdict(list)
        for trial in trials:
            row = reconstruct(trial)
            row["held"] = held
            rows.append(row)
            groups[(trial["case"], key(trial["config"]))].append(trial)
        for (case, config), group in groups.items():
            assert len(group) == 3
            for trial in group:
                assert trial["before_prelude"] == group[0]["before_prelude"]
                assert trial["opportunities"][0]["before"] == group[0]["opportunities"][0]["before"]
                compact = trial["final_state"]["base"]["base"]["base"]
                captures = trial["final_state"]["value"]["records"]
                first = next(c for c in captures if compact["base"]["inquiry"]["records"][c["inquiry"]]["decision"]["input"]["owner"] == 0)
                query = compact["provenance"]["queries"][first["query"]]
                context = {"base": compact["base"]["inquiry"]["records"][first["inquiry"]]["decision"]["input"],
                           **compact["contexts"]["values"][query["context"]]}
                identity = (held, case, config)
                if identity in first_inputs:
                    assert first_inputs[identity] == context
                first_inputs[identity] = context
                reference = group[0]["final_state"]["base"]["base"]["base"]
                assert compact["base"]["inquiry"]["records"][:first["inquiry"]] == reference["base"]["inquiry"]["records"][:first["inquiry"]]
                end = trial["opportunities"][0]["before"]["assessment_end"]
                assert trial["final_state"]["base"]["base"]["assessment"]["records"][:end] == group[0]["final_state"]["base"]["base"]["assessment"]["records"][:end]
            if case == "uncertain_minor":
                pair = groups[("mirrored_future", config)]
                for a, b in zip(group, pair):
                    assert a["opportunities"] == b["opportunities"]
            if case == "mirrored_future":
                assert first_inputs[(held, "uncertain_minor", config)] == first_inputs[(held, case, config)]
    totals = []
    for held in (False, True):
        for mode in MODES:
            steps = [s for r in rows if r["held"] == held and r["mode"] == mode for s in r["steps"]]
            asks = [s for s in steps if s["selected"] != "Pause"]
            totals.append(dict(held=held, mode=mode, questions=len(asks), pause=len(steps)-len(asks),
                changed=sum(s["changed"] for s in asks), time=sum(s["time"] for s in steps),
                methods=dict(collections.Counter(s["selected"]["Ask"]["strategy"] for s in asks)),
                no_change=sum(not s["changed"] for s in asks),
                suppressed_claims=sum(s["suppressed_claim"] for s in asks),
                positive_novelty_without_change=sum(s["positive_novelty_without_change"] for s in asks),
                received_without_change=sum(bool(s["new_receipts"]) and not s["changed"] for s in asks),
                no_receipt=sum(not s["new_receipts"] for s in asks),
                top_ties=sum(s["positive_top_tie"] for s in asks),
                saturated_selected=sum(s["winner"]["leverage"] == 100 for s in asks),
                expectation_changes=sum(s["expectation_changed"] for s in asks),
                learning_changes=sum(s["cell_changed"] for s in asks)))
    classifications = collections.Counter()
    groups = collections.defaultdict(dict)
    for row in rows:
        groups[(row["held"], row["case"], key(row["config"]))][row["mode"]] = row
    for (held, case, config), group in groups.items():
        for a, b in itertools.combinations(MODES, 2):
            x, y = group[a], group[b]
            qdiff = [s["selected"] for s in x["steps"]] != [s["selected"] for s in y["steps"]]
            adiff = x["actions"] != y["actions"]
            label = "C" if qdiff else "Equivalent"
            if adiff:
                persistent = x["food"] != y["food"]
                if not qdiff:
                    label = "E_A" if persistent else "F"
                else:
                    label = "E_Q" if x["pre"] == y["pre"] and x["post"] != y["post"] and persistent else "F"
            classifications[f"{held}:{a}/{b}:{label}"] += 1
    # Protect factual review anchors, not an assumed negative adoption verdict.
    core = { (r["case"], r["mode"]): r for r in rows if not r["held"] }
    positive = core[("same_importance", "StatusValue")]["steps"][0]
    assert (positive["support_before"], positive["support_after"]) == (0, 50)
    far = core[("far_resolution", "StatusValue")]["steps"][1]
    assert (far["support_before"], far["support_after"]) == (5, 24)
    negative = core[("same_uncertainty", "StatusValue")]
    assert negative["steps"][0]["positive_top_tie"] and negative["steps"][0]["suppressed_claim"]
    assert negative["steps"][0]["novelty"] == 50 and negative["food"][:2] == [3, 4]
    assert core[("same_uncertainty", "Unchanged")]["food"][:2] == [2, 5]
    print(json.dumps({"reconstructed_trials": len(rows), "exact_replay_files": 3 if args.replay_dir else 0,
                      "totals": totals, "classifications": dict(classifications), "trials": rows}, indent=2))


if __name__ == "__main__":
    main()
