"""Read-only registered-case coverage; exact references, no new outcomes."""
import argparse,json
from pathlib import Path
import review023 as r,analyze023
ROOT=Path(__file__).resolve().parents[2]
def build():
    directory=ROOT/'experiments/023';wanted={1,3,11,13,17,19,21,23};trials={n:t for n,t in enumerate(r.read(directory/'trials.jsonl.gz')) if n in wanted};metrics=json.loads((directory/'independent-metrics.json').read_bytes());cases=[]
    def add(name,ordinal,refs,condition,detail):
        cases.append(dict(case=name,ordinal=ordinal,references=refs,covered=bool(condition),detail=detail))
    t=trials[13];q=next(tr for tr in t['history']['transitions'] if not tr['completed'] and tr['commitment']['work']=='Query')
    add('single activity continuation',13,['checkpoints[0]'],t['checkpoints'][0]['active']==[q['commitment']['id']],'Single globally exclusive inquiry spans reporting15; no other actor decisions invented.')
    t=trials[1];h=t['history'];social=[(i,tr) for i,tr in enumerate(h['transitions']) if not tr['completed'] and isinstance(tr['commitment']['work'],dict) and tr['tick']==8]
    add('two disjoint ongoing resource scenes',1,[f'history.transitions[{i}]' for i,tr in social],len(social)==2 and len({a for i,tr in social for a in tr['commitment']['participants']})==4,'Two independently consented resource commitments at8.')
    crossing=next(x for x in metrics[19]['strict_crossings'] if x['kind']=='Resource')
    add('resource scene crossing checkpoint',19,[next(f'checkpoints[{i}]' for i,c in enumerate(trials[19]['checkpoints']) if c['tick']==crossing['checkpoint'])],True,str(crossing))
    td=trials[3]['history'];first=[tr['commitment']['id'] for tr in td['transitions'] if not tr['completed'] and isinstance(tr['commitment']['work'],dict) and tr['tick']==8];ends=[tr['tick'] for tr in td['transitions'] if tr['completed'] and tr['commitment']['id'] in first]
    add('different concurrent completion times',3,['history.transitions'],sorted(ends)==[10,11],'The first pair is released10while the other continues to11.')
    add('delayed meal near former closing boundary',11,['history.boundaries[4].choices[2]'],metrics[11]['hunger_area'][2]-metrics[10]['hunger_area'][2]==5,'Compared10/11meal at8versus9; causal-slices clean_unproductive_cost.')
    add('claim crossing checkpoint',1,['checkpoints[0]','history.grants[0]'],20 in t['checkpoints'][0]['active'] and h['grants'][0]['commitment']==20 and h['grants'][0]['tick']==18,'Claim20:14-18across17.')
    add('pending invitation crosses checkpoint',1,['checkpoints[0]'],any(h['boundaries'][p['source']]['tick']<17<h['boundaries'][p['source']]['tick']+2 for p in t['checkpoints'][0]['pending']),'Own protocol references survive report; no renewal/reset.')
    tb=trials[11]['history'];i,a=next((i,a) for i,a in enumerate(tb['admissions']) if a['reason']=='Unavailable')
    add('blocked recipient',11,[f'history.admissions[{i}]'],a['tick']==5,'Busy recipient is unavailable, not a fabricated rejecting reply.')
    tq=trials[13]['history'];c=q['commitment'];finish=next(tr['tick'] for tr in tq['transitions'] if tr['completed'] and tr['commitment']['id']==c['id'])
    add('inquiry reaches commitment deadline',13,['history.transitions','history.final_state.base.base.base.base.inquiry.records[0]'],finish==c['deadline']==19,'Direct query6-19; selected method cost10plus3. Supplemental wrong-query-deadline control rejects.')
    own=h['boundaries'][12]['choices'][0]['input']['retained'];point=analyze023.physical(t)[18]
    add('stale environmental observation',1,['history.boundaries[12].choices[0]'],own['observation']['stock']-own['own_takes']==3 and point['stock']==2,'Own upperbound3; observer stock2excluded from policy.')
    scarce=trials[23];points=analyze023.physical(scarce);empty=next(tick for tick,p in points.items() if sum(p['food'])+p['stock']==0);later=[(i,tr) for i,tr in enumerate(scarce['history']['transitions']) if not tr['completed'] and isinstance(tr['commitment']['work'],dict) and empty<tr['tick']<scarce['history']['start']+scarce['config']['horizon']]
    add('continued activity after ALL food exhausted',23,[f'history.transitions[{later[0][0]}]'] if later else [],bool(later) and points[empty]['consumed']==scarce['history']['initial_total'],'All oneinitialunit consumed by'+str(empty)+'; later empty contact persists. Distinct from19cacheexhaustion with heldfood.')
    add('genuine idle opportunity',21,['history.boundaries[0]'],all(d['selected']=='Wait' for d in trials[21]['history']['boundaries'][0]['choices']),'One registered ordinary tick at zerofood/zerohunger; drain is not voluntary idle.')
    comparison=json.loads((directory/'comparisons.json').read_bytes())['pairs'][10]
    add('full selected/executed/state null',21,['history'],comparison['selected_execution_state_null_ignoring_public_end_only'],'20/21equal except intended public-end input projection; raw inputs are not claimed equal.')
    add('harmful opportunity cost',11,['history.boundaries[4].choices[2]','history.final_state.base.base.base.base.base.events[3]'],metrics[11]['hunger_area'][2]-metrics[10]['hunger_area'][2]==5,'Invite64displaces Consume5, meal delayed one tick, sceneLeave/no transfer.')
    add('multi-checkpoint sustained history',17,['checkpoints'],len([c for c in trials[17]['checkpoints'] if 'Reporting' in c['labels'] and c['tick']<198])>10 and metrics[17]['peak_social']==2,'192ordinaryticks, same fouractors, no reset, simultaneous work preserved.')
    nominal=next((i,c) for i,c in enumerate(t['checkpoints']) if 'NominalStop' in c['labels'])
    add('finite cutoff with unfinished work',1,[f'checkpoints[{nominal[0]}]',f"checkpoints[{len(t['checkpoints'])-1}]"],bool(nominal[1]['active'] or nominal[1]['pending']) and not t['checkpoints'][-1]['active'] and not t['checkpoints'][-1]['pending'],'Nominal70work/pending retained, settled at103. Separate H3regression covers inquiry-spanned cutoff.')
    assert len(cases)==16
    return dict(cases=cases,covered=sum(c['covered'] for c in cases),total=len(cases),qualification='Witness extraction after fixed outcomes, not fixture tuning. Some references name whole typed collections where ordering is itself evidence. Not statistical coverage of natural worlds.')
def main():
    p=argparse.ArgumentParser();p.add_argument('--verify',action='store_true');a=p.parse_args();out=build();path=ROOT/'experiments/023/coverage.json'
    if a.verify:assert json.loads(path.read_bytes())==out
    else:
        with path.open('x',encoding='utf-8') as f:json.dump(out,f,indent=2);f.write('\n')
    print('023 registeredcase coverage:',out['covered'],'/',out['total'])
if __name__=='__main__':main()
