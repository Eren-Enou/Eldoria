"""Post-evaluation causal extractions of registered019 histories, not new trials.
Use the independent tick integrator; assert primitive first decisions and executions.
"""
import gzip,json
from pathlib import Path
import review019 as v
D=v.ROOT/'experiments/019'
DEFAULT=dict(window=24,idle_gap=1,interval=4,stock=12,hunger=20,order='Ascending')
def main():
    keep={}
    for name in ('seed-42','held-out-seed-42','seed-1-focus'):
        for line,t in enumerate(v.read(D,name),1):
            c=t['config'];case=t['case'];mode=t['mode'];label=None
            if c==DEFAULT:
                if case in ('useful_contact','unproductive_contact') and mode in ('Baseline','Scale110'):label=case+'/'+mode
                if t['seed']==42 and case=='mutual' and mode in ('Baseline','NoHistory'):label='memory/'+mode
                if t['seed']==42 and case in ('information','misleading','formerly_useful') and mode=='Baseline':label=case
                if t['seed']==42 and case=='ongoing' and mode=='Baseline':label='ongoing'
            if t['seed']==42 and case=='recurring' and mode=='Baseline':label='gap'+str(c['idle_gap'])
            if t['seed']==42 and case=='information' and c==dict(DEFAULT,order='Descending') and mode in ('Baseline','Scale90'):label='information_desc/'+mode
            if t['seed']==42 and case=='ongoing' and c==dict(DEFAULT,stock=4) and mode in ('EatFirst','Deadline'):label='low_stock/'+mode
            if t['seed']==42 and case=='ongoing' and c==dict(DEFAULT,window=48) and mode in ('EatFirst','Deadline'):label='long_window/'+mode
            if label is None:continue
            label=str(t['seed'])+'/'+label
            base=t['final_state']['base']['base']['base']['base']['base'];first=t['periods'][0]['steps'][0]['activity']
            steps=[dict(period=pno,step=sno,tick=t['points'][w['activity']['observed']]['state']['tick'],query=w['activity']['query'],started=w['activity']['started'],decisions=[None if d is None else dict(owner=i,food=d['contact']['input']['own']['food'],hunger=d['contact']['input']['own']['hunger'],scores=[x['score'] for x in d['candidates']],selected=d['selected'],contact=d['contact']['selected']) for i,d in enumerate(w['activity']['decisions'])]) for pno,p in enumerate(t['periods']) for sno,w in enumerate(p['steps'])]
            events=[dict(id=e['id'],tick=e['tick'],actor=e['decision']['actor'],action=e['decision']['selected'],food=e['decision']['food'],hunger=e['decision']['hunger'],transferred=e['transferred']) for e in base['events'][t['initial']['state']['events']:]]
            meals=[x for p in t['periods'] for w in p['steps'] for x in w['activity']['consumption']]
            queries=t['final_state']['base']['base']['base']['base']['inquiry']['records'][t['initial']['state']['queries']:]
            learning=[r['outcome'] for r in t['final_state']['learning']['records'] if r['inquiry']>=t['initial']['state']['queries']]
            high=[dict(tick=s['tick'],owner=d['owner'],food=d['food'],hunger=d['hunger'],scores=d['scores'],selected=d['selected']) for s in steps for d in s['decisions'] if d and d['food']>0 and d['hunger']==100 and d['selected']=='Invite']
            keep[label]=dict(archive=name+'.jsonl.gz',record=line,mode=mode,case=case,seed=t['seed'],epoch=t['clock']['epoch'],hunger_area=v.area(t),first=steps[0],first_responses=[dict(owner=r['input']['own']['own']['id'],selected=r['selected'],food=r['input']['own']['own']['food'],hunger=r['input']['own']['own']['hunger']) for r in first['responses']],events=events,meals=meals,queries=[dict(id=q['id'],selected=q['decision']['selected'],time_spent=q['time_spent']) for q in queries],learning=learning,period_states=[dict(period=i,tick=t['points'][p['after']]['state']['tick'],actors=[dict(id=a['id'],food=a['food'],hunger=a['hunger'],trust=a['trust']) for a in t['points'][p['after']]['state']['agents']]) for i,p in enumerate(t['periods'])],high_hunger_invites=high[:5],first_actor0_input=v.fingerprint(first['decisions'][0]['contact']['input']))
    a,b=keep['42/useful_contact/Baseline'],keep['42/useful_contact/Scale110']
    assert a['first']['decisions'][0]['scores']==[0,80,78] and b['first']['decisions'][0]['scores']==[0,80,85]
    assert [(e['tick'],e['action'],e['actor'],e['transferred']) for e in a['events'][:2]]==[(3,'Offer',1,0),(4,'Accept',0,1)]
    assert [(e['tick'],e['action'],e['actor'],e['transferred']) for e in b['events'][:3]]==[(3,'Request',0,0),(4,'Offer',1,0),(5,'Accept',0,1)]
    assert a['meals'][0]['actor']==0 and a['meals'][0]['tick']==1
    assert a['hunger_area']==[10960,4505,10200] and b['hunger_area']==[8355,6685,10200]
    x,y=keep['42/unproductive_contact/Baseline'],keep['42/unproductive_contact/Scale110']
    assert a['first_actor0_input']==b['first_actor0_input']==x['first_actor0_input']==y['first_actor0_input']
    assert y['first']['decisions'][0]['selected']=='Invite' and y['first_responses'][1]['selected']=='Wait'
    assert x['hunger_area']==[10640,8260,10200] and y['hunger_area']==[11680,8250,10200]
    assert keep['42/gap1']['hunger_area']==[9015]*3 and keep['42/gap5']['hunger_area']==[8640]*3
    assert any(c['hunger_before']==10 for c in keep['42/gap5']['meals']) and all(c['hunger_before']==5 for c in keep['42/gap1']['meals'])
    assert keep['42/ongoing']['high_hunger_invites']
    assert keep['42/memory/Baseline']['hunger_area']==[8280,10200,10200] and keep['42/memory/NoHistory']['hunger_area']==[9925,10200,10200]
    print(json.dumps(dict(histories=len(keep),post_evaluation_diagnostics=True,registered_only=True,slices=keep),separators=(',',':')))
if __name__=='__main__':main()
