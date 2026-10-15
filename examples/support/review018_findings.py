"""Post-evaluation018 causal diagnostics of registered histories, not new trials.
Use the independent full reader for primitive checks, then assert raw causal anchors.
"""
import collections,json
from pathlib import Path
import review018 as v
D=v.ROOT/'experiments/018'

def events(t):return t['final_state']['base']['base']['base']['base']['base']['events']
def at(t,tick):return next(w['activity'] for p in t['periods'] for w in p['steps'] if t['points'][w['activity']['observed']]['state']['tick']==tick)
def state(t,s):return t['points'][s['observed']]['state']
def eat(t,owner):return [c for p in t['periods'] for w in p['steps'] for c in w['activity']['consumption'] if c['actor']==owner]
def main():
    keep={};twins={};references={}
    wanted={'ongoing':{'Fixed','Serial','Recurring'},'history':{'Serial','Recurring'},'recurring':{'Fixed','Recurring'},'no_food':{'Serial','Recurring'},'information':{'Recurring'},'misleading':{'Recurring'},'formerly_useful':{'Recurring'}}
    for line,t in enumerate(v.read(D,'seed-42'),1):
        if t['mode'] in wanted.get(t['case'],set()):
            key=t['case']+'/'+t['mode'];keep[key]=t;references[key]=dict(archive='seed-42.jsonl.gz',record=line)
        if t['case'] in ('information','private_mirror'):
            s=t['periods'][0]['steps'][0]['activity'];twins[t['case'],t['mode']]=dict(focal=s['decisions'][0],source=s['responses'][2]['selected'] if s['responses'] else None,queries=t['points'][t['periods'][-1]['after']]['state']['queries']-t['initial']['state']['queries'])
    for line,t in enumerate(v.read(D,'held-out-seed-42'),1):
        c=t['config']
        if t['case']=='information' and c['order']=='Descending' and c['budget']==24 and t['mode'] in ('Recurring','NoHistory'):keep['help/'+t['mode']]=t;references['help/'+t['mode']]=dict(archive='held-out-seed-42.jsonl.gz',record=line)
        if t['case']=='competing_eater' and c['interval']==16 and t['mode'] in ('Recurring','NoHistory'):keep['mixed/'+t['mode']]=t;references['mixed/'+t['mode']]=dict(archive='held-out-seed-42.jsonl.gz',record=line)
    assert len(keep)==16
    checked={k:v.verify(t) for k,t in keep.items()}
    a,b=keep['recurring/Fixed'],keep['recurring/Recurring'];assert a['initial']==b['initial']
    for owner in range(3):
        assert [(c['tick'],c['hunger_before'],c['hunger_after']) for c in eat(b,owner)]==[(5,5,0),(9,5,0),(13,5,0)]
        assert not eat(a,owner)
    assert checked['recurring/Recurring']['metrics']['transfers']==0
    a,b=keep['ongoing/Fixed'],keep['ongoing/Recurring'];assert a['initial']==b['initial'] and a['profiles']==b['profiles']
    x,y=at(a,6),at(b,6);assert x['people']==y['people']==[dict(id=0,available=False),dict(id=1,available=False),dict(id=2,available=True)]
    assert x['decisions'][2]['selected']=='Wait' and y['decisions'][2]['selected']=='Consume'
    assert [(c['tick'],c['hunger_before'],c['hunger_after']) for c in eat(b,2)[:5]]==[(3,20,0),(7,5,0),(11,5,0),(15,5,0),(19,5,0)]
    assert [(e['tick'],e['decision']['selected'],e['decision']['actor'],e['transferred']) for e in events(a) if 51<=e['tick']<=53]==[(51,'Offer',0,0),(52,'Accept',1,1)]
    assert [(e['tick'],e['decision']['selected'],e['decision']['actor'],e['transferred']) for e in events(b) if 51<=e['tick']<=53]==[(51,'Request',0,0),(52,'Offer',1,0),(53,'Accept',0,1)]
    assert checked['ongoing/Fixed']['metrics']['transfers']==11 and checked['ongoing/Recurring']['metrics']['transfers']==43
    assert checked['ongoing/Fixed']['period_states'][2]['food']==[0,22,11] and checked['ongoing/Recurring']['period_states'][2]['food']==[0,17,0]
    assert checked['ongoing/Recurring']['final'][0]['hunger']==100
    # More interior eating can remain socially inert even with repeated hunger.
    c=checked['ongoing/Serial'];d=checked['ongoing/Recurring'];cmp=v.compare(c,d)
    assert not cmp['resource_action_changed'] and not cmp['trust_changed'] and not cmp['material_changed'] and cmp['material_periods']==10 and cmp['reconverged']
    assert (c['metrics']['hunger_reduced'],d['metrics']['hunger_reduced'])==(785,635)
    # Strongest harmful resource chain, with a common local decision despite
    # legitimately different private partner inventory.
    a,b=keep['history/Serial'],keep['history/Recurring'];assert a['initial']==b['initial'] and a['profiles']==b['profiles']
    assert [c['tick'] for c in eat(b,1)[:4]]==[48,52,56,60]
    assert [c['tick'] for c in eat(b,2)[:5]]==[62,72,75,78,84]
    assert [c['tick'] for c in eat(a,2)[:2]]==[62,85]
    x,y=at(a,98),at(b,98)
    assert x['decisions'][0]['contact']==y['decisions'][0]['contact']
    assert x['decisions'][0]['contact']['selected']==dict(Invite=dict(recipient=2,kind='Resource'))
    assert x['responses'][2]['selected']==y['responses'][2]['selected']==dict(Accept=0)
    sa,sb=[s['responses'][2]['input']['own']['own'] for s in (x,y)]
    assert (sa['food'],sb['food'])==(4,0) and dict(sa,food=0)==sb
    ea,eb=events(a),events(b);assert ea[24]['decision']==eb[24]['decision'] and ea[24]['tick']==eb[24]['tick']==101
    assert ea[25]['decision']['selected']=='Offer' and eb[25]['decision']['selected']=='Leave' and ea[25]['tick']==eb[25]['tick']==102
    assert ea[25]['decision']['hunger']==eb[25]['decision']['hunger']==20
    assert ea[26]['tick']==103 and ea[26]['transferred']==1
    assert not any(e['transferred'] for e in eb if 101<=e['tick']<=103)
    aa,bb=checked['history/Serial'],checked['history/Recurring']
    assert aa['metrics']['transfers']==10 and bb['metrics']['transfers']==4
    assert aa['metrics']['consumed']==bb['metrics']['consumed']==12
    assert [a['hunger'] for a in aa['final']]==[100,95,65] and [a['hunger'] for a in bb['final']]==[100,100,100]
    assert (aa['final'][0]['trust']['2'],bb['final'][0]['trust']['2'])==(98,-68)
    assert keep['no_food/Serial']['final_state']==keep['no_food/Recurring']['final_state']
    # New information is not automatically truthful or productive.
    iq=checked['information/Recurring']['inquiries'][0];mq=checked['misleading/Recurring']['inquiries'][0];fq=checked['formerly_useful/Recurring']['inquiries'][0]
    assert (iq['start'],iq['end'],iq['outcome']['support_before'],iq['outcome']['support_after'])==(43,56,20,60)
    assert (mq['outcome']['support_before'],mq['outcome']['support_after'])==(20,-30) and checked['misleading/Recurring']['metrics']['new_misleading_assimilation']==1
    assert fq['outcome']['support_before']==fq['outcome']['support_after']==10 and fq['outcome']['novelty']=='Redundant'
    # NoHistory sometimes improves one need outcome, and can trade one person's
    # result against another's under a different registered physical interval.
    h,n=checked['help/Recurring'],checked['help/NoHistory'];assert [a['hunger'] for a in h['final']]==[100,100,100] and [a['hunger'] for a in n['final']]==[100,95,100]
    h,n=checked['mixed/Recurring'],checked['mixed/NoHistory'];assert [a['hunger'] for a in h['final']]==[40,35,45] and [a['hunger'] for a in n['final']]==[35,45,45]
    distinct=0;later=0
    for mode in v.MODES:
        a,b=[twins[case,mode] for case in ('information','private_mirror')];assert a['focal']==b['focal'];distinct+=a['source']!=b['source'];later+=a['queries']!=b['queries']
    assert distinct>0 and later>0
    def location(key,tick):
        t=keep[key]
        for pno,p in enumerate(t['periods']):
            for index,w in enumerate(p['steps']):
                s=w['activity']
                if t['points'][s['observed']]['state']['tick']==tick:return dict(period=pno,step=index,observed_point=s['observed'],after_point=s['after'],need_records=[w['need_before'],w['need_after']],query=s['query'],started=s['started'])
        raise AssertionError((key,tick))
    causal_references={key+'@'+str(tick):location(key,tick) for key,tick in [('recurring/Recurring',4),('ongoing/Fixed',48),('ongoing/Recurring',6),('ongoing/Recurring',48),('history/Serial',98),('history/Recurring',98),('history/Recurring',101),('information/Recurring',41)]}
    print(json.dumps(dict(histories=len(keep),references=references,causal_references=causal_references,post_evaluation_diagnostics=True,repeat_meals=[5,9,13],repeated_occupation_meals=[3,7,11,15,19],reversed_transfer=[51,52,53],harmful_common_choice_tick=98,harmful_provider_action_tick=102,foregone_transfer_tick=103,harmful_final_hunger=[[100,95,65],[100,100,100]],harmful_final_trust=[98,-68],interior_null_resource_actions=True,reconverged_material_periods=10,private_focal_twins=7,distinct_first_private_responses=distinct,different_later_query_counts=later,information_support=[20,60],misleading_support=[20,-30],redundant_support=[10,10],history_ablation_benefit=[100,95,100],history_ablation_mixed=[[40,35,45],[35,45,45]]),sort_keys=True))
if __name__=='__main__':main()
