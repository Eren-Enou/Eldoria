"""Verify every projected020 frame/action against its original source reference."""
import gzip,json,re,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
def resolve(t,path):
    value=t
    for key,index in re.findall(r'([a-z_]+)|\[([0-9]+)\]',path):value=value[key] if key else value[int(index)]
    return value
class FiniteDemo(unittest.TestCase):
    def test_all_original_frames_and_record_references(self):
        d=ROOT/'experiments/020';worlds=json.loads((d/'demo-data.json').read_bytes());by_record={w['record']:w for w in worlds};self.assertEqual(len(by_record),10)
        with gzip.open(d/'trials.jsonl.gz','rt',encoding='utf-8') as f:
            for ordinal,line in enumerate(f):
                if ordinal>=70:break
                if ordinal not in by_record:continue
                t=json.loads(line);w=by_record[ordinal];self.assertEqual(w['config'],t['config']);self.assertEqual(len(w['frames']),len(t['points']))
                for frame in w['frames']:
                    p=resolve(t,frame['ref']);self.assertEqual(frame['tick'],p['state']['tick']);self.assertEqual(frame['pending'],p['pending']);self.assertEqual(frame['stock'],p['cache']['stock']);self.assertEqual(frame['acquisitions'],p['acquisitions'])
                    self.assertEqual(frame['agents'],[{k:a[k] for k in ('id','food','hunger')} for a in p['state']['agents']])
                    consumed=sum(x['kind']=='Consumed' and x['tick']<=frame['tick'] for x in w['items'])
                    self.assertEqual(sum(a['food'] for a in frame['agents'])+frame['stock']+consumed,t['initial_total'])
                for item in w['items']:
                    source=resolve(t,item['ref']);self.assertGreaterEqual(w['frames'][item['frame']]['tick'],item['tick'])
                    if item['kind']=='Selected Acquire':
                        self.assertEqual(source['selected'],'Acquire');self.assertEqual(item['value']['own'],source['acquiring_base']['contact']['input']['own'])
                        self.assertEqual(item['value']['candidates'],source['candidates']);self.assertEqual(item['value']['cache'],source['cache'])
                        self.assertEqual(resolve(t,item['value']['input_ref']),source['acquiring_base']['contact']['input'])
                    else:self.assertEqual(item['value'],source)
        self.assertEqual((d/'demo.html').read_bytes(),(ROOT/'tools/history-explorer/finite-demo.html').read_bytes())
        ongoing=next(w for w in worlds if w['record']==43)
        self.assertEqual([x['kind'] for x in ongoing['items'] if x['tick']==52],['Accept','Granted','Selected Acquire'])
if __name__=='__main__':unittest.main()
