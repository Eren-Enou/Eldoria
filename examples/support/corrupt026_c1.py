"""Frozen14 corruption categories through C1 verifier plus three C1 rule checks."""
import argparse,copy,json,types
import review026 as frozen,review026_c1 as reader
from identity026_c1 import ROOT,identity
def inherited(anchor):
    p=ROOT/'examples/support/corrupt026.py';source=p.read_text(encoding='utf-8');seam='import review026 as r\n';assert source.count(seam)==1
    def verify(inner,expected=None,findings=None):
        outer=dict(anchor,trial=inner);return reader.verify(outer,expected,findings)
    ns={'__file__':str(p),'__name__':'independent026c1_corruptions','r':types.SimpleNamespace(verify=verify,review025_food_fate=frozen.review025_food_fate)}
    exec(compile(source.replace(seam,''),str(p),'exec'),ns)
    return ns['corruptions'](anchor['trial'])
def inspect(archive):
    anchors={}
    for t in reader.read(archive):
        cfg=t['trial']['config']
        if cfg['mode']=='Baseline'and cfg['source025']in (0,1):anchors[cfg['source025']]=t
        if len(anchors)==2:break
    rejected=inherited(anchors[1]);anchor=anchors[0]
    def first_response(t):return next(r for b in t['trial']['body']['history']['boundaries']for r in b['responses']if any(c['reply']=='Continue'for c in r['candidates']))
    def nonzero(t):next(c for c in first_response(t)['candidates']if c['reply']=='Continue')['score']=1
    def bad_reference(t):t['response_extensions'].append(dict(boundary=0,response=0,owner=0,tick=0,outgoing='Wait'))
    def tied_accept(t):
        r=first_response(t);top=max(c['score']for c in r['candidates']);tied=[c for c in r['candidates']if c['score']==top];assert len(tied)>=2;r['selected']=tied[1]['reply']
    for name,mutate in [('continue_nonzero',nonzero),('extension_reference',bad_reference),('incoming_tie',tied_accept)]:
        t=copy.deepcopy(anchor);mutate(t)
        try:reader.verify(t)
        except (AssertionError,KeyError,ValueError,TypeError):rejected.append(name)
        else:raise AssertionError(('C1 corruption accepted',name))
    assert len(rejected)==17
    return dict(anchors={'inherited14':'source0251/Baseline','c1_rule':'source0250/Baseline'},rejected=rejected)
def main():
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('--output');a=p.parse_args();identity();result=inspect(a.archive)
    if a.output:
        with open(a.output,'x',encoding='utf-8')as out:json.dump(result,out,indent=2);out.write('\n')
    print('C1 independent corruption checks PASS',len(result['rejected']))
if __name__=='__main__':main()
