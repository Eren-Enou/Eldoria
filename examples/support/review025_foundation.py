"""Fresh025 protection review: stdlib-only scalar/phase reconstruction.
No evaluator, simulator, viewer, or prior review module imports.
"""
import argparse
import copy
import gzip
import hashlib
import json
import subprocess
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]

def scalar(choice):
    inp=choice['input'];local=inp['legacy'];own=local['own'];cell=inp['raw_retained']
    duration=None
    if cell:
        o=cell['observation']
        if (cell['owner']==own['id'] and o['id']==0 and o['observed_at']<=local['now']
            and max(0,o['stock']-cell['own_takes'])>=2 and o['accessible']
            and o['extraction_ticks']>0 and o['preparation_ticks']>0
            and (o['input_units'],o['edible_units'],o['waste_units'])==(2,1,1)):
            duration=o['extraction_ticks']+o['preparation_ticks']
    eligible=(own['food']==0 and own['hunger']>0
        and any(p['id']==own['id'] and p['available'] for p in local['people'])
        and duration is not None and 0<duration<=33 and local['now']+duration<=local['end'])
    expected=dict(action='Prepare',eligible=eligible,base_score=own['hunger'] if eligible else -1,
        meal_relief=0,work_effort=duration if eligible else 0,
        final_score=own['hunger']-duration if eligible else -1)
    terms=choice['candidates']
    assert [t['action'] for t in terms]==['Wait','Consume','Invite','Acquire','Prepare']
    assert terms[-1]==expected
    assert not (terms[1]['eligible'] and terms[-1]['eligible']), 'Impossible co-eligibility'
    assert not (inp['retained'] is not None and cell is not None), 'Two source slots'
    assert choice['selected']==max(terms,key=lambda term:term['final_score'])['action']
    return eligible

def phases(t):
    settings=t['config']['preparation'];raw=settings['raw_initial'];waste=0
    h=t['history'];processes={p['commitment']:p for p in t['material']['processes']}
    assert len(processes)==len(t['material']['processes'])
    starts={r['commitment']['id']:r['commitment'] for r in h['transitions'] if not r['completed']}
    ends={r['commitment']['id']:r['tick'] for r in h['transitions'] if r['completed']}
    escrow={};done=set();outcomes={k:0 for k in ['Extracted','Insufficient','Prepared','Failed']}
    rank=lambda p:p['owner'] if t['config']['world']['order']=='Ascending' else 3-p['owner'] if t['config']['world']['order']=='Descending' else (p['owner']+4-p['start']%4)%4
    expected=sorted((tick,phase,rank(p),p['commitment']) for p in processes.values()
        for phase,tick in enumerate((p['extract_at'],p['finish_at'])))
    operations=t['material']['operations'];assert len(operations)==len(expected)
    for n,(key,o) in enumerate(zip(expected,operations,strict=True)):
        tick,phase,_,cid=key;p=processes[cid];c=starts[cid]
        assert (c['work'],c['participants'],c['start'],c['deadline'])==('Prepare',[p['owner']],p['start'],p['finish_at'])
        assert p['extract_at']==p['start']+settings['extraction_ticks']
        assert p['finish_at']==p['extract_at']+settings['preparation_ticks']==ends[cid]
        assert (o['id'],o['tick'],o['commitment'],o['owner'],o['source'])==(n,tick,cid,p['owner'],0)
        assert o['raw_before']==raw and o['waste_before']==waste
        if phase==0:
            assert cid not in escrow and o['escrow_before']==0
            success=raw>=2
            escrow[cid]=2 if success else 0;raw-=escrow[cid]
            assert o['outcome']==('Extracted' if success else 'Insufficient')
            assert o['food_after']==o['food_before']
        else:
            assert cid in escrow and cid not in done and o['escrow_before']==escrow[cid]
            success=escrow[cid]==2;escrow[cid]=0;waste+=int(success);done.add(cid)
            assert o['outcome']==('Prepared' if success else 'Failed')
            assert o['food_after']==o['food_before']+int(success)
        assert (o['raw_after'],o['escrow_after'],o['waste_after'])==(raw,escrow[cid],waste)
        outcomes[o['outcome']]+=1
    assert raw==t['material']['final_raw'] and waste==t['material']['final_waste']
    assert not any(escrow.values()) and not t['material']['final_active']
    core=h['final_state']
    while 'base' in core:core=core['base']
    consumed=sum(m['consumed'] for m in core['cognition']['consumption'])
    assert h['initial_total']==h['initial_stock']+settings['raw_initial']+sum(a['food'] for a in h['initial'])
    assert h['initial_total']==raw+waste+h['final_stock']+sum(a['food'] for a in h['final_agents'])+consumed
    return outcomes

def inspect():
    rows=[];first=None
    with gzip.open(ROOT/'experiments/025/trials.jsonl.gz','rb') as stream:
        for i,line in enumerate(stream):
            t=json.loads(line);eligible=sum(scalar(d) for b in t['history']['boundaries'] for d in b['choices'])
            rows.append(dict(index=i,eligible=eligible,outcomes=phases(t),history_sha256=hashlib.sha256(line.rstrip(b'\n')).hexdigest()))
            if i==1:first=t
    assert len(rows)==50
    totals=lambda r:{k:sum(x['outcomes'][k] for x in r) for k in ['Extracted','Insufficient','Prepared','Failed']}
    assert totals(rows[:48])==dict(Extracted=77,Insufficient=40,Prepared=77,Failed=40)
    assert totals(rows)==dict(Extracted=81,Insufficient=44,Prepared=81,Failed=44)
    failures=[]
    for name,mutate,check in [
        ('hunger',lambda t:t['history']['boundaries'][0]['choices'][0]['input']['legacy']['own'].update(hunger=79),lambda t:scalar(t['history']['boundaries'][0]['choices'][0])),
        ('extra_score',lambda t:t['history']['boundaries'][0]['choices'][0]['candidates'][-1].update(final_score=75),lambda t:scalar(t['history']['boundaries'][0]['choices'][0])),
        ('waste',lambda t:t['material']['operations'][4].update(waste_after=0),phases),
        ('deadline',lambda t:t['material']['processes'][0].update(finish_at=7),phases),
    ]:
        t=copy.deepcopy(first);mutate(t)
        try:check(t)
        except AssertionError:failures.append(name)
        else:raise AssertionError(('Corruption accepted',name))
    assert len(failures)==4
    ancestry=[]
    for a,b in [('68b4c70','b2487dc'),('b2487dc','f7b207e'),('f7b207e','a755019'),('a755019','a3a7150')]:
        subprocess.run(['git','merge-base','--is-ancestor',a,b],cwd=ROOT,check=True)
        ancestry.append([subprocess.check_output(['git','rev-parse',a],cwd=ROOT,text=True).strip(),subprocess.check_output(['git','rev-parse',b],cwd=ROOT,text=True).strip()])
    return dict(schema='woi-025-foundation-review/v1',worlds=rows,primary=totals(rows[:48]),all=totals(rows),
        controls=failures,publication_ancestry=ancestry,
        chronology_qualification='Behavioral registration published before runtime; corrected source freeze, gate and outcomes co-published atf7b207e. Pre-corrected-run freeze is recorded by the runtime gate, not independently established by a separate publication timestamp.',
        decision='ACCEPT WITH QUALIFICATION; scientific protection only; no adoption')

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--output',type=Path);parser.add_argument('--expected',type=Path);args=parser.parse_args()
    result=inspect()
    if args.expected:assert result==json.loads(args.expected.read_bytes())
    if args.output:
        with args.output.open('x',encoding='utf8') as out:json.dump(result,out,indent=2);out.write('\n')
    print('025 fresh review:50 scalar/phase/ledger reconstructions,4 controls and publication ancestry PASS')
