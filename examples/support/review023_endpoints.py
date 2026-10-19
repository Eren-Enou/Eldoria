"""Read-only nominal-vs-drain audit and supplemental terminal corruption checks."""
import argparse,copy,json
from pathlib import Path
import analyze023,review023
ROOT=Path(__file__).resolve().parents[2]
def audit():
    directory=ROOT/'experiments/023';metrics=json.loads((directory/'independent-metrics.json').read_bytes());rows=[];checks=[]
    for ordinal,trial in enumerate(review023.read(directory/'trials.jsonl.gz')):
        h=trial['history'];nominal=h['start']+trial['config']['horizon'];points=analyze023.physical(trial)
        before=[sum(points[tick]['hunger'][owner] for tick in range(h['start'],nominal)) for owner in range(4)]
        drain=[sum(points[tick]['hunger'][owner] for tick in range(nominal,h['end'])) for owner in range(4)]
        assert [a+b for a,b in zip(before,drain,strict=True)]==metrics[ordinal]['hunger_area']
        rows.append(dict(ordinal=ordinal,nominal_tick=nominal,final_tick=h['end'],nominal_physical=points[nominal],hunger_area_before_nominal=before,hunger_area_drain=drain,resource_events_after_nominal=sum(e['tick']>nominal for e in review023.core(h)['events']),new_ordinary_choices_at_or_after_nominal=sum(len(b['choices']) for b in h['boundaries'] if b['tick']>=nominal)))
        if ordinal==1:
            bad=copy.deepcopy(trial);b=copy.deepcopy(bad['history']['boundaries'][-1]);b['tick']=bad['history']['end']+1;bad['history']['boundaries'].append(b)
            try:review023.verify(bad)
            except (AssertionError,KeyError,IndexError,ValueError):checks.append(dict(category='ghost decision beyond actual drained terminal',rejected=True))
            else:raise AssertionError('terminal ghost accepted')
        if ordinal==13:
            bad=copy.deepcopy(trial);query=next(tr['commitment'] for tr in bad['history']['transitions'] if not tr['completed'] and tr['commitment']['work']=='Query');cid=query['id']
            for tr in bad['history']['transitions']:
                if tr['commitment']['id']==cid:tr['commitment']['deadline']+=1
            try:review023.verify(bad)
            except (AssertionError,KeyError,IndexError,ValueError):checks.append(dict(category='atomic query deadline differs from exact method cost',rejected=True))
            else:raise AssertionError('wrong query deadline accepted')
    assert len(checks)==2 and all(not row['new_ordinary_choices_at_or_after_nominal'] for row in rows)
    pairs=[dict(ordinals=[i,i+1],hunger_area_continuous_minus_windowed_before_nominal=[b-a for a,b in zip(rows[i]['hunger_area_before_nominal'],rows[i+1]['hunger_area_before_nominal'],strict=True)],hunger_area_continuous_minus_windowed_drain=[b-a for a,b in zip(rows[i]['hunger_area_drain'],rows[i+1]['hunger_area_drain'],strict=True)]) for i in range(0,40,2)]
    return dict(rows=rows,pairs=pairs,supplemental_controls=checks,qualification='Nominal and final differ by33real charged ticks. No new ordinary choice during drain; already proposed protocol/resource work may complete. Hunger area is left-continuous tick integral, not welfare.')
def main():
    p=argparse.ArgumentParser();p.add_argument('--verify',action='store_true');a=p.parse_args();out=audit();path=ROOT/'experiments/023/endpoint-audit.json'
    if a.verify:assert json.loads(path.read_bytes())==out
    else:
        with path.open('x',encoding='utf-8') as f:json.dump(out,f,indent=2);f.write('\n')
    print('023 nominal/drain boundaries and2supplemental controls PASS')
if __name__=='__main__':main()
