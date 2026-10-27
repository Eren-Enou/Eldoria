"""Post-run comparison of all registered pairs; does not select/delete trials."""
import argparse,copy,json
from pathlib import Path
import review027 as r,review025_findings as f
def signature(t):
    h=t['body']['history'];c=f.base(h)
    return dict(transitions=h['transitions'],admissions=h['admissions'],events=c['events'],meals=c['cognition']['consumption'],material=t['body']['material']['operations'])
def endpoint(t):
    b=t['body'];h=b['history'];return dict(agents=h['final_agents'],stock=h['final_stock'],raw=b['material']['final_raw'],waste=b['material']['final_waste'])
def compare(a,b,ma,mb):
    ba=a['body'];bb=b['body'];ha=ba['history'];hb=bb['history'];first=None;aligned=0
    for bi,(x,y)in enumerate(zip(ha['boundaries'],hb['boundaries'])):
        for ci,(p,q)in enumerate(zip(x['choices'],y['choices'])):
            assert p['input']==q['input'],'pre-divergence inputs mismatch'
            aligned+=1
            if p['selected']!=q['selected']:
                own=q['input']['legacy']['own'];now=q['input']['legacy']['now']
                record=next(d for d in b['learning']['decisions']if(d['boundary'],d['choice'])==(bi,ci))
                receipts=[v for v in b['learning']['receipts']if v['signal']in record['retained']]
                first=dict(boundary=bi,choice=ci,owner=own['id'],tick=now,fixed=p['selected'],learned=q['selected'],retained=record['retained'],receipts=receipts,expected=record['expected'],adjustment=record['adjustment'],fixed_terms=p['candidates'],learned_terms=q['candidates'],raw_retained=q['input']['raw_retained'],actual_work=[v for v in bb['material']['processes']if v['owner']==own['id']and v['start']==now])
                break
        if first:break
        assert x['tick']==y['tick']and len(x['choices'])==len(y['choices'])
    executed=signature(a)!=signature(b)
    assert bool(first)==executed,'no unrelated executed mismatch'
    def measures(m,t):
        v=m['metrics'];return dict(attempts=v['preparation_attempts'],success=v['completed_preparations'],failed=v['failed_extractions'],work_ticks=m['work_ticks'],meals=m['meals'],hunger_area=m['hunger_area'],ordinary_hunger_area=m['ordinary_hunger_area'],transfers=m['transfers'],proposals=v['proposals'],food=v['final_food'],trust=v['final_directed_trust'],raw=v['final_raw'],waste=v['final_waste'],score_changes=m['learning']['score_changes'],counterfactual_changes=m['learning']['counterfactual_changes'],expired=m['learning']['expired'],capacity_evicted=m['learning']['capacity_evicted'])
    return dict(source025=a['config']['source025'],role=a['config']['world']['world']['role'],executed_changed=executed,physical_endpoint_equal=all(endpoint(a)[k]==endpoint(b)[k]for k in ('stock','raw','waste'))and[(v['food'],v['hunger'])for v in ha['final_agents']]==[(v['food'],v['hunger'])for v in hb['final_agents']],complete_endpoint_equal=endpoint(a)==endpoint(b),matched_choices_before_divergence=aligned,first_divergence=first,fixed=measures(ma,a),experience=measures(mb,b))
def main():
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('findings');p.add_argument('output');a=p.parse_args()
    ts=iter(r.read(a.archive));ms=json.loads(Path(a.findings).read_bytes());assert len(ms)==100
    rows=[compare(next(ts),next(ts),ms[n],ms[n+1])for n in range(0,100,2)]
    assert next(ts,None)is None
    with open(a.output,'x',encoding='utf-8')as out:json.dump(rows,out,indent=2);out.write('\n')
    print('027 all50 matched pairs; executed changes',[v['source025']for v in rows if v['executed_changed']])
if __name__=='__main__':main()
