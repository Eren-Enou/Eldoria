"""Read-only 013 scoring/learning/resource reconstruction. No Rust scorer imported.
Reuse reviewed 006–012 primitives and independently implement transient reconsideration.
Normal Python assertions required; stdout only; no prepared cache required for archives.
"""
from review012 import *
ROOT=Path(__file__).resolve().parents[2]
MODES=("Unchanged","GlobalProgress","Contextual","Pulse","Ungated")
def reconstruct(t):
    snapshot = t["final_state"]
    compact = snapshot["base"]["base"]["base"]
    assessment = snapshot["base"]["base"]["assessment"]
    state = snapshot["learning"]
    inquiries = compact["base"]["inquiry"]["records"]
    info = compact["base"]["base"]["cognition"]["information"]
    assert state["mode"] == (t["mode"] if t["mode"] in ["Unchanged","GlobalProgress"] else "Contextual") and assessment["method"] == "Grouped"
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
                reconsidered = False
                own_concern = next(v for v in own["concerns"] if v["id"] == ask["concern"])
                prior = 55 if ask["strategy"] == "Direct" else 65
                if t["mode"] in ["Pulse", "Ungated"] and c["attempts"] >= 2 and c["prior_or_learned"] <= prior//2 and (t["mode"] == "Ungated" or own_concern["age"]%4 == 0):
                    speaker = next(v for v in own["sources"] if v["id"] == ask["source"])
                    hint = next((h for h in reversed(local["hints"]) if h["source"] == ask["source"] and h["event"] == own_concern["event"]), None)
                    known = hint["known"] if hint else None
                    dependent = local["settings"]["provenance"] and known is not None and any(k["event"] == own_concern["event"] and k["known"] == known for k in local["knowledge"])
                    gain = max(0,min(100,prior+old.trunc(speaker["credibility"],10))) * (not dependent)
                    benefit = c["importance"]*gain//100
                    reconsidered = benefit > c["benefit"]
                    c["benefit"] = max(c["benefit"],benefit)
                    c["score"] = c["benefit"]-c["cost"]
                used.append(dict(action=c["action"],context=ctx,learning_record=learned["last_inquiry"] if learned else None,score=c["score"],reconsidered=reconsidered,age=own_concern["age"]))
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
    return dict(case=t["case"],config=t["config"],mode=t["mode"],steps=steps,metrics=dict(scheduled),history_metrics=dict(metrics))

def resources(t):
    events=t["final_state"]["base"]["base"]["base"]["base"]["base"]["events"]
    actions=[];previous=[4]*7
    for resource in t["resources"]:
        before=[a["food"] for a in resource["actors_before"]];after=[a["food"] for a in resource["actors_after"]]
        assert before==previous and sum(before)==sum(after)
        current=events[resource["start"]:resource["end"]]
        assert current and current[0]["decision"]["selected"]==old.readiness(resource["actors_before"][0],resource["partner"],t["config"]["amount"])
        balances={a["id"]:a["food"] for a in resource["actors_before"]}
        for event in current:
            d=event["decision"];h,g,c,trust,v=d["hunger"],d["generosity"],d["caution"],d["trust"],d["episodic_valence"]
            last=d["observation"]["last_signal"];last=last["action"] if last else None
            legal=["Leave"]
            if last=="Offer":legal += ["Accept","Refuse"]
            elif last=="Request":legal += (["Offer"] if d["food"]>=t["config"]["amount"] else [])+["Refuse"]
            else:legal += ["Request"]+(["Offer"] if d["food"]>=t["config"]["amount"] else [])
            legal.sort(key=("Offer","Request","Accept","Refuse","Leave").index)
            scores=dict(Accept=h+20+old.trunc(trust,4)-c//5,Offer=g-h//2-c//2+trust+v-(60 if h>=60 and d["food"]<=d["observation"]["amount"] else 0),Request=h-35+old.trunc(trust,4)-c//4,Refuse=h//2+c//2-g//2-old.trunc(trust,2),Leave=0)
            assert d["candidates"]==[dict(action=a,score=scores[a]) for a in legal] and d["selected"]==max(legal,key=lambda a:scores[a])
            assert event["balances_before"]==[balances[p] for p in event["participants"]] and sum(event["balances_before"])==sum(event["balances_after"])
            if event["transferred"]:
                assert d["selected"]=="Accept" and event["transferred"]==d["observation"]["amount"]
                actor=event["participants"].index(d["actor"])
                assert event["balances_after"][actor]-event["balances_before"][actor]==event["transferred"]
            else:assert event["balances_before"]==event["balances_after"]
            balances.update(zip(event["participants"],event["balances_after"]))
        assert [balances[i] for i in range(7)]==after
        previous=resource["persistent_food"]
        assert after[0]-previous[0]==resource["consumption"]["consumed"] and after[1:]==previous[1:]
        actions.append([e["decision"]["selected"] for e in current])
    assert t["opportunities"][-1]["after"]["food"]==[a["food"] for a in t["resources"][-1]["actors_before"]]
    return actions,previous

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--directory",type=Path,default=ROOT/"experiments/013")
    parser.add_argument("--preview",action="store_true")
    parser.add_argument("--self-test",action="store_true")
    parser.add_argument("--replay-dir",type=Path)
    args=parser.parse_args()
    if (args.directory/"archive-manifest.json").exists():
        for name,m in json.loads((args.directory/"archive-manifest.json").read_bytes()).items():
            compressed=(args.directory/(name+".json.gz")).read_bytes();raw=gzip.decompress(compressed)
            assert (len(compressed),hashlib.sha256(compressed).hexdigest(),len(raw),hashlib.sha256(raw).hexdigest())==(m["gzip_bytes"],m["gzip_sha256"],m["json_bytes"],m["json_sha256"])
            if args.replay_dir and name!="development-preview":assert raw==(args.replay_dir/(name+".json")).read_bytes()
    rows=[];originals={};totals=collections.defaultdict(collections.Counter)
    for held,name in [(False,"seed-42"),(True,"held-out-seed-42")]:
        if held and args.preview:continue
        for t in read(args.directory,name):
            row=reconstruct(t);actions,food=resources(t);row.update(held=held,actions=actions,food=food)
            counter=totals[(held,t["mode"])];counter.update(row["metrics"]);counter["trials"]+=1
            compact=t["final_state"]["base"]["base"]["base"];queries=compact["base"]["inquiry"]["records"]
            for opportunity,step in zip(t["opportunities"],row["steps"],strict=True):
                assert opportunity["before"]["food"]==opportunity["after"]["food"]
                assert opportunity["before"]["inquiry_end"]<opportunity["after"]["inquiry_end"]
                assert step["selected"]==queries[step["inquiry"]]["decision"]["selected"]
                counter["asks"]+=step["selected"]!="Pause";counter["pause"]+=step["selected"]=="Pause";counter["time"]+=step["time"]
                candidate=next((c for c in step["used"] if c["action"]==step["selected"]),None)
                step["reconsidered"]=bool(candidate and candidate["reconsidered"]);step["score"]=candidate["score"] if candidate else 0;step["age"]=candidate["age"] if candidate else None
                counter["reconsidered"]+=step["reconsidered"];del step["used"]
            row["changes"]=t["changes"];originals[(held,t["case"],old.key(t["config"]),t["mode"])]=t;rows.append(row)
            for change in t["changes"]:
                opportunity=t["opportunities"][change["round"]]
                query=next(r for r in queries[opportunity["before"]["inquiry_end"]:opportunity["after"]["inquiry_end"]] if r["decision"]["input"]["owner"]==0)
                observed=query["tick"]-query["time_spent"]
                known=provenance_prefix(compact,observed).get(change["source"],[])
                assert len(known)==change["knowledge"]
                if change["kind"]=="acquire":
                    window=compact["provenance"]["windows"][change["inspections"]-1]
                    assert window["event"]==change["event"] and window["tick"]<=observed
                    assert any(d["input"]["own"]["id"]==change["source"] and d["selected"] for d in window["decisions"])
                    assert any(k["event"]==change["event"] for k in known)
                if change["kind"]=="provider eviction pressure":
                    assert len(known)==32 and not any(k["event"]==change["event"] for k in known)
    classes=collections.Counter();groups=collections.defaultdict(dict)
    for row in rows:groups[(row["held"],row["case"],old.key(row["config"]))][row["mode"]]=row
    for key,modes in groups.items():
        for left,right in itertools.combinations(MODES,2):
            a,b=modes[left],modes[right];query=lambda r:[s["selected"] for s in r["steps"]]
            same_material=(a["actions"],a["food"])==(b["actions"],b["food"])
            cls="Equivalent" if same_material and query(a)==query(b) else "E" if same_material else "F" if query(a)!=query(b) else "Ambiguous";classes[f'{key[0]}:{left}/{right}:{cls}']+=1
            x,y=originals[(*key,left)],originals[(*key,right)]
            for xo,yo in zip(x["opportunities"],y["opportunities"],strict=True):
                assert {k:v for k,v in xo["before"].items() if k!="cells"}=={k:v for k,v in yo["before"].items() if k!="cells"}
                rs=lambda t,o:[r["decision"]["selected"] for r in t["final_state"]["base"]["base"]["base"]["base"]["inquiry"]["records"][o["before"]["inquiry_end"]:o["after"]["inquiry_end"]]]
                if rs(x,xo)!=rs(y,yo):break
    for key,t in originals.items():
        held,case,config,mode=key
        if case not in ["hidden_twin","mirrored_future"]:continue
        other=originals[(held,"later_useful",config,mode)];before=min(t["config"]["change"],16)
        for a,b in zip(t["opportunities"][:before],other["opportunities"][:before],strict=True):assert a["before"]==b["before"]
        if case=="hidden_twin":
            a,b=t["opportunities"][before],other["opportunities"][before]
            def local(t,o):
                c=t["final_state"]["base"]["base"]["base"]
                r=next(r for r in c["base"]["inquiry"]["records"][o["before"]["inquiry_end"]:o["after"]["inquiry_end"]] if r["decision"]["input"]["owner"]==0)
                reference=next(q for q in c["provenance"]["queries"] if q["inquiry"]==r["id"])
                context=c["contexts"]["values"][reference["context"]]
                return r["decision"]["input"],context,r["decision"]["selected"]
            assert local(t,a)==local(other,b)
    if args.self_test:
        sample=next(t for k,t in originals.items() if k[3]=="Pulse")
        for mutate in [lambda t:t["final_state"]["learning"]["records"][0]["after"].update(expected_change=99),lambda t:t["final_state"]["learning"]["records"][0]["basis"][0].update(support=99)]:
            bad=copy.deepcopy(sample);mutate(bad)
            try:reconstruct(bad)
            except AssertionError:pass
            else:raise AssertionError("corruption accepted")
    if not args.preview:
        summary=read(args.directory,"summary");assert len(summary)==1170
        row_keys={(r["held"],r["case"],old.key(r["config"]),r["mode"]):r for r in rows};seeds=collections.defaultdict(set)
        for s in summary:
            k=(s["held"],s["case"],old.key(s["config"]),s["mode"]);assert k in row_keys and s["seed"] not in seeds[k];seeds[k].add(s["seed"])
            r=row_keys[k]
            if s["seed"]==42:
                assert s["actions"]==r["actions"] and s["persistent_food"]==r["food"]
                assert [x["selected"] for x in s["steps"]]==[x["selected"] for x in r["steps"]]
                for a,b in zip(s["steps"],r["steps"],strict=True):assert all(a[k]==b[k] for k in ["inquiry","selected","outcome","before","after","time","age"])
            for step in s["steps"]:
                if step["outcome"] is not None:
                    out=step["outcome"];changed=out["support_before"]!=out["support_after"] or out["status_before"]!=out["status_after"]
                    prior=step["before"]["expected_change"] if step["before"] else (55 if step["selected"]["Ask"]["strategy"]=="Direct" else 65)
                    assert step["after"]["expected_change"]==(prior+100*changed)//2
        assert len(seeds)==195 and all(v=={0,1,2,3,42,2**64-1} for v in seeds.values())
    diagnostics=[]
    if not args.preview:
        for t in read(args.directory,"seed-diagnostic"):
            reconstructed=reconstruct(t);actions,food=resources(t)
            last=t["resources"][-1];actor=last["actors_before"][0]
            decision=t["final_state"]["base"]["base"]["base"]["base"]["base"]["events"][last["start"]]["decision"]
            diagnostics.append(dict(seed=t["seed"],mode=t["mode"],expectation=actor["expectation"],trust=actor["trust"],memory_valence_sum=sum(m["valence"] for m in actor["memories"] if m["partner"]==last["partner"]),episodic_valence=decision["episodic_valence"],offer_score=next((c["score"] for c in decision["candidates"] if c["action"]=="Offer"),None),actions=actions,food=food))
            if t["seed"]==42:assert t==originals[(True,t["case"],old.key(t["config"]),t["mode"])]
    print(json.dumps(dict(independent_trials=len(rows),totals=[dict(held=h,mode=m,**v) for (h,m),v in totals.items()],classifications=dict(classes),diagnostics=diagnostics,trials=rows),indent=2))

if __name__=="__main__":main()
