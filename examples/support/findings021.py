"""Read-only scientific anchors, separate from the021 simulation and scorer."""
import collections,gzip,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def main():
    d=ROOT/'experiments/021';a=json.loads(gzip.decompress((d/'analysis.json.gz').read_bytes()))['trials'];rs=json.loads(gzip.decompress((d/'comparisons.json.gz').read_bytes()))['comparisons']
    def pair(l,r):return next(x for x in rs if x['left']==l and x['right']==r)
    assert len(a)==214 and len(rs)==157
    m=collections.Counter()
    for x in a:m.update(x['metrics'])
    assert (m['Granted'],m['Depleted'],m['consumed'],m['transfers'],m['queries'],m['grant_after_expiry'])==(396,203,934,671,34,6)
    for mode,count,choices,period in [('LastSeen',57,43,14),('Expire12',44,33,12),('FreshReference',28,6,6),('Unobserved',28,24,24)]:
        rows=[x for x in rs if x['mode']==mode]
        assert (len(rows),sum(x['first_choice_difference'] is not None for x in rows),sum(not all(x['period_material_equal']) for x in rows))==(count,choices,period)
        if mode!='Unobserved':assert all(x['final_material_equal'] for x in rows)
    useful=pair(81,82);f=useful['first_choice_difference']
    assert (f['tick'],f['owner'],f['left']['selected'],f['right']['selected'])==(28,0,'Invite','Acquire')
    assert f['left']['retained'] is None and f['right']['retained']['observation']['observed_at']==24
    assert useful['transfers']==[22,43] and useful['hunger_area']==[[45230,39035,45525],[43035,35215,44115]]
    assert useful['period_material_equal']==[True]+[False]*11+[True]*12
    # Same finite supply, different distribution/timing: not a universal welfare rank.
    mixed=pair(1,2);assert mixed['hunger_area']==[[11560,10200,8425],[13040,9430,9430]]
    reversal=pair(186,187);assert reversal['hunger_area']==[[14400,14400,12080],[14400,14400,12120]]
    assert a[153]['metrics']['Depleted']==3 and a[154]['metrics']['Depleted']==1 and a[153]['hunger_area']==a[154]['hunger_area']
    assert 'Depleted' not in a[163]['metrics'] and a[162]['metrics']['Depleted']==1
    assert all(x<y for x,y in zip(a[162]['hunger_area'],a[163]['hunger_area']))
    assert all(pair(l,r)['first_choice_difference'] is None for l,r in [(21,22),(41,42),(91,92),(121,122)])
    assert pair(31,32)['first_choice_difference'] is not None and pair(31,32)['meals_equal'] and all(pair(31,32)['period_material_equal'])
    print('021 scientific anchors PASS:useful/mixed/costly/null/inert/reconvergence/expiry tradeoff;no global ranking')
if __name__=='__main__':main()
