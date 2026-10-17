# Original020 observer demonstration

The main History Explorer still uses its explicit018/019 schema.020 introduces new
cache/pending-claim state,so this task supplies a separate small read-only adapter
instead of presenting020 as a compatible019 record. It is not a simulation driver,
an actor perspective,a complete inquiry browser or a foundation dependency.

From the repository root:

```powershell
python -m http.server 8766 --bind 127.0.0.1 --directory experiments/020
```

Open [the local demonstration](http://127.0.0.1:8766/demo.html). Choose one of ten
original core histories,move between captured states or select a recorded action.
The page shows CacheId0,stock/access,owned food,hunger,pending claim durations,
grant/failure,original source paths and the conservation ledger. The material filter
separates actual meals/transfers from offers and requests. Individual grants preserve
their atomic balances even if the next captured state contains several completions.
There are no synthetic intermediate states or per-unit ownership identities.
Same-tick rows follow the declared physical phases:resource action,claim completion,
meal,contact allocation,then a later cohort's selection. In ongoing/Need at52,the
actual Accept precedes the concurrent grant and subsequent new Acquire choice.
The projection regression checks that real slice;it does not infer new causal edges.
All-records also exposes exact contact allocation dispositions,so a skipped invitation
is visible as Unavailable/Declined rather than being mistaken for an executed scene.

`finite-demo.py` produces the presentation from original record ordinals and exact
field references. `test_finite_demo.py` compares every displayed frame and action
with those original references and checks every displayed ledger. Selected Acquire
details show original fields and a reference to the complete local legacy input;
they are an explicit projection,not an additional observation available to policy.

Actual in-app browser verification:

- Contested/Acquire record24:tick1 has three pending attempts,all observing the same
  one-unit stock. At4,records1/2 are Depleted with no inventory credit;total remains1.
- Sharing/Need record33:material filter contains actual Accept event5 at16,then
  actor1's meal at17. Clicking opens exact event/consumption references. Tick16 has
  owned1/consumed0;tick17 has owned0/consumed1 and hunger100→60.
- Lost_contact/Acquire record29:descending grant goes to2,actor0's record1 is Depleted.
  At4 ledger owned1+consumed1+cache0=initial2;it does not award the losing actor food.
- Ongoing/Need record43:End reaches the actual final capture at576,cache0,owned0,
  consumed36,total36/36 and12 completed claims. No console warnings/errors observed.
- Cache/material filters and Previous/Next/End controls were exercised. No viewport
  override or mock data was used. Ten histories are a demonstration,not all164 UI tests.

Genuine screenshots:[pending claims](viewer-pending.jpg),[executed sharing](viewer-sharing.jpg).
The inherited018/019 milestone has its own
[acceptance](../../tools/history-explorer/validation/causal-navigation-acceptance.md),
602-source traversal,24 Python tests,actual JS tests and real historical browser cases.

Limits:source field panels are observer-only;complete inquiries/provenance and paired
comparison remain in original archives/readers. Temporal adjacency of a later meal
does not by itself prove which fungible unit was consumed. This adapter does not
reconstruct unavailable cognition,alter old imports or make scientific claims from UI.
