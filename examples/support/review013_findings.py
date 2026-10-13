"""Second, read-only 013 review: derive scores, FIFO bases, age and material outcomes.

Standard library only; imports neither Rust nor the earlier reconstruction. Archives
are observer evidence for offline checking, never a simulation input. Emit stdout.
"""
import collections
import copy
import csv
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DIRECTORY = ROOT / "experiments/013"
MODES = ("Unchanged", "GlobalProgress", "Contextual", "Pulse", "Ungated")
ACTIVE = {"Open", "Partial"}


def load(name):
    return json.loads(gzip.decompress((DIRECTORY / (name + ".json.gz")).read_bytes()))


def key(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def trunc(value, divisor):
    return (abs(value) // divisor) * (-1 if value < 0 else 1)


def grouped(items, event):
    evidence = any(i["event"] == event and i["quality"] > 0 and
                   "Claim" not in i["origin"] for i in items)
    groups = {}
    for item in items:
        if item["event"] != event or (evidence and "Claim" in item["origin"]):
            continue
        tag, value = next(iter(item["origin"].items()))
        if tag == "NativeReading":
            value = (value["speaker"], value["channel"])
        origin = (("Claim", "Disclosure", "NativeReading", "Known", "Unknown").index(tag), value)
        pair = groups.setdefault(origin, [0, 0])
        index = 0 if item["claim"] else 1
        pair[index] = max(pair[index], item["quality"])
    positive = negative = 0
    for origin in sorted(groups):
        p, n = groups[origin]
        positive += (100 - positive) * p // 100
        negative += (100 - negative) * n // 100
    return positive - negative


def bases(trial):
    assessment = trial["final_state"]["base"]["base"]["assessment"]
    live, prefixes = {}, [{}]
    for record in assessment["records"]:
        items = live.setdefault(str(record["owner"]), [])
        incoming = copy.deepcopy(record["incoming"])
        assert record["basis_before"] == grouped(items, incoming["event"])
        revised = []
        if "Known" in incoming["origin"] and incoming["message"] is not None:
            for item in items:
                if (item["event"], item["communicator"], item["message"]) == (
                    incoming["event"], incoming["communicator"], incoming["message"]
                ) and "Unknown" in item["origin"]:
                    item["origin"] = incoming["origin"]
                    revised.append(item["receipt"])
        evicted = items.pop(0)["receipt"] if len(items) == 32 else None
        items.append(incoming)
        assert record["evicted"] == evicted and record["revised_attribution"] == revised
        assert record["support"] == grouped(items, incoming["event"])
        assert record["retained"] == [i["receipt"] for i in items]
        prefixes.append(copy.deepcopy(live))
    assert live == assessment["items"]
    return prefixes


def scores(own, local, basis, cells, mode):
    candidates = [dict(action="Pause", importance=0, attempts=0, prior_or_learned=0,
                       opportunity_value=0, expected_gain=0, benefit=0, cost=0,
                       score=0, history_record=None, notice=None)]
    details = []
    for concern in own["concerns"]:
        if concern["status"] not in ACTIVE or concern["created_scene"] >= own["scene"]:
            continue
        assert concern["owner"] == own["owner"]
        b = next(v for v in basis if v["concern"] == concern["id"])
        for source in own["sources"]:
            assert source["id"] != own["owner"]
            for method, prior, effort in [("Direct", 55, 10), ("Evidence", 65, 28)]:
                old = next((c for c in own["cells"] if (c["concern"], c["source"], c["strategy"]) ==
                            (concern["id"], source["id"], method)), None)
                learned = old["expected"] if old and own["settings"]["use_history"] else prior
                offer = next((o for o in reversed(source["offers"]) if o["event"] == concern["event"]), None)
                opportunity = 0
                if offer and method == "Evidence":
                    origin = {"Evidence": {"subject": concern["target"], "channel": offer["channel"]}}
                    seen = any(s["event"] == concern["event"] and s["origin"] == origin and
                               s["quality"] >= offer["quality"] for s in own["seen"])
                    attempted = old and old["attempted_offer"] == [offer["channel"], offer["quality"]]
                    if not seen and not attempted:
                        strongest = max((s["quality"] for s in own["seen"] if s["event"] == concern["event"]), default=0)
                        opportunity = min(90, max(0, offer["quality"] - strongest + 30))
                hint = next((h for h in reversed(local["hints"]) if h["event"] == concern["event"] and
                             h["source"] == source["id"]), None)
                known = hint["known"] if hint else None
                dependent = local["settings"]["provenance"] and known is not None and any(
                    k["event"] == concern["event"] and k["known"] == known for k in local["knowledge"])
                independence = 0 if dependent else 100
                gain = min(100, max(0, max(learned, opportunity) + trunc(source["credibility"], 10))) * independence // 100
                context = "Any" if mode == "GlobalProgress" else b["context"]
                cell = next((c for c in cells if (c["source"], c["strategy"], c["context"]) ==
                             (source["id"], method, context)), None)
                effective = min(gain, max(cell["expected_change"], opportunity)) if cell and mode != "Unchanged" else gain
                benefit = concern["importance"] * effective // 100
                eligible = (mode in {"Pulse", "Ungated"} and old is not None and old["attempts"] >= 2 and
                            learned <= prior // 2 and (mode == "Ungated" or concern["age"] % 4 == 0))
                optimism = concern["importance"] * (min(100, max(0, prior + trunc(source["credibility"], 10))) * independence // 100) // 100
                raised = eligible and optimism > benefit
                if eligible:
                    benefit = max(benefit, optimism)
                cost = effort + max(0, own["hunger"]) // 10 + (offer["effort"] if offer and method == "Evidence" else 0)
                action = {"Ask": dict(concern=concern["id"], source=source["id"], strategy=method)}
                candidates.append(dict(action=action, importance=concern["importance"], attempts=old["attempts"] if old else 0,
                                       prior_or_learned=learned, opportunity_value=opportunity, expected_gain=gain,
                                       benefit=benefit, cost=cost, score=benefit-cost,
                                       history_record=old["last_record"] if old and own["settings"]["use_history"] else None,
                                       notice=offer["notice"] if offer else None))
                details.append(dict(action=action, age=concern["age"], eligible=eligible, raised=raised,
                                    independence=independence, base_gain=gain, progress=cell["expected_change"] if cell else None,
                                    score=benefit-cost, without_reconsideration=concern["importance"] * effective // 100-cost))
    selected = max(candidates, key=lambda c: c["score"])["action"]
    return candidates, selected, details


def verify(trial):
    compact = trial["final_state"]["base"]["base"]["base"]
    records = compact["base"]["inquiry"]["records"]
    capture = trial["final_state"]["learning"]
    prefixes = bases(trial)
    ages = collections.Counter()
    versions = set()
    for change in compact["base"]["base"]["concerns"]["transitions"]:
        before, after = change["before"], change["after"]
        if after:
            if before:
                increment = change["reason"] in {"later encounter", "later inquiry meeting"}
                assert after["age"] == min(2**32-1, before["age"] + increment)
                if increment:
                    ages[change["reason"]] += 1
            else:
                assert after["age"] == 0
            versions.add(key(after))
    cells, steps = {}, {}
    for learning in capture["records"]:
        r = records[learning["inquiry"]]
        own = r["decision"]["input"]
        assert all(key(c) in versions for c in own["concerns"])
        query = next(q for q in compact["provenance"]["queries"] if q["inquiry"] == r["id"])
        local = compact["contexts"]["values"][query["context"]]
        items = prefixes[learning["assessment_before"]].get(str(own["owner"]), [])
        basis = [dict(concern=c["id"], event=c["event"], support=grouped(items,c["event"]),
                      context="EvidencePresent" if any(i["event"] == c["event"] and i["quality"] > 0 and
                                                       "Claim" not in i["origin"] for i in items) else "ClaimFallback")
                 for c in own["concerns"] if c["status"] in ACTIVE]
        assert basis == learning["basis"]
        live = cells.setdefault(own["owner"], [])
        candidates, selected, detail = scores(own, local, basis, live, trial["mode"])
        assert r["decision"]["candidates"] == candidates and r["decision"]["selected"] == selected
        assert r["food_before"] == r["food_after"]
        outcome = learning["outcome"]
        if selected != "Pause" and r["cell_after"] is not None:
            ask = selected["Ask"]
            b = next(b for b in basis if b["concern"] == ask["concern"])
            support = grouped(prefixes[learning["assessment_after"]].get(str(own["owner"]), []), b["event"])
            assert outcome["support_before"] == b["support"] and outcome["support_after"] == support
            assert outcome["status_before"] == r["concern_before"]["status"] and outcome["status_after"] == r["concern_after"]["status"]
            assert outcome["time"] == r["time_spent"]
            changed = support != b["support"] or outcome["status_before"] != outcome["status_after"]
            ctx = "Any" if trial["mode"] == "GlobalProgress" else b["context"]
            before = next((c for c in live if (c["source"], c["strategy"], c["context"]) ==
                           (ask["source"], ask["strategy"], ctx)), None)
            assert before == learning["before"]
            prior = before["expected_change"] if before else (55 if ask["strategy"] == "Direct" else 65)
            after = dict(source=ask["source"], strategy=ask["strategy"], context=ctx,
                         attempts=min(2**32-1, before["attempts"]+1) if before else 1,
                         expected_change=(prior+100*changed)//2, last_inquiry=r["id"])
            assert after == learning["after"]
            if before:
                live[live.index(before)] = after
            else:
                if len(live) == 32:
                    assert learning["evicted"] == live.pop(0)
                live.append(after)
        else:
            assert outcome is None and learning["after"] is None
        if own["owner"] == 0:
            steps[r["id"]] = dict(inquiry=r["id"], selected=selected, candidates=detail, outcome=outcome,
                                  time=r["time_spent"], old_before=r["cell_before"], old_after=r["cell_after"])
    assert {str(k):v for k,v in cells.items() if v} == {k:v for k,v in capture["cells"].items() if v}
    scheduled = []
    for opportunity in trial["opportunities"]:
        focal = next(r for r in records[opportunity["before"]["inquiry_end"]:opportunity["after"]["inquiry_end"]]
                     if r["decision"]["input"]["owner"] == 0)
        scheduled.append(dict(round=opportunity["round"], **steps[focal["id"]]))
    return scheduled, ages


def material(trial):
    events = trial["final_state"]["base"]["base"]["base"]["base"]["base"]["events"]
    previous, result = [4]*7, []
    for scene in trial["resources"]:
        actors = scene["actors_before"]
        assert [a["food"] for a in actors] == previous
        current = events[scene["start"]:scene["end"]]
        for event in current:
            d = event["decision"]
            actor = actors[d["actor"]]
            memories = [m for m in actor["memories"] if m["partner"] == d["observation"]["partner"]]
            if event == current[0]:
                valence = max(-40,min(40,sum(m["valence"] for m in memories[-4:])))
                assert d["episodic_valence"] == valence
                assert d["trust"] == actor["trust"].get(str(d["observation"]["partner"]),0)
            assert sum(event["balances_before"]) == sum(event["balances_after"])
        before, after = [a["food"] for a in actors], [a["food"] for a in scene["actors_after"]]
        assert sum(before) == sum(after)
        previous = scene["persistent_food"]
        assert after[0]-previous[0] == scene["consumption"]["consumed"] and after[1:] == previous[1:]
        d = current[0]["decision"]
        result.append(dict(round=scene["round"], event=scene["start"], actions=[e["decision"]["selected"] for e in current],
                           offer_score=next((c["score"] for c in d["candidates"] if c["action"] == "Offer"), None),
                           trust=d["trust"], episodic_valence=d["episodic_valence"], food=previous,
                           partner_memories=[m for m in actors[0]["memories"] if m["partner"]==scene["partner"]]))
    return result


def main():
    hashes = json.loads((DIRECTORY/"frozen-001-012-sha256.json").read_bytes())
    assert len(hashes) == 135
    for name, digest in hashes.items():
        assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest() == digest
    for name, entry in json.loads((DIRECTORY/"archive-manifest.json").read_bytes()).items():
        compressed = (DIRECTORY/(name+".json.gz")).read_bytes()
        raw = gzip.decompress(compressed)
        assert (len(compressed), hashlib.sha256(compressed).hexdigest(), len(raw), hashlib.sha256(raw).hexdigest()) == (
            entry["gzip_bytes"], entry["gzip_sha256"], entry["json_bytes"], entry["json_sha256"])
    preview = load("development-preview")
    summaries = load("summary")
    final_core = [r for r in summaries if r["seed"] == 42 and not r["held"]]
    assert len(preview) == len(final_core) == 65 and len(summaries) == 1170
    repairs = []
    for old, new in zip(preview, final_core, strict=True):
        assert (old["case"], old["mode"]) == (new["case"], new["mode"])
        inquiry_changed = [s["selected"] for s in old["steps"]] != [s["selected"] for s in new["steps"]]
        material_changed = (old["actions"], old["persistent_food"]) != (new["actions"], new["persistent_food"])
        if inquiry_changed or material_changed:
            repairs.append(dict(case=old["case"],mode=old["mode"],inquiry_changed=inquiry_changed,material_changed=material_changed))
    assert sum(r["inquiry_changed"] for r in repairs) == 14
    assert sum(r["material_changed"] for r in repairs) == 5
    assert {r["case"] for r in repairs if r["material_changed"]} == {"hidden_twin"}
    trials = load("seed-42") + load("held-out-seed-42")
    totals, rows, findings, pair_counts = collections.defaultdict(collections.Counter), {}, [], collections.Counter()
    for trial in trials:
        steps, ages = verify(trial)
        scenes = material(trial)
        config = trial["config"]["support"]
        rows[(trial["case"], config, trial["mode"])] = (trial, steps, scenes)
        counts = totals[(config != 20,trial["mode"])]
        for step in steps:
            ask = step["selected"] != "Pause"
            detail = next((c for c in step["candidates"] if c["action"] == step["selected"]), None)
            outcome = step["outcome"]
            changed = outcome is not None and (outcome["support_before"] != outcome["support_after"] or outcome["status_before"] != outcome["status_after"])
            counts.update(asks=ask,pause=not ask,time=step["time"],reconsidered=bool(detail and detail["raised"]),
                          changed=changed,no_receipt=bool(outcome and not outcome["receipt_acquired"]))
            if detail and detail["raised"]:
                counts["raised_no_change"] += not changed
        if trial["case"] in {"later_useful", "useful_offer", "unchanged", "competition", "longitudinal", "misleading_success", "forgetting"}:
            discoveries = [dict(round=s["round"], inquiry=s["inquiry"], selected=s["selected"], outcome=s["outcome"])
                           for s in steps if s["outcome"] and (s["outcome"]["support_before"] != s["outcome"]["support_after"] or
                                                              s["outcome"]["status_before"] != s["outcome"]["status_after"])]
            findings.append(dict(case=trial["case"],support=config,mode=trial["mode"],discoveries=discoveries,
                                 raised_rounds=[s["round"] for s in steps if any(c["raised"] and c["action"]==s["selected"] for c in s["candidates"])],
                                 asks=sum(s["selected"]!="Pause" for s in steps),scenes=scenes))
    original_totals = json.loads((DIRECTORY/"analysis.json").read_bytes())["totals"]
    for row in original_totals:
        actual = totals[(row["held"],row["mode"])]
        assert all(actual[k] == row[k] for k in ["asks","pause","time","reconsidered","changed","no_receipt"])
    for row in csv.DictReader((DIRECTORY/"comparisons.csv").open(encoding="utf-8",newline="")):
        a, astep, amaterial = rows[(row["case"],int(row["support"]),row["left"])]
        b, bstep, bmaterial = rows[(row["case"],int(row["support"]),row["right"])]
        queries = [s["selected"] for s in astep] != [s["selected"] for s in bstep]
        actions = [s["actions"] for s in amaterial] != [s["actions"] for s in bmaterial]
        food = amaterial[-1]["food"] != bmaterial[-1]["food"]
        assert (queries,actions,food) == tuple(row[k] == "True" for k in ["query_changed","actions_changed","food_changed"])
        classification = "Equivalent" if not any((queries,actions,food)) else "E" if not (actions or food) else "F" if queries else "Ambiguous"
        assert classification == row["class"]
        pair_counts[(row["held"],classification)] += 1
    positive = rows[("later_useful",20,"Pulse")][1][9]
    assert positive["selected"] == {"Ask":dict(concern=0,source=4,strategy="Direct")}
    assert positive["outcome"]["support_before"] == 20 and positive["outcome"]["support_after"] == 60
    assert positive["candidates"][0]["age"] == 12
    assert (positive["candidates"][0]["score"], positive["candidates"][0]["without_reconsideration"]) == (40,-8)
    for method in ["Contextual", "Pulse"]:
        changes = rows[("longitudinal",40,method)][1]
        assert any(s["selected"] != "Pause" and s["selected"]["Ask"]["concern"] == 2 and
                   s["outcome"]["support_after"] == 64 for s in changes)
        assert not any(s["selected"] != "Pause" and s["selected"]["Ask"]["concern"] == 1 and
                       s["outcome"]["support_after"] != s["outcome"]["support_before"] for s in changes)
    offer = rows[("useful_offer",20,"Ungated")][1][16]
    assert offer["candidates"][0]["score"] == 43 and offer["candidates"][1]["score"] == 3
    assert rows[("useful_offer",20,"Ungated")][2][-1]["food"][:2] == [3,4]
    assert rows[("useful_offer",20,"Contextual")][2][-1]["food"][:2] == [2,5]
    # Scratch corruption rejection checks the second implementation, not Rust replay.
    sample = rows[("later_useful",20,"Pulse")][0]
    for kind in ["score", "support", "age"]:
        bad = copy.deepcopy(sample)
        if kind == "score":
            bad["final_state"]["base"]["base"]["base"]["base"]["inquiry"]["records"][18]["decision"]["candidates"][1]["score"] += 1
        elif kind == "support":
            bad["final_state"]["learning"]["records"][18]["basis"][0]["support"] += 1
        else:
            bad["final_state"]["base"]["base"]["base"]["base"]["base"]["concerns"]["transitions"][0]["after"]["age"] = 1
        try:
            verify(bad)
        except AssertionError:
            pass
        else:
            raise AssertionError("scratch corruption accepted: " + kind)
    diagnostics = []
    for trial in load("seed-diagnostic"):
        verify(trial)
        scenes = material(trial)
        last = scenes[-1]
        assert any(m["event"] == trial["targets"][0] and m["valence"] < 0 for m in last["partner_memories"])
        assert last["episodic_valence"] == 0
        diagnostics.append(dict(seed=trial["seed"], mode=trial["mode"], final=last))
    print(json.dumps(dict(protected_hashes=135,trials=len(trials),corruptions_rejected=3,diagnostics=diagnostics,repairs=repairs,
                          totals=[dict(held=h,mode=m,**v) for (h,m),v in totals.items()],
                          pairs=[dict(held=h,classification=c,count=v) for (h,c),v in pair_counts.items()],
                          findings=findings,positive_decision=positive),indent=2))


if __name__ == "__main__":
    main()
