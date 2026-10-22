# Experiment025 post-delivery acceptance challenge

## Verdict: CONFIRM WITH QUALIFICATION

At reviewed revision24d0cdc9aaf638514aca6f327ed1990bba9e474f, the narrow
preparation-to-later-use finding survives an additional independent challenge.
The proposed concern was that a debit after preparation might spend different,
preexisting food and therefore overstate use of prepared output. Exact enumeration
of every compatible discrete-unit attribution confirms72primary outputs were
used; nutrition and individual meal identity remain less determinate.

This addendum does not rewrite the original review, archives, comparisons or
source freezes.025 remains reviewed opt-in research, unprotected and unadopted.
No026, new simulation behavior or canon change was introduced. The user authorized
continuation with engineering judgment; this bounded acceptance check precedes
any further research. [Pre-analysis method](development/experiment-025-acceptance-plan.md).

## What was independently reconstructed

`examples/support/review025_food_fate.py` imports only the standard library, not
simulation code, prior reviewers, evaluators or the viewer. It independently checks
compressed/decoded/member hashes for all50 originals and reconstructs7,033actual
material-affecting rows: resource events, ready grants, meals and raw/escrow/
conversion operations in the implemented physical phase order. All recorded
before/after inventories and six-compartment mass conservation agree. It is a
focused physical/attribution challenge, not a replacement cognition/clock validator.

Each actually produced food unit gets a hypothetical observer tag at conversion.
At every actual meal or transfer, all tags or untagged initial/cache units that
could legally account for the debit remain possible. The checker tracks joint
assignments, consumed tags and ever-transferred tags; it never silently chooses
a convenient attribution. This enumerates discrete-unit identities compatible
with the actual ledger, not identities the simulation stored or agents could know.
No probability, future-aware choice or observer-to-cognition channel is added.

The maximum realized set is46hypotheses (record47), versus a fail-closed audit cap
of100,000. This cap belongs to the offline checker; live cognitive/work bounds
are unchanged. The report remains separate from frozen evidence. Unit controls
cover mixed-food meals, joint certainty with individual ambiguity, mixed transfers,
recipient meals, eventual exhaustion, unused output and impossible debit.

## Exact attribution results

| Measurement | Primary48records | All50including two reporting placebos |
|---|---|---|
| Created edible output |77|81|
| Guaranteed eventually consumed OR transferred |72|76|
| Guaranteed neither consumed nor transferred before end |5|5|
| Output with ambiguous ever-use status |0|0|
| Prepared units eventually consumed: compatible range |63–67|67–71|
| Prepared units ever transferred: compatible range |13–14|13–14|

These are exact compatibility ranges under recorded integer-unit food accounting,
not confidence intervals. A transferred unit can later be consumed, so meal and
transfer ranges overlap and must not be added. Summing counts across separate
world ledgers is descriptive accounting; repeated seeds/placebos are not
independent statistical evidence.

Original reporting of59first-owner meal debits and13first-owner transfer debits
answers a different question. It records the first later debit from the producer.
The new63–67meal range includes later consumption after circulation and handles
all fungible assignments. Neither report is retroactively replaced. The original
69isolated primary-unit-to-first-debit cases are a stricter attribution witness
than eventual use; eventual use can become certain after later expenditure even
when the first debit's unit identity was initially ambiguous.

## Strong witness and retained uncertainty

Record17 owner2's first output (`material.operations[3]`, commitment12) is
guaranteed transferred but may or may not eventually be consumed: consumed0–1,
ever-transferred1–1 across compatible assignments. This independently supports
the original event13transfer at20 while refusing to assign its recipient's later
meal to that specific unit. Across all four outputs in17, between1and2are eventually
consumed across compatible assignments; all4are used by transfer or meal. Physical use is not physiological
benefit, adequate sharing, or proof that Invite120 over Consume30 was preferable.

The five guaranteed unused outputs are exactly27owner0 at operation4 and29owners
0–3 at operations4–7. Existing opportunity reconstruction shows they finish after
ordinary cutoff with no later ordinary meal choice. They remain right-censored
non-use, not voluntary persistent neglect. No new outcome was generated to fill
that gap. The co-eligible Prepare-versus-Consume gap remains structurally absent
under the registered food0eligibility.

## What this changes about the next question

This check weakens an explanation that the positive result merely counted unrelated
food debits. It does not support adding more material by default:72of77created
outputs already see physical use, while valuation and distribution can still
produce worse individual hunger histories. Record43's unseen ready-cache unit
also prevents equating raw exhaustion with no objective edible opportunity.

The best next question is narrower than whether agents can use output: **when do
existing local activity and social scores make consuming, retaining or sharing
owned food advantageous or costly across different subsequent opportunities?**
Any future study should keep prepared-output use, hunger trajectories, relationship
consequences and distribution separate, compare unchanged simpler policies, and
retain adverse future reversals. This is a research handoff, not a026specification
or implementation, and no numerical scoring formula is proposed here.

## Verification and integrity

The exact separately saved report reconstructs deterministically. Original025
archive hash remains
`7dd09f8b1ef2deed2232cda11da541a1075d57017a5364542e01fe628047d569`.
All408portable/425raw protected originals and114original runtime/24helper/
158observer2/4JSONbridge/107foundation source bindings remain unchanged.
Current scientific gate, root formatting/Clippy/tests and targeted checks are
recorded in [validation](development/experiment-025-acceptance-validation.json).
The previous long normal foundation gate is reused only at identical107transitive
sources; this read-only helper is not imported by that gate. Viewer source is
unchanged, so its previously passing72test/browser acceptance is not rerun.
No new runtime/storage benchmark is warranted for unchanged simulation behavior;
offline attribution-set size and report size are reported separately.

Reproduce:

```powershell
python -B -m unittest discover -s examples/support -p test_review025_food_fate.py -v
python -B examples/support/review025_food_fate.py --expected docs/development/experiment-025-food-fate.json
cargo run --release --locked -j 1 --example validate025_evidence
```

Original review and new check agree on the acceptance boundary: a real finite
productive capability and later voluntary physical use, with no sustainable
community, global welfare, biological survival, optimality or adoption conclusion.
