# Experiment 002 specification and pre-implementation design

2026-10-06. Hypothesis: event-specific evidence of scarcity can reduce blame for
an earlier refusal and change a later choice at identical physical circumstances.
Weak testimony should remain uncertain; contradictory direct evidence should take
precedence. These are explicit computational hypotheses, not calibrated psychology.

Preserve the Experiment 001 default policy and serialized reports. Add cognition
as a separate ECS resource keyed by stable agent IDs, with bounded event-specific
beliefs. Communication scenes are explicit one-step encounters between the original
participants, referencing a refusal. A claim is legal regardless of its truth.
A voluntary disclosure exposes a recorded circumstance at that refusal through a
narrow observation channel; policies never receive private partner components.
Disclosure is a controlled experimental capability, not automatic omniscience.

Evidence: testimony weight clamp(50 + trust/2, 0, 90); direct disclosure weight 100.
A belief is a signed scarcity support score. Only strictly stronger evidence
replaces it: repeated testimony cannot accumulate certainty. Scarcity support >=60
reinterprets rejection as possible self-protection, with zero instead of negative
valence. Negative evidence can restore the original valence. All revisions retain
before/after memories and evidence IDs. Original events are immutable. Recompute
trust by replaying original contributions with revised values substituted, applying
the original clamp each step: this avoids duplicate credit and saturation errors.
Only retained episodes can be revised; beliefs are capped at 16 per individual.

Consumption: explicit between-scene phase, one food unit for a hungry individual,
reducing hunger by 40 to minimum zero. No food means unchanged unsatisfied hunger.
Record every balance, hunger, and sink amount, including unsuccessful consumption.
No automatic hunger growth, starvation or clock-dependent metabolism. Scarcity at
refusal means food <=1 and hunger >=60. The default policy already protects hungry
agents; consumption makes inventory usable and gives scarcity a physical cost.

A: silent refusal. B: immediate testimony. C: delayed disclosure after testimony.
D: unverified false scarcity claim. E: false claim contradicted by disclosure.
F: helpful versus refusal history before identical current refusal/testimony.
G: exact replay, seed sweep, disjoint population pairs using the same APIs.
Compare silent, testimony and disclosure branches at identical present probe
states. Save full events, cognition audit, snapshots, consumption and decisions.
Keep historical Experiment 001 artifacts; save compatibility evidence here.

Tradeoffs: event-specific beliefs do not generalize across times or people;
confidence is ordinal, not a probability. Disclosure assumes a verifiable shared
record of the past circumstances (an experimental sensor), which is stronger than
ordinary observation. Audit storage, relationships and original scenes grow;
streaming/persistence is deferred. Explicit communication scheduling isolates
information effects but does not model spontaneous choice to speak or deception.

Policy extension: Experiment 002 uses the existing policy with a 60-point Offer
opportunity cost when hunger >=60 and giving would leave no food. Abundant donors
avoid this penalty. It is selected explicitly; Experiment 001 retains its policy.
