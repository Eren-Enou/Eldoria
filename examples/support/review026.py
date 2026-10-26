"""026 independent local scoring plus unchanged independent025 physics/cognition.

No simulator/evaluator imports. The only replaced legacy reader dependency is
selection; its original Physical and Slots classes remain exact. Audit is output,
never a scoring input. The declared026 namespaces are projected for legacy checks.
"""
import argparse,copy,gzip,json,types
from pathlib import Path
import material025
import review025_findings
import review025_food_fate
from identity026 import ROOT,identity

def selection(i,original,valuation,mode):
    assert valuation=='Baseline'
    result=material025.selection(i,original,valuation)
    p=result['terms'][4]
    if mode=='ProjectedNeed' and p['eligible']:
        h=i['legacy']['own']['hunger'];d=i['raw_retained']['observation']
        duration=d['extraction_ticks']+d['preparation_ticks']
        assert 0<=h<=100 and 0<duration<=33
        need=min(100,h+5*(duration//4))
        p['base_score']=need;p['final_score']=need-duration
        result['selected']=max(result['terms'],key=lambda c:c['final_score'])['action']
    return result

READERS={}
def reader(mode):
    assert mode in ('Baseline','ProjectedNeed')
    if mode not in READERS:
        path=ROOT/'examples/support/review025.py';source=path.read_text(encoding='utf-8')
        seam='import material025\n';assert source.count(seam)==1
        adapter=types.SimpleNamespace(Physical=material025.Physical,Slots=material025.Slots,
            selection=lambda i,o,v:selection(i,o,v,mode))
        namespace={'__file__':str(path),'__name__':'independent026_'+mode,'material025':adapter}
        exec(compile(source.replace(seam,''),str(path), 'exec'),namespace)
        READERS[mode]=namespace['verify']
    return READERS[mode]

def body_projection(trial):
    body=copy.deepcopy(trial['body'])
    assert body['schema']=='experiment-026-body/v1'
    assert body['history']['schema']=='experiment-026-physical-history/v1'
    body['schema']='experiment-025-trial/v1'
    body['history']['schema']='experiment-025-physical-history/v1'
    return body

def verify(trial,expected=None,findings=None):
    assert set(trial)=={'schema','config','body','projection_audit'}
    assert trial['schema']=='experiment-026-trial/v1'
    cfg=trial['config'];assert set(cfg)=={'source025','mode','world'}
    assert cfg['mode'] in ('Baseline','ProjectedNeed') and type(cfg['source025']) is int and 0<=cfg['source025']<50
    registered=json.loads((ROOT/'experiments/026/registered-matrix.json').read_bytes())
    assert cfg==registered[2*cfg['source025']+int(cfg['mode']=='ProjectedNeed')]
    if expected is not None:assert cfg==expected
    body=body_projection(trial);assert body['config']==cfg['world']
    metrics=reader(cfg['mode'])(body)
    audit=[];changed_scores=changed_selection=eligible=0
    for bi,b in enumerate(body['history']['boundaries']):
        for di,d in enumerate(b['choices']):
            i=d['input'];own=i['legacy']['own'];p=d['candidates'][4]
            duration=None
            if p['eligible']:
                o=i['raw_retained']['observation'];duration=o['extraction_ticks']+o['preparation_ticks'];eligible+=1
            projected=None if duration is None else min(100,own['hunger']+5*(duration//4))
            baseline=-1 if duration is None else own['hunger']-duration
            audit.append(dict(boundary=bi,choice=di,owner=own['id'],tick=i['legacy']['now'],hunger=own['hunger'],duration=duration,
                intervals=None if duration is None else duration//4,projected_need=projected,baseline_prepare=baseline,final_prepare=p['final_score']))
            changed_scores+=p['final_score']!=baseline
            baseline_terms=copy.deepcopy(d['candidates']);baseline_terms[4]['final_score']=baseline
            changed_selection+=d['selected']!=max(baseline_terms,key=lambda x:x['final_score'])['action']
    assert trial['projection_audit']==audit,'026 projection audit mismatch'
    physical=review025_findings.physical(body)
    fate=review025_food_fate.reconstruct(body)
    result=dict(source025=cfg['source025'],mode=cfg['mode'],metrics=metrics,eligible=eligible,
        changed_scores=changed_scores,same_input_selection_changes=changed_selection,
        hunger_area=physical['area'],ordinary_hunger_area=physical['pre'],food_fate=fate)
    if findings is not None:assert result==findings,'026 findings differ from independent reconstruction'
    return result

def read(path):
    with (gzip.open(path,'rb') if str(path).endswith('.gz') else open(path,'rb')) as stream:
        for line in stream:yield json.loads(line)

def main():
    parser=argparse.ArgumentParser();parser.add_argument('archive');parser.add_argument('--baseline',action='store_true');parser.add_argument('--output');parser.add_argument('--expected');args=parser.parse_args()
    matrix=identity();expected=matrix[::2] if args.baseline else matrix
    rows=[]
    for i,t in enumerate(read(args.archive)):
        assert i<len(expected)
        try:rows.append(verify(t,expected[i]))
        except Exception as error:raise AssertionError(('026 independent row',i))from error
    assert len(rows)==len(expected)
    if args.expected:assert rows==json.loads(Path(args.expected).read_bytes()),'026 archived findings mismatch'
    if args.output:
        with open(args.output,'x',encoding='utf-8')as out:json.dump(rows,out,indent=2);out.write('\n')
    print('026 independent scores/eligibility/ties/slots/physics/cognition/consent/food-fate PASS',len(rows))
if __name__=='__main__':main()
