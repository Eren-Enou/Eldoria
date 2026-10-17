"""Explicit observer-only020 schema adapter; no simulation or inferred outcomes."""
import gzip,json,shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
SELECTION={('sharing','Closed'),('sharing','Need'),('lost_contact','Closed'),('lost_contact','Acquire'),('contested','Acquire'),('ongoing','Closed'),('ongoing','Need'),('information','Need'),('stranded','Need'),('stranded','Acquire')}
def phase(item):
    # Declared020 physical phases,not a causal inference from adjacent rows.
    kind=item['kind']
    return 4 if kind=='Selected Acquire' else 3 if kind.startswith('Contact ') else 2 if kind=='Consumed' else 1 if kind in ('Granted','Depleted','Overflow') else 0
def main():
    worlds=[]
    with gzip.open(ROOT/'experiments/020/trials.jsonl.gz','rt',encoding='utf-8') as f:
        for ordinal,line in enumerate(f):
            if ordinal>=70:break
            t=json.loads(line)
            if (t['case'],t['mode']) not in SELECTION:continue
            base=t['final_state']['base']['base']['base']['base']['base'];epoch=t['initial']['state']['tick']
            frames=[dict(ref=f'points[{i}]',tick=p['state']['tick'],stock=p['cache']['stock'],capacity=p['cache']['capacity'],accessible=p['cache']['accessible'],pending=p['pending'],acquisitions=p['acquisitions'],agents=[dict(id=a['id'],food=a['food'],hunger=a['hunger']) for a in p['state']['agents']]) for i,p in enumerate(t['points'])]
            items=[]
            for i,r in enumerate(t['records']):
                items.append(dict(tick=r['completed_at'],kind=r['outcome'],actor=r['attempt']['actor'],ref=f'records[{i}]',value=r,frame=next(k for k,p in enumerate(t['points']) if p['state']['tick']>=r['completed_at'] and p['acquisitions']>i)))
            for pno,p in enumerate(t['periods']):
                for sno,w in enumerate(p['steps']):
                    for i,a in enumerate(w['activity']['allocations']):
                        items.append(dict(tick=t['points'][w['activity']['admitted']]['state']['tick'],kind='Contact '+a['disposition'],actor=a['invitation']['proposer'],ref=f'periods[{pno}].steps[{sno}].activity.allocations[{i}]',value=a,frame=w['activity']['admitted']))
                    for owner,d in enumerate(w['choices']):
                        if d is None or d['selected']!='Acquire':continue
                        b=d['acquiring_base'];ref=f'periods[{pno}].steps[{sno}].choices[{owner}]'
                        items.append(dict(tick=d['cache']['observed_at'],kind='Selected Acquire',actor=owner,ref=ref,value=dict(cache=d['cache'],inquiry_lane_free=d['inquiry_lane_free'],policy=d['policy'],candidates=d['candidates'],selected=d['selected'],own=b['contact']['input']['own'],input_ref=ref+'.acquiring_base.contact.input'),frame=w['activity']['observed']))
                    for i,c in enumerate(w['activity']['consumption']):
                        items.append(dict(tick=c['tick'],kind='Consumed',actor=c['actor'],ref=f'periods[{pno}].steps[{sno}].activity.consumption[{i}]',value=c,frame=next(k for k,p in enumerate(t['points']) if p['state']['tick']>=c['tick'])))
            for e in base['events']:
                if e['tick']<=epoch:continue
                items.append(dict(tick=e['tick'],kind=e['decision']['selected'],actor=e['decision']['actor'],ref=f'final_state.base.base.base.base.base.events[{e["id"]}]',value=e,frame=next(i for i,p in enumerate(t['points']) if p['state']['events']>e['id'])))
            worlds.append(dict(label=f"{t['case']} / {t['mode']}",record=ordinal,seed=t['seed'],config=t['config'],epoch=epoch,initial_total=t['initial_total'],frames=frames,items=sorted(items,key=lambda x:(x['tick'],phase(x)))))
    assert len(worlds)==10
    d=ROOT/'experiments/020'
    (d/'demo-data.json').write_text(json.dumps(worlds,separators=(',',':'))+'\n',encoding='utf-8',newline='\n')
    shutil.copyfile(Path(__file__).with_name('finite-demo.html'),d/'demo.html')
    print('020 authentic demo:10 histories,',(d/'demo-data.json').stat().st_size,'data bytes')
if __name__=='__main__':main()
