# First walkthrough: follow an individual, then challenge the explanation

This walkthrough uses original Experiment019 records reviewed at
`bc15b4551e19455cf8545019213a04cc2ee9df2b`. It runs no simulation.
The later display-only native-receipt wording correction is `a7ae325`.
See the [observations](history-explorer-observations-019.md) for exact causal
analysis and [user guide](history-explorer-user-guide.md) for all controls.

## Start and find the first choice

1. In the repository root, run:

   ```powershell
   python -B tools/history-explorer/explorer.py
   ```

   Open [History Explorer](http://127.0.0.1:8765) and keep its terminal running.
   If it is already running, use the existing page. A second server on the same
   port is unnecessary. Python3.10+ is sufficient; no Rust build is needed to view.

   **Notice:** replay displays recorded evidence; Play does not ask an agent to act.

2. The default is useful_contact, seed42, Baseline, original core record103.
   Expand **Source provenance and verification** and confirm archive
   `seed-42.jsonl.gz`, record103, case/mode/seed and successful original compressed,
   decoded and member verification. Collapse it afterward to reduce scrolling.
   If another history is open, expand **Choose a history and browse its timeline**,
   search `useful_contact`, choose the Baseline/seed42/window24/gap1/I4/Ascending
   entry ending `#103`, then **Open selected history**.

   **Notice:** the generic stock parameter12 does not override this case's actual
   individual food2/6/0. An ordinal without its archive is not a unique identity.

3. Select **Individual0**, enter0 in **Jump to world tick**, click **Jump**, then
   **Next decision of individual**. Choose **Actor Perspective**. This reaches
   period0/step0, point0, tick0; a tick jump alone can select an earlier snapshot.
   On a narrow screen, use the sticky **Perspective** link to reach the inspector.

   **Notice:** food2/H80, Wait0, Consume80, Invite78; Consume wins. Both resource
   targets are eligible with an eight-tick reservation. Expand own episodes/trust:
   both are empty. Expand public availability: all three people are available.
   Their private food and traits are not part of individual0's view.

## Follow a meal, a response and a real transfer

4. Click **Next event** to reach the meal at tick1,
   `periods[0].steps[0].activity.consumption[0]`. Switch to **Observer Perspective**.
   Read the meal's original values: food2→1,H80→40.

   **Notice:** an observer event may have no captured actor decision. “Not captured”
   is not zero or an invitation to infer cognition. Observer cards can be latest
   captures at that tick; follow explicit points for exact decision scope.

5. Return to tick0's decision using Jump0/Next decision. Its **Follow recorded
   references** links include Event0 Offer@3 and Event1 Accept@4. In Actor view the
   labels start **Observer reference:** and clicking explicitly changes perspective.
   Open Event1. Alternatively Jump4 first lands on a need update; **Next event**
   then reaches the Accept.

   **Notice:** the source path ends `events[1]`. Participants are[1,0], food
   [6,1]→[5,2], transferred1, termination Agreement. Individual1 chose to invite
   after individual0 chose to eat. Consent and arbitration produced a1→0 encounter.
   Next decision for individual0 at4 and subsequent Offer7/Accept8 show it giving
   food back. Trust and retained episode contribution affect those later scores.

## Compare the alternative history

6. Expand the history library if collapsed. Press **A · Small score change**, wait
   for **Paired comparison ready**, choose Observer view, then **Jump to first
   voluntary difference**. Compare Baseline C103 with Scale110 C107.

   **Notice:** initial state/profiles and physical settings match. At0 individual0's
   Invite changes78→85 while Consume stays80. First execution differs at1 (meal);
   the resource histories then show Offer3/Accept4 versus Request3/Offer4/Accept5.
   The initiation direction changes, while food still first moves from1 to0.

7. Use **Inspect right history** to open Scale110's complete timeline. Jump5 and
   use Next event for Accept; switch individual selectors0/1/2 at actual decisions.
   To see terminal comparisons, reopen preset A and choose **Compare final recorded
   states**. Read the all-individual table, not just the selected actor.

   **Notice:** eight meals are redistributed2/6/0→4/4/0. Lower pressure for0 comes
   with higher pressure for1. There is no policy-wide “winner” label to infer.

8. Try **B · Private cooperation reversal**, then Inspect right to open C113.
   At0, individual0 again chooses Invite85. Jump1; use Next event until the moment
   is **Consent**, select individual1 and Actor view.

   **Notice:** source1 has independently eaten, now food5/H0; Wait0 beats accepting
   the invitation at−2. Actor0's original input was identical to useful_contact.
   This is declined consent, not a resource transfer failure or proof of hostility.

## Follow the long history past exhaustion

9. In the library search `ongoing`. Choose exactly seed42/Baseline/window24/gap1/
   I4/Ascending/stock12/H parameter20, **#85**, and Open. Confirm provenance says
   core `seed-42.jsonl.gz` record85 and horizon576. This is not longitudinal #91.

   **Notice:** default ongoing starts with12 food per person. It carries state
   across24 windows; later periods are not resets.

10. Select individual0. Jump101/Next decision, then106/Next decision, Actor view.
    Both are period4, steps4/8, observed points120/126.

    **Notice:** eating is legal at100, but contact scores133/188 win. The first
    encounter obtains food at106; the next offers a unit back at110. A contact score
    is not expected net food gain. Try individual2 at a decision: it can act
    independently, but its invitations need not be accepted or matched.

11. Jump258 and select Observer view. Point304 is the first captured all-food-zero
    state, H[60,25,100]. The last meal consumed individual0's last unit at258.
    The ledger reads initial36 = owned0 + consumed36.

    **Notice:** knowing everyone is empty is observer knowledge. Individual0 knows
    its own inventory and its legitimately observed encounters, not that global fact.

12. Jump264/Next decision, Actor view. Expand **Own retained episodes** and **Own
    directed trust**. Then Jump268, Observer view and **Next event** for e112 Leave.
    Repeat at300/Next decision and400/Next decision; finally jump576.

    **Notice:** first post-exhaustion scene46 starts266, Request267/Leave268,
    transfer0. At400 individual0 retains16 recent Request/Departure episodes with
    zero valence and trust1=100, still selecting contact133. The final state is all
    food0/H100. There are65 such completed late encounters, not130 meetings;
    invitations marked Declined/Reciprocal belong to a different counting stage.

## Keep a counterexample beside the story

13. Preset **C**: jump16/Next decision and inspect the actual2→0 meeting, Leave19,
    meal17 versus20 and later states24/48/72/100. A delayed meal after useless contact
    can lower modeled pressure; it does not establish smarter cooperation.

14. Preset **D**: Jump45/Next decision of individual0. Expand arbitration and actual
    execution references in Observer view: planned inquiry loses, `query=null`.
    **Next executed inquiry (Observer)** reaches an actual later question. Baseline
    query0 belongs to2 and stays Open at63; Scale90 query0 belongs to0 and resolves
    at64 through provenance receipt1. An empty native-response field alone does not
    exclude another delivery channel. Never learn from the unexecuted tick45 plan.

15. Preset **E**: inspect right history at0; resource reservation8 exceeds window6,
    so targets are ineligible. Preset **F**: compare final states; entire final
    Snapshots are equal. Preset **G**: choose Observation gaps in the moment filter;
    no decisions were recorded inside those marked intervals.

Use **Show recorded moments → Resource actions** and the significant-event buttons
to reduce clutter. The list displays nearby records rather than every event in the
long trial at once. If a moment is a need update instead of the decision you wanted,
read its type/reference, then use Next decision/Next event. If a pair is loading,
wait for its ready status before interpreting the previous screen. None of these
controls creates a new action or reconstructs missing actor state.
