"""Derived023comparison, separate from primitive validation and original data."""
import collections,copy,gzip,hashlib,json
from pathlib import Path
import review023 as r
ROOT=Path(__file__).resolve().parents[2]
def encoded(x):return json.dumps(x,separators=(',',':'),sort_keys=True).encode()
def physical(t):
    h=t['history'];b=r.core(h);food=[a['food'] for a in h['initial']];hunger=[a['hunger'] for a in h['initial']];stock=h['initial_stock'];consumed=0;ops=collections.defaultdict(list)
    for n,x in enumerate(h['needs']):ops[x['tick']].append((0,n,'need',x))
    for n,x in enumerate(b['events']):
        if n>=h['bootstrap_events']:ops[x['tick']].append((1,n,'event',x))
    for n,x in enumerate(h['grants']):ops[x['tick']].append((2,n,'grant',x))
    for n,x in enumerate(b['cognition']['consumption']):ops[x['tick']].append((3,n,'meal',x))
    out={}
    for tick in range(h['start'],h['end']+1):
        for _,_,kind,x in sorted(ops[tick]):
            if kind=='need':hunger=x['after'][:]
            elif kind=='event':
                for a,f in zip(x['participants'],x['balances_after'],strict=True):food[a]=f
            elif kind=='grant':stock=x['stock_after'];food[x['owner']]=x['food_after']
            else:food[x['actor']]=x['food_after'];hunger[x['actor']]=x['hunger_after'];consumed+=x['consumed']
        out[tick]=dict(food=food[:],hunger=hunger[:],stock=stock,consumed=consumed)
    return out

def decisions(t):
    result={}
    for n,b in enumerate(t['history']['boundaries']):
        for j,d in enumerate(b['choices']):
            result[b['tick'],d['input']['legacy']['own']['id']]=dict(ref=f'history.boundaries[{n}].choices[{j}]',selected=d['selected'],contact=d['contact'] if d['selected']=='Invite' else None)
    return result

def records(t,category):
    h=t['history'];out=collections.defaultdict(list)
    if category=='admission':
        for n,a in enumerate(h['admissions']):out[a['tick']].append(dict(ref=f'history.admissions[{n}]',value=dict(invitation=a['invitation'],reason=a['reason'])))
    else:
        for n,e in enumerate(r.core(h)['events']):
            if n>=h['bootstrap_events']:out[e['tick']].append(dict(ref=f'history.final_state.base.base.base.base.base.events[{n}]',value=dict(kind='resource',participants=e['participants'],action=e['decision']['selected'],transferred=e['transferred'])))
        for n,g in enumerate(h['grants']):out[g['tick']].append(dict(ref=f'history.grants[{n}]',value=dict(kind='claim',owner=g['owner'],outcome=g['outcome'])))
        for n,m in enumerate(r.core(h)['cognition']['consumption']):out[m['tick']].append(dict(ref=f'history.final_state.base.base.base.base.base.cognition.consumption[{n}]',value=dict(kind='meal',owner=m['actor'])))
        queries=h['final_state']['base']['base']['base']['base']['inquiry']['records']
        for n,q in enumerate(queries):out[q['tick']].append(dict(ref=f'history.final_state.base.base.base.base.inquiry.records[{n}]',value=dict(kind='query',owner=q['decision']['input']['owner'],selected=q['decision']['selected'])))
    return out

def first_records(a,b,category):
    aa,bb=records(a,category),records(b,category)
    for tick in sorted(aa.keys()|bb.keys()):
        if [x['value'] for x in aa[tick]]!=[x['value'] for x in bb[tick]]:return dict(tick=tick,windowed=aa[tick],continuous=bb[tick])
    return None

def without_public_end(value):
    if isinstance(value,dict):
        return {k:('<public opportunity projection>' if k=='end' and set(value)=={'own','profile','now','end','people','query'} else without_public_end(v)) for k,v in value.items()}
    if isinstance(value,list):return [without_public_end(v) for v in value]
    return value

def compare(a,b,ma,mb,ordinals):
    ca,cb=dict(a['config']),dict(b['config']);ca.pop('mode');cb.pop('mode');assert ca==cb
    ha,hb=a['history'],b['history'];assert all(ha[k]==hb[k] for k in ('initial','profiles','initial_stock','initial_total','initial_contexts','start','end','bootstrap_events'))
    aa,bb=decisions(a),decisions(b);choice=None
    for tick,owner in sorted(aa.keys()&bb.keys()):
        x,y=aa[tick,owner],bb[tick,owner]
        if (x['selected'],x['contact'])!=(y['selected'],y['contact']):choice=dict(tick=tick,owner=owner,windowed=x,continuous=y);break
    opportunity=next((dict(tick=tick,owner=owner,windowed=aa.get((tick,owner)),continuous=bb.get((tick,owner))) for tick,owner in sorted(aa.keys()|bb.keys()) if (tick,owner) not in aa or (tick,owner) not in bb),None)
    pa,pb=physical(a),physical(b);material=lambda x:(x['food'],x['stock'],x['consumed']);diff=[tick for tick in pa if material(pa[tick])!=material(pb[tick])]
    recon=next((tick for tick in pa if diff and tick>diff[0] and material(pa[tick])==material(pb[tick])),None)
    return dict(ordinals=ordinals,case=ca['case'],role=ca['role'],period=ca['period'],phase=ca['phase'],matched_initial=True,common_endpoint=ha['end'],first_selected_choice=choice,first_missing_opportunity=opportunity,first_admission=first_records(a,b,'admission'),first_execution=first_records(a,b,'execution'),first_material=diff[0] if diff else None,temporary_material_reconvergence=recon,terminal_food_equal=material(pa[ha['end']])==material(pb[hb['end']]),terminal_hunger_equal=ma['final_hunger']==mb['final_hunger'],terminal_semantic_trust_equal=ma['final_directed_trust']==mb['final_directed_trust'],exact_physical_history_equal=ha==hb,selected_execution_state_null_ignoring_public_end_only=without_public_end(ha)==without_public_end(hb),hunger_area_continuous_minus_windowed=[y-x for x,y in zip(ma['hunger_area'],mb['hunger_area'],strict=True)],windowed_metrics=ma,continuous_metrics=mb)

def main():
    directory=ROOT/'experiments/023';metrics=json.loads((directory/'independent-metrics.json').read_bytes());pairs=[];placebos=[];cache={};previous=None
    for n,t in enumerate(r.read(directory/'trials.jsonl.gz')):
        if n<40:
            if n%2==0:previous=t
            else:pairs.append(compare(previous,t,metrics[n-1],metrics[n],[n-1,n]))
            if n in (1,13):cache[t['config']['case']]=(n,t['history'])
        else:
            original,history=cache[t['config']['case']];assert history==t['history'],'reporting placebo changed actual history';placebos.append(dict(ordinals=[original,n],case=t['config']['case'],exact_history_equal=True))
    summary=dict(worlds=42,matched_pairs=20,controlled_pairs=12,sensitivity_pairs=8,reporting_placebos=placebos,pairs_with_selected_choice_difference=sum(x['first_selected_choice'] is not None for x in pairs),pairs_with_executed_difference=sum(x['first_execution'] is not None for x in pairs),terminal_food_differences=sum(not x['terminal_food_equal'] for x in pairs),terminal_hunger_differences=sum(not x['terminal_hunger_equal'] for x in pairs),terminal_trust_differences=sum(not x['terminal_semantic_trust_equal'] for x in pairs),selected_execution_state_nulls=sum(x['selected_execution_state_null_ignoring_public_end_only'] for x in pairs),pairs_with_temporary_food_reconvergence=sum(x['temporary_material_reconvergence'] is not None for x in pairs),pairs_with_any_higher_hunger_area=sum(any(d>0 for d in x['hunger_area_continuous_minus_windowed']) for x in pairs),pairs_with_any_lower_hunger_area=sum(any(d<0 for d in x['hunger_area_continuous_minus_windowed']) for x in pairs),worlds_with_strict_crossing=sum(bool(m['strict_crossings']) for m in metrics),worlds_with_unfinished_nominal_work=sum(bool(m['unfinished_at_nominal']['active'] or m['unfinished_at_nominal']['pending']) for m in metrics))
    (directory/'comparisons.json').write_text(json.dumps(dict(summary=summary,pairs=pairs),separators=(',',':'))+'\n',encoding='utf-8');print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
