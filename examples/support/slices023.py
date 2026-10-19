"""Read-only causal slices and matched pre-choice boundary audit from original023."""
import argparse,copy,hashlib,json,re
from pathlib import Path
import review023 as r
ROOT=Path(__file__).resolve().parents[2]
def resolve(t,ref):
    value=t
    for name,index in re.findall(r'([A-Za-z_][A-Za-z_0-9]*)|\[(\d+)\]',ref):value=value[name] if name else value[int(index)]
    return value

def local(t,tick,owner):
    for bi,b in enumerate(t['history']['boundaries']):
        if b['tick']==tick:
            for di,d in enumerate(b['choices']):
                if d['input']['legacy']['own']['id']==owner:
                    i=d['input'];return dict(ref=f'history.boundaries[{bi}].choices[{di}]',tick=tick,owner=owner,food=i['legacy']['own']['food'],hunger=i['legacy']['own']['hunger'],public_end=i['legacy']['end'],retained=i['retained'],selected=d['selected'],candidates=d['candidates'],contact=d['contact'],contact_candidates=d['contact_candidates'])
    raise AssertionError('missing causal owned choice')

def same_input_without_end(a,b):
    a=copy.deepcopy(a);b=copy.deepcopy(b);a['legacy'].pop('end');b['legacy'].pop('end');return a==b

def build():
    d=ROOT/'experiments/023';comparison=json.loads((d/'comparisons.json').read_bytes());pairs=comparison['pairs'];matched=[];saved={};previous=None
    for n,t in enumerate(r.read(d/'trials.jsonl.gz')):
        if n in (0,1,4,5,10,11,13,19):saved[n]=t
        if n>=40:continue
        if n%2==0:previous=t;continue
        report=pairs[n//2];diff=report['first_selected_choice'];limit=diff['tick'] if diff else previous['history']['end'];a={(b['tick'],x['input']['legacy']['own']['id']):x['input'] for b in previous['history']['boundaries'] for x in b['choices']};b={(b['tick'],x['input']['legacy']['own']['id']):x['input'] for b in t['history']['boundaries'] for x in b['choices']}
        checked=[key for key in a.keys()&b.keys() if key[0]<=limit]
        assert {k for k in a if k[0]<=limit}=={k for k in b if k[0]<=limit}, 'pre-choice opportunity mismatch'
        assert checked and all(same_input_without_end(a[k],b[k]) for k in checked), 'pre-first-selected input mismatch'
        matched.append(dict(ordinals=[n-1,n],through_tick=limit,captured_inputs_checked=len(checked),identical_except_registered_public_end=True))
    t=saved[1];h=t['history'];claim=next((i,tr) for i,tr in enumerate(h['transitions']) if not tr['completed'] and tr['commitment']['id']==20)
    report=next((i,c) for i,c in enumerate(t['checkpoints']) if c['tick']==17)
    grant=next((i,g) for i,g in enumerate(h['grants']) if g['commitment']==20)
    event=next((i,e) for i,e in enumerate(r.core(h)['events']) if e['tick']==22 and e['participants']==[0,2] and e['decision']['selected']=='Accept')
    positive=dict(ordinal=1,actor_local_before=local(t,14,0),admitted_claim_ref=f'history.transitions[{claim[0]}]',checkpoint_ref=f'checkpoints[{report[0]}]',grant_ref=f'history.grants[{grant[0]}]',new_local_input=local(t,18,0),downstream_accept_ref=f'history.final_state.base.base.base.base.base.events[{event[0]}]',observer_effect=dict(grant=grant[1],later_transfer=event[1]['transferred'],later_participants=event[1]['participants'],later_food=event[1]['balances_after']),qualification='This is a continuation/material-use chain, not a net benefit. Control0cannot fit a4tickclaim at14 before17 and acquires at17, finishing21; its hunger/state already differ after choice10.')
    assert report[1]['active'].count(20)==1 and grant[1]['tick']==18 and positive['new_local_input']['food']==1 and event[1]['transferred']==1
    negative=dict(ordinals=[4,5],windowed_first=local(saved[4],10,0),continuous_first=local(saved[5],10,0),later_continuous_choice=local(saved[5],18,0),hunger_area_continuous_minus_windowed=pairs[2]['hunger_area_continuous_minus_windowed'],windowed_final_food=[a['food'] for a in saved[4]['history']['final_agents']],continuous_final_food=[a['food'] for a in saved[5]['history']['final_agents']],windowed_final_hunger=[a['hunger'] for a in saved[4]['history']['final_agents']],continuous_final_hunger=[a['hunger'] for a in saved[5]['history']['final_agents']],qualification='At first divergence inputs match except budget; continued contact outranks an eligible meal. Higher hunger area for allfour is a measured cost, not comprehensive welfare.')
    meals=r.core(saved[4]['history'])['cognition']['consumption'];mi,meal=next((i,m) for i,m in enumerate(meals) if m['actor']==0 and m['tick']==11)
    negative['actual_control_meal_ref']=f'history.final_state.base.base.base.base.base.cognition.consumption[{mi}]';negative['control_meal']=meal
    # Exact direct-query completion, distinct from its selection capture/private response.
    q=saved[13];queries=q['history']['final_state']['base']['base']['base']['base']['inquiry']['records'];query=queries[0]
    inquiry=dict(ordinal=13,selection=local(q,4,0),checkpoint_ref=next(f'checkpoints[{i}]' for i,c in enumerate(q['checkpoints']) if c['tick']==15),query_ref='history.final_state.base.base.base.base.inquiry.records[0]',tick=query['tick'],time_spent=query['time_spent'],selected=query['decision']['selected'],response_present=query['response'] is not None,realized_value=query['realized_value'],qualification='Legitimate information outcome; no material benefit in zerofood fixture. Longer48periodcontrols36/37both complete one query; shortperiod11masks allDirect15 reservations.')
    clean=saved[11];control=saved[10];ch=clean['history'];cb=r.core(ch)
    cm=r.core(control['history'])['cognition']['consumption'];control_meal_i,control_meal=next((i,m) for i,m in enumerate(cm) if m['actor']==2 and m['tick']==8)
    meal_i,meal=next((i,m) for i,m in enumerate(cb['cognition']['consumption']) if m['actor']==2 and m['tick']==9)
    ev_i,event=next((i,e) for i,e in enumerate(cb['events']) if e['tick']==10 and e['scene']==1)
    clean_cost=dict(ordinals=[10,11],windowed_choice=local(control,7,2),continuous_choice=local(clean,7,2),continuous_pending_choice=local(clean,8,2),windowed_meal_ref=f'history.final_state.base.base.base.base.base.cognition.consumption[{control_meal_i}]',continuous_meal_ref=f'history.final_state.base.base.base.base.base.cognition.consumption[{meal_i}]',zero_transfer_event_ref=f'history.final_state.base.base.base.base.base.events[{ev_i}]',windowed_meal=control_meal,continuous_meal=meal,event=event,hunger_area_continuous_minus_windowed=pairs[5]['hunger_area_continuous_minus_windowed'],qualification='Clean modest cost: first matched choice differs only in budget. Contact displaces immediate meal, pending token permits meal a tick later, admitted scene then Leave/no transfer. Does not prove every contact is harmful or authorize retuning.')
    assert event['decision']['selected']=='Leave' and event['transferred']==0 and pairs[5]['hunger_area_continuous_minus_windowed']==[0,0,5,0]
    resolve(control,clean_cost['windowed_meal_ref']);resolve(clean,clean_cost['continuous_meal_ref']);resolve(clean,clean_cost['zero_transfer_event_ref']);resolve(saved[4],negative['actual_control_meal_ref'])
    out=dict(schema='experiment-023-causal-slices/v1',archive_sha256=hashlib.sha256((d/'trials.jsonl.gz').read_bytes()).hexdigest(),matched_prefix_audit=matched,positive_continuation=positive,clean_unproductive_cost=clean_cost,costly_choice=negative,inquiry_crossing=inquiry)
    for section in (positive,negative,inquiry):
        ordinal=section.get('ordinal',section.get('ordinals',[None])[-1]);original=saved[ordinal]
        for key,value in section.items():
            if key.endswith('_ref') and key!='actual_control_meal_ref':resolve(original,value)
    return out

def main():
    p=argparse.ArgumentParser();p.add_argument('--verify',action='store_true');a=p.parse_args();out=build();path=ROOT/'experiments/023/causal-slices.json'
    if a.verify:assert json.loads(path.read_bytes())==out
    else:
        with path.open('x',encoding='utf-8') as f:json.dump(out,f,indent=2);f.write('\n')
    print('023 causal source references and20matched pre-choice prefixes PASS')
if __name__=='__main__':main()
