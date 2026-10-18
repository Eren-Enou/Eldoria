# Concurrent program working notes

Provisional engineering notes,not experiment evidence or a passed022design gate.

## C complete

See tools/history-explorer/validation/knowledge-integration/acceptance.md and current
checkpoint. All41Python/4JS,214refs,main8765 actualsource→grant→meal,negative/expiry/
null/mixed/stored-zero/rivalcases pass. MainPID123980/tab3,prototype8766 preserved;
8767/testtab5 stopped/closed. Csourcecommit follows B e059fcb. No022code exists.

## Prospective D investigation leads (NOT a design gate or runtime)

No022 specification,matrix or runtime exists. Must finishB/C milestones and write
concurrent-world-architecture.md + prospective022spec/gate BEFORE runtime edits.
CoreSimulation::new alreadyacceptsarbitrary IDs and Runtime.active isBTreeSet;
coreadvance processesall disjointscenes once/tick. Higherencounter/recurring/021
adapters hard-code3/singleactivepair. Oldrecurring sync uses3array;cannotbeusedfor4.
021 freezes src/lib.rs and src/simulation.rs exactly. Keepbothbyte-identical.
A scratch Rustinclude-facade test inTEMPfailed(relativechildmodulelookup);do not
waste time building a copied/full-crate facade withoutnecessity.
Read-only freeze-name scan found no source locks for src/experiment.rs,
src/simulation/adaptive.rs,src/simulation/provenance.rs,src/model.rs (verifyagainst
allinventoryauthoritybeforeediting). A possible narrow additive wiring route is a
new experimental public child under experiment.rs and private adapter child under
simulation/adaptive.rs. This couldaccessoriginalSimulation privatecoreandreuseall
oldcognition withoutmodifyingfrozen parent declarations. Do not assumeapproved.
Pureadditivemoduledeclarations leaveoldfunctions intact;documentnamespace choice.
A four-person clock needs actualquery-time updates too:adaptive/provenance have
explicitRuntime.tick increments followedby sync_recurring calls;theyarenot source-
locked. A possible opt-in clockhookafterthose existingcalls,absent-resource no-op
foroldworlds,mustbeprospectivelyspecified/verified;notimplemented. RootSim time
writes are frozen;inspectexactquery callpath beforeassuminghookcoverage.
Relevant source:provenance.rs ~840–930(provenance_follow),adaptive.rs ~650–830
(inquiry_response). NativeDirectcalls these;rootreceive_information doesnot add
query time. Newkernelcouldsync4 beforecoreSchedule.run(resourcephase);do notinstall
old3personClock. Atomicqueriesremainexclusivewithallotherwork,noinventedmid-query
choices/glimpses. Need actualadmitted/rejectedoverlap witnesses.

Potential minimal registry needs explicit Send1tick,Reply1tick,Resource,Claim,Meal
lifecycles plus bounded nonoccupying pending invitationtokens;not a global2tick
handshake that freezesfreepeople. Pending ownproposal preventsduplicateoutgoing;
source repliesindependently whilephysicallyfree;actualadmissionrechecksavailability.
These are design leads only;derivelegality/compatibility fromsource and register
beforeoutcomes. Allfour may send thenrespond nexttick withoutdoublebooking;old
reciprocity/Continue semantics mustremainactualconsent,noforcedpairs. Cap2vs1social
execution intervention,otherworkmatched. Uselegitimatepriortrustfixtures topermit
0↔1/2↔3 whileallpartnersremaincandidates;includepriorlessranking-dominancenegative.
Needno observer-only globalcountersinnewactorinput:consider actor-local protocol
encountercounter/context ratherthanfeedingunobservedothers' Runtime.scene count.
No newsemantic cognition/scoring/retentionretuning. Aboveideas arenot chosen yet.


Further D issues to inspect prospectively:the fixed3person recurring clock cannot
advanceactor3 insideatomicqueries. Existing Direct inquiry timewrites/calls are in
adaptive.rs/provenance.rs,followedby sync_recurring;these files areunfrozen butold
semanticsremainprotected. A newresource-guarded sync hook may be possible there,
witholdworldabsent-resource no-op andall001–021exactreplay;parentRootSim staysbyte-
identical. InspectnativeSelfKnowledge/acquiredreply branchesbeforedeclaringcoverage.
CoreSchedule.run incrementsoneTickandprocessesallactive disjointscenes;new4clock
couldsyncbeforethatresourcephase,withoutinstallingold3clock. Noquery overlapsany
physicalcommitment;real admitted/rejected witnessesrequired,noinventedinteriorchoices.

Do notletglobalRuntime.scene count revealunobservedothers' history innewinput;
considerowner-local protocol encountercounter(context,notnewsemanticmemory).
Do notreusean oldobserver-captured full outgoingInput ascurrentrecipientcognition:
pendingprotocol mayretainintent/target,butresponseprojectionsmustusecurrentlocal
state andexistingresponse arithmetic. NeedexplicittypedSend/Replyreservation phases
andnonoccupyingboundedpendingtokens,not a global2tickhandshakefreezingfreeactors.
OwnpendingduplicateInvite maskisphysicalprotocollegality,notretunedutility;register
itprospectively. Resourcecap2vs1 is intervention;otherphysicalworkmatched.

Legitimatepriorinteractions can establishpairpreferenceswhileall3otherpartners
remaincandidates. Includepriorlesslowest-IDrankingnegative;noforcedpairingsor
cooperation. ReusecoreResourcelegality,oldboundedcognition/caps/GroupFIFO,006/012
queryrules and021datedknowledge. No planner/retentionretuning. NewprivateWorldwrapper
mustnotexposeunsafe genericSimulation methodsduringclaims/handshakes. These remain
investigationleads,not a selecteddesignorpassedgate;writeD/Ebefore022runtime.
