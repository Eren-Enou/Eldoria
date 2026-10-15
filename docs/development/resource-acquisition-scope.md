# Finite environmental food acquisition:scope only

This is a technical proposal after [019 research](../../experiments/019/results.md),
not an implemented system or the start of020. Independent019 scientific review should
precede another behavioral experiment. Do not adopt a new priority or acquisition
rule from this document automatically.

## Compact question

Can a locally visible, costly and contested transfer from a finite environmental
stock change voluntary resource contacts and persistent need histories under common
physics, without guaranteeing successful acquisition? Distinguish acquisition from
redistribution between people, and both from creation. A finite cache delays exhaustion;
it does not establish production, renewable supply or a sustainable economy.

An initial controlled comparison should hold total potential food constant, assigning
some units to an unowned cache rather than initially to agents. Otherwise adding
food and enabling acquisition confound supply with access. Include empty, scarce and
abundant finite caches, imperfect competition, wasted time and later exhaustion.
No farming,hunting,jobs,markets,currency,travel,institutions or replenishment.

## Ownership, observation and accounting

Represent a stable domain CacheId and nonnegative stock as physical world state,
not actor cognition or an ECS handle in an archive. Food is unowned until a successful
resolution transfers one unit to an individual. Acquisition must never take another
agent's private inventory. Existing inventory overflow checks remain authoritative.

Expose only currently local access and observed stock/cost. With stationary actors,
one co-present cache is sufficient to test the channel;there is no need to introduce
navigation. A stale observation can become wrong after another acquisition. Policy
receives own food/hunger plus this narrow observation,not rivals' private needs,
future availability,observer history or guaranteed success. No new cognitive store
is justified merely by adding a currently visible object.

Conserve:initial agent food + initial cache food = current agent food + current cache
food + actual consumption. Explicit destruction is zero unless separately specified.
Acquisition decreases cache stock exactly when agent food increases. Validate ownership,
current availability,capacity and stock at resolution;negative stock and double grants
are illegal. Record failed attempts and real time even when food is gone by completion.
No refund or later gift should repair an inconvenient failure.

## Minimum time and competition design

First assess an atomic acquisition,analogous to existing atomic inquiry,using the
unchanged recurring clock for everyone. Four logical ticks is a candidate engineering
cost to examine,not a calibrated or approved value. A later specification must justify
cost and fixed sensitivity levels before outcomes. Do not infer exact success from
that public bound. The choice competes with eating,contact,inquiry and waiting.

Simultaneous claims need explicit deterministic arbitration based on legitimate
proposals,not future hunger or a desired beneficiary. Compare Ascending/Descending/
Rotating order. Define whether unsuccessful valid claimants spend the shared attempt
interval and show the cost;do not silently skip failed attempts. Revalidate stock at
completion and cap grants. A selected actor cannot also negotiate or inquire during
its acquisition. Atomic execution initially limits observation of interior availability;
stepped/disjoint scheduling is a separate architectural question,not a prerequisite
to testing finite acquisition. Reuse old commitment rules where applicable rather
than inventing a general planner.

The exact cohort arbitration between acquisition and existing contact is unresolved.
It needs a prospective legality/time design before implementation. A controller must
not pick an activity after seeing which outcome is favorable. The019 result also
does not justify a new universal score combining acquisition,trust and hunger.

## Evidence and acceptance boundaries

Each attempt needs actor,cache,selection input,public observation,bound,actual start/
end,legality,outcome,stock/inventory before/after,elapsed need change and causal links.
Preserve misses,contention,poor timing,earlier consumption of potentially shareable
food and later refusal. Run far enough to see a subsequent need/contact consequence.

Require exact replay,full conservation,actor-local/private-state twins,empty-cache,
late-depletion,overflow,overlap/termination and read-only historical gates. Independently
reconstruct cache and agent ledgers with clock progression. Keep new evidence separate
from001–018 and unprotected019. Wall times and observer storage remain separate.

Before code,resolve:which opportunity schedule is physically grounded,how simultaneous
attempt costs are charged,which minimal local priority is defensible,and whether a
finite channel can answer the question without arbitrary new scoring. Renewal and
ecological sustainability remain too ambiguous for this proposal. No implementation
or experiment number is assigned here.
