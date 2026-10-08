# Experiment 001–008 contract and coverage map

Audit scope: finalized modes through `experiment-008-v1`; no behavior changes.
This map supplements the individual specifications/results, not replaces them.
Authority: [AGENTS](../AGENTS.md), [constitution](simulation/SIMULATION_CONSTITUTION.md),
[architecture](architecture.md), [history audit](history-audit.md),
[foundation results](../reinforcement/results.md), and each experiment's completed
results, implementation, tests and frozen evidence. Earlier recommendations are
historical hypotheses, not instructions to implement them. In particular 007's
uncertain-attribution recommendation was superseded by finalized 008 assessment.

## Universal requirements, with mode-specific retention

Preserve identifiable causality, information locality, persistent consequences,
individual perspective distinct from objective truth, resource accounting,
controlled reproducibility, voluntary outcomes, historical traceability,
replaceable behavior systems and measured complexity.

An individual must never recover information merely because observer history
contains it. Eviction of one cognitive representation does not erase separately
established persistent state or its causal consequences. Policies receive own
state and legitimate local/public projections, never ECS World, private partner
components, observer indexes, hidden roots, future outcomes or deceptive intent
as listener knowledge. Resolver access for validation, explicit narrow observation
and historical accounting does not authorize arbitrary policy access.

The legacy native evidence path retains first-contact testimony weighting,
received-source immutability and sign-wise received-evidence maxima independently
of bounded beliefs. Those persistent consequences can affect later legitimate
receipts after belief eviction. The foundation index is a rebuildable acceleration
of that established mechanism, not an added observation power. It cannot recreate
an evicted resource episode. Experiment 008 support instead comes exclusively from
its prospective retained assessment items. Neither retention model is imposed on
the other. Trust, credibility, answer expectations and concern uncertainty likewise
have independent ownership and lifetimes.

## Exact cognitive bounds and eviction

Capacities are per owner, across partners/events unless stated. Sources are current
constants and mutation paths, not guessed psychological limits. All are heuristics.

| Collection | Capacity / source | Deterministic retention | Audit and survival |
|---|---|---|---|
| Resource episodes | 16, `model::MEMORY_CAPACITY` | FIFO, `behavior::remember` | Original event interpretations remain; directed trust survives |
| Event-specific beliefs | 16, same constant | Remove matching event then append; FIFO at capacity | Information before/after records reconstruct eviction; no separate eviction vector; trust/revisions and legacy evidence consequences survive |
| Communication episodes | 16, same constant | FIFO, `intentional::remember` | Talk records preserve originals; credibility/expectations survive |
| Concerns, including terminal | 8, `concerns::CAPACITY` | Oldest terminal first; otherwise least importance, oldest ID tie | Explicit abandonment/eviction transitions; evicted concern cannot be pursued |
| Inquiry cells | 32, `inquiry::CELL_CAPACITY` | Updated cell moves to back; FIFO capacity | Explicit eviction using last record; forgetting may restore exploratory prior |
| Inquiry signatures | 32, `SIGNATURE_CAPACITY` | Retained equal/weaker duplicate ignored; stronger replaces and appends; FIFO | Receipt-linked eviction; forgotten content may look novel |
| Inquiry episodes | 16, `EPISODE_CAPACITY` | FIFO | Explicit record-linked eviction |
| Contacts | 16, `CONTACT_CAPACITY` | Existing contact unchanged; FIFO new contact | Explicit source-linked eviction; current public co-presence still legitimate |
| Public offers | 32, `SIGNATURE_CAPACITY` | Matching offer refreshed/replaced; FIFO capacity | Explicit notice-linked eviction; attempted-offer cell prevents endless renewal while retained |
| Own configured capabilities | 32, `CELL_CAPACITY` | Matching event updated; FIFO capacity | Interventions and explicit event-linked eviction; immutable received source cannot be rewritten |
| Provenance knowledge | 32, `provenance::CAPACITY` | Every acquisition/receipt appends, FIFO | Explicit receipt eviction; no archive recovery by provider |
| Attribution hints | 32, same constant | Matching event/source replaced, FIFO capacity | Explicit receipt-linked eviction; metadata is not evidence |
| Credibility comparison keys | 32, same constant | Duplicate comparison suppressed while retained; FIFO new keys | Explicit receipt-linked eviction; forgetting limits deduplication lifetime |
| Assessment receipt items | 32, `assessment::CAPACITY` | Every delivered item appends, FIFO before assessment | Immutable record names evicted/retained receipts; no historical maxima supply support |

Protocol bounds are separate: resource scene budget 1–100 (fixtures use six),
intentional conversation four decisions, concern reply two turns, one inquiry per
participant/opportunity with one bounded reply; inquiry meetings 2–8 individuals,
inspection at most seven inspectors, structured transmission depth at most three
(`provenance::MAX_DEPTH`, two intermediate relays). No capacity was increased.

Sparse trust/credibility/expectation maps, legacy evidence summaries/source slots,
objective records, completed scenes, circumstances, interventions, revisions,
audit contexts, roots/receipts and eviction logs are not globally bounded.

## Mode inventory

Each row below is an enforcement contract, not a claim about human psychology.
The linked specification/results supply equations and controlled evidence.

### 001 — Experience has consequences

[Specification](../experiments/001/specification.md), [results](../experiments/001/results.md).
Question/demonstration: can actual helpful/refusal histories change later decisions
at matched current material state? Yes in controlled local-policy probes; participant
interpretations differ. Resource policy accepts only own Agent and Observation.
Food transfers only on legal acceptance; offers reserve nothing. Constructor rejects
preloaded episodes, preventing inserted history/dangling references.

Episodes retain event/partner/action/interpretation/valence (16 FIFO). Eviction removes
episode access and recency contribution, not clamped directed trust. Future sharing
scores can still depend on trust; policy cannot reconstruct an episode from Events.
Observer retains full original decisions, balances and both interpretations.
Heuristics: integer utility, last four partner episodes clamped ±40, trust ±100,
interpretation valences and deterministic action tie order. Unsupported: personality,
general planning, unlimited memory. Compatibility: original utility policy/CLI/report
defaults and seed-42 archive remain exact. Later scarcity and communication are opt-in.
Growth: relationships/scenes/events and full report clones; serial resolution.

### 002 — Information can change interpretation

[Specification](../experiments/002/specification.md), [results](../experiments/002/results.md).
Question/demonstration: can legitimate event-specific information revise retained
interpretation and later relationship consequences without editing an old event?
Testimony is not truth checked; Disclosure releases one instrumented historical
predicate. Scarcity policy adds an own-inventory opportunity cost. Consumption is
an explicit idle phase, one food unit reducing positive hunger by 40.

Resource episodes and beliefs each retain 16. Belief refresh removes/reappends that
event. Evicted episodes cannot be recreated/reinterpreted; revisions already applied
remain in aggregate trust with ordered replacement contributions and saturation.
First-contact testimony weighting persists independently of belief retention.
Later actions may use surviving trust; a new legitimate receipt may form a new belief
without recovering a forgotten episode. Observer alone retains circumstances, original
interpretations and revision links. Heuristics: testimony weight `clamp(50+trust/2,0,90)`,
strictly stronger absolute evidence replaces (ties keep prior), support 60 relief.
Unsupported: arbitrary proposition reasoning/omniscient correction. 001 stays exact;
later Fallible is a distinct evidence-kind rule, not a rewrite of legacy Disclosure.
Growth: receipts, consumption, revisions and directed relationships.

### 003 — Communication is a choice

[Specification](../experiments/003/specification.md), [results](../experiments/003/results.md).
Question/demonstration: do costs, local goals and remembered responses change bounded
speech selection? Yes, including supported known-false scarcity claims and unsuccessful
persuasion. TalkInput exposes own state/profile, local belief/credibility/episodes,
public protocol, and refuser's narrow own historical scarcity bit. Listener receives
Explain for both truthful and deceptive claims, without intent/truth annotations.

Communication episodes are 16 FIFO, alongside 16 beliefs/resource episodes. Evicted
response episodes stop contributing to response-history utility, but directed
credibility and resource trust persist and can change future claims/actions. Own
historical-self disclosure is an explicit observation channel, not resurrected episodic
memory. Observer retains every turn, intent annotation and actual input. Heuristics:
profile utilities, credibility +20/−30 clamped ±40, testimony10–90, silence tie preference.
Silence does not prove dishonesty. Unsupported: language, global reputation, unified
speech/action planner, unrestricted deception. Enabling is prospective/idempotent;
001–002 archives unchanged. Growth: directed credibility and full decision payloads.

### 004 — Anticipation can affect choice

[Specification](../experiments/004/specification.md), [results](../experiments/004/results.md).
Question/demonstration: can local predicted challenge/answer consequences alter choice
and actual public responses train expectations? Predictor receives TalkInput and own
Context with public quality/effort, excluding sensor seed/channel/outcome and partner
internals. Sample only a selected evidence response; forced terminal closure is censored.

Earlier bounded episodes/beliefs retain their limits. Directed challenge/answer
expectations and native sign-wise received-reading maxima survive their eviction.
These established derived consequences may affect predictions/later receipts; no
episode is restored. Reading source identity remains immutable across redelivery.
Observer retains forecasts/errors and objective correctness, unavailable as policy
labels. Heuristics: quarter-error update, priors50, anticipated losses40/8, strongest
positive minus negative received quality, sensor noise design and scaled credibility.
Unsupported: perfect forecasts, recursive mind modeling, unlimited lookahead.
Legacy 001–003 unchanged; 008 uses a different retained support basis only when enabled.
Limits: finite-horizon delayed-lie loophole, selective feedback, unbounded partner
expectations, forecast searches and evidence summary state.

### 005 — Unresolved matters persist

[Specification](../experiments/005/specification.md), [results](../experiments/005/results.md).
Question/demonstration: can consequential uncertainty survive an encounter and later
prompt voluntary follow-up, partial resolution or abandonment? FollowInput has own
concerns/needs/goal/answer expectation and public partner/quality only. Reopen requires
the relevant partner at a later encounter, at most one matter per participant.

Eight concerns include terminal entries. A retained concern's uncertainty, status,
importance and event/receipt links outlive an evicted belief/resource episode; later
follow-up can depend on it, but cannot recreate the episode or revise its lost valence.
An evicted concern is unavailable. Resolved can reopen; Abandoned stays abandoned even
after later evidence updates uncertainty. Observer retains creations/transitions and
response links. Heuristics: importance 40, absolute support 60 closure, age/failure/cost
scoring and quarter-error response learning. Unsupported: guaranteed eventual answers,
general intent planning, autonomous meetings. 006 replaces follow-up selection only
when opted in; 005's age/failure abandonment branch remains available. Earlier archives
exact. Growth: transitions and decision inputs; bounded concern scans do not bound history.

### 006 — Inquiry adapts to expected useful information

[Specification](../experiments/006/specification.md), [results](../experiments/006/results.md).
Question/demonstration: can local source/method usefulness decline, change selection,
Pause and recover after a genuinely new voluntary public offer? Input has own concerns,
cells/signatures/credibility and current public co-presence/offers; no private evidence
inventory, expertise, noise, outcome or future response. Importance differs from value.

Cells 32, signatures 32, inquiry episodes 16, contacts 16, offers 32 and capabilities 32
obey the table. Concern uncertainty and legacy trust/credibility/expectations/native
evidence consequences survive independent episode/signature eviction. Forgetting can
restore inappropriate optimism/novelty; archive queries cannot recover evicted cells,
offers or signatures. Current co-presence is a legitimate new opportunity. Observer
retains notices, meetings, selected replies, learning and capacity evictions. Heuristics:
Direct 55/Evidence 65 priors, mean update, novelty caps, costs 10/28 and opportunity floor.
Only valid selected responses teach; attempted offers cannot indefinitely renew value.
Pause neither abandons nor resolves. Unsupported: global expert search/travel, Shannon
entropy, third-party relay in native006. Old005 selection preserved; 007 adds legitimate
provider acquisitions; 008 does not retune006 novelty. Growth: audits and older-history
lookups; meetings are externally supplied, not scheduled by profile changes.

### 007 — Testimony has locally known provenance

[Specification](../experiments/007/specification.md), [results](../experiments/007/results.md).
Question/demonstration: distinguish locally known corroboration/relay, hidden-lineage
overconfidence, later revelation and conflict. Inquiry/inspection/share policies use
own bounded knowledge and legitimate public attribution only. Objective roots/parents/
depth and private sensors are resolver/audit fields, not local knowledge.

Knowledge/hints/comparison keys each 32 FIFO; prior bounds unchanged. Provider must
retain an acquisition to share: root archive cannot restore it. Forgotten lineage can
make a repeated origin seem independent. Directed credibility, trust, concerns and
native legacy evidence consequences survive separately; earlier policy may use them,
but no forgotten acquisition/hint/comparison key is recovered. Retained attribution
revelation replaces matching event/provider/message assumptions only; original receipts
stay immutable. Observer reconstructs actual root/parent chain and hidden attribution.
Heuristics: known token vs unknown speaker grouping, residual support, novelty caps,
negative credibility quality discount, zero/100 overlap. Structured relay faithfully
copies acquired content/available token; it cannot forge provenance. Unsupported:
recursive gossip, perfect independence detection, global reputation, natural witness
discovery. Native original-refuser responses remain; mixed native/provenance overwrite
behavior belongs to007 and is not silently fixed there. 001–006 exact;008 opt-in adds
assessment. Growth: roots/receipts/exchanges/queries; grouping O(32 log32), comparisons
O(32²); distinct root IDs do not establish statistical independence.

### 008 — Coherent bounded local assessment

[Specification](../experiments/008/specification.md), [results](../experiments/008/results.md).
Question/demonstration: does one local basis remove final channel/order reversals while
preserving uncertainty/corroboration/correction/forgetting? Finalized **Grouped** combines
sign-wise maxima within known origins, then residual integer support in stable origin
order. Max remains a comparison control; Legacy remains007 semantics.

Assessment retains 32 delivered items (including redundant receipts), across events,
with frozen origin/content/received quality and explicit Native/Provenance namespace.
FIFO can change support. Forgotten assessment items cannot be restored from native
observer maxima or audit replay. Separately retained provider acquisitions can still
be voluntarily redelivered; that is a new legitimate encounter. Resource episodes,
trust, beliefs, concerns and provider knowledge retain independent lifetimes. Losing
an episode prevents later memory/trust replacement for it even if a concern resolves.
Observer retains immutable incoming items, before/after support, attribution replacements,
evicted and retained references. Enabling is prospective, imports no old receipts,
is idempotent for same method and rejects switching method within a run.

Heuristics: origin grouping, frozen weights, positive-quality evidence over claim
fallback, residual formula, FIFO 32 and support 60. Independent acquisition 50+50→75;
known relay 50→50; equal opposition→0; hidden relay can yield wrong confidence;
later attribution can reduce support/reopen concern. Strong evidence does not erase
contradictory retained evidence. Same acquisition keeps its public origin across
adapters; anonymous readings cannot be equated using hidden roots. Invariance requires
identical retained inputs/weights; prefixes, eviction and legitimate credibility
changes are outside that comparison. Old modes/exports remain exact. Unsupported:
probabilistic calibration, universal belief network, uncertain/forged attribution.
Limits:006 novelty may differ from joint support delta; assessment duplicates bounded
local storage and adds unbounded audit records containing up to 32 receipt references.

## Coverage audit

Classification: **D** directly asserted by automated tests; **I** indirectly covered
by replay/reconstruction or type/API separation; **E** controlled evidence/manual
source review only; **U** currently uncovered as a general automated guarantee.
Tests are in `tests/` unless `src/` is named. Function names are exact search keys.
Rows group inseparable clauses; classification applies to the stated scope, not an
exhaustive proof of every possible input or every future system.

| Contract | Class | Coverage / evidence |
|---|---|---|
| 001 actual histories affect matched-present decisions | D | `invariants::history_changes_choices_at_identical_present_across_seeds`, `own_history_changes_policy_without_partner_information` |
| 001 private partner isolation, subjective divergence | D | `private_partner_state_cannot_change_unobserved_decision`, `refusal_has_distinct_subjective_interpretations` |
| 001 legal actions, participant validity, conservation, termination, overflow | D | `property_sweep_accounting_legality_traceability_and_termination`, `invalid_participants_amounts_and_overlapping_scenes_rejected`, `inability_timeout_and_overflow_are_safe` |
| Episode FIFO/causal references and persistent trust | D | `bounded_memory_retains_aggregate_relationship_and_valid_event_ids` |
| 002 claims not truth checked, narrow disclosure, original vs revised interpretation | D | `claims_are_local_not_truth_checked_and_disclosure_is_event_specific`, `controlled_information_history_and_conflict_sweep` |
| 002 consumption sink/hunger, atomic invalid information | D | `consumption_accounting_unmet_need_and_bounds`, `information_validation_is_atomic_and_scenes_terminate` |
| Belief bounds/episode non-restoration and saturated replacement trust | D | `bounded_beliefs_and_evicted_memory_cannot_be_rewritten`, `saturated_trust_replay_uses_replacement_not_naive_refund`, foundation long-history scan |
| 003 voluntary costs/history, silence/intent/credibility distinctions | D | `counterfactual_sweep_disclosure_requests_evidence_and_history`, `history_ablation_isolates_request_choice_and_credibility`, `deception_has_private_knowledge_incentive_and_no_intent_leak`, `false_and_true_claims_are_indistinguishable_until_evidence` |
| 003 legality/termination, profile lifecycle, prospective enable, episode16 | D | `invalid_policy_terminates_without_information_or_state_changes`, `profile_validation_and_mode_lifecycle_are_atomic`, `enabling_mode_does_not_reopen_historical_refusals`, `episode_bounds_and_credibility_saturation` |
| 004 anticipation learned only from actual public selected response, censor closure | D | `expectation_only_ablation_and_actual_learning_records`, `population_forecast_audit_reconstructs_expectations_from_public_responses` |
| 004 private sensor/partner isolation, invalid predictor, effort/noise/conflict | D | `hidden_partner_and_sensor_outcomes_cannot_affect_initial_prediction`, `invalid_predictor_terminates_atomically_without_feedback`, `explicit_effort_ticks_resource_accounting_and_population_replay`, `fallible_evidence_is_not_truth_and_conflicts_do_not_accumulate`, `sampled_noise_is_reproducible_but_not_universally_correct` |
| Persistent legacy evidence survives belief eviction without restoring episode/trust | D | NEW `contract_001_008::legacy_evidence_consequences_survive_eviction_without_restoring_episodes`; contrasts with008 retained-basis tests |
| 005 independent concern lifetime, threshold, relevance, later encounter, at most one | D | `persistence_creation_threshold_and_idle_run`, `only_relevant_partner_can_be_asked_and_history_alone_changes_choice`, `followup_can_substantiate_an_earlier_claim_and_age_ties_are_stable` |
| 005 eviction/abandonment, partial/wrong closure, retained-event immutability | D | `capacity_eviction_is_explicit_and_concerns_outlive_episodes`, `hidden_truth_is_not_in_follow_policy_and_wrong_evidence_can_close`, `external_evidence_refreshes_concern_and_preserves_original_memory`, `transitions_reconstruct_bounded_state_and_accounting` |
| 005 response/time/invalid action accounting | D | `config_legality_and_invalid_policy_terminate`, `combined_audits_account_for_every_tick_and_reconstruct_population_concerns` |
| 006 source/method learning vs importance, novelty/Pause/local alternatives | D | `local_history_alone_changes_strategy_and_source_choice`, `novelty_categories_provenance_and_no_truth_dependency`, `controlled_scenarios_multi_seed_and_full_replay` |
| 006 offers voluntary, no endless promised renewal, private input isolation | D | `new_opportunity_is_voluntary_and_repeat_notice_does_not_manufacture_value`, `an_unfulfilled_offer_cannot_perpetually_restore_an_exhausted_method`, `private_source_changes_are_not_in_inquiry_inputs_and_external_mismatch_is_safe` |
| 006 all capacities, deterministic eviction, selected valid learning and time | D | `src/inquiry::bounded_source_strategy_and_information_storage_audits_eviction`, `contact_episode_offer_and_capability_capacity_is_audited`, `inquiry_audit_reconstructs_selected_learning_and_valid_receipts`, `population_replay_accounting_bounds_and_explicit_ticks` |
| 007 known/hidden/revealed/conflicting lineage, local credibility distinct | D | `independent_known_hidden_revelation_and_conflict_have_distinct_local_consequences`, `credibility_is_learned_from_local_comparisons_and_independence_changes_selection`, `repeated_strong_proof_cannot_bootstrap_credibility_or_independence` |
| 007 voluntary inspection before sampling; faithful copy, parent/depth validation | D | `roots_are_acquired_voluntarily_before_sampling_and_cannot_be_rewritten`, `relay_limits_and_parent_receipts_are_checked_without_recovering_evicted_source_knowledge`, `all_seed_trials_replay_and_audit_references_and_accounting_are_consistent` |
| 007 FIFO 32 knowledge/hints/keys, no evicted acquisition recovery | D | `all_new_local_collections_are_bounded_and_evicted_provider_cannot_use_archive`, `forgetting_is_fifo_and_can_make_an_old_lineage_novel_again` |
| 007 hidden source/sensor state and invalid local decisions | D | `alternative_source_decision_cannot_see_its_private_acquisition`, `policies_receive_no_hidden_sensor_or_partner_provenance_and_invalid_choices_do_not_learn` |
| 008 retained-input order/repetition/grouping/event isolation | D | `shared_basis_removes_native_testimony_order_and_repeat_reversals`, `pure_rules_only_use_supplied_local_items_and_repeats_are_idempotent`; NEW `grouped_assessment_is_event_local_and_permutation_invariant_with_unequal_weights` |
| 008 same acquisition across receipt namespaces, no resampling | D | `same_acquisition_keeps_one_origin_across_both_delivery_adapters`, `new_adapter_samples_nothing_and_keeps_policy_input_and_weights_matched` |
| 008 wrong confidence, contradictory evidence, later attribution, new correction | D | `confidence_can_be_coherently_wrong_and_new_correction_can_remain_ambiguous`, `known_independent_shared_unknown_and_revealed_dependence_are_distinct` |
| 008 FIFO forgetting/no observer maxima, prospective enable, atomic invalid receipts | D | `fifo_eviction_is_real_and_observer_native_maxima_do_not_restore_evidence`, `enable_is_prospective_idempotent_and_does_not_resurrect_archived_proofs`, `invalid_native_payload_and_unavailable_acquisition_do_not_mutate_assessment` |
| 008 locality/audit truth exclusion, voluntary inquiry/bounded population | D | `local_assessment_does_not_receive_hidden_roots_truth_or_private_sensors`, `voluntary_inquiry_consumes_actual_unified_receipt_without_forcing_resolution`, `population_is_bounded_reproducible_and_native_only_outputs_are_preserved` |
| Old-mode exact archives/replays including finalized 008 | D | existing archived tests in004–007/foundation; NEW `finalized_experiment008_reproduces_its_archive_exactly`; read-only `validate_contract` checks001–008 suites/populations/readable reports |
| Captured contexts immutable; compact expansion uses original decisions | D | foundation `runtime_references_match_migration_and_do_not_mutate_past_contexts`, `compact_roundtrip_all_controlled_traces_and_exact_archive`, `invalid_policy_output_is_retained_without_normalizing_it_away` |
| References: existence, owner, namespace, availability, parent/local attribution | D | foundation `corrupted_references_fail_explicitly`, `observer_truth_cannot_be_substituted_for_undisclosed_local_attribution`;008 snapshot validation tests |
| Derived indexes equal authoritative rebuild, signed saturation/ordering | D | foundation `long_history_indexes_rebuild_and_saturation_matches_authoritative_scan`, `src/history::prefix_updates_match_full_clamped_replay_with_signed_revisions` |
| No serialized transient ECS handles; replaceable pure policies | I | private World/Index and function signatures; serialized controlled archive equality; source inspection |
| No arbitrary observer recovery throughout all future code | U | Current local projections and targeted negative tests are D; Rust types do not prove noninterference for arbitrary future additions. Review every new projection/resolver bridge |
| No fixed narrative, no unsupported capability/scientific promotion | E | Pure selection and negative/failed controls; specifications/results/manual scope review, not a scientific emergence detector |
| All bounds unchanged, deterministic JSON distinct from timing | I | constants/source audit, bound tests, full replay; evaluator JSON excludes clocks; timing CSV separate |
| Complete immutable runtime history (all record families) | I | append paths, reconstruction/reference tests and exact archives; validators are targeted, not an exhaustive forensic verifier |
| Performance/complexity across long arbitrary histories | E | foundation and001–008 measurements; no heap/long-world/million-agent proof |

## References, actions and enforcement procedure

IDs belong to record collections, not a global number space. Stable AgentId owns
cognition; resource event/scene IDs index Runtime; InformationScene owns native receipt
IDs; TalkRecord/Conversation own communication IDs; Inquiry records own cell/episode
references; provenance roots/receipts/windows/exchanges/queries own their namespaces.
`audit::{InquiryId,QueryId,ContextId,ReceiptId}` wrap compact references; assessment
`Receipt::{Native,Provenance}` explicitly distinguishes delivery namespaces. Observation
tokens identify disclosed local origins, not permission to consult hidden roots.
Parent receipt must precede its child; references require valid owner/event/availability.
Legacy IDs remain numeric within documented collection context, not universal newtypes.

All resource, speech, follow, inquiry, inspection and exchange actions require current
participant/legality validation, deterministic resolution, explicit food/time accounting,
structured outcome and bounded termination. Invalid attempts retain diagnostic records
where the protocol charges time; invalid public API calls validate before mutation.
No implicit food production/metabolism, automatic travel, gossip or promised response.
Interventions are fixture/configuration changes, not policy-selected actions.

Before changing any established mechanism: locate its row/mode, state hypothesis and
tradeoffs, preserve actual historical decision inputs, test locality/accounting and
termination, keep cognition eviction separate from observer retention, then run:

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -j 1 -- -D warnings
cargo test --locked -j 1
cargo run --release --locked -j 1 --example validate_contract
```

`validate_contract` reads frozen archives and writes nothing. Existing evaluate002–008
examples regenerate files in their relative experiment directories: execute them in an
isolated evidence workspace when verifying historical output, never over the frozen
repository archives. Run existing `evaluate` and `validate_history` there as well.
Throughput is required for runtime/rule/performance changes; keep wall-clock output
outside deterministic JSON and retain its established record-count denominator.

## Discrepancies resolved and intentionally unsolved limits

1. The audit's initially proposed universal forgetting rule was too broad. User
   clarification preserves separately established persistent consequences in each
   mode. Legacy evidence maxima after belief eviction are not a regression to fix;
   they do not restore episodic access. Experiment 008 retains its stricter support basis boundary.
2. 007 recommends uncertain attribution; completed 008 instead finalized coherent
   Grouped assessment. The later completed specification/results and accepted commit
   `9b54619` establish authority. Max is a control, not a second default; no 009 begins.
3. Older specifications/results describe scans/quadratic admission and duplicate query
   payloads. Foundation reinforcement supersedes those implementation costs with
   indexes/referential contexts while preserving behavioral output; old measurements
   remain historical, not current performance promises.
4. Early episode/belief eviction is reconstructible from complete records rather than
   a dedicated eviction log. Do not claim identical audit schemas across modes.
5. Persistent observer summaries are not agent-accessible maps; their legitimate
   legacy resolver effects are explicitly narrow. New policies cannot query them.
6. `docs/simulation/SIMULATION_CONSTITUTION.md` was an empty placeholder. It now
   codifies the ten existing AGENTS requirements and the clarified retention rule;
   it does not introduce new simulation semantics.

Known limits intentionally retained: unbounded archives/partner maps/audit payloads;
worst-case relationship-suffix revision; older forecast/history scans; snapshot clones
and serialization; targeted rather than exhaustive causal validation; external meeting
opportunities;006 novelty vs008 joint-support mismatch; frozen received quality;
finite-horizon deception; mistaken confidence and forgetting. No database, general
event framework, planner, compression/history deletion or large refactor is justified
by this documentation/test audit. Observer exports are not resumable world saves.
No arbitrary logical inference, language generation, autonomous search, global reputation,
recursive gossip, factions/institutions/culture/consensus, consciousness, Souls/free will,
broad social emergence or million-agent capability is demonstrated by these controls.
