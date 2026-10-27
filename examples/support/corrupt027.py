"""Scratch mutations only; originals are never written."""
import copy,json,sys
from pathlib import Path
import review027 as r
def main():
    source=r.identity()
    t=next(t for t in r.read(sys.argv[1]) if t['config']['mode']=='Experience'and t['learning']['receipts']and any(d['adjustment']for d in t['learning']['decisions']))
    def receipt(x):return x['learning']['receipts'][0]
    def choice(x):return next(d for b in x['body']['history']['boundaries']for d in b['choices']if d['candidates'][4]['eligible'])
    def op(x):return x['body']['material']['operations'][0]
    cases=[]
    def case(name,mutate):
        v=copy.deepcopy(t);mutate(v)
        try:r.verify(v,source)
        except (AssertionError,KeyError,IndexError,ValueError,TypeError):cases.append(dict(name=name,rejected=True));return
        raise AssertionError(('accepted corruption',name))
    case('private experience imported',lambda x:receipt(x)['signal'].update(owner=(receipt(x)['signal']['owner']+1)%4))
    case('future receipt early',lambda x:receipt(x)['signal'].update(received=receipt(x)['signal']['received']-1))
    case('observer stock in cognition',lambda x:receipt(x)['signal'].update(stock=99))
    case('forgotten state restored',lambda x:x['learning']['cells'][0].append(dict(owner=0,received=0,success=True)))
    case('missing live receipt',lambda x:x['learning']['receipts'].pop(0))
    case('wrong outcome/update',lambda x:receipt(x)['after'][0].update(success=not receipt(x)['after'][0]['success']))
    case('capacity overflow',lambda x:receipt(x)['after'].extend([dict(owner=0,received=0,success=True)]*3))
    case('unregistered Prepare bonus',lambda x:choice(x)['candidates'][4].update(final_score=999))
    case('changed eligibility',lambda x:choice(x)['candidates'][4].update(eligible=False))
    def fictitious(x):
        core=x['body']['history']['final_state']
        while 'base'in core:core=core['base']
        core['cognition']['consumption'][0]['consumed']=2
    case('fictitious meal',fictitious)
    case('false later recipient-use attribution',lambda x:receipt(x)['signal'].update(recipient_ate=True))
    case('duplicate output',lambda x:x['body']['material']['operations'].append(copy.deepcopy(op(x))))
    def waste(x):next(o for o in x['body']['material']['operations']if o['outcome']=='Prepared')['waste_after']-=1
    case('missing waste',waste)
    case('hidden free replenishment',lambda x:op(x).update(raw_after=op(x)['raw_after']+2))
    case('modified registered parameters',lambda x:x['config']['world']['preparation'].update(raw_initial=16 if x['config']['world']['preparation']['raw_initial']!=16 else 0))
    case('wrong source identity',lambda x:x.update(source='0'*64))
    case('invented Taken delivery',lambda x:x['body']['material']['memories'][0].update(kind='Taken'))
    Path(sys.argv[2]).write_text(json.dumps(dict(source=source,cases=cases),indent=2)+'\n')
    print('027 rejected',len(cases),'scratch corruptions')
if __name__=='__main__':main()
