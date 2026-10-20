# Continuous activity valuation — observational diagnosis

Observed baseline: original Experiment023 at336ab08, independently reviewed in
[023review](../experiment-023-review.md). This is analysis of recorded decisions,
not a new treatment or a proposed winning policy. No original archive or policy
was changed. Qualified001–023protection is published in4da9fdc after all required
checks passed. Treatment still requires separately published fixed preregistration.

## Evidence and method

[Observations](continuous-activity-valuation-observations.json) contain42per-world
summaries and bounded exact witnesses, rebuilt by
[diagnose023_activity.py](../../examples/support/diagnose023_activity.py).
The helper uses original primitive records and the new independent review's resource
arithmetic, never evaluator success flags. Actual main-browser inspection covered
originals19,10,11,1,2,13and21, including owned score tables, executed transfer/meal
records, inquiry receipt links and a complete-null choice.
[Browser observations](activity-valuation-browser/observations.json).

Across the42fixtures,7193ordinary choices include951eligible consumption options;
738are not selected,180at modeled hunger100. These are repeated decisions within
constructed worlds, including two duplicate reporting-placebo histories, not rates
in a population or independent samples. Among deferred-food outgoing proposals,
305receive Execute admissions,344Reciprocal,52Declined and37Unavailable. Executed
proposer scenes include260with a transfer and45without one; reciprocal references
are not extra scenes. No scene count defines welfare or total usefulness.

## What current scores actually mean

Let H be own hunger, F own food, G generosity, C caution, T own directed trust
for the considered partner, and V the clamped[-40,40]sum of the last four RETAINED
partner episodes. Signed division truncates toward zero; ties retain the first
candidate. These are heuristic conventions, not calibrated common units.

| Activity | Score and legal/local boundary |
|---|---|
| Wait |0, always eligible. |
| Consume |H if F>0,H>0and at least one public tick remains; otherwise-1. Completion takes one actual tick, debits one food and reduces hunger40(floor0). |
| Resource contact |max(0,Request,legal Offer)+floor(H/2)-2. Request=H-35+trunc(T/4)-floor(C/4). Offer=G-floor(H/2)-floor(C/2)+T+V, minus60when H>=60and F<=1. Offer excluded when own food is zero. Public partner availability and eight-tick reservation gate eligibility. |
| Inquiry contact |Selected inherited bounded/local query score minus2, provided its source is available, the exclusive lane is free and public strategy cost+5fits. Direct cost10, Evidence28. It competes with resource contacts; it is not a fifth top-level action. |
| Acquire |H when own food is zero, hunger positive, own retained accessible upper stock>0, owner/date valid, own actor available and declared duration fits. Otherwise-1. Actual completion may find stock depleted. |

Top-level order is Wait,Consume,Invite,Acquire. Contact order is Wait, resource
partners in public order, then the selected inquiry. The strongest positive contact
becomes the Invite activity. A live outgoing protocol token masks duplicate Invite
only, allowing another legal action. This explains some meals without any change
to valuations. Busy participants cannot choose; query masking protects other work.

Inquiry's existing006expected gain combines own prior/learned source×method value,
new public offer value and credibility, clamped0–100. Benefit is importance×gain/100;
cost includes strategy effort and floor(H/10), plus declared Evidence offer effort.
007discounts known overlapping local provenance.012Contextual can reduce gain using
bounded learned observed-change expectations for that context. No actual response
is known before choice. This composite then subtracts the contact protocol penalty.
These scales are independently designed heuristics, not one estimated future utility.

Resource invitation scoring evaluates hypothetical OWN initial resource actions,
not expected partner inventory, consent, transfer or net nutritional gain. Its hunger
term can increase contact faster than consumption. Even with T=0and C=20, at H=100
the Request contribution gives60+50-2=108, above Consume100. Positive trust is thus
not necessary for some high-hunger dominance. This is algebraic decomposition of
the existing rule, not an executed no-history intervention.

Public duration controls eligibility but is not generally priced proportionally:
meals1tick, send/reply1each, resource reservation8including protocol versus up to6
scene ticks, acquisition2/4/8, query15/33. Actual scene duration is uncertain and
must not be used retrospectively as a policy input. Age/staleness does not discount
LastSeen acquisition score; only its legitimate retained upper bound gates it.
Own food excludes acquisition but does not reduce hypothetical Request value.
Unsuccessful-contact feedback016is not active in this History contact selector.
Zero-valence departures do not erase established trust. Episode loss likewise need
not erase separately persistent trust/concerns/expectations.

## Representative actual decisions and consequences

| Witness | Local choice | Executed consequence and interpretation |
|---|---|---|
| Original10,actor2@7 |Food1,H5; Consume5eligible, eight-tick contact cannot fit before14. |Meal7–8. This is a legal closure-driven meal, not proof of calibrated preference. |
| Original11,actor2@7 |Same input except public end40; Invite0score64beats Consume5. |Send7–8, token masks duplicate Invite, meal8–9; admitted scene ends Leave10, zero transfer; hunger area+5only for actor2. Actual cost, not hypothetical loss. |
| Original1,actor1@15 |Food4,H30; contact would score48but own outgoing token masks Invite. |Consume executes15–16. A protocol gap, not universal EatFirst behavior. |
| Original1,actor0@14/18 |Own receipt stock4at14; Acquire30beats Invite13. After grant18, food1,H35; Invite2score48beats Consume35. |Claim20crosses report17and grants18; real Offer/Accept transfer22, donor0food1→0and recipient2food1→2. Useful material availability/social effects, not proof of benefit to all or unique-unit necessity. |
| Original2,actor2@6 |Food0,H80; contact3score85beats Acquire80. |Consent and resource scene deliver food at11from zero→one; actor2selects Consume85at11and meal12reduces H85→45. A direct nutritional contact chain. Later acquisition is also selected at12with H45. |
| Original19,actor2@72 |Food1,H100; trust3=98,V28; Request84,Offer66, plus50minus2→Invite132over Consume100. |Legal contact continues;104food-owned/H100ordinary non-consumption decisions occur across this world before cutoff. Not starvation forced by absent food. |
| Original19,actor2@77 |Food2,H100,trust3=100,V28; Offer128beats Request85; Invite176over Consume100. |Repeated real transfers circulate food. Terminal two units remain unconsumed; all hunger caps100. Calling every transfer useful would conceal this failure. |
| Original13 |Actual Direct inquiry, selected from legitimate concern/source candidates. |Query6–19crosses report15; native response null but provenance receipt1→cognitive receipt0; support0→50,Open→Partial. Information acquired is distinct from novelty40and material benefit. |
| Originals20/21 |Food0,H0,empty own cache receipt; resource contacts eligible but score-2below Wait0. |All ordinary choices Wait, no physical activity; full behavioral/state null despite intended public-budget difference. Cutoff/drain is not voluntary rest. |

Original10/11food ledgers reconverge at9and terminal food matches, while trust and
integrated hunger differ.14/15similarly reconverge at10with different later social
state.18/19temporarily reconverge at12but terminal food differs. Reconvergence is
not retroactive deletion of executed or social consequences.

## Causal classification and confounds

- **Valuation:** the own-state score construction demonstrably lets contact exceed
  eligible consumption. At H100it can do so without positive trust. Learned trust
  and retained episodes contribute additional score; their unique necessity for
  particular later choices is not established by one observed sequence.
- **Accumulated history:** positive transfer interpretations can increase directed
  trust/retained valence; trust persists beyond16episode eviction. This is legitimate
  history, not observer recovery. It does not estimate the probability of nutrition.
- **Local information:** people expose availability, not inventory/hunger. Invitations
  may reasonably pursue uncertain partners; later zero transfer alone does not prove
  the earlier choice unreasonable. LastSeen can be stale after rival depletion.
- **Legality/timing:** reporting closure, busy work, pending-token duplicate masks,
  recipient consent and atomic exclusivity remove otherwise scored alternatives.
  Food-owned does not mean consumption was eligible at every physical tick.
- **Scarcity:** original23really consumes its only unit by5. No scoring formula can
  create additional food. Original19is different: two held units remain unused.
- **Hunger law/endpoints:** cap100hides later deprivation differences; no mortality
  is modeled. Common charged drain can alter terminal hunger. Measure before cutoff,
  during drain and over the full history separately.
- **Fixture selection:** fixed co-presence, constructed bootstrap, severe period11
  and unchanged profiles are controlled sensitivities, not natural community samples.
  More meals, food retained, transfers, trust or information are separate dimensions.

## Observability defect found and repaired

Actual browser selection of original1's resource event22opened need22instead.
023inserts/sorts checkpoints after the native022timeline assigned IDs; inherited
IDs no longer addressed final positions and checkpoint IDs were absent. Version6
therefore had an ordinary event-list navigation defect. Original source references,
archives, policy choices and physical outcomes were intact. The prior passing tests
did not cover this identity condition; broad browser navigation claims needed a
narrow qualification. The incorrect event capture was excluded from browser evidence.

A new all42-world regression failed before repair. Minimal read-only repair assigns
IDs after final sorting and increments the derived adapter version6→7with matching
frontend validation. All nine targeted tests passed, including actual source,
privacy, comparison and new index/reference coverage. Actual browser then opened
Accept22with exact event17and recorded balances[1,1]→[0,2], plus original2transfer11
and meal12correctly. The full57-test viewer suite subsequently passed in326.880seconds; the nine
targeted023tests passed in31.452seconds.
No simulation/retention/source-frozen implementation was modified for this repair.

## Scientific recommendation before design

Test one identifiable change to the relative valuation of an eligible own meal,
using current own need and established physical effect as a transparent heuristic.
Do not force meals, suppress all social behavior or price actual future outcomes.
Compare hunger urgency, declared-duration sensitivity and legitimately retained
contact feedback as distinct alternatives before choosing a primary treatment.
A combined rule would confound their effects; new history state is unnecessary
for isolating current meal valuation. Preserve unchanged acquisition/inquiry,
consent, clock, resource supply, memory/learning and deterministic ties.

Require all three activities to compete, useful contact still possible, missed
useful contact and worse outcomes valid, owned-food neglect still possible, full
nulls and longitudinal/reconvergent histories. Register fixed arithmetic before
implementation; do not tune to eliminate these023failures. Any future material
preparation study must separate new supply/work from activity valuation. This
observational diagnosis does not establish an optimal replacement or authorize025.
