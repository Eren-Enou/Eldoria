# Experiment 003 hypothesis and protocol

Written before behavior changes, 2026-10-07. Existing 21 tests, formatting, Clippy
and both throughput modes pass. Preserve Experiment 001/002 defaults and evidence.

Hypothesis: a pure local policy can select silence, testimony, direct disclosure,
or requests according to relationship value, hunger, privacy cost and remembered
responses. Verified claim consistency can alter future testimony credibility
without conflating credibility with relationship trust. A known false scarcity
claim may be chosen when preserving a relationship outweighs honesty/risk costs.

Opt-in mode: after resource scenes resolve, Simulation::run automatically opens
one bounded conversation per new refusal. Harnesses configure circumstances and
preferences, never schedule messages. Four turns maximum: refuser, listener,
refuser response if requested, listener closure. Silence at opening permits a
question, not automatic disclosure. Requests cannot force an answer. No resource
transfer during conversation; each decision spends one clock tick and one slot.
Original food protocol remains replaceable and unchanged.

Pure policy inputs: own Agent, own profile (privacy, relationship goal, honesty,
evidence-request cost), own bounded communication episodes, own credibility for
partner, own event-specific belief, last public communication action and turns
remaining. Only the original refuser receives their own historical scarcity fact.
Neither party receives the other's preferences, hunger, inventory, beliefs or
credibility. Scarcity means food<=1 and hunger>=60 at the original refusal.

Actions: Silence, Explain (truthful testimony), ProvideEvidence (same narrow
instrumented disclosure as 002), AskExplanation, AskEvidence, Mislead (false
scarcity claim only when speaker knows historical scarcity is false). Explicit
legality, candidate scores, local inputs, selected action, costs, objective truth
and evidence links are audited. Truth annotations are observer-only. Belief and
memory revision reuse 002's evidence rule and clamp-correct trust replacement.

Minimal credibility: per-directed-partner offset [-40,40], initially zero;
testimony weight clamp(50+offset,10,90), independent of relationship trust.
A disclosure verifying a claim adds 20, contradicting subtracts 30, once per
conversation. Silence never proves lying. Communication episodes cap at 16;
recent provided evidence encourages asking (+10 per episode), silence in response
to a question discourages it (-20), capped [-40,40]. These are model hypotheses,
not calibrated psychology. Credibility maps/audits can grow; document and measure.

Speaker value = relationship goal + own trust/4. Scarce truthful Explain score
=value-privacy-hunger/5-10 (+20 after explanation request). Evidence costs 30
instead of 10 and gets +50 after evidence request. For non-scarce evidence,
value=honesty/2 (willingness to substantiate even inconvenient facts). Non-scarce
truthful explanation has no relationship benefit. Mislead score=value-honesty-
privacy/2-caution/2-10. Silence score zero and wins ties. Listener asks why when
goal+hunger/5+response-history-30>0. For unverified testimony, evidence request
score=goal+(100-|belief support|)/2+response-history-request-cost. Fully verified
beliefs or insufficient remaining turns preclude further requests.

A/B/F compare privacy costs with all physical/history conditions fixed. C varies
listener relationship goal. D varies request cost, isolating unsupported versus
substantiated belief and the later resource choice. E forms actual verified or
unhelpful conversations then restores matched current profiles/physical state;
report residual trust/memory differences and use local ablations to isolate
communication history. G varies honesty under abundant private inventory; trace
false-claim incentive and possible contradiction. H seed replay and 100/1000-agent
pairs, with food consumption, full audit and prior-experiment compatibility.

Tradeoffs: post-refusal phase avoids rewriting working resource negotiation but
communication does not compete with offers in the same turn. Disclosure retains
002's powerful instrumented sensor. No exploitation of revealed vulnerability is
simulated: privacy is a utility cost, not an invented downstream attacker. Deception
has no language ambiguity. Bounded episodes and separate audit preserve history;
no reputational propagation, institutions, Observer UI or Experiment 004 work.
