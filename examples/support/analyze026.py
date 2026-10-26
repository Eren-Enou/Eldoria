"""Prospectively defined descriptive pair analysis; no global utility ranking."""
import argparse,collections,copy,hashlib,json
from pathlib import Path
import review026 as r
from identity026 import identity
def digest(value):return hashlib.sha256(json.dumps(value,sort_keys=True,separators=(',',':')).encode()).hexdigest()
def execution(body):
    out=copy.deepcopy(body);out.pop('valuations');out.pop('checkpoints');out.pop('config')
    for b in out['history']['boundaries']:
        for d in b['choices']:d.pop('candidates')
    return out
def first_cause(a,b):
    ah,bh=a['body']['history'],b['body']['history']
    for k in ('initial','initial_stock','initial_total','profiles','start','end','bootstrap_events'):assert ah[k]==bh[k]
    for bi,(ab,bb)in enumerate(zip(ah['boundaries'],bh['boundaries'])):
        assert {k:v for k,v in ab.items()if k!='choices'}=={k:v for k,v in bb.items()if k!='choices'},'unexplained difference before first selection'
        assert len(ab['choices'])==len(bb['choices'])
        for di,(ad,bd)in enumerate(zip(ab['choices'],bb['choices'])):
            assert ad['input']==bd['input'] and ad['contact_candidates']==bd['contact_candidates'] and ad['contact']==bd['contact']
            assert ad['candidates'][:4]==bd['candidates'][:4]
            assert ad['candidates'][4]['eligible']==bd['candidates'][4]['eligible']
            if ad['selected']!=bd['selected']:
                i=ad['input'];raw=i['raw_retained']
                return dict(boundary=bi,choice=di,tick=ab['tick'],owner=i['legacy']['own']['id'],
                    input_sha256=digest(i),hunger=i['legacy']['own']['hunger'],retained_raw=raw,
                    baseline_scores=[x['final_score']for x in ad['candidates']],
                    treatment_scores=[x['final_score']for x in bd['candidates']],
                    baseline_selected=ad['selected'],treatment_selected=bd['selected'])
    assert execution(a['body'])==execution(b['body']),'unexplained execution divergence without prior changed choice'
    return None
def state(t,finding):
    body=t['body'];h=body['history'];core=r.review025_food_fate.core(h);events=core['events'][h['bootstrap_events']:]
    meals=[0]*4
    for row in core['cognition']['consumption']:meals[row['actor']]+=row['consumed']
    occupied=[collections.Counter()for _ in range(4)]
    ends={q['commitment']['id']:q['tick']for q in h['transitions']if q['completed']}
    for q in h['transitions']:
        if q['completed']:continue
        c=q['commitment'];kind='Resource'if isinstance(c['work'],dict)else c['work']
        for owner in c['participants']:occupied[owner][kind]+=ends[c['id']]-c['start']
    transfers=[0]*4
    for e in events:
        for owner,before,after in zip(e['participants'],e['balances_before'],e['balances_after']):transfers[owner]+=after-before
    operations=collections.Counter(o['outcome']for o in body['material']['operations'])
    return dict(hunger_area=finding['hunger_area'],ordinary_hunger_area=finding['ordinary_hunger_area'],
        meals=meals,terminal_hunger=[x['hunger']for x in h['final_agents']],food=[x['food']for x in h['final_agents']],
        stock=h['final_stock'],raw=body['material']['final_raw'],waste=body['material']['final_waste'],
        consumed=sum(meals),attempts=len(body['material']['processes']),success=operations['Prepared'],failures=operations['Failed'],
        occupied=[dict(x)for x in occupied],transferred_units=sum(e['transferred']for e in events),net_transfers=transfers,
        social_admissions=sum(x['reason']=='Execute'for x in h['admissions']),
        trust=[{k:v for k,v in x['trust'].items()if v}for x in h['final_agents']],
        memory_sha256=[digest(x['memories'])for x in h['final_agents']],
        final_state_sha256=digest(h['final_state']),execution_sha256=digest(execution(body)),food_fate=finding['food_fate'])
def analyze(archive,findings):
    matrix=identity();assert len(findings)==100
    stream=iter(r.read(archive));pairs=[];placebos={}
    for i in range(50):
        a,b=next(stream),next(stream);assert a['config']==matrix[2*i] and b['config']==matrix[2*i+1]
        cause=first_cause(a,b);aa=state(a,findings[2*i]);bb=state(b,findings[2*i+1])
        end=lambda s:[s['raw'],s['waste'],s['stock'],s['food'],s['consumed']]
        pairs.append(dict(source025=i,cohort='controlled'if i<32 else'held_out'if i<48 else'report_placebo',first_cause=cause,
            baseline=aa,treatment=bb,hunger_area_delta=[y-x for x,y in zip(aa['hunger_area'],bb['hunger_area'])],
            meal_delta=[y-x for x,y in zip(aa['meals'],bb['meals'])],edible_endpoint_equal=(aa['stock'],aa['food'])==(bb['stock'],bb['food']),
            common_mass_endpoint_equal=end(aa)==end(bb),trust_equal=aa['trust']==bb['trust'],
            memory_equal=aa['memory_sha256']==bb['memory_sha256'],execution_equal=aa['execution_sha256']==bb['execution_sha256']))
        if i in (0,48):
            for t in (a,b):placebos[i,t['config']['mode']]=digest(execution(t['body']))
        if i in (1,49):
            for t in (a,b):placebos[i,t['config']['mode']]=digest(execution(t['body']))
    assert next(stream,None)is None
    for mode in ('Baseline','ProjectedNeed'):
        assert placebos[0,mode]==placebos[48,mode] and placebos[1,mode]==placebos[49,mode],'report placebo changed execution'
    def summary(rows):
        return dict(pairs=len(rows),changed_selection=[p['source025']for p in rows if p['first_cause']],
            execution_null=[p['source025']for p in rows if p['execution_equal']],
            changed_but_common_mass_reconverged=[p['source025']for p in rows if p['first_cause'] and p['common_mass_endpoint_equal']],
            baseline={k:sum(p['baseline'][k]for p in rows)for k in ('attempts','success','failures','consumed','transferred_units')},
            treatment={k:sum(p['treatment'][k]for p in rows)for k in ('attempts','success','failures','consumed','transferred_units')},
            hunger_higher=[dict(source025=p['source025'],owner=n,delta=d)for p in rows for n,d in enumerate(p['hunger_area_delta'])if d>0],
            hunger_lower=[dict(source025=p['source025'],owner=n,delta=d)for p in rows for n,d in enumerate(p['hunger_area_delta'])if d<0])
    return dict(schema='experiment-026-comparisons/v1',primary=summary(pairs[:48]),all=summary(pairs),pairs=pairs,
        reporting_placebos='same complete execution after declared config/report/valuation-audit projection;both modes',
        interpretation='Whole-policy matched effects; later same-input counterfactual counts are distinct. Hunger, meals, social contact and transfers are not a global welfare scale.')
def main():
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('findings');p.add_argument('--output');p.add_argument('--expected');a=p.parse_args()
    result=analyze(a.archive,json.loads(Path(a.findings).read_bytes()))
    if a.expected:assert result==json.loads(Path(a.expected).read_bytes())
    if a.output:
        with open(a.output,'x',encoding='utf-8')as out:json.dump(result,out,indent=2);out.write('\n')
    print('026 complete matched-prefix/causal/reporting-placebo comparisons PASS;50 policy pairs')
if __name__=='__main__':main()
