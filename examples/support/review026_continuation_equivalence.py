"""Read-only post-failure proof check; no successor simulation or policy tuning.

Compare the established clamp-and-reselect rule with direct Continue=0 construction
on every recorded025 and026-v1 response. This does not repair the frozen producer.
"""
import argparse,copy,gzip,hashlib,json
from pathlib import Path
import review015 as old
from identity026 import ROOT,identity

def direct(input):
    own=input['own'];actor=own['own'];available=next(p['available']for p in own['people']if p['id']==actor['id'])
    candidates=[dict(reply='Wait',score=0)]
    for invitation in input['incoming']:
        score=-1 if not available else old.resource_value(actor,invitation['proposer'])if invitation['kind']=='Resource'else own['profile']['relationship_goal']-own['profile']['privacy']-2
        candidates.append(dict(reply=dict(Accept=invitation['proposer']),score=score))
    if input['outgoing']['selected']!='Wait':candidates.append(dict(reply='Continue',score=0))
    return candidates,max(candidates,key=lambda c:c['score'])['reply']

def inspect():
    identity();rows=[];digest=hashlib.sha256();missing=[]
    for label,path in [('original025',ROOT/'experiments/025/trials.jsonl.gz'),('frozen026prefix',ROOT/'experiments/026/failure/attempt-001.jsonl.gz')]:
        worlds=responses=pending=0
        with gzip.open(path,'rb')as stream:
            for ordinal,line in enumerate(stream):
                t=json.loads(line);h=t['body']['history']if label=='frozen026prefix'else t['history'];worlds+=1
                for bi,b in enumerate(h['boundaries']):
                    for ri,response in enumerate(b['responses']):
                        i=response['input'];cs,_=old.response(i)
                        for c in cs:
                            if c['reply']=='Continue':c['score']=0
                        expected=(cs,max(cs,key=lambda c:c['score'])['reply'])
                        assert expected==direct(i)==(response['candidates'],response['selected'])
                        digest.update(json.dumps(i,sort_keys=True,separators=(',',':')).encode()+b'\n');responses+=1
                        if i['outgoing']['selected']!='Wait':
                            pending+=1
                            if not missing:
                                mutated=copy.deepcopy(i);selected=mutated['outgoing']['selected']
                                mutated['outgoing']['candidates']=[c for c in mutated['outgoing']['candidates']if c['intent']!=selected]
                                try:old.response(mutated)
                                except StopIteration:pass
                                else:raise AssertionError('missing-membership control did not reproduce undefined lookup')
                                assert direct(mutated)==direct(i)
                                missing.append(dict(archive=label,ordinal=ordinal,boundary=bi,response=ri,selected=selected,
                                    kind='synthetic membership-only local-input control;not an executed world',
                                    original_rule='Continue clamped0;first maximum',direct_candidates=direct(mutated)[0],direct_selected=direct(mutated)[1]))
        rows.append(dict(archive=label,worlds=worlds,responses=responses,pending_outgoing_responses=pending))
    assert rows[0]['worlds']==50 and rows[1]['worlds']==43 and missing
    return dict(schema='experiment-026-response-equivalence-review/v1',status='PASS read-only defined-response equivalence;not a successor execution gate',
        archives=rows,input_stream_sha256=digest.hexdigest(),missing_membership_control=missing,
        proof='On the defined domain, c::respond returns the same input,Wait0,incoming Accept scores and optional Continue0,then reselects first maximum. The looked-up outgoing score and preliminary e::respond selection are discarded. Direct zero construction yields the identical public Response. Undefined missing-member inputs become defined under the already-established zero rule;this extension must be declared prospectively in a separate successor.',
        limits='Recorded inputs overlap across archives;counts are records,not independent statistical evidence. No original/frozen source changed;no new trajectory or welfare conclusion.')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--output',type=Path);p.add_argument('--expected',type=Path);a=p.parse_args();result=inspect()
    if a.expected:assert result==json.loads(a.expected.read_bytes())
    if a.output:
        with a.output.open('x',encoding='utf-8')as out:json.dump(result,out,indent=2);out.write('\n')
    print(json.dumps(dict(status=result['status'],archives=result['archives'])))
