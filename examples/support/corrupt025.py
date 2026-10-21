"""Scratch-only025negative controls. Missing witnesses stay explicitly missing."""
import argparse
import copy
import gzip
import json
from pathlib import Path

import review025 as r


def choices(t):
    return [(bi,di,d)for bi,b in enumerate(t['history']['boundaries'])for di,d in enumerate(b['choices'])]


def locate(t,category):
    h=t['history'];mat=t['material']
    if category in ('wrong_final_raw','wrong_need','clock','false_consent','phantom_transfer'):
        if category=='wrong_final_raw':return ()
        if category=='wrong_need':return (0,)if h['needs']else None
        if category=='clock':return (0,)if h['boundaries']else None
        if category=='false_consent':return next(((bi,di)for bi,b in enumerate(h['boundaries'])for di,_ in enumerate(b['responses'])),None)
        return next(((n,)for n,e in enumerate(r.core(h)['events'])if n>=h['bootstrap_events']and e['transferred']==0),None)
    outcome={'missing_waste':'Prepared','early_food':'Extracted','lost_escrow':'Extracted','duplicate_grant':'Insufficient','wrong_source':'Extracted','early_completion':'Prepared','failure_refund':'Failed'}.get(category)
    if outcome:return next(((n,)for n,o in enumerate(mat['operations'])if o['outcome']==outcome),None)
    if category=='double_booking':
        starts=[x['commitment']for x in h['transitions']if not x['completed']and x['commitment']['work']=='Prepare']
        for first in starts:
            second=next((c for c in starts if c['id']!=first['id']and max(first['start'],c['start'])<min(first['deadline'],c['deadline'])),None)
            if second:return(first['id'],second['id'])
        return None
    for bi,di,d in choices(t):
        i=d['input']
        if category in ('wrong_hunger','private_partner','future_input','observer_stock'):return(bi,di)
        if category=='wrong_prepare_score'and d['candidates'][4]['eligible']:return(bi,di)
        if category=='changed_legacy_meal_score'and d['candidates'][1]['eligible']:return(bi,di)
        if category=='stable_tie':
            maximum=max(c['final_score']for c in d['candidates'])
            other=next((c['action']for c in d['candidates']if c['final_score']==maximum and c['action']!=d['selected']),None)
            if other:return(bi,di,other)
        if category=='observer_recovery'and i['raw_retained']is None:
            owner=i['legacy']['own']['id'];now=i['legacy']['now']
            candidates=[m['after']for m in mat['memories']if m['owner']==owner and m['tick']<now and m['after']is not None]
            if candidates:return(bi,di,candidates[-1])
    if category=='redelivery_receipt':
        for n,m in enumerate(mat['memories']):
            if m['kind']=='Sense'and m['after']['receipt']>0 and any(p['owner']==m['owner']and p['kind']=='OverwrittenByReady'for p in mat['memories'][:n]):return(n,)
    return None


CATEGORIES=('wrong_final_raw','wrong_need','clock','wrong_hunger','private_partner','future_input','observer_stock',
            'missing_waste','early_food','lost_escrow','duplicate_grant','wrong_source','early_completion','failure_refund',
            'double_booking','wrong_prepare_score','changed_legacy_meal_score','stable_tie','observer_recovery',
            'redelivery_receipt','false_consent','phantom_transfer')


def mutate(t,category,where):
    h=t['history'];mat=t['material']
    if category=='wrong_final_raw':mat['final_raw']+=1
    elif category=='wrong_need':h['needs'][where[0]]['after'][0]+=1
    elif category=='clock':h['boundaries'][where[0]]['tick']+=1
    elif category=='false_consent':
        response=h['boundaries'][where[0]]['responses'][where[1]]
        response['selected']='Continue'if response['selected']!='Continue'else dict(Accept=response['input']['incoming'][0]['proposer'])
    elif category=='phantom_transfer':r.core(h)['events'][where[0]]['transferred']=1
    elif category in ('missing_waste','early_food','lost_escrow','duplicate_grant','wrong_source','early_completion','failure_refund'):
        op=mat['operations'][where[0]]
        if category=='missing_waste':op['waste_after']-=1
        elif category=='early_food':op['food_after']+=1
        elif category=='lost_escrow':op['escrow_after']=0
        elif category=='duplicate_grant':op['outcome']='Extracted'
        elif category=='wrong_source':op['source']=1
        elif category=='early_completion':op['tick']-=1
        else:
            p=next(p for p in mat['processes']if p['commitment']==op['commitment'])
            for tr in h['transitions']:
                if tr['commitment']['id']==p['commitment']and tr['completed']:tr['tick']=p['extract_at']
    elif category=='double_booking':
        first,second=where
        owner=next(tr['commitment']['participants'][0]for tr in h['transitions']if tr['commitment']['id']==first)
        for tr in h['transitions']:
            if tr['commitment']['id']==second:tr['commitment']['participants']=[owner]
    elif category=='redelivery_receipt':mat['memories'][where[0]]['after']['receipt']=0
    else:
        bi,di=where[:2];d=h['boundaries'][bi]['choices'][di]
        audit=next(v for v in t['valuations']if(v['boundary'],v['choice'])==(bi,di))
        if category=='wrong_hunger':d['input']['legacy']['own']['hunger']+=1
        elif category=='private_partner':d['input']['partner_private_food']=[1,2,3,4]
        elif category=='future_input':d['input']['future_partner']=3
        elif category=='observer_stock':d['input']['actual_current_raw']=mat['final_raw']
        elif category in ('wrong_prepare_score','changed_legacy_meal_score'):
            index=4 if category=='wrong_prepare_score'else 1
            d['candidates'][index]['final_score']+=1
            audit['selection']['terms'][index]['final_score']+=1
        elif category=='stable_tie':d['selected']=where[2];audit['selection']['selected']=where[2]
        else:
            assert category=='observer_recovery'
            # Past, genuinely unavailable information, not a future fake or a
            # second simultaneous slot. Other persistent consequences stay intact.
            d['input']['retained']=None
            d['input']['raw_retained']=copy.deepcopy(where[2])


def run(path):
    witnesses={}
    with(gzip.open(path,'rb')if str(path).endswith('.gz')else open(path,'rb'))as source:
        for ordinal,line in enumerate(source):
            trial=json.loads(line)
            for category in CATEGORIES:
                if category in witnesses:continue
                where=locate(trial,category)
                if where is not None:witnesses[category]=(ordinal,line,where)
            if len(witnesses)==len(CATEGORIES):break
    results=[]
    for category in CATEGORIES:
        if category not in witnesses:
            results.append(dict(category=category,exercised=False,reason='No matching original witness; no new world generated'))
            continue
        ordinal,line,where=witnesses[category]
        bad=json.loads(line);mutate(bad,category,where)
        try:r.verify(bad)
        except AssertionError:
            results.append(dict(category=category,exercised=True,ordinal=ordinal,rejected=True))
        else:raise AssertionError('Undetected025scratch corruption: '+category)
    return results


def main():
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('--output');args=p.parse_args()
    result=run(args.archive)
    if args.output:
        with Path(args.output).open('x',encoding='utf-8')as out:json.dump(result,out,indent=2);out.write('\n')
    print('025scratch controls:',sum(r['exercised']for r in result),'rejected;',sum(not r['exercised']for r in result),'explicitly unavailable')


if __name__=='__main__':main()
