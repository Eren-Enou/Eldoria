"""C1 independent reader:unchanged valuation/physics/cognition;one response seam."""
import argparse,copy,gzip,json,types
from pathlib import Path
import material025,review015 as legacy,review026 as frozen
from review026_continuation_equivalence import direct
from identity026_c1 import ROOT,DIRECTORY,identity,sha

class OldProxy:
    def __getattr__(self,name):return direct if name=='response'else getattr(legacy,name)
READERS={}
def complete_reader(mode):
    if mode not in READERS:
        p=ROOT/'examples/support/review025.py';source=p.read_text(encoding='utf-8')
        for seam in ('import material025\n','import review015 as old\n'):
            assert source.count(seam)==1;source=source.replace(seam,'')
        adapter=types.SimpleNamespace(Physical=material025.Physical,Slots=material025.Slots,
            selection=lambda i,o,v:frozen.selection(i,o,v,mode))
        ns={'__file__':str(p),'__name__':'independent026c1_'+mode,'material025':adapter,'old':OldProxy()}
        exec(compile(source,str(p),'exec'),ns);READERS[mode]=ns['verify']
    return READERS[mode]
INNER=None
def inner_verify(trial,expected=None,findings=None):
    global INNER
    if INNER is None:
        p=ROOT/'examples/support/review026.py';source=p.read_text(encoding='utf-8')
        assert source.count("metrics=reader(cfg['mode'])(body)")==1
        ns={'__file__':str(p),'__name__':'independent026c1_inner'}
        exec(compile(source,str(p),'exec'),ns)
        ns['reader']=complete_reader;INNER=ns['verify']
    return INNER(trial,expected,findings)

def response_context(body):
    h=body['history'];proposals=[];expected_extensions=[]
    admitted={(a['source'],a['invitation']['proposer']):a['tick']for a in h['admissions']}
    for bi,b in enumerate(h['boundaries']):
        for ri,response in enumerate(b['responses']):
            assert set(response)=={'input','candidates','selected'}
            i=response['input'];assert set(i)=={'own','outgoing','incoming'}
            owner=i['own']['own']['id'];now=i['own']['now']
            cs,_=legacy.initiate(i['own'],'History')
            pending=[p for p in proposals if p['proposer']==owner and admitted.get((p['source'],p['proposer']),2**64-1)>=now]
            selected='Wait'if not pending else dict(Invite=dict(recipient=pending[0]['recipient'],kind=pending[0]['kind']))
            assert i['outgoing']==dict(input=i['own'],mode='History',candidates=cs,selected=selected),'C1 response pending/current contact reconstruction'
            candidates,chosen=direct(i)
            assert (response['candidates'],response['selected'])==(candidates,chosen)
            if selected!='Wait'and not any(c['intent']==selected for c in cs):
                expected_extensions.append(dict(boundary=bi,response=ri,owner=owner,tick=now,outgoing=selected))
        for d in b['choices']:
            if d['selected']=='Invite':
                inv=d['contact']['Invite'];proposals.append(dict(source=bi,proposer=d['input']['legacy']['own']['id'],recipient=inv['recipient'],kind=inv['kind']))
    return expected_extensions

def verify(trial,expected=None,findings=None):
    assert set(trial)=={'schema','source_freeze_sha256','correction','trial','response_extensions'}
    assert trial['schema']=='experiment-026-c1-trial/v1'and trial['correction']=='ContinueZeroBeforeLookup'
    freeze=DIRECTORY/'source-freeze.json';source=sha(freeze)if freeze.exists()else'UNFROZEN'
    assert trial['source_freeze_sha256']==source
    inner=trial['trial'];cfg=inner['config']
    if source=='UNFROZEN':assert cfg['mode']=='Baseline'or cfg['source025']<21,'unknown corrected outcome before source freeze'
    metrics=inner_verify(inner,expected)
    extensions=response_context(inner['body'])
    assert trial['response_extensions']==extensions,'C1 extension audit mismatch'
    metrics['response_extensions']=extensions
    if findings is not None:assert metrics==findings
    return metrics

def read(path):
    with (gzip.open(path,'rb')if str(path).endswith('.gz')else open(path,'rb'))as stream:
        for line in stream:yield json.loads(line)
def main():
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('--scope',choices=('baseline','prefix','matrix'),default='matrix');p.add_argument('--output');p.add_argument('--expected');a=p.parse_args()
    matrix=identity();expected=matrix[::2]if a.scope=='baseline'else matrix[:43]if a.scope=='prefix'else matrix
    rows=[]
    for i,t in enumerate(read(a.archive)):
        assert i<len(expected)
        try:rows.append(verify(t,expected[i]))
        except Exception as e:raise AssertionError(('C1 independent row',i))from e
    assert len(rows)==len(expected)
    if a.expected:assert rows==json.loads(Path(a.expected).read_bytes())
    if a.output:
        with open(a.output,'x',encoding='utf-8')as out:json.dump(rows,out,indent=2);out.write('\n')
    print('C1 independent valuation/physics/cognition/consent/pending intentions/extension audit PASS',len(rows))
if __name__=='__main__':main()
