# Using History Explorer

History Explorer is a local window into recorded World of Individuals histories.
It displays what was captured, what was selected, what actually executed and how
state continued. It does not run a simulation, invent motives, complete failed
inquiries, change canon, offer gameplay or score policies using newer rules.

For a hands-on sequence, use the [first walkthrough](history-explorer-first-walkthrough.md);
the [observational report](history-explorer-observations-019.md) follows its exact
original references and qualifies the resulting development recommendations.

## Start and open evidence

Install Python 3.10 or later and use a modern browser. From the Eldoria repository:

```powershell
python -B tools/history-explorer/explorer.py
```

The browser opens http://127.0.0.1:8765. Keep the terminal open. Ctrl+C stops the
server. Add `--no-browser` for manual browser opening or `--port 8766` for another
local port. Full data preparation is optional:

```powershell
python -B tools/history-explorer/explorer.py --prepare
```

Normal startup discovers336 complete019 and164 complete020 histories.019 uses a
bundled derived catalog;020 builds a derived catalog on first launch. A trial is
not trusted because it appears in that list. Opening it hashes
the original compressed bytes, the entire decoded archive including newlines, and
its exact JSONL record (019 one-based,020 original zero-based). It validates membership, identity and point
references, and checks manifest identities against protected inventory when present.
Source provenance is expandable beneath the title. Scientific status comes from
repository review/protection documents; an archive hash does not validate a theory.

Default load is genuine useful_contact / seed42 / Baseline, core archive record103.
Search the trial list by case, policy, seed, window, gap or ordering; select a result
and press Open selected history. Configuration parameters are experimental inputs;
case setup can override individual initial food/hunger. Read actual state from its
captured input, not the generic stock/hunger configuration parameter.

The seven preset buttons open real pairs: small score change, private reversal,
Deadline/delayed meal, unexecuted inquiry, conservative admission, complete null and
observation gaps. They are recorded evidence witnesses, not scripted narrative scenes.

Six additional020 presets show sharing after contention, missed donor/costly failure,
three claimants for one unit, later depletion, complete null and reconvergence.
Inspect the right history to follow its individual decisions. In Actor Perspective,
sharing/Need trial33 individual1 at0 sees stock1 and scores Acquire80/Invite58;
it does not know its later loss. Observer tick4 shows separate grant/failure balances;
tick16 shows Accept and tick17 the recipient's meal. In lost_contact/Acquire29,
the original busy-response omission precedes Unavailable2 and the depleted result4.

Filter “Acquisition admissions / busy intervals” or “Grants and failures” to navigate
actual records. At an admission, the linked state point precedes cohort reservations;
the admission itself gives the new busy interval. At a same-tick completion, each
operation retains its own before/after balances while the linked point can include
other completions. No intermediate full state is fabricated. Observer cache details
remain hidden from actor panels. Full Grouped assessment/progress cells absent from
captured activity inputs remain explicitly unavailable.

020 comparisons include physical-start qualifications: Owned redistributes initial
stock into inventories; Closed changes access; Ignored/Need/Acquire share open cache
physics and differ in selection. Equal terminal food/hunger can coexist with different
trust/history. Common-time panels show the actual captured tick, not interpolation.

## Follow an individual

Choose Individual 0, 1 or 2. On narrow screens, expand “Choose a history and browse
its timeline” for library controls; the individual selector stays visible. Sticky
Histories / Current moment / Perspective links move between panels.

Actor Perspective shows only an allowlisted projection
of that individual's captured input at the selected moment:

* Activity choices: own food/hunger, episodes, directed trust, concerns, local
  source/attribution knowledge, captured learning/candidates, public availability,
  scores, eligibility, selected action and public reservation bounds.
* Recipient responses: their separately captured current own input, incoming
  invitations and actual reply candidates. Earlier activity state is not carried
  forward as though it were the response input.
* Exact recorded points: the selected individual’s captured own Agent fields,
  including retained episodes/trust. This is an owned-state capture, explicitly
  **not a decision**. No nearest-time fallback or observer-event reconstruction.
* Resource actions: actual own food/hunger/traits, observed partner signal, candidate
  scores, directed trust/episodic contribution and original memory-cause references.
  Reference IDs do not restore forgotten episode contents.

“Not captured at this moment” means no supported captured input/field supplies the
value. “Not known to this individual” identifies observer/private information that
is outside the local boundary. Neither means zero. Full Grouped assessment bases and
separate Contextual progress cells are not in the activity input; the viewer does not
fill them from later audit records. At atomic inquiry completion there is no invented
internal actor snapshot. Navigate to the next captured decision to see later local state.

Observer Perspective is visibly labeled and uses a different background. It exposes
all captured agents, inventories, trust, resource accounting, consent, arbitration,
actual commitments/events/queries and recorded concern/assessment history. A selected
step can show its subsequent outcomes with their original references; those are not
what the actor knew when choosing. Whole-world comparison results are observer-only.
Switching perspective changes the displayed information boundary, not the history.

## Navigate time and consequences

Use Previous/Next for any recorded moment, Previous/Next event for significant
records, Play/Pause and speed for recorded replay, and the slider or world-tick jump
for longer distances. N jumps to the selected individual's next activity decision.
Arrow keys navigate, Space toggles playback, [ and ] move between significant events
when focus is outside editable controls/buttons. Native controls also support Tab,
Enter and arrow-key operation. The event filter and list show up to 45 nearby records;
move the timeline to reveal the next portion. No choices are invented during gaps.

World tick, period/window, point ordinal and event/query ID are separate quantities.
References retain namespaces such as `points[3]`, `needs[0]` and
`final_state.base.base.base.base.base.events[1]`. Same-tick navigation order follows
recorded step/point counters and protocol phases. It does not imply a fractional
world tick or an unrecorded deliberation.

Follow recorded-reference buttons from a decision to before/observed/sent/admitted/
after points, actual scene events and a native executed query. Observer references
switch explicitly to Observer Perspective. An invitation can be declined, contended
or reciprocal; provisional acceptance is not execution. A planned inquiry with a
null step.query did not execute in that step. A meeting need not transfer anything.
A received response need not be correct, novel or useful.

At events lacking an exact point, the observer card labels the latest captured
snapshot at or before the tick. It may precede that event's outcome or include other changes at the same tick. Follow the
linked after-point; the viewer does not interpolate an Agent state. Assessment and
concern prefixes use recorded counters and remain observer audit, never actor recovery.

## Worked example: a score change that really executes

Preset A uses Experiment019, seed42, useful_contact, default window24/gap1/interval4/
Ascending order. Baseline is `experiments/019/seed-42.jsonl.gz`, record103; Scale110
is record107. At period0 step0, observed point0/tick0, individual0 owns food2 and has
hunger80. Baseline records Wait0, Consume80, Invite78 and selects Consume. Scale110
records Invite85 and selects Invite under otherwise identical focal inputs.

In the single Baseline history, Next event reaches its meal at tick1: food2→1 and
hunger80→40. The actual resource commitment is 1→0. Resource event0 at tick3 is
individual1's Offer; event1 at tick4 is individual0's Accept, transferring one food.
Scale110 instead records Request3/Offer4/Accept5 (events0–2) in the 0→1 direction.
Follow the after-points or jump forward to inspect continuing material/trust effects.
These events demonstrate a different executed history, not a globally better policy.

In comparison, switch to Observer Perspective. Initial state/profiles and clock
settings match; the first voluntary difference is tick0. The first actual execution
difference includes the Baseline meal at1, before the resource-action difference at3.
Final hunger is [100,95,100] versus [100,100,100]; pressure-time has opposing individual
effects. Final equality and full-state equality are separately reported.

Other useful witnesses:

* Preset B, unproductive_contact records109/113: the focal initial input is the same
  as A, but private cooperation differs. Inspect it through Observer Perspective;
  Actor Perspective does not expose the partner's private food/profile.
* Preset C, ongoing stock4 held records182/183, EatFirst/Deadline: first choice
  difference16; actual 2→0 commitment, Leave19/event6, meal17 versus20. Hunger
  differences persist through24/48/72 but terminal material reconverges. It is a
  delayed-meal effect after an ineffective encounter, not improved cooperation.
* Preset D, information Descending held records7/10: jump45, then Next decision
  of individual to select period0 step3, observed point7. A tick jump first selects
  the earliest record at that tick, which can be a need update. Baseline intends Inquiry but actual matching starts resource
  2→0; no query executes there. Later actual queries51→63/64 have different results.
* Preset E compares core record103 with short-window held record49. The four-tick
  actual Offer/Accept fits six ticks retrospectively, but the public bound8 excludes
  that short-window resource invitation. No actor knew the future duration.
* Preset F, no_food core records7/10: no voluntary/executed difference and complete
  final Snapshot equality. Preset C instead has material reconvergence with unequal
  complete final Snapshots. Equal food/hunger does not erase history or relationships.

## Trace actual consequences (Observer only)

Choose **Observer Perspective**, then **Trace recorded chain (Observer)** in the
reference controls. The new section follows a recorded activity cohort through
outgoing invitations, independent recipient replies, arbitration, actual resource
actions or inquiry, delivery and recorded consequences. The cohort selector can
also open a different original period/step without pretending it occurred at the
current replay moment. Click a typed node to open its exact original source record;
the replay moves to the recorded containing moment. Before/after links open exact
captured points. A scene result opens at its terminal resource action.

Original IDs and ordinals remain separate: resource scene, resource event, native
query, provenance query, native receipt and provenance receipt are different
namespaces. A null native response can coexist with a real provenance delivery.
H10 query0 now explicitly links provenance receipt1 and assessment record1,
support20→60/Partial→Resolved. Empty H7 inquiry stays empty. Novelty, support/status
progress and objective correctness are distinct; the interface does not label
every reply useful or true.

The **Exact typed edges** disclosure states each recorded basis for a connection.
Earlier event nodes can be recorded memory causes; the first later resource decision
that explicitly names a scene event is linked too. These references do not recover
an episode for an actor or establish that it was the sole cause of a later decision.
Missing causal links remain unknown. Pre-epoch cause records can be inspected in
the exact-source panel with an explicit warning; no earlier replay moment is invented.

Under **Find resource encounters and invitations**, choose **Completed encounters ·
zero transferred** and set **After world tick (exclusive)** to258 in ongoing/C85.
The result is65 completed scenes, each Request→Leave/Withdrawal, not130 actions or
195 invitation dispositions. A click opens the original scene and its terminal
action. Other filters distinguish transfers, Request→Refuse, other completions,
Declined, Reciprocal and unexecuted proposal dispositions. Reciprocal is a paired
proposal, not an additional encounter or a failed transfer. Only the first80 results
are rendered; the displayed count includes every matching captured replay record.

The personal summary separates selected activities, actual meals, resource actions,
completed scenes and inquiries for the chosen individual. Counts cover captured
replay periods; pre-epoch preparation is excluded. There is no interpolation of
thought, memory or actor-private state. All these derived chains and summaries
disappear in Actor Perspective. Original source verification/import boundaries are
unchanged. See the [causal-navigation acceptance](../../tools/history-explorer/validation/causal-navigation-acceptance.md).

## Compare histories

Compare this history opens the paired area. Select any indexed trial as the right
history and press Compare selected histories, or choose a preset. Playback and tick
navigation align both worlds by world time. A column without a decision at that exact
tick says so. Observer states carry their own captured timestamps; no period-index
alignment or synthetic intermediate state is used.

Inspect right history opens that trial’s full timeline near the current world tick.
Return to single history closes the pair and keeps the left trial.

The comparison distinguishes initial state/profile matching, configuration and
physical-clock differences, unequal endpoints, first voluntary choice versus schedule
difference, first actual execution, first resource action, temporary material
differences at shared ticks, terminal material reconvergence and complete final
Snapshot equality. A configuration mismatch is a qualification, not proof of an
unfair policy comparison. Complete Snapshot equality is not a claim that every
activity audit/candidate score or need record in the entire Trial is identical.

## Import, errors and performance

Import / export downloads a versioned JSON display package. Open that file through
the local-file control to verify it against registered source membership again.
Imported timeline/actor/status claims are discarded and regenerated. Unknown schemas,
unregistered/invalid sources and changed trials are rejected; previous valid data
remain visible with an error status. The release does not accept arbitrary simulation
saves or run new worlds to repair missing evidence. Source files are never changed.

Optional `--include018` indexes valid final 018 and 019 archives (slower full scan).
The 018 adapter preserves its own modes, configuration and physics; the invalid
clock-integration development bundle remains excluded. Formats001–017 are unsupported.

Trial replay starts at its captured clock epoch. Earlier preparation records remain
in the complete original trial/export; a separate pre-epoch replay and fully clickable
provenance graph are not implemented.

A missing Python command requires a Python installation or its local launcher. If
port8765 is occupied, choose another port. A source hash failure requires checking
repository integrity, not silently repairing an archive. A stale/corrupt derived cache
can be replaced through --prepare in the ignored target directory; never delete or
regenerate original experiment evidence. If the page cannot reach the server, keep
its terminal running and check the displayed port.

Measured engineering results and exact browser coverage are in the application's
[acceptance record](../../tools/history-explorer/validation/acceptance.md). Normal
opening streams the selected archive and decodes one trial, not the whole corpus.
The largest registered 019 trial is17.12MB raw / approximately29.47MB exported; the
Python measurement process peaked around201MB. Browser object memory is additional.
Only a single trial or pair is loaded; long-history comparison can still be costly.
No population scalability or simulation throughput improvement is claimed.

## Separate Experiment020 demonstration

The main018/019 reader remains unchanged by020. An explicit small observer-only
`finite-demo.py` adapter supplies ten original020 histories with current cache stock,
pending attempts,local selection fields,grant/failure,meals,transfers and exact source
references. Serve `experiments/020` on localhost8766 and open `/demo.html`.
It has no actor mode or complete inquiry browser and never feeds cognition or
scientific validation. See the [actual acceptance and screenshots](../../experiments/020/viewer-acceptance.md)
for commands,source verification and limitations.020 remains unprotected research.
