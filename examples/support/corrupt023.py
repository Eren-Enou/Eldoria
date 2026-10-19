"""Nineteen scratch-only independent-reader corruption categories."""
import copy,json
from pathlib import Path
import review023 as r

def pick(path,predicate):
    for t in r.read(path):
        if predicate(t):return t
    raise AssertionError('registered witness absent for corruption')

def starts(t):return [x for x in t['history']['transitions'] if not x['completed']]
def choices(t):return [d for b in t['history']['boundaries'] for d in b['choices']]
def base(t):return r.core(t['history'])
def run(path,output=None):
    simple=pick(path,lambda t:t['config']['case']=='paired' and t['config']['mode']=='Continuous')
    query=pick(path,lambda t:any(r.kind(x['commitment'])=='Query' for x in starts(t)))
    claim=pick(path,lambda t:len(t['history']['grants'])>=2)
    unfinished=pick(path,lambda t:any((x['active'] or x['pending']) and 'NominalStop' in x['labels'] for x in t['checkpoints']))
    long=pick(path,lambda t:t['config']['case']=='exhaustion' and t['config']['mode']=='Continuous')
    cases=[]
    def add(name,source,change):
        bad=copy.deepcopy(source);change(bad);cases.append((name,bad))
    add('world tick',simple,lambda t:t['history']['boundaries'][0].update(tick=t['history']['start']+1))
    add('reporting checkpoint',simple,lambda t:t['checkpoints'][0].update(tick=t['checkpoints'][0]['tick']+1))
    def reservation(t):
        tr=starts(t)[0];tr['commitment']['participants']=[(tr['commitment']['participants'][0]+1)%4]
    add('actor reservation',simple,reservation)
    add('commitment identity',simple,lambda t:starts(t)[0]['commitment'].update(id=999999))
    def deadline(t):
        cid=starts(t)[0]['commitment']['id']
        for tr in t['history']['transitions']:
            if tr['commitment']['id']==cid:tr['commitment']['deadline']+=1
    add('deadline',simple,deadline)
    def duplicate(t):
        trs=t['history']['transitions'];i=next(i for i,tr in enumerate(trs) if tr['completed']);trs.insert(i+1,copy.deepcopy(trs[i]))
    add('duplicate release',simple,duplicate)
    def skipped(t):
        trs=t['history']['transitions'];del trs[next(i for i,tr in enumerate(trs) if tr['completed'])]
    add('skipped completion',simple,skipped)
    def booking(t):
        ss=starts(t);a=ss[0];b=next(b for b in ss[1:] if b['tick']==a['tick']);b['commitment']['participants']=a['commitment']['participants'][:]
    add('double-booked actor',simple,booking)
    def transfer(t):
        event=next(e for e in base(t)['events'][t['history']['bootstrap_events']:] if e['transferred']);event['transferred']+=1
    add('double resource transfer',simple,transfer)
    def consent(t):
        resp=next(resp for b in t['history']['boundaries'] for resp in b['responses'] if isinstance(resp['selected'],dict));resp['selected']='Wait'
    add('false consent',simple,consent)
    def future(t):
        d=next(d for d in choices(t) if d['input']['retained']);d['input']['retained']['observation']['observed_at']=d['input']['legacy']['now']+1
    add('future observation',simple,future)
    add('incorrect need pulse',simple,lambda t:t['history']['needs'][0]['after'].__setitem__(0,t['history']['needs'][0]['after'][0]+1))
    def missing(t):
        i=next(i for i,g in enumerate(t['history']['grants']) if g['outcome']=='Granted');del t['history']['grants'][i]
    add('missing claimed output',claim,missing)
    def ghost(t):
        h=t['history'];nominal=h['start']+t['config']['horizon'];b=next(b for b in h['boundaries'] if b['tick']>=nominal);b['choices']=[copy.deepcopy(choices(t)[0])]
    add('ghost action after nominal cutoff',simple,ghost)
    def lost(t):
        row=next(x for x in t['checkpoints'] if 'NominalStop' in x['labels']);row['active']=[];row['pending']=[]
    add('lost unfinished commitment',unfinished,lost)
    add('false terminal snapshot',simple,lambda t:t['history']['final_agents'][0].update(food=t['history']['final_agents'][0]['food']+1))
    def recovery(t):
        for d in reversed(choices(t)):
            own=d['input']['legacy']['own'];retained={m['event'] for m in own['memories']}
            for event in base(t)['events']:
                if own['id'] in event['participants'] and event['id'] not in retained:
                    old=event['interpretations'][event['participants'].index(own['id'])]
                    if own['memories']:own['memories'][0]=copy.deepcopy(old)
                    else:own['memories'].append(copy.deepcopy(old))
                    return
        raise AssertionError('no unavailable episode witness')
    add('restored forgotten cognition',long,recovery)
    add('invalid arbitration',simple,lambda t:t['history']['admissions'][0].update(reason='Capacity'))
    add('wrong event reference',simple,lambda t:base(t)['events'][-1].update(id=999999))
    results=[]
    for name,bad in cases:
        try:r.verify(bad)
        except (AssertionError,KeyError,IndexError,ValueError,StopIteration) as error:
            results.append(dict(category=name,rejected=True,error=type(error).__name__,detail=str(error)))
        else:raise AssertionError('corruption accepted: '+name)
    assert len(results)==19
    if output:Path(output).write_text(json.dumps(results,indent=2)+'\n',encoding='utf-8')
    print('023 scratch-only corruption categories rejected:',len(results))
    return results
if __name__=='__main__':
    import argparse
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('--output');a=p.parse_args();run(a.archive,a.output)
