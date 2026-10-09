"""Independent read-only 012 reconstruction; no Rust/treatment scorer imported.

Reuse the independent reviewed 006–011 integer/Grouped/novelty primitives;
implement the new estimator, local-context projection and discount separately.
Output stdout only. Accept raw evaluator directory or deterministic archive directory.
"""
import argparse
import collections
import copy
import gzip
import hashlib
import itertools
import json
from pathlib import Path
import review011 as old

ROOT = Path(__file__).resolve().parents[2]
MODES = ("Unchanged", "GlobalProgress", "Contextual")
ACTIVE = {"Open", "Partial"}

def read(directory, name):
    raw = directory / (name + ".json")
    if raw.exists():
        return json.loads(raw.read_bytes())
    return json.loads(gzip.decompress((directory / (name + ".json.gz")).read_bytes()))

def context(items, event):
    return "EvidencePresent" if any(i["event"] == event and i["quality"] > 0 and
        "Claim" not in i["origin"] for i in items) else "ClaimFallback"

def lineage(k, settings):
    return (0, k["known"]) if settings["provenance"] and k["known"] is not None else (
        1, k["communicator"] if settings["speakers"] else 0)

def provenance_support(items, event, settings):
    groups = {}
    for k in items:
        if k["event"] == event:
            v = groups.setdefault(lineage(k, settings), [0, 0])
            side = 0 if k["claim"] else 1
            v[side] = max(v[side], k["quality"])
    p, n = 0, 0
    for key in sorted(groups):
        a, b = groups[key]
        p += (100-p)*a//100
        n += (100-n)*b//100
    return p-n

def provenance_prefix(compact, observed):
    live = {}
    for r in compact["provenance"]["receipts"]:
        if r["tick"] > observed:
            break
        settings = compact["provenance"]["settings"][str(r["listener"])]
        k = r["knowledge"]
        items = live.setdefault(r["listener"], [])
        prior = [x for x in items if x["event"] == k["event"]]
        shared = any(lineage(x, settings) == lineage(k, settings) for x in prior)
        revelation = settings["provenance"] and k["known"] is not None and any(
            x["communicator"] == k["communicator"] and x["message"] == k["message"] and
            x["known"] is None for x in prior)
        conflict = any(lineage(x, settings) != lineage(k, settings) and x["claim"] != k["claim"] for x in prior)
        before = provenance_support(items, k["event"], settings)
        if revelation:
            for x in items:
                if (x["event"], x["communicator"], x["message"]) == (k["event"], k["communicator"], k["message"]):
                    x["known"] = k["known"]
        if len(items) == 32:
            items.pop(0)
        items.append(copy.deepcopy(k))
        after = provenance_support(items, k["event"], settings)
        kind = "Revelation" if revelation else "Shared" if shared else "Conflict" if conflict else "First" if not prior else "Corroboration"
        value = min(abs(after-before),40 if kind == "Revelation" else 10) if kind in ("Revelation","Shared") else min(k["quality"],40 if kind == "First" else 30)
        assert (r["evaluation"]["kind"],r["evaluation"]["incremental_value"]) == (kind,value)
        assert r["food_before"] == r["food_after"]
    return live

def assessment_prefixes(compact, assessment):
    live, prefixes, times = {}, [{}], []
    info = compact["base"]["base"]["cognition"]["information"]
    receipts = compact["provenance"]["receipts"]
    for r in assessment["records"]:
        items = live.setdefault(r["owner"], [])
        item = r["incoming"]
        assert not r["revised_attribution"]
        if "Native" in item["receipt"]:
            receipt = info[item["receipt"]["Native"]]
            assert (receipt["listener"],receipt["event"],receipt["scarce"],receipt["reliability"]) == (
                r["owner"],item["event"],item["claim"],item["quality"])
        else:
            relay = receipts[item["receipt"]["Provenance"]]
            k = relay["knowledge"]
            assert relay["listener"] == r["owner"] and relay["event"] == item["event"]
            assert (item["message"],item["claim"],item["quality"],item["origin"]) == (
                k["message"],k["claim"],k["quality"],{"Known":k["known"]} if k["known"] is not None else {"Unknown":k["communicator"]})
            receipt = info[relay["cognition_receipt"]]
        times.append(receipt["tick"])
        assert r["basis_before"] == old.grouped(items,item["event"])
        items.append(item)
        evicted = items.pop(0)["receipt"] if len(items)>32 else None
        assert r["evicted"] == evicted and r["retained"] == [i["receipt"] for i in items]
        assert r["support"] == old.grouped(items,item["event"]) == receipt["after"]["support"]
        prefixes.append(copy.deepcopy(live))
    return prefixes,times

def baseline(record, local, support):
    # Independently validate every old candidate field before testing the new discount.
    expected = copy.deepcopy(record)
    best, highest = "Pause",0
    for c in expected["decision"]["candidates"]:
        c["benefit"] = c["importance"]*c["expected_gain"]//100
        c["score"] = c["benefit"]-c["cost"]
        if c["score"]>highest:
            best,highest = c["action"],c["score"]
    expected["decision"]["selected"] = best
    old.scores(expected,local,support,"Unchanged")
    return expected["decision"]

def reconstruct(t):
    snapshot = t["final_state"]
    compact = snapshot["base"]["base"]["base"]
    assessment = snapshot["base"]["base"]["assessment"]
    state = snapshot["learning"]
    inquiries = compact["base"]["inquiry"]["records"]
    info = compact["base"]["base"]["cognition"]["information"]
    assert state["mode"] == t["mode"] and assessment["method"] == "Grouped"
    assert snapshot["base"]["retention"]["method"] == "Fifo"
    assert len(state["records"]) == len(inquiries)-state["first_inquiry"]
    prefixes,times = assessment_prefixes(compact,assessment)
    cells, verified, metrics = {},{},collections.Counter()
    for index, capture in enumerate(state["records"]):
        r = inquiries[capture["inquiry"]]
        assert r["id"] == index+state["first_inquiry"] and r["valid"]
        own = r["decision"]["input"]
        query = next(q for q in compact["provenance"]["queries"] if q["inquiry"] == r["id"])
        assert query["owner"] == own["owner"] == capture["owner"]
        assert query["observed_at"] == r["tick"]-r["time_spent"] and query["completed_at"] == r["tick"]
        pre,post = capture["assessment_before"],capture["assessment_after"]
        assert 0<=pre<=post<=len(times)
        assert all(x<=query["observed_at"] for x in times[:pre]) and (pre==len(times) or times[pre]>query["observed_at"])
        assert all(x<=query["completed_at"] for x in times[:post]) and (post==len(times) or times[post]>query["completed_at"])
        items = prefixes[pre].get(own["owner"],[])
        supports = {c["event"]:old.grouped(items,c["event"]) for c in own["concerns"]}
        basis = [dict(concern=c["id"],event=c["event"],support=supports[c["event"]],context=context(items,c["event"])) for c in own["concerns"] if c["status"] in ACTIVE]
        assert capture["basis"] == basis and len(basis)<=8
        local = compact["contexts"]["values"][query["context"]]
        assert local["knowledge"] == provenance_prefix(compact,query["observed_at"]).get(own["owner"],[])
        for signature in own["seen"]:
            receipt = info[signature["receipt"]]
            assert receipt["listener"] == own["owner"] and receipt["tick"]<=query["observed_at"]
        for c in own["cells"]:
            prior = inquiries[c["last_record"]]
            assert prior["id"]<r["id"] and prior["decision"]["input"]["owner"]==own["owner"] and prior["cell_after"]==c
        expected = baseline(r,local,supports)
        live = cells.setdefault(own["owner"],[])
        best,highest = "Pause",0
        used=[]
        for c in expected["candidates"]:
            if c["action"] != "Pause":
                ask = c["action"]["Ask"]
                b = next(b for b in basis if b["concern"]==ask["concern"])
                ctx = "Any" if t["mode"]=="GlobalProgress" else b["context"]
                learned = next((s for s in live if (s["source"],s["strategy"],s["context"])==(ask["source"],ask["strategy"],ctx)),None)
                if t["mode"]!="Unchanged" and learned:
                    gain=min(c["expected_gain"],max(learned["expected_change"],c["opportunity_value"]))
                    c["benefit"]=c["importance"]*gain//100
                    c["score"]=c["benefit"]-c["cost"]
                used.append(dict(action=c["action"],context=ctx,learning_record=learned["last_inquiry"] if learned else None,score=c["score"]))
            if c["score"]>highest:
                best,highest=c["action"],c["score"]
        expected["selected"]=best
        assert r["decision"]==expected and capture["selected"]==best
        assert r["food_before"]==r["food_after"]
        before,after,evicted,outcome = None,None,None,None
        if r["response"] and not r["response"]["valid"]:
            assert r["cell_after"] is None and not r["response"]["receipts"] and not r["response"]["signatures"]
            assert r["time_spent"] == 1+(10 if best["Ask"]["strategy"]=="Direct" else 28)+r["response"]["time_spent"]
        elif r["response"] or best=="Pause":
            old.learning(r,compact,info)
        elif best!="Pause":
            ask=best["Ask"]
            exchange=next(x for x in compact["provenance"]["exchanges"] if query["observed_at"]<x["tick"]<=r["tick"])
            assert exchange["valid"] and exchange["decision"]["input"]["own"]["id"]==ask["source"]
            assert exchange["decision"]["input"]["partner"]==own["owner"]
            if query["receipt"] is None:
                assert exchange["receipt"] is None and r["realized_value"]==0 and r["novelty"]=="None"
            else:
                receipt=compact["provenance"]["receipts"][query["receipt"]]
                assert receipt["listener"]==own["owner"] and receipt["communicator"]==ask["source"]
                assert receipt["id"]==exchange["receipt"] and query["observed_at"]<receipt["tick"]<=r["tick"]
                provenance_prefix(compact,r["tick"])
                novelty={"Shared":"Redundant","Conflict":"Conflict","Revelation":"Stronger"}.get(receipt["evaluation"]["kind"],"New")
                assert (r["novelty"],r["realized_value"])==(novelty,receipt["evaluation"]["incremental_value"])
            assert r["time_spent"]==1+(10 if ask["strategy"]=="Direct" else 28)+exchange["time_spent"]
            old_cell=next((c for c in own["cells"] if all(c[k]==ask[k] for k in ask)),None)
            assert r["cell_before"]==old_cell
            assert r["cell_after"]["expected"]==((old_cell["expected"] if old_cell else (55 if ask["strategy"]=="Direct" else 65))+r["realized_value"])//2
        if best!="Pause" and r["cell_after"] is not None:
            ask=best["Ask"];b=next(b for b in basis if b["concern"]==ask["concern"])
            ctx="Any" if t["mode"]=="GlobalProgress" else b["context"]
            outcome=dict(support_before=b["support"],support_after=old.grouped(prefixes[post].get(own["owner"],[]),b["event"]),
                status_before=r["concern_before"]["status"],status_after=r["concern_after"]["status"],
                receipt_acquired=any(a["owner"]==own["owner"] and a["incoming"]["event"]==b["event"] for a in assessment["records"][pre:post]),
                novelty=r["novelty"],novelty_value=r["realized_value"],time=r["time_spent"])
            location=next((i for i,s in enumerate(live) if (s["source"],s["strategy"],s["context"])==(ask["source"],ask["strategy"],ctx)),None)
            before=copy.deepcopy(live[location]) if location is not None else None
            changed=outcome["support_before"]!=outcome["support_after"] or outcome["status_before"]!=outcome["status_after"]
            prior=before["expected_change"] if before else (55 if ask["strategy"]=="Direct" else 65)
            after=dict(source=ask["source"],strategy=ask["strategy"],context=ctx,attempts=min(2**32-1,before["attempts"]+1) if before else 1,expected_change=(prior+100*changed)//2,last_inquiry=r["id"])
            if location is not None:live[location]=after
            else:
                if len(live)==32:evicted=live.pop(0)
                live.append(after)
            if own["owner"]==0:
                metrics["updates"]+=1;metrics["changed"]+=changed
                metrics["positive_novelty_no_change"]+=r["realized_value"]>0 and not changed
                metrics["no_receipt"]+=not outcome["receipt_acquired"]
                metrics["prediction_error_sum"]+=100*changed-prior
        assert (capture["outcome"],capture["before"],capture["after"],capture["evicted"])==(outcome,before,after,evicted)
        assert len(live)<=32
        if own["owner"]==0:
            verified[r["id"]]=dict(selected=best,outcome=outcome,used=used,time=r["time_spent"],before=before,after=after,valid_learning=r["cell_after"] is not None)
    assert {str(k):v for k,v in cells.items() if v}=={k:v for k,v in state["cells"].items() if v}
    steps=[]
    for o in t["opportunities"]:
        r=next(r for r in inquiries[o["before"]["inquiry_end"]:o["after"]["inquiry_end"]] if r["decision"]["input"]["owner"]==0)
        steps.append(dict(phase=o["phase"],inquiry=r["id"],**verified[r["id"]]))
    scheduled=collections.Counter()
    for step in steps:
        outcome=step["outcome"]
        if outcome is not None:
            changed=outcome["support_before"]!=outcome["support_after"] or outcome["status_before"]!=outcome["status_after"]
            scheduled["updates"]+=1;scheduled["changed"]+=changed
            scheduled["positive_novelty_no_change"]+=outcome["novelty_value"]>0 and not changed
            scheduled["no_receipt"]+=not outcome["receipt_acquired"]
            prior=step["before"]["expected_change"] if step["before"] else (55 if step["selected"]["Ask"]["strategy"]=="Direct" else 65)
            scheduled["prediction_error_sum"]+=100*changed-prior
    events=compact["base"]["base"]["events"][t["opportunities"][-1]["after"]["event_end"]:]
    food=[a["food"] for a in t["after_action"]["agents"]]
    persistent=[a["food"] for a in t["persistent"]["agents"]]
    start_food=[a["food"] for a in t["opportunities"][-1]["after"]["agents"]]
    assert sum(start_food)==sum(food) and sum(food)-sum(persistent)==t["consumption"]["consumed"]
    for e in events:
        assert sum(e["balances_before"])==sum(e["balances_after"])
        d=e["decision"]
        h,g,c,trust,v=d["hunger"],d["generosity"],d["caution"],d["trust"],d["episodic_valence"]
        last=d["observation"]["last_signal"]
        last=last["action"] if last else None
        actions=["Leave"]
        if last=="Offer":actions += ["Accept","Refuse"]
        elif last=="Request":actions += (["Offer"] if d["food"]>=t["config"]["amount"] else [])+["Refuse"]
        else:actions += ["Request"]+(["Offer"] if d["food"]>=t["config"]["amount"] else [])
        order=("Offer","Request","Accept","Refuse","Leave")
        actions.sort(key=order.index)
        expected_scores=dict(Accept=h+20+old.trunc(trust,4)-c//5,
            Offer=g-h//2-c//2+trust+v-(60 if h>=60 and d["food"]<=d["observation"]["amount"] else 0),
            Request=h-35+old.trunc(trust,4)-c//4,Refuse=h//2+c//2-g//2-old.trunc(trust,2),Leave=0)
        assert d["candidates"]==[dict(action=a,score=expected_scores[a]) for a in actions]
        assert d["selected"]==max(actions,key=lambda a:expected_scores[a])
        actor=e["participants"].index(d["actor"])
        if e["transferred"]:
            assert d["selected"]=="Accept" and e["transferred"]==d["observation"]["amount"]
            assert e["balances_after"][actor]-e["balances_before"][actor]==e["transferred"]
        else:assert e["balances_before"]==e["balances_after"]
    pre=t["opportunities"][-1]["after"]["agents"][0]
    assert events[0]["decision"]["selected"]==old.readiness(pre,t["resource_partner"],t["config"]["amount"])
    assert food[0]-persistent[0]==t["consumption"]["consumed"] and food[1:]==persistent[1:]
    return dict(case=t["case"],config=t["config"],mode=t["mode"],steps=steps,actions=[e["decision"]["selected"] for e in events],food=persistent,metrics=dict(scheduled),history_metrics=dict(metrics))

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--directory",type=Path,default=ROOT/"experiments/012")
    parser.add_argument("--preview",action="store_true")
    parser.add_argument("--replay-dir",type=Path)
    parser.add_argument("--self-test",action="store_true")
    args=parser.parse_args()
    names=["seed-42"] if args.preview else ["seed-42","held-out-seed-42"]
    if (args.directory/"archive-manifest.json").exists():
        manifest=json.loads((args.directory/"archive-manifest.json").read_bytes())
        for name,m in manifest.items():
            compressed=(args.directory/(name+".json.gz")).read_bytes();raw=gzip.decompress(compressed)
            assert (len(compressed),hashlib.sha256(compressed).hexdigest(),len(raw),hashlib.sha256(raw).hexdigest())==(m["gzip_bytes"],m["gzip_sha256"],m["json_bytes"],m["json_sha256"])
            if args.replay_dir and name!="development-summary":assert raw==(args.replay_dir/(name+".json")).read_bytes()
    rows=[];originals=collections.defaultdict(dict)
    for name in names:
        for t in read(args.directory,name):
            rows.append(dict(held=name.startswith("held"),**reconstruct(t)))
            originals[(name,t["case"],old.key(t["config"]))][t["mode"]]=t
    if args.self_test:
        sample=next(t for modes in originals.values() for t in modes.values() if t["mode"]=="Contextual" and t["case"]=="novel_no_progress")
        def change_update(t):t["final_state"]["learning"]["records"][0]["after"]["expected_change"]+=1
        def change_context(t):t["final_state"]["learning"]["records"][0]["basis"][0]["context"]="Any"
        def change_outcome(t):t["final_state"]["learning"]["records"][0]["outcome"]["support_after"]+=1
        for mutate in (change_update,change_context,change_outcome):
            corrupted=copy.deepcopy(sample);mutate(corrupted)
            try:reconstruct(corrupted)
            except AssertionError:pass
            else:raise AssertionError("independent reader accepted corrupted cognitive evidence")
    for modes in originals.values():
        for left,right in itertools.combinations(MODES,2):
            a,b=modes[left],modes[right]
            for x,y in zip(a["opportunities"],b["opportunities"],strict=True):
                clean=lambda o:{k:v for k,v in o["before"].items() if k!="cells"}
                assert clean(x)==clean(y) and x["participants"]==y["participants"]
                def selected(t,o):
                    rs=t["final_state"]["base"]["base"]["base"]["base"]["inquiry"]["records"]
                    return next(r for r in rs[o["before"]["inquiry_end"]:o["after"]["inquiry_end"]] if r["decision"]["input"]["owner"]==0)["decision"]["selected"]
                if selected(a,x)!=selected(b,y):break
    if not args.preview:
        summary=read(args.directory,"summary")
        assert len(summary)==1800
        signatures={};seeds=collections.defaultdict(set)
        rows_by_key={(r["held"],r["case"],old.key(r["config"]),r["mode"]):r for r in rows}
        for s in summary:
            k=(s["held"],s["case"],old.key(s["config"]),s["mode"])
            assert k in rows_by_key and s["seed"] not in seeds[k]
            seeds[k].add(s["seed"])
            signature=old.key([[x["selected"] for x in s["steps"]],s["actions"],s["persistent_food"]])
            assert signatures.setdefault(k,signature)==signature
            if s["seed"]==42:
                r=rows_by_key[k]
                assert (s["actions"],s["persistent_food"])==(r["actions"],r["food"])
                for recorded,reconstructed in zip(s["steps"],r["steps"],strict=True):
                    assert all(recorded[field]==reconstructed[field] for field in ("phase","inquiry","selected","outcome","before","after","time"))
        assert len(seeds)==180 and all(v==set(range(8))|{42,2**64-1} for v in seeds.values())
    groups=collections.defaultdict(dict)
    for r in rows:groups[(r["held"],r["case"],old.key(r["config"]))][r["mode"]]=r
    classes=collections.Counter();totals=collections.defaultdict(collections.Counter)
    for modes in groups.values():
        assert set(modes)==set(MODES)
        assert all(modes[mode]["steps"][0]["selected"]==modes["Unchanged"]["steps"][0]["selected"] for mode in MODES)
        for left,right in itertools.combinations(MODES,2):
            a,b=modes[left],modes[right]
            queries=lambda r:[s["selected"] for s in r["steps"]]
            cls="Equivalent" if queries(a)==queries(b) and a["actions"]==b["actions"] and a["food"]==b["food"] else "C" if a["actions"]==b["actions"] and a["food"]==b["food"] else "D" if queries(a)!=queries(b) else "Ambiguous"
            classes[f'{a["held"]}:{left}/{right}:{cls}']+=1
    for r in rows:
        total=totals[(r["held"],r["mode"])];total.update(r["metrics"]);total["trials"]+=1
        for s in r["steps"]:
            total["time"]+=s["time"];total["asks"]+=s["selected"]!="Pause";total["pause"]+=s["selected"]=="Pause"
            total["invalid_reply"]+=s["selected"]!="Pause" and not s["valid_learning"]
    result=dict(independent_trials=len(rows),classifications=dict(classes),totals=[dict(held=h,mode=m,**v) for (h,m),v in totals.items()],trials=rows)
    print(json.dumps(result,indent=2))

if __name__=="__main__":main()
