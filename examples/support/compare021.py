"""Read-only matched021 contrasts. No global ranking or prepared success label."""
import argparse,collections,gzip,json
from pathlib import Path
import review021
ROOT=Path(__file__).resolve().parents[2]
def key(t):return json.dumps([t['seed'],t['case'],t['policy'],t['config']],sort_keys=True)
def compact(t):
    steps=[]
    for pno,p in enumerate(t['periods']):
        for sno,w in enumerate(p['steps']):
            s=w['activity'];choices=[]
            for owner,d in enumerate(w['choices']):
                if d is None:choices.append(None);continue
                base=d['acquiring_base'] if d['selected']=='Acquire' else s['decisions'][owner]
                choices.append(dict(selected=d['selected'],retained=d['retained'],legacy=base['contact']['input'],candidates=d['candidates']))
            steps.append(dict(period=pno,step=sno,tick=t['points'][s['before']]['state']['tick'],choices=choices))
    root=t['final_state']['base']['base']['base']['base']['base']
    return dict(initial=t['initial'],profiles=t['profiles'],steps=steps,records=t['records'],
                final_state=t['final_state'],final_memory=t['final_memory'],
                events=root['events'],consumption=root['cognition']['consumption'])
def contrast(left,right,la,ra):
    assert left['initial']==right['initial'] and left['profiles']==right['profiles']
    first=None;memory=None
    for l,r in zip(left['steps'],right['steps']):
        if l['tick']!=r['tick']:break
        for owner,(a,b) in enumerate(zip(l['choices'],r['choices'])):
            if a is None or b is None:assert a==b;continue
            if memory is None and a['retained']!=b['retained']:memory=[l['period'],l['step'],owner,l['tick']]
            if a['selected']!=b['selected']:
                assert a['legacy']==b['legacy'],'unmatched pre-choice local state'
                first=dict(period=l['period'],step=l['step'],owner=owner,tick=l['tick'],left=a,right=b);break
        if first:break
    # Endpoints are projections, not universal welfare or cognitive equality.
    def totals(a):return [x['food'] for x in a['final']], [x['hunger'] for x in a['final']],a['cache']['stock']
    def claims(t):return [dict(actor=r['attempt']['actor'],cache=r['attempt']['cache'],start=r['attempt']['start'],end=r['attempt']['end'],**{k:r[k] for k in ('completed_at','cache_before','cache_after','food_before','food_after','hunger','outcome')}) for r in t['records']]
    def actions(t):return [{k:e[k] for k in ('tick','participants','outcome','transferred')}|dict(actor=e['decision']['actor'],selected=e['decision']['selected']) for e in t['events']]
    periods=[l==r for l,r in zip(la['period_states'],ra['period_states'])]
    transfers=[la['metrics'].get('transfers',0),ra['metrics'].get('transfers',0)]
    return dict(first_memory_difference=memory,first_choice_difference=first,
                final_material_equal=totals(la)==totals(ra),period_material_equal=periods,
                final_legacy_export_equal=left['final_state']==right['final_state'],
                final_memory_equal=left['final_memory']==right['final_memory'],
                claim_records_equal=left['records']==right['records'],
                executed_claims_equal=claims(left)==claims(right),
                meals_equal=left['consumption']==right['consumption'],
                resource_events_equal=left['events']==right['events'],
                resource_execution_equal=actions(left)==actions(right),
                final_trust_equal=[a['trust'] for a in la['final']]==[a['trust'] for a in ra['final']],
                transfers=transfers,hunger_area=[la['hunger_area'],ra['hunger_area']],
                metrics=[la['metrics'],ra['metrics']])
def main():
    p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,default=ROOT/'experiments/021');p.add_argument('--analysis',type=Path,required=True);p.add_argument('--output',type=Path);p.add_argument('--expected',type=Path);a=p.parse_args()
    b=a.analysis.read_bytes();analysis=json.loads(gzip.decompress(b) if a.analysis.suffix=='.gz' else b)['trials']
    groups=collections.defaultdict(dict)
    for index,t in enumerate(review021.read(a.directory,'trials')):groups[key(t)][t['mode']]=(index,compact(t),analysis[index])
    rows=[]
    for group,modes in groups.items():
        if 'CurrentOnly' not in modes:continue
        li,l,la=modes['CurrentOnly']
        for mode,(ri,r,ra) in modes.items():
            if mode=='CurrentOnly':continue
            rows.append(dict(group=json.loads(group),left=li,right=ri,mode=mode,**contrast(l,r,la,ra)))
    result=dict(comparisons=rows)
    if a.expected:assert result==json.loads(gzip.decompress(a.expected.read_bytes())),'matched contrasts changed'
    if a.output:
        assert not a.output.exists();a.output.write_text(json.dumps(result,separators=(',',':'))+'\n',encoding='utf-8',newline='\n')
    print(json.dumps(dict(groups=len(groups),comparisons=len(rows),by_mode={m:dict(comparisons=len(rs),choice_differences=sum(r['first_choice_difference'] is not None for r in rs),final_material_differences=sum(not r['final_material_equal'] for r in rs),period_material_differences=sum(not all(r['period_material_equal']) for r in rs),area_differences=sum(r['hunger_area'][0]!=r['hunger_area'][1] for r in rs)) for m in sorted({r['mode'] for r in rows}) if (rs:=[r for r in rows if r['mode']==m])})))
if __name__=='__main__':main()
