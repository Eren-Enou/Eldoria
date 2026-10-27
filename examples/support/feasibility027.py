"""Read-only pre-treatment count from original C1 ProjectedNeed worlds."""
import collections, gzip, json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]

def inspect():
    rows = []
    for line in gzip.open(ROOT/'experiments/026/continuation-c1/trials.jsonl.gz', 'rb'):
        t = json.loads(line)['trial']
        if t['config']['mode'] != 'ProjectedNeed':
            continue
        b = t['body']
        receipts = [m for m in b['material']['memories'] if m['kind'] in ('Taken', 'Insufficient')]
        eligible = [d for q in b['history']['boundaries'] for d in q['choices'] if d['candidates'][4]['eligible']]
        later = [d for d in eligible if any(m['owner'] == d['input']['legacy']['own']['id'] and m['tick'] <= d['input']['legacy']['now'] for m in receipts)]
        recent = [d for d in later if any(m['owner'] == d['input']['legacy']['own']['id'] and 0 <= d['input']['legacy']['now']-m['tick'] < 24 for m in receipts)]
        rows.append(dict(source025=t['config']['source025'], receipts=len(receipts), later_eligible=len(later), within24=len(recent)))
    totals = collections.Counter()
    for r in rows:
        totals.update({k:r[k] for k in ('receipts','later_eligible','within24')})
    return dict(worlds=len(rows), worlds_with_later=sum(r['later_eligible']>0 for r in rows), totals=dict(totals), rows=rows)

if __name__ == '__main__':
    print(json.dumps(inspect(), indent=2))
