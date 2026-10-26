"""Same frozen026 comparisons through an explicit C1 envelope projection."""
import argparse,json,types
from pathlib import Path
import review026 as frozen,review026_c1 as reader
from identity026_c1 import ROOT,DIRECTORY,identity,sha
def inner_rows(path):
    source=sha(DIRECTORY/'source-freeze.json')if (DIRECTORY/'source-freeze.json').exists()else'UNFROZEN'
    for row in reader.read(path):
        assert row['schema']=='experiment-026-c1-trial/v1'and row['correction']=='ContinueZeroBeforeLookup'and row['source_freeze_sha256']==source
        yield row['trial']
def analyze(archive,findings):
    identity();p=ROOT/'examples/support/analyze026.py';source=p.read_text(encoding='utf-8')
    seam='import review026 as r\n';assert source.count(seam)==1
    ns={'__file__':str(p),'__name__':'independent026c1_comparisons','r':types.SimpleNamespace(read=inner_rows,review025_food_fate=frozen.review025_food_fate)}
    exec(compile(source.replace(seam,''),str(p),'exec'),ns)
    result=ns['analyze'](archive,findings);result['schema']='experiment-026-c1-comparisons/v1'
    result['correction']='ContinueZeroBeforeLookup;original formula/matrix unchanged;v1 remains failed'
    result['response_extensions']=[dict(row=i,source025=f['source025'],mode=f['mode'],references=f['response_extensions'])for i,f in enumerate(findings)if f['response_extensions']]
    return result
def main():
    p=argparse.ArgumentParser();p.add_argument('archive');p.add_argument('findings');p.add_argument('--output');p.add_argument('--expected');a=p.parse_args()
    result=analyze(a.archive,json.loads(Path(a.findings).read_bytes()))
    if a.expected:assert result==json.loads(Path(a.expected).read_bytes())
    if a.output:
        with open(a.output,'x',encoding='utf-8')as out:json.dump(result,out,indent=2);out.write('\n')
    print('C1 complete50 paired comparisons/first causes/report placebos PASS')
if __name__=='__main__':main()
