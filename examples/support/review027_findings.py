"""Post-outcome complementary027 null-mechanism audit, stdlib only.
No transition implementation or original evaluator/reconstruction imports.
"""
import collections,copy,gzip,hashlib,json,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def digest(b):return hashlib.sha256(b).hexdigest()
def expect(cells):return 50 if not cells else 100*sum(c['success']for c in cells)//len(cells)
def main():
    counts=collections.Counter();margins=collections.Counter();receipt_kinds=collections.Counter();witnesses={};physical_equal=[]
    path=ROOT/'experiments/027/trials.jsonl.gz'
    with gzip.open(path,'rb')as stream:
        while line:=stream.readline():
            a=json.loads(line);b=json.loads(stream.readline());assert(a['config']['mode'],b['config']['mode'])==('Fixed','Experience')
            source=a['config']['source025'];assert source==b['config']['source025']
            aa=copy.deepcopy(a['body']);bb=copy.deepcopy(b['body'])
            bi=bb['history'];cells=[[]for _ in range(4)];timeline=[]
            for m in bb['material']['memories']:
                if m['kind']in('Taken','Insufficient'):timeline.append((m['tick'],0,m['id'],m))
            for bn,q in enumerate(bi['boundaries']):
                for cn,c in enumerate(q['choices']):timeline.append((c['input']['legacy']['now'],1,(bn,cn),c))
            expected_decisions={};signal_records=[]
            for now,kind,ref,row in sorted(timeline,key=lambda x:(x[0],x[1],x[2])):
                owner=row['owner']if kind==0 else row['input']['legacy']['own']['id'];before=copy.deepcopy(cells[owner]);live=[c for c in before if c['received']+24>now]
                counts['expired_before_terminal']+=len(before)-len(live)
                if kind==0:
                    receipt_kinds[row['kind']]+=1
                    prior=expect(live)
                    if len(live)==2:
                        evicted=live.pop(0);counts['capacity_evictions']+=1
                        witnesses.setdefault('capacity_eviction',dict(source025=source,memory=ref,tick=now,owner=owner,before=before,evicted=evicted))
                    signal=dict(owner=owner,received=now,success=row['kind']=='Taken');live.append(signal)
                    counts['updates_changing_expectation']+=prior!=expect(live)
                    physical=bb['material']['operations'][row['physical']]
                    assert physical['owner']==owner and physical['tick']==now
                    assert physical['outcome']==('Extracted'if signal['success']else'Insufficient')
                    signal_records.append(dict(memory=ref,signal=signal,physical=physical))
                else:
                    own=row['input']['legacy']['own'];term=row['candidates'][4];n=expect(live)
                    assert all(c['owner']==owner and c['received']<=now for c in live)
                    expected_decisions[ref]=copy.deepcopy(live)
                    if before!=live:witnesses.setdefault('decision_expiry',dict(source025=source,boundary=ref[0],choice=ref[1],tick=now,before=before,retained=live))
                    if live and row['input']['raw_retained']is None:
                        assert not term['eligible']
                        counts['retained_feedback_without_raw_knowledge']+=1
                        witnesses.setdefault('raw_absent_learning_survives',dict(source025=source,boundary=ref[0],choice=ref[1],tick=now,owner=owner,retained=live,prepare=term))
                    if term['eligible']and live:
                        assert n==100,'tested eligible feedback is success-only'
                        counts['eligible_retained_sites']+=1
                        delta=(n-50)//5
                        old=aa['history']['boundaries'][ref[0]]['choices'][ref[1]]
                        assert old['input']==row['input']
                        h=own['hunger'];o=row['input']['raw_retained']['observation'];d=o['extraction_ticks']+o['preparation_ticks'];fixed=min(100,h+5*(d//4))-d
                        assert term['final_score']==fixed+delta==old['candidates'][4]['final_score']+delta
                        others=max(row['candidates'][:4],key=lambda x:x['final_score'])
                        gap=fixed-others['final_score'];margins[gap]+=1
                        if old['selected']=='Prepare':counts['already_preferred_Prepare']+=1
                        else:
                            counts['other_candidate_remains_preferred']+=1
                            assert fixed+delta<=others['final_score']
                        assert old['selected']==row['selected']
                        witnesses.setdefault('score_only_success',dict(source025=source,boundary=ref[0],choice=ref[1],owner=owner,tick=now,retained=live,receipts=[x for x in signal_records if x['signal']in live],hunger=h,duration=d,fixed_prepare=fixed,learned_prepare=fixed+delta,other_best=others,selected=row['selected']))
                    fixed_term=aa['history']['boundaries'][ref[0]]['choices'][ref[1]]['candidates'][4]
                    # Remove only the prospectively permitted score representation.
                    row['candidates'][4]=copy.deepcopy(fixed_term)
                    bb['valuations'][next(i for i,v in enumerate(bb['valuations'])if(v['boundary'],v['choice'])==ref)]['selection']=copy.deepcopy(aa['valuations'][next(i for i,v in enumerate(aa['valuations'])if(v['boundary'],v['choice'])==ref)]['selection'])
                cells[owner]=live
            assert aa==bb,'something other than Prepare score representations changed'
            physical_equal.append(source)
            assert all(d['retained']==expected_decisions[(d['boundary'],d['choice'])]for d in b['learning']['decisions'])
            assert b['learning']['cells']==[[],[],[],[]]
            counts['terminal_expirations']+=sum(len(c)for c in cells)
    assert len(physical_equal)==50
    activation=json.loads((ROOT/'experiments/027/activation.json').read_bytes())
    for rev,name in [(activation['runtime_revision'],'experiments/027/source-freeze.json'),(activation['registration_revision'],'experiments/027/registration-bindings.json')]:
        frozen=subprocess.check_output(['git','show',rev+':'+name],cwd=ROOT)
        assert frozen.replace(b'\r\n',b'\n')==(ROOT/name).read_bytes().replace(b'\r\n',b'\n'),'published manifest membership changed'
    archive=json.loads((ROOT/'experiments/027/archive-manifest.json').read_bytes());assert digest(path.read_bytes())==archive['gzip_sha256']
    result=dict(pairs_body_equal_except_registered_scores=physical_equal,counts=dict(counts),receipt_kinds=dict(receipt_kinds),fixed_prepare_minus_best_other_histogram=dict(sorted(margins.items())),witnesses=witnesses,publication_manifest_membership='exact',review_scope='post-outcome complementary audit; no new outcomes or policy change')
    with open(sys.argv[1],'x',encoding='utf-8')as out:json.dump(result,out,indent=2);out.write('\n')
    print(json.dumps(dict(counts=dict(counts),receipt_kinds=dict(receipt_kinds),margins=dict(sorted(margins.items())))))
if __name__=='__main__':main()
