"""Read-only post-evaluation reconstruction of preregistered seed1 positive cases.
All five modes and both selected orders retained; this is not a held-out discovery.
"""
import gzip,json,collections,copy
from pathlib import Path
import review016 as independent
ROOT=Path(__file__).resolve().parents[2]
def main():
    ts=json.loads(gzip.decompress((ROOT/'experiments/016/seed-1-decline.json.gz').read_bytes()))
    assert len(ts)==10
    rows=[independent.verify(t) for t in ts];groups=collections.defaultdict(dict)
    for t,r in zip(ts,rows):
        assert t['seed']==1 and t['case']=='decline'
        groups[t['config']['order']][t['mode']]=(t,r)
    for order,modes in groups.items():
        assert set(modes)==set(independent.MODES)
        initial=modes['Control'][0]['initial']
        assert all(t['initial']==initial for t,r in modes.values())
        assert all(modes[m][0]['periods'][0]['waves'][0]['wave']==modes['Control'][0]['periods'][0]['waves'][0]['wave'] for m in ['Recent','Typed','Untyped'])
    assert [(groups[o]['Control'][1]['metrics']['consumed'],groups[o]['Recent'][1]['metrics']['consumed']) for o in ['Ascending','Rotating']]==[(9,11),(11,12)]
    control=groups['Ascending']['Control'][0];recent=groups['Ascending']['Recent'][0]
    at=lambda t,tick:next(c for c in t['periods'][0]['waves'] if c['wave']['before']['tick']==tick)
    assert at(control,53)['wave']['before']==at(recent,53)['wave']['before']
    assert at(recent,45)['available'][2][0]['partner']==0
    assert at(recent,45)['wave']['proposals'][2]['selected']==dict(Invite=dict(recipient=1,kind='Resource'))
    assert at(control,45)['wave']['proposals'][2]['selected']==dict(Invite=dict(recipient=0,kind='Resource'))
    assert at(control,53)['wave']['responses'][1]['selected']==dict(Accept=0)
    response=at(recent,53)['wave']['responses'][1]
    assert response['selected']==dict(Accept=2)
    assert [(c['reply'],c['score']) for c in response['candidates']][1:3]==[(dict(Accept=0),2),(dict(Accept=2),42)]
    assert control['periods'][0]['after']['agents'][1]['trust']['2']==-28
    assert recent['periods'][0]['after']['agents'][1]['trust']['2']==-56
    events=lambda t:t['final_state']['base']['base']['base']['base']['base']['events']
    assert events(recent)[11]['tick']==57 and events(recent)[11]['decision']['selected']=='Refuse'
    assert events(control)[13]['tick']==72 and events(control)[13]['transferred']==1
    assert events(recent)[13]['tick']==71 and events(recent)[13]['decision']['selected']=='Leave'
    assert [a['food'] for a in control['periods'][1]['after']['agents']]==[2,0,7]
    assert [a['food'] for a in recent['periods'][1]['after']['agents']]==[2,3,3]
    # Extraction must reproduce the previously archived all-seed summary outcomes.
    summary=json.loads(gzip.decompress((ROOT/'experiments/016/summary.json.gz').read_bytes()))
    for t,r in zip(ts,rows):
        prior=next(s for s in summary if s['seed']==1 and s['case']=='decline' and s['mode']==t['mode'] and s['config']==t['config'])
        assert prior['consumed']==r['metrics']['consumed'] and prior['final_food']==[a['food'] for a in r['final']] and prior['final_hunger']==[a['hunger'] for a in r['final']]
    core=json.loads(gzip.decompress((ROOT/'experiments/016/seed-42.json.gz').read_bytes()))
    selected={t['mode']:t for t in core if t['case']=='late_willing' and t['mode'] in ('Control','Recent','Typed')}
    for t in selected.values():independent.verify(t)
    typed=selected['Typed'];control=selected['Control'];recent=selected['Recent']
    assert typed['profile_changes'][0]['tick']==49
    w49=at(typed,49);w58=at(typed,58)
    assert w49['available'][0][0]==dict(partner=2,kind='Inquiry',outcome='Declined',received=45,until=57)
    assert w49['wave']['proposals'][0]['selected']==dict(Invite=dict(recipient=2,kind='Resource'))
    for tick in [49,52,55]:
        w=at(typed,tick)['wave'];x=w['execution'];es=events(typed)[x['before']['events']:x['after']['events']]
        assert len(es)==1 and es[0]['decision']['selected']=='Leave' and es[0]['transferred']==0
    assert w58['wave']['proposals'][0]['selected']=='Wait'
    assert all(c['kind']!='Inquiry' for c in w58['available'][0])
    assert 58+15>typed['periods'][0]['end']==65
    for t in [control,recent]:
        x=at(t,49)['wave']['execution'];assert x['invitation']['kind']=='Inquiry' and x['after']['tick']==64
        outcome=next(r['outcome'] for r in t['final_state']['learning']['records'] if r['inquiry']>=t['initial']['queries'] and r['outcome'] is not None)
        assert (outcome['support_before'],outcome['support_after'])==(20,60)
    assert not any(w['wave']['execution'] and w['wave']['execution']['invitation']['kind']=='Inquiry' for p in typed['periods'] for w in p['waves'])
    print(json.dumps(dict(histories=10,post_evaluation_extraction=True,costly_primary_reconstructions=3,trials=rows),separators=(',',':')))

if __name__=='__main__':main()
