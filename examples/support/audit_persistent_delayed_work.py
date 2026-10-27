"""Read-only engineering evidence audit;no simulation transitions or archive writes."""
import argparse,gzip,hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def sha(data):return hashlib.sha256(data).hexdigest()
def audit(directory):
 out=Path(directory).resolve(strict=True);out.relative_to(ROOT/'target')
 receipt=json.loads((out/'receipt.json').read_bytes());worlds=receipt['worlds'];assert len(worlds)==100 and [w['index']for w in worlds]==list(range(100))
 runtime=receipt['runtime'];matrix=json.loads((ROOT/'experiments/026/registered-matrix.json').read_bytes())
 actual_report_rows=[];summary=[];witnesses={}
 with gzip.open(ROOT/'experiments/026/continuation-c1/trials.jsonl.gz','rb')as original:
  for index,line in enumerate(original):
   expected=json.loads(line);record=worlds[index];directory=out/f'world-{index}'
   data=(directory/'history.jsonl').read_bytes().rstrip(b'\n');envelope=json.loads(data)
   assert sha(data)==record['history_sha256']and envelope['runtime']==runtime
   trial=envelope['c1_projection'];assert trial==expected and trial['trial']['config']==matrix[index]
   assert envelope['policy_identity']==trial['source_freeze_sha256']+':'+matrix[index]['mode']
   assert sha(json.dumps(trial,sort_keys=True,separators=(',',':')).encode())==record['canonical_C1_sha256']
   baseline=json.loads((out/f'baseline-{index}.json').read_bytes());assert baseline['runtime']==runtime
   splits=record['split_witnesses'];assert len(splits)==record['splits']
   for split in splits:
    assert sha((out/f"state-{index}-{split['step']}.cbor").read_bytes())==split['state_sha256']
    b=baseline['states'][split['step']]
    assert all(split[k]==b[k]for k in ('step','tick','state_sha256','material_work','raw','waste'))
    assert split['pending_social']==b['pending']
   body=trial['trial']['body'];h=body['history']
   reports=[c['tick']for c in body['checkpoints']if 'Reporting'in c['labels']]
   ticks={s['tick']for s in splits}
   crossed=any(any(a['tick']<t<=b['tick']and a['step']in{c['step']for c in splits}and b['step']in{c['step']for c in splits}for a,b in zip(baseline['states'],baseline['states'][1:]))for t in reports)
   if crossed:actual_report_rows.append(index)
   summary.append({'row':index,'source025':record['source025'],'mode':record['mode'],'splits':record['splits'],'history_sha256':record['history_sha256'],'canonical_C1_sha256':record['canonical_C1_sha256'],'conserved_material':record['conserved_material'],'final_actors':record['final_actors']})
   if index==47:
    cuts=[s for s in splits if s['tick']in(7,8,10,11,31,32,33,34,95,96,106,107,129)]
    assert {10,11}<=ticks and 11 in reports
    witnesses['active_preparation_and_actual_report']={'row':index,'cuts':cuts,'actual_reporting_tick':11}
   if index==11:
    cut=next(s for s in splits if s['tick']==2);failed=next(a for a in cut['material_work']if a['phase']=='Failed')
    assert failed['process']['finish_at']==6 and failed['escrow']==0
    witnesses['contention_full_cost_failure']={'row':index,'cut':cut,'later_operations':[o for o in body['material']['operations']if o['commitment']==failed['process']['commitment']]}
   if index==43:
    assert {8,9,10,17,18}<=ticks
    witnesses['C1_pending_domain']={'row':index,'extensions':trial['response_extensions'],'cuts':[s for s in splits if s['tick']in(8,9,10,17,18)]}
   if index==39:
    process=next(p for p in body['material']['processes']if p['owner']==2 and p['start']==11)
    core=h['final_state']
    while 'base'in core:core=core['base']
    meal=next(m for m in core['cognition']['consumption']if m['actor']==2 and m['tick']==18)
    witnesses['later_meal_after_resumed_work']={'row':index,'process':process,'operations':[o for o in body['material']['operations']if o['commitment']==process['commitment']],'meal':meal,'saved_ticks':[s['tick']for s in splits]}
   if index==31:
    core=h['final_state']
    while 'base'in core:core=core['base']
    event=next(e for e in core['events']if e['id']==25)
    assert event['tick']==38 and event['transferred']==1
    witnesses['actual_later_transfer']={'row':index,'event':{k:event[k]for k in ('id','tick','scene','participants','balances_before','balances_after','transferred')},'selected':event['decision']['selected'],'limits':'No invented fungible unit IDs or isolated whole-trajectory transfer effect.'}
 independent=json.loads((out/'independent-resumed-validation.json').read_bytes());assert independent['worlds']==100 and independent['exit']==0
 recovery=json.loads((out/'recovery-validation.json').read_bytes());assert all(recovery[k]is True for k in ('parent_files_byte_identical','selected_full_state_exact','original_terminal_exact'))
 faults=json.loads((out/'storage-faults.json').read_bytes());assert len(faults['checks'])==6 and faults['parent_byte_identical']
 return {'schema':'persistent-C1-readonly-engineering-audit/v1','runtime':runtime,'worlds_exact_original':100,'full_state_splits':sum(w['splits']for w in worlds),'actual_report_crossing_rows':actual_report_rows,'worlds':summary,'witnesses':witnesses,'independent_resumed_worlds':100,'recovery':{k:recovery['recovery'][k]for k in ('schema','generation','tick','state_sha256','prefix_bytes','prefix_sha256','later_committed_bytes','later_committed_sha256','tail_bytes','tail_sha256','tail_status')},'storage_controls':faults,'limitations':['Selected points are finite boundary coverage,not every possible interruption.','Pause selector uses absolute periodic buckets;actual reporting is start-relative.Only independently observed actual report crossings are claimed.','No new-mode force-kill or power-loss claim;canonical inherited transport tests are separate.','Strict runtime/schema and registered100 configurations;no arbitrary cross-source migration.','Exact replay and deterministic cases are not independent statistical samples.']}
def main():
 p=argparse.ArgumentParser();p.add_argument('directory');p.add_argument('--output');p.add_argument('--expected');a=p.parse_args();result=audit(a.directory)
 if a.expected:assert result==json.loads(Path(a.expected).read_bytes())
 if a.output:
  with open(a.output,'x',encoding='utf-8')as f:json.dump(result,f,indent=2);f.write('\n')
 print('Read-only engineering audit PASS',result['worlds_exact_original'],result['full_state_splits'],'actual report crossings',len(result['actual_report_crossing_rows']))
if __name__=='__main__':main()
