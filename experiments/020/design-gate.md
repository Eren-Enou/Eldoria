# Experiment020 design gate

Gate C/D: **PASS with explicit scheduling qualification**,before treatment code or
matrix outcomes. Viewer milestone `831d8fc` is committed/pushed. The new user request
authorizes020;older “do not begin020” navigation statements described the previous
task boundary. They do not authorize021 or adoption/protection of020.

The [specification](specification.md) fixes164 worlds,two acquisition policies,
four-tick baseline/two/eight sensitivities and exact supply/access controls. No
outcome has been inspected to choose these values.

| Mandatory question | Implementation-independent answer |
|---|---|
| 1. Food conserved for every outcome? | A grant is paired stock−1/inventory+1; failure debits neither; only an existing legal meal destroys food. A u64 ledger checks every captured world state and each operation. |
| 2. Two people can claim the last unit? | Yes. All proposals use the same pre-admission local observation; no unit is reserved at start. Both legal attempts run their full duration. |
| 3. Can one unit be granted twice? | No. Completion rechecks current stock and atomically debits before each credit. Subsequent same-tick claimants see the depleted physical stock in resolution,not in their earlier input. |
| 4. Loser has a consequence? | Its actor is unavailable for the entire attempt; shared need pressure advances; otherwise feasible invitations may fail. No refund or automatic fallback. Compare actual later actions,not just a failure counter. |
| 5. One consistent global clock? | Each tick is charged once across concurrent resource/claim work. Existing recurring sync handles progression. Claims complete on their declared deadline; atomic inquiry cannot leap over a pending claim. |
| 6. Busy actor cannot do incompatible work? | Acquisition reserves the runtime busy set; new020 lifecycle excludes claimants from decisions,meals,resource participants and recipient response opportunities until release. A private world wrapper does not expose the generic Simulation API during claims. |
| 7. Actor chooses acquisition? | A pure local selector compares recorded candidates. The resolver only validates/resolves submitted choices; it does not select a beneficiary or desired future action. |
| 8. Local information only? | Own legacy input plus currently observable cache and public lane availability. No rival inventory/traits,future claims,truth,scenario names,seed or observer audit. |
| 9. Observation may be stale? | Stock1 can be observed by multiple actors before commitment; one completion can deplete it before another completes. Earlier input is immutable. New current zero is ineligible. |
| 10. Existing consent/resource behavior preserved? | Original pure recipient/arbitration and resource resolution remain; busy recipients have no response opportunity. New query-overlap admission rejection is explicit and has no substituted fallback allocation. Old rules/bodies stay frozen. |
| 11. Nulls possible? | Empty/null have no stock;Closed/Ignored ignore access in actual behavior;Need may retain available stock due old scores. No actual difference means no claimed acquisition effect. |
| 12. A more active policy can do worse? | Acquiring can reserve someone whom a donor would otherwise meet,lose contention,or shift access toward a different eater. lost_contact/ordering diagnose this; lack of an adverse result remains a registered null. |
| 13. Primary total food matched? | All five modes preserve owned+cache total;Closed/Ignored/Need/Acquire also match owned starts. Owned is separately labeled placement,not pure access. |
| 14. Scoring separate from physical access? | Closed versus Ignored holds old ranking;Ignored/Need/Acquire share physical access and differ in ranking. Duration and order vary on fixed registered families without policy changes. |
| 15. Independently reconstructable? | Record current actor input/choice,live busy state,attempt IDs/deadlines,ordered resolution,balances,needs,consumption,actual social/query records and after-state. A separate reader checks primitives and rejects corruptions without calling020 policy/evaluator. |

## Why inquiry is explicitly exclusive

The existing recurring adapter advances resource negotiations one tick at a time,
but `provenance_follow` executes an atomic multi-tick inquiry. The frozen shared
clock synchronizes pressure without a callback for a new cache deadline. Letting a
query leap over an active claim would either delay the grant beyond its declared
duration or require retroactively constructing an unobserved material update.
Neither is acceptable. A general concurrent inquiry scheduler is beyond this study.

Therefore020 declares atomic inquiry mutually exclusive with active acquisitions.
Existing pending claims make the public inquiry lane unavailable. A newly selected
claim can close it during a handshake: preserve raw provisional allocations and
record the final unavailable allocation plus explicit atomic-overlap reason. Do
not count a query,grant information,learn or replace it with a preferred meeting.
If short claims finish before arbitration,the lane can legitimately reopen; consent
still must have been given at the real response phase. With no claims,actual old
queries retain their original atomic timing and cognitive semantics. This is an
explicit limitation and measured competing opportunity,not an inquiry improvement.

New cache reservations can share time with the independent third actor's existing
resource pair; the old resolver releases only its own participants. Acquisition
releases only its own actor. No agent can belong to both. When nobody is free,an
observer tick advances work/needs without invented choices or preparation. Otherwise
existing cohort preparation remains,so legacy concern-age/cohort timing can mediate
later inquiry differences and must not be called pure information improvement.

## Scope and go/no-go boundary

Use a new opt-in adapter/private wrapper and existing public/protected primitives.
Frozen `simulation/recurring.rs` has private bookkeeping helpers; duplicating the
minimum current-prototype bookkeeping locally is preferable to altering their
historical source or exposing a general scheduler. The cache has one physical state,
at most three live claims and no new cognitive cells. Report duplicated adapter
size/cost honestly. No default branch inserts this state into old worlds.

Stop and report if implementation cannot meet the written timing/legality boundary
without changing frozen rules,if exact historical control fails unexplained,or if
new unreported knowledge is needed to obtain an outcome. Bugs must be corrected to
this frozen design; retain invalid candidate evidence/disclosure if produced.
Scientific review/protection and any next experiment remain separate future tasks.
