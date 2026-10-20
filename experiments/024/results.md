# Experiment 024 — Locally Grounded Activity Valuation

Scientific verdict: **ACCEPT WITH QUALIFICATION** as evidence of an isolated,
locally reconstructible intervention. MealRelief remains opt-in and unadopted.
This study does not establish a generally better policy and is not protected by
the 001–023 foundation.

## Question and prospective treatment

Can a small, actor-local change to ordinary activity valuation change executed
physical and social histories under Continuous opportunities? The published
preregistration is commit 80325c1e85bbdef0344d7b723618becbf0208ef8. Registration,
source freeze, selector controls and exact Baseline compatibility preceded the
first MealRelief world. No weight or scenario was changed after inspecting outcomes.

Baseline preserves Experiment 023's ordinary scores. MealRelief adds
`min(own_hunger, 40)` only to an already eligible Consume candidate; its final
score is `H + min(H, 40)`. The constant is the existing meal's maximum physical
hunger reduction, not a fitted result. This is a heuristic comparison term, not
expected utility, predicted success, a learned value or a duration-normalized
return. All other candidate scores and the first-strict-maximum tie order
Wait, Consume, Invite, Acquire remain unchanged. Pending invitation masks apply
before this projection. Recipient replies and independent consent are unchanged;
a recipient can be reserved before an ordinary meal decision.

The pure selector receives the actor's captured local input and four legal
candidates. Observer family/source labels, other actors' private state, future
outcomes and original history are excluded. No new persistent cognitive field,
capacity, supply, hunger law, action, scheduling rule or planner was added.
Consumption and acquisition compete sequentially: their positive-food versus
zero-food eligibility makes them mutually exclusive at an instant.

## Original evidence and reconstruction

There are 56 worlds: 27 matched Baseline/MealRelief pairs and two reporting
placebos. Ordinals throughout this report are **zero-based JSONL ordinals**.
The first 20 pairs use the registered original Continuous configurations; six
pairs vary deterministic seeds and one extends exhaustion to horizon 768.
The seed repetitions are parameter sensitivity, not independent statistical
population observations. Several seeds yield identical histories.

Original archive: `trials.jsonl.gz`, SHA-256
`98da25f34239d8c704c0fffad33d33c4194d8ada46771ff11ba2c0abead19b2f`.
Its decoded JSONL SHA-256 is
`fa650c05cf433a5723df4e5f4be81bddc1065d8cdfc50d161dad5d51010a26d7`.
The manifest also records every original member hash. Sources were fixed before
execution: 96 runtime/registration bindings and 17 validation dependencies.

The full reader reconstructs primitive physical, cognitive, consent, inquiry,
retention, time, eligibility and scoring evidence. A complementary reader imports
no evaluator or full reader and independently reconstructs physical timelines,
resource/contact arithmetic, activity scores, stable ties and matched prefixes.
All 56 full reconstructions, all 27 exact pre-divergence local-input/opportunity
prefix comparisons, both exact reporting placebos and 56 exact Rust byte replays
passed. All 34 deliberate scratch corruption categories were rejected, including
past-but-unavailable episode recovery, private/future inputs, real stable ties,
consent, accounting, audit references and final state. Originals were not modified.

## Aggregate findings

Nineteen pairs change both selected activity and primitive execution. Nineteen
change terminal food ledgers and directed trust; thirteen change terminal hunger.
Eight pairs have identical behavior and state after ignoring only Consume score
fields. Four of those have fully identical history bodies without even that
projection. The two report-period placebos exactly match their respective original
histories and score audits. Three pairs temporarily reconverge in the physical
food ledger; this does not erase their cognitive or social history.

MealRelief produces more meals in 15 pairs and fewer in four. Social scenes
increase in 11 pairs and decrease in five. Fifteen pairs have a lower full-run
hunger area for at least one actor, seven have a higher area for at least one;
these categories overlap. Before nominal stopping, 19 pairs have at least one
lower area and seven at least one higher area. Charged drain can reverse an
individual's full-run comparison. Hunger areas use the left state for each real
elapsed tick, split explicitly at the nominal stop. No aggregate welfare score
is assigned.

## Actual causal witnesses

**Costly earlier consumption, ordinals 0/1.** At tick 18 actor 0 owns one food,
H=35, a dated local cache receipt and established relationship history. Both
worlds have identical input at `history.boundaries[12].choices[0]`. Baseline
ranks Invite(2, Resource)=48 above Consume=35; MealRelief ranks Consume=70.
The first executed difference is tick 19: treatment actor 0 consumes 1→0,
H35→0 (actor 2 also consumes). Baseline instead executes the later 0/2 social
scene and transfer at tick 22, event 17; its actor 0 meal occurs at tick 28,
H45→5. Treatment's next 0/2 transfer occurs at tick 30. Worlds are not reset.
Final meals are 8 versus 7, social scenes 28 versus 29, and hunger-area deltas
(treatment minus baseline) are [55,1985,215,1560]. Pre-stop deltas are
[-105,500,55,240]: the first eater's early relief does not establish a better
later trajectory. This preserves a real unfavorable treatment result, not a
hypothetical counterfactual. Nor does it prove that every delayed contact was
intrinsically valuable.

**More consumption without eliminating contact, ordinals 6/7.** At tick 54
actor 0 owns one food and H=80. Baseline Invite=113 beats Consume=80;
MealRelief Consume=120 beats the same Invite. Treatment executes a meal at
55, H80→40, and actor 1 consumes at 56. Baseline executes further offers,
requests and transfers instead. Final meals rise 8→10, scenes fall 28→27,
transfers remain 14, and full hunger-area deltas are [-1320,-1850,0,0].
The finite cache grants remain four. This supports a specific material effect,
not a universally preferable consumption rule.

**Longitudinal food neglect, ordinals 18/19.** At tick 14 actor 0 owns one
food, H30: Invite48 beats Baseline Consume30, while treatment Consume60
wins. The treatment meal executes at 15. Baseline eventually consumes three
of five units and retains two; treatment consumes all five. Baseline has 104
eligible nonselected meals at H100, treatment zero. Scenes increase 172→189,
but transfers collapse 87→4 and zero-transfer scenes rise 85→185. Food
circulation and nutritional use are different dimensions. Full hunger-area
deltas are [-2070,0,-2640,0]; everyone ends at H100 after finite supply is
exhausted. The 768-horizon pair 52/53 preserves that cumulative difference,
not continuing nutritional improvement. More time with no food creates more
empty encounters, not a sustainable community.

**Real nulls and information limits.** Ordinals 20/21 are identical empty-cache,
zero-food, initial-H0 worlds; the new term cannot make unavailable food edible.
Inquiry-quiet pairs 12/13,32/33,36/37 also have full history-body nulls.
Busy-recipient, scarcity and conflict pairs can have score differences without
behavioral differences. All ten executed queries across the matrix are Direct,
with native null responses and legitimate provenance receipts; retained support
moves 0→50 and concern Open→Partial, while novelty40 is a separate observation.
No inquiry-target or information-outcome treatment effect is demonstrated.
No Evidence-strategy stress result is established here.

## Complete paired results

Each area vector is treatment minus baseline, ordered actors 0–3. Counts are
actual executions. Repeated configurations are intentionally retained.

| Ordinals | Case / family | First choice / execution | Meals B→M | Scenes B→M | Full hunger-area delta |
|---|---|---|---|---|---|
| 0/1 | paired / inherited_controlled | 18 actor0 / 19 | 8→7 | 28→29 | [55, 1985, 215, 1560] |
| 2/3 | durations / inherited_controlled | 11 actor2 / 12 | 6→8 | 27→27 | [2320, -850, -2160, -3890] |
| 4/5 | mixed / inherited_controlled | 6 actor2 / 7 | 6→8 | 27→27 | [-1320, -1850, -110, -25] |
| 6/7 | consumption / inherited_controlled | 54 actor0 / 55 | 8→10 | 28→27 | [-1320, -1850, 0, 0] |
| 8/9 | acquisition / inherited_controlled | 58 actor0 / 59 | 1→3 | 31→31 | [-1160, -1120, 0, 0] |
| 10/11 | busy_recipient / inherited_controlled | none | 4→4 | 29→29 | [0, 0, 0, 0] |
| 12/13 | inquiry_quiet / inherited_controlled | none | 0→0 | 27→27 | [0, 0, 0, 0] |
| 14/15 | inquiry_conflict / inherited_controlled | none | 4→4 | 23→23 | [0, 0, 0, 0] |
| 16/17 | ongoing / inherited_controlled | 18 actor0 / 19 | 10→12 | 89→92 | [-1665, 2840, -1905, 720] |
| 18/19 | exhaustion / inherited_controlled | 14 actor0 / 15 | 3→5 | 172→189 | [-2070, 0, -2640, 0] |
| 20/21 | null / inherited_controlled | none | 0→0 | 0→0 | [0, 0, 0, 0] |
| 22/23 | scarcity / inherited_controlled | none | 1→1 | 30→30 | [0, 0, 0, 0] |
| 24/25 | paired / inherited_sensitivity | 18 actor0 / 19 | 8→7 | 28→29 | [55, 1985, 215, 1560] |
| 26/27 | paired / inherited_sensitivity | 18 actor1 / 19 | 7→9 | 27→28 | [-80, -1380, 345, -900] |
| 28/29 | durations / inherited_sensitivity | 11 actor2 / 12 | 4→8 | 35→37 | [-1240, -1910, -3890, -1725] |
| 30/31 | paired / inherited_sensitivity | 54 actor2 / 55 | 1→9 | 30→27 | [-1200, -1160, -3515, -2795] |
| 32/33 | inquiry_quiet / inherited_sensitivity | none | 0→0 | 35→35 | [0, 0, 0, 0] |
| 34/35 | acquisition / inherited_sensitivity | 44 actor0 / 45 | 1→5 | 34→33 | [-1640, -4190, 0, 0] |
| 36/37 | inquiry_quiet / inherited_sensitivity | none | 0→0 | 43→43 | [0, 0, 0, 0] |
| 38/39 | inquiry_conflict / inherited_sensitivity | none | 4→4 | 39→39 | [0, 0, 0, 0] |
| 40/41 | paired / new_seed_sensitivity | 18 actor0 / 19 | 8→7 | 28→29 | [55, 1985, 215, 1560] |
| 42/43 | consumption / new_seed_sensitivity | 22 actor2 / 23 | 8→10 | 28→27 | [-1320, -1850, -10, 0] |
| 44/45 | exhaustion / new_seed_sensitivity | 14 actor0 / 15 | 3→5 | 173→189 | [-2070, 0, -2640, 0] |
| 46/47 | paired / new_seed_sensitivity | 18 actor0 / 19 | 8→7 | 28→29 | [55, 1985, 215, 1560] |
| 48/49 | consumption / new_seed_sensitivity | 22 actor2 / 23 | 8→10 | 28→27 | [-1320, -1850, -10, 0] |
| 50/51 | exhaustion / new_seed_sensitivity | 14 actor0 / 15 | 3→5 | 172→189 | [-2070, 0, -2640, 0] |
| 52/53 | exhaustion / longer_horizon_sensitivity | 14 actor0 / 15 | 3→5 | 342→381 | [-2070, 0, -2640, 0] |

## Interpretation, limitations and unsupported conclusions

Demonstrated: legitimate relative activity scores can propagate into executed
meals, transfers, relationship histories and persistent material divergence.
The added meal term can alleviate specific owned-food neglect, leave choices
unchanged, and create worse measured hunger trajectories elsewhere. Strong
contact can still outrank treatment consumption (the pure selector control
preserves Invite176 above Consume140); the rule does not force survival.

Mechanism-dependent: the min(H,40) bonus changes score comparability but does
not calibrate uncertainty, scarce social opportunities or expected long-term
value. Incoming consent/reservation paths remain an independent cause of meal
delay. Persistent directed trust and retained episodic contributions are distinct;
eviction does not erase separately established trust or authorize archive recovery.

Ambiguous: changed social trajectories interact with consent, food circulation,
need, retained experiences and phase. A first meal is an identifiable upstream
cause of divergence, not unique causal attribution for every later transfer.
Temporary food reconvergence does not demonstrate full-world reconvergence.
Capped hunger and no death/health model limit survival interpretations. Aggregate
scene/transfer counts cannot rank cooperation or welfare. All four actors have
small engineered profiles and bounded fixtures; held-out seeds are not naturally
arising social environments. Longer exhaustion cannot identify delayed productive
work because this experiment adds no such work or supply.

Unsupported: optimal choice, human realism, general intelligence, sustainable
communities, institutions, universal survival advantage, calibrated utility,
indefinite operation, or a reason to adopt MealRelief globally. There is no
justification for adding material production merely to hide valuation failures.

Reconstruction and original causal references are retained in
`independent-metrics.json`, `independent-findings.json`, `comparisons.json` and
`causal-slices.json`. Performance and completion verification are reported
separately; wall-clock measurements are not deterministic simulation evidence.
The strongest open question is whether a transparent local valuation can account
for the *opportunity cost of consuming now* without learning from unavailable
counterfactual outcomes or suppressing useful independent contact.
