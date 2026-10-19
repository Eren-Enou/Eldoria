"""Read-only actual inquiry delivery-route/progress audit, no policy changes."""
import argparse,json
from pathlib import Path
import review023
ROOT=Path(__file__).resolve().parents[2]
def audit():
    rows=[]
    for ordinal,t in enumerate(review023.read(ROOT/'experiments/023/trials.jsonl.gz')):
        snap=t['history']['final_state'];compact=snap['base']['base']['base'];queries=compact['base']['inquiry']['records'];prov=compact['provenance']
        for n,q in enumerate(queries):
            pq=next(p for p in prov['queries'] if p['inquiry']==q['id']);rid=pq['receipt'];receipt=prov['receipts'][rid] if rid is not None else None
            learning=next(x for x in snap['learning']['records'] if x['inquiry']==q['id'])
            rows.append(dict(ordinal=ordinal,owner=q['decision']['input']['owner'],query_ref=f'history.final_state.base.base.base.base.inquiry.records[{n}]',start=q['tick']-q['time_spent'],completion=q['tick'],selected=q['decision']['selected'],native_response_present=q['response'] is not None,provenance_receipt_ref=None if receipt is None else f'history.final_state.base.base.base.provenance.receipts[{rid}]',communicator=None if receipt is None else receipt['communicator'],listener=None if receipt is None else receipt['listener'],cognition_receipt=None if receipt is None else receipt['cognition_receipt'],novelty_value=q['realized_value'],observed_progress=learning['outcome']))
    return dict(rows=rows,qualification='Native006response object is not the whole007delivery route. Presence/novelty/retained support/status progress and material usefulness are distinct. All recorded023queries are Direct; no utility or truth calibration inferred.')
def main():
    p=argparse.ArgumentParser();p.add_argument('--verify',action='store_true');a=p.parse_args();result=audit();path=ROOT/'experiments/023/inquiry-outcome-audit.json'
    if a.verify:assert json.loads(path.read_bytes())==result
    else:
        with path.open('x',encoding='utf-8') as f:json.dump(result,f,indent=2);f.write('\n')
    print('023 actual inquiry receipt routes/progress checked:',len(result['rows']))
if __name__=='__main__':main()
