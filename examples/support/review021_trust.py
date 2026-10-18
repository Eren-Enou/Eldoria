"""Read-only semantic projection of independently reconstructed final trust.

Absent directed entries mean zero. Keep raw-map shape changes separate;no scorer,
simulation or outcome generator is imported,and no original evidence is rewritten.
"""
import gzip,json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
def main():
    d=ROOT/'experiments/021'
    a=json.loads(gzip.decompress((d/'analysis.json.gz').read_bytes()))['trials']
    rows=json.loads(gzip.decompress((d/'comparisons.json.gz').read_bytes()))['comparisons']
    def matrix(index):
        return [[agent['trust'].get(str(other),0) for other in range(3)] for agent in a[index]['final']]
    result={}
    for mode in ('LastSeen','Expire12','FreshReference','Unobserved'):
        selected=[r for r in rows if r['mode']==mode]
        changed=[[r['left'],r['right']] for r in selected if matrix(r['left'])!=matrix(r['right'])]
        shape=[[r['left'],r['right']] for r in selected if matrix(r['left'])==matrix(r['right']) and not r['final_trust_equal']]
        result[mode]=dict(value_differences=changed,shape_only_differences=shape)
    assert result==json.loads((d/'trust-comparisons.json').read_bytes())['modes']
    assert [len(result[m]['value_differences']) for m in result]==[10,6,4,15]
    assert result['Expire12']['shape_only_differences']==[[161,163]]
    assert (matrix(81)[1][2],matrix(81)[2][1],matrix(82)[1][2],matrix(82)[2][1])==(0,0,100,100)
    print('021 semantic trust PASS:LastSeen10/Expire12 6/Fresh4/Unobserved15;161/163 zero-map shape only')
if __name__=='__main__':main()
