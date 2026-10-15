"""Read-only post-evaluation causal slices; no new worlds or policy retuning."""
import argparse,json
from pathlib import Path
import review017 as independent

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--directory',type=Path,default=independent.ROOT/'experiments/017');args=ap.parse_args()
    ts=independent.read(args.directory,'held-out-seed-42')
    chosen={}
    for t in ts:
        if (t['case'],t['config']['order'],t['config']['budget'])==('competing_eater','Descending',24) and t['mode'] in ('SerialActive','Active'):chosen['eater/'+t['mode']]=t
        if (t['case'],t['config']['order'],t['config']['budget'])==('history','Ascending',48) and t['mode'] in ('Active','NoHistory'):chosen['history/'+t['mode']]=t
    assert len(chosen)==4
    checked={k:independent.verify(t) for k,t in chosen.items()}
    a,b=chosen['eater/SerialActive'],chosen['eater/Active']
    at=lambda t,tick:next(s for s in t['periods'][0]['steps'] if t['points'][s['observed']]['state']['tick']==tick)
    agents=lambda t,s:t['points'][s['observed']]['state']['agents']
    events=lambda t:t['final_state']['base']['base']['base']['base']['base']['events']
    assert a['initial']==b['initial'] and a['profiles']==b['profiles']
    for x,y in zip(at(a,0)['decisions'],at(b,0)['decisions'],strict=True):assert x['contact']==y['contact'] and x['selected']==y['selected']
    d=at(b,0)['decisions'][2]
    assert [c['score'] for c in d['candidates']]==[0,60,98] and d['selected']=='Invite'
    assert at(b,0)['responses'][0]['selected']==dict(Accept=1)
    for tick,hunger in [(2,60),(3,20)]:
        s=at(b,tick);assert s['people']==[dict(id=0,available=False),dict(id=1,available=False),dict(id=2,available=True)]
        assert s['active_before']['scene']==0 and s['active_before']['start']==2
        assert s['decisions'][2]['selected']=='Consume' and s['decisions'][2]['candidates'][2]==dict(action='Invite',eligible=False,score=-1)
        assert agents(b,s)[2]['hunger']==hunger and s['consumption'][0]['tick']==tick+1
    x,y=agents(a,at(a,4)),agents(b,at(b,4));assert x[:2]==y[:2]
    assert y[2]==dict(x[2],food=0,hunger=0)
    assert (x[2]['food'],x[2]['hunger'])==(2,60)
    assert at(a,4)['decisions'][2]['selected']=='Invite' and at(b,4)['decisions'][2]['selected']=='Wait'
    assert at(a,16)['responses'][0]['selected']==dict(Accept=2)
    assert [(e['tick'],e['decision']['selected'],e['transferred']) for e in events(a)[8:10]]==[(19,'Offer',0),(20,'Accept',1)]
    assert not any(2 in e['participants'] for e in events(b))
    assert events(a)[9]['interpretations'][1]['valence']==24
    assert checked['eater/SerialActive']['metrics']['consumed']==4 and checked['eater/Active']['metrics']['consumed']==5
    assert [(p['food'],p['hunger']) for p in checked['eater/SerialActive']['final']]==[(2,0),(0,0),(0,20)]
    assert [(p['food'],p['hunger']) for p in checked['eater/Active']['final']]==[(1,0),(0,0),(0,0)]
    a,b=chosen['history/Active'],chosen['history/NoHistory']
    assert a['initial']==b['initial'] and a['profiles']==b['profiles']
    assert at(a,41)['decisions'][0]['contact']['selected']==dict(Invite=dict(recipient=2,kind='Resource'))
    assert at(b,41)['decisions'][0]['contact']['selected']==dict(Invite=dict(recipient=1,kind='Resource'))
    assert [e['decision']['selected'] for e in events(a)[4:6]]==['Offer','Refuse']
    assert [e['decision']['selected'] for e in events(b)[4:6]]==['Leave','Leave']
    assert [(e['tick'],e['transferred']) for e in events(b)[6:14] if e['transferred']]==[(51,1),(55,1),(59,1),(63,1)]
    assert checked['history/NoHistory']['inquiries'][0]['outcome']['support_after']==60
    assert checked['history/NoHistory']['inquiries'][0]['outcome']['status_after']=='Resolved'
    assert checked['history/Active']['metrics']['consumed']==1 and checked['history/NoHistory']['metrics']['consumed']==0
    assert checked['history/Active']['final'][0]['hunger']==0 and checked['history/NoHistory']['final'][0]['hunger']==30
    print(json.dumps(dict(histories=4,post_evaluation_diagnosis=True,occupation=[2,4],consumption_ticks=[3,4],foregone_transfer_tick=20,positive_consumed=[4,5],harmful_history_consumed=[1,0],harmful_history_hunger=[0,30],useful_query_without_meal=True),sort_keys=True))

if __name__=='__main__':main()
