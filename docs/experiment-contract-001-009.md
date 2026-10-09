# Protected experiment contract: 001–009

This foundation extends the unchanged [001–008 contract](experiment-contract-001-008.md)
by reference. Its per-experiment mechanisms, lifetimes, compatibility obligations
and coverage remain authoritative. The historical audit's scope ends at 008;
that scope statement does not prohibit the subsequently finalized opt-in 009.
This document incorporates finalized 009 only. It neither accepts nor reinterprets 010.

## Cross-experiment invariant

An individual must never recover information merely because objective/observer
history still contains it. Eviction of one cognitive representation does not imply
that every causal consequence or separately established persistent state disappears.
The inherited 001–008 retention table remains unchanged. The following contract
adds the 009 research modes without changing legacy defaults or lifetimes.

## Experiment 009: bounded retention

**Question and scope.** Can equal-capacity retention preserve distinctions that
FIFO loses? Finalized rules are `experiment-009-v1`. Reuse Grouped 008 assessment,
007 local attribution, 006 inquiry state and 005 concerns. Capacity is exactly 32.
FIFO remains the default; Quality and Salient require explicit enablement after
Grouped assessment, while idle. Enabling does not import earlier observer evidence.
Method is fixed for a run. No retention weights are retuned here.

**Local boundary and heuristic.** The pure, replaceable decision input contains
the actor identity, at most 32 retained items plus one incoming item, at most eight
own concerns, and the selected method. Per local event/origin/sign, the highest
received-quality item represents its group; equal quality selects the newest.
Other duplicates score zero. Quality scores representatives by received quality.
Salient adds the maximum current active own concern importance for that event.
Resolved/abandoned concerns add nothing; importance is recomputed at each receipt.
Opposing signs remain separate. FIFO scores all items zero. These are heuristics,
not truth, independence or future-value estimates.

No objective truth, hidden root/depth, future relevance, scenario name, observer
history, partner-private inventory or outcome label belongs in this input. Locally
disclosed origin tokens are legitimate; undisclosed shared origins are not joined.

**Eviction and ties.** Below capacity retain every item. Overflow drops the lowest
score, then the oldest item index; the incoming item can itself be rejected.
Removing an acquisition removes its accessible content and contribution to the
retained assessment basis. Later support may therefore change. This prospective
008/009 rule does not retroactively erase earlier modes' persistent consequences.

**Attribution and redelivery.** Legitimately received known attribution can revise
matching *retained* unknown assumptions for the event, communicator and message.
It cannot resurrect an evicted acquisition. A legitimate redelivery is a new local
receipt, including when the provider retains an older acquisition. It is not archive
recovery. Zero-quality reappraisal cannot recover an old origin or received weight.

**Surviving state and behavior.** Cached beliefs, established trust/credibility,
concerns, learned inquiry estimates and resource consequences keep their separately
established lifetimes. Later behavior may depend on them. Surviving retained items
may support later assessment and policy input. An evicted item's contents and
undisclosed provenance are unavailable; observer copies are not policy channels.
Episode eviction does not demand wholesale deletion of all such state.

**Audit.** Append-only observer records capture actual bounded inputs, candidates,
scores, selected eviction, affected-event support before/after, fallback reason and
assessment cursor. Historical validation uses captured decisions rather than
substituting a newer policy; independent tests rescore built-in choices. Invalid
native decisions fail before cognition/clock changes. Invalid replacement decisions
inside an already paid protocol use an explicitly audited local FIFO fallback to
complete accounting; built-ins do not require that fallback. Audit retention does
not authorize restoring unavailable cognition. Observer storage continues to grow.

## Established findings and preserved failures

The [009 specification](../experiments/009/specification.md),
[results](../experiments/009/results.md), frozen report, summary and archives are
evidence, not new scenario instructions. Quality preserves most demonstrated
distinctions. Salient adds a narrow weak-active-concern distinction. Selective
modes preserve older support and opposing signs under low-value traffic.

Preserve the counterexamples: both selective modes can retain strong wrong
evidence; FIFO can retain a minor item later useful to an Offer policy probe while
selective modes choose Leave. Hidden shared provenance can cause local
overconfidence. Equal-valued representatives still lose old items. Concern expiry
removes a bonus but does not guarantee eviction of independently high quality.
Held-out minor-item advantages do not necessarily change even the action probe.

These are retained-state distinctions and probes, **not evidence of improved
executed behavior**. Do not claim global superiority, truth-aware selection,
future usefulness, human likeness, broad cognition or consciousness. The 128-seed
controlled/held-out runs are deterministic parameter sensitivity, not external
generalization. Preserve documented fixture limitations and negative outcomes.

## Coverage and compatibility

`tests/experiment009.rs` directly covers equal capacity, duplicate representation,
conflict, ties, local concern scoring, matched inputs, extreme-seed replay, mistaken
retention, FIFO's minor-item advantage, receipt accounting, bounded populations,
audit tampering/reordering, invalid-decision rejection/fallback and enablement
without history import. This pass adds an explicit default-FIFO/opt-in regression
and strengthens existing wrong-evidence, fresh-redelivery and evicted-attribution
assertions for all relevant methods. It does not assert that every persistent
consequence disappears when an acquisition does.

Input types and resolver boundaries exclude forbidden information; these targeted
tests do not constitute a proof against every future information leak. Changes
must preserve that boundary as well as the ten constitutional requirements.

`docs/foundation-001-009-sha256.json` protects the exact 90-file inventory and bytes
under `experiments/001` through `009`, including compressed archives, manifests,
reports and separate timing evidence. The read-only normal foundation gate is:

```text
cargo run --release --locked -j 1 --example validate_contract
cargo run --release --locked -j 1 --example validate009
```

The first preserves the existing 001–008 gate and invokes the shared 009 verifier;
the second runs that verifier independently. Python's standard library verifies
gzip/raw sizes and SHA-256 directly, without generated cache prerequisites or
writing files. Current seed-42 serialization must match both frozen JSON archives
byte-for-byte. All published outcome counts must match across seeds 0–127;
those seeds plus `u64::MAX` receive exact replay and causal validation. Nothing
regenerates frozen evidence. Experiment 010 is excluded from foundation acceptance.

## Complexity and storage

Retention compares at most 33 candidates and eight concerns: representative
selection is quadratic in that fixed candidate bound; concern scoring is bounded
by 33 × 8. Grouped assessment remains bounded at 32. Observer audit records grow
with receipts and captured bounded inputs/support maps; audit capture and
serialization are substantial costs, not bounded cognitive memory.

The archived 009 whole-pipeline measurements were approximately 2.4–2.53 times
the frozen 008 pipeline at tested population sizes. They do not isolate policy CPU
or establish resident-memory cost. Core JSON is 49,468,354 bytes (gzip 1,053,254);
held-out JSON is 206,624,726 bytes (gzip 4,275,623). Wall-clock measurements remain
separate from deterministic evidence. This integrity pass changes no runtime
mechanism and therefore requires no new throughput benchmark.
