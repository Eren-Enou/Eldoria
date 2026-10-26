"""Post-failure, read-only evidence gate. This cannot certify completed026 science.

Keeps the preregistered16-file runtime/readers unchanged. Independently reconstructs
the completed prefix, reproduces the same failure, and rejects completion claims.
"""
import gzip,hashlib,json,subprocess,sys,uuid
from pathlib import Path
import analyze026 as a,corrupt026 as c,review026 as r
from identity026 import ROOT,identity,sha

def comparisons(trials,findings):
    pairs=[]
    for i in range(0,42,2):
        left,right=trials[i:i+2];cause=a.first_cause(left,right)
        ls,rs=a.state(left,findings[i]),a.state(right,findings[i+1])
        pairs.append(dict(source025=right['config']['source025'],first_cause=cause,baseline=ls,treatment=rs,
            hunger_area_delta=[y-x for x,y in zip(ls['hunger_area'],rs['hunger_area'])],
            meal_delta=[y-x for x,y in zip(ls['meals'],rs['meals'])]))
    return dict(status='PARTIAL DIAGNOSTIC ONLY;43 completed rows,21 complete pairs,one orphan;not the registered100-world result',pairs=pairs)

def main():
    directory=Path(sys.argv[1])if len(sys.argv)>1 else ROOT/'experiments/026/failure'
    m=json.loads((directory/'manifest.json').read_bytes())
    assert m['status']=='INCONCLUSIVE_EXECUTION_FAILURE'
    assert (m['intended_worlds'],m['completed_worlds'],m['failed_ordinal'],m['failed_source025'])==(100,43,43,21)
    assert m['source_freeze_sha256']==sha(ROOT/'experiments/026/source-freeze.json')
    assert m['post_failure_reader_sha256']==sha(Path(__file__).resolve())
    matrix=identity()
    for name,binding in m['members'].items():
        assert Path(name).name==name
        data=(directory/name).read_bytes()
        if binding['mode']=='lf':data=data.replace(b'\r\n',b'\n')
        else:assert binding['mode']=='raw'
        assert hashlib.sha256(data).hexdigest()==binding['sha256'],name
    with gzip.open(directory/'attempt-001.jsonl.gz','rb')as stream:raw=stream.read()
    assert len(raw.splitlines())==43 and hashlib.sha256(raw).hexdigest()==m['raw_prefix_sha256']
    trials=list(r.read(directory/'attempt-001.jsonl.gz'));expected=json.loads((directory/'findings.json').read_bytes())
    assert len(trials)==len(expected)==43
    for i,t in enumerate(trials):r.verify(t,matrix[i],expected[i])
    assert comparisons(trials,expected)==json.loads((directory/'comparisons.json').read_bytes())
    assert c.corruptions(trials[2])==json.loads((directory/'corruptions.json').read_bytes())['rejected']
    output=(ROOT/'target'/('026-failure-verify-'+uuid.uuid4().hex)).resolve()
    assert output.is_relative_to((ROOT/'target').resolve());output.mkdir()
    with (output/'stdout.log').open('wb')as stdout,(output/'stderr.log').open('wb')as stderr:
        process=subprocess.run([str(ROOT/'target/delayed-work-valuation/release/delayed-work-valuation.exe'),'matrix',str(output/'prefix.jsonl')],cwd=ROOT,stdout=stdout,stderr=stderr)
    assert process.returncode==101,'Frozen failure changed;do not reinterpret as completed science'
    assert (output/'prefix.jsonl').read_bytes()==raw
    error=(output/'stderr.log').read_text(encoding='utf-8').replace('\\','/')
    assert '026 row43 source21 ProjectedNeed'in error and 'src/encounters.rs:210:14'in error
    assert 'Option::unwrap()'in error and 'None'in error
    identity()
    print('026 FAILURE EVIDENCE PASS:43 exact prefix rows/21 complete pairs/43 independent reconstructions/14 scratch rejections/same frozen row43 panic. Registered100-world scientific gate remains FAILED;no behavioral fix or adoption.')
if __name__=='__main__':main()
