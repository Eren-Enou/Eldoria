# Experiment 004 pre-implementation hypotheses

2026-10-07. Baseline: fmt, Clippy, 32 tests, all three throughput modes pass.
Preserve all earlier modes, serialized reports and archived evidence.

Hypothesis: anticipating an observable evidence challenge can deter a lie with
positive immediate utility; repeated absence of challenges can make that same lie
worth attempting. A local expectation of evidence provision can change whether
verification seems worth its cost. No partner policy/state is consulted.

Smallest extension: optional Foresight ECS resource and pure replaceable predictor
inside the existing four-turn conversation resolver. Reuse original candidate
utilities/legality and add recorded consequences. For a claim: anticipate
AskEvidence with own partner-specific challenge percentage. Mislead loses
40*p/100; truthful Explain loses 8*p/100 for expected challenge burden.
AskEvidence gains p(answer)*known evidence reliability/100/4. Providing evidence
that could contradict one's own prior false claim loses 40*reliability/100.
No tree search, mental simulation of partner internals or hidden belief access.
Silence retains score zero. Prediction can be disabled without removing new evidence.

Expectations start from explicit priors (default challenge=50, answer=50), not
objective partner facts. After a selected claim/request gets an actual next
response, update the appropriate directed expectation by (outcome*100-p)/4 using
integer truncation. Record original probability, public response, signed error
and updated expectation. Only actual responses teach: no counterfactual feedback
for unchosen actions, and no private belief feedback. Finite-horizon closure can
cause biased expectations; evaluate and document this boundary.

Fallible evidence: a public sensor description (reliability 0..90, effort 0..100)
produces a signed scarcity reading. Seeded reading correctness probability is
50+reliability/2 percent; sample keys include seed, refusal ID, source index and
speaker, with wrapping deterministic mixing. Reliability is ordinal belief weight,
not a calibrated posterior. Accurate/Inverted fixture channels explicitly isolate
reliability from reading direction; they are experimental sensor interventions.
Policy inputs know quality/effort, never the sampled reading before disclosure.
Optionally disclose two independent channels to study conflicting readings.

New evidence receipts supersede unsupported testimony about the same proposition.
For direct fallible readings, support=max positive reliability-max negative
reliability across received sources. Thus equal opposing readings yield zero;
repetition cannot accumulate confidence. This evidence-combination rule is opt-in
through a distinct evidence kind; 002/003 retain strongest-evidence precedence.
Original event/interpretation immutability and replacement trust replay remain.
Credibility updates only if combined evidence support reaches magnitude 60, with
consistency deltas scaled by that magnitude. A noisy reading can wrongly reduce
credibility: consistency with evidence is not omniscient truth verification.

Disclosure effort subtracts from immediate utility and advances the clock by that
many ticks, in addition to one tick per receipt. Food remains conserved during
communication; explicit consumption remains unchanged. Privacy preference and
request-cost terms remain from 003. A request cannot compel costly disclosure.

Scenarios A/B: immediate versus anticipation, unknown/high/low challenge priors;
C: disclosure privacy cost; D: controlled strong/weak readings; E: opposite equal
readings; F: verification/request cost and disclosure effort; G/H: six actual
truthful training encounters with challenging or non-challenging listeners,
followed by matched abundant-state deception probes and local expectation-only
ablations. I: seeded errors, deterministic suites, 100/1000 populations and exact
prior-report compatibility. Save full reports, interpretation/credibility changes,
forecast error records and timings separately. No Experiment 005 implementation.

Tradeoffs: same post-refusal four-turn protocol, no expanded world; a coarse
binary expectation not general planning; hand-set loss values; independent noisy
sensor fixtures not a full observation ecology. Maps/audits can grow, bounded
local episodes remain capped. Measure costs rather than optimize prematurely.
