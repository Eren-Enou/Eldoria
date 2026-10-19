"use strict";
const byId = id => document.getElementById(id);
let trial, source, base, ordinal = 1, clock = 0;

function work(commitment) {
  return typeof commitment.work === "object" ? "Resource" : commitment.work;
}
function original(path, value) {
  byId("reference").value = `original ordinal ${ordinal} · ${path}`;
  byId("original").textContent = JSON.stringify(value, null, 2);
}
function link(label, path, value) {
  const button = document.createElement("button");
  button.textContent = label;
  button.onclick = () => original(path, value);
  return button;
}
function card(host, title, lines) {
  const element = document.createElement("article");
  element.className = "card";
  const heading = document.createElement("strong");
  heading.textContent = title;
  element.append(heading);
  for (const line of lines) {
    const paragraph = document.createElement("p");
    paragraph.textContent = line;
    element.append(paragraph);
  }
  host.append(element);
  return element;
}
function intervals() { return ConcurrentModel.intervals(trial); }
function gantt() {
  const ns = "http://www.w3.org/2000/svg";
  const svg = document.createElementNS(ns, "svg");
  svg.setAttribute("viewBox", "0 0 1000 215");
  svg.setAttribute("role", "img");
  svg.setAttribute("aria-label", "Four actor lanes on one shared world-time axis");
  const scale = 880 / (trial.end - trial.start);
  const x = tick => 100 + (tick - trial.start) * scale;
  function node(tag, attrs, text) {
    const e = document.createElementNS(ns, tag);
    for (const [key, value] of Object.entries(attrs)) e.setAttribute(key, value);
    if (text !== undefined) e.textContent = text;
    svg.append(e);
    return e;
  }
  const colors = { Resource: "#98d6b9", Claim: "#d4b47d", Meal: "#99b5e0", Query: "#b7a1dd", Send: "#486a6d", Reply: "#7b9795" };
  for (let actor = 0; actor < 4; actor++) {
    node("text", { x: 4, y: 44 + actor * 42, fill: "#dce9e2", "font-size": 13 }, `Individual ${actor}`);
    node("line", { x1: 100, x2: 980, y1: 39 + actor * 42, y2: 39 + actor * 42, stroke: "#33494b" });
    for (const { record, index, end } of intervals().filter(i => i.record.commitment.participants.includes(actor))) {
      const c = record.commitment;
      const rect = node("rect", { x: x(record.tick), y: 28 + actor * 42, width: Math.max(2, (end - record.tick) * scale), height: 22, rx: 2, fill: colors[work(c)] });
      const title = document.createElementNS(ns, "title");
      title.textContent = `${work(c)} #${c.id}, ${record.tick}–${end}, participants ${c.participants.join(" / ")}`;
      rect.append(title);
      rect.setAttribute("tabindex", "0");
      rect.setAttribute("role", "button");
      rect.setAttribute("aria-label", title.textContent);
      rect.onclick = () => original(`transitions[${index}]`, record);
      rect.onkeydown = event => { if (event.key === "Enter") rect.onclick(); };
    }
  }
  const step = trial.config.horizon > 96 ? 24 : 8;
  for (let tick = trial.start; tick <= trial.end; tick += step) {
    node("text", { x: x(tick), y: 207, fill: "#9eb8b2", "font-size": 11 }, tick);
  }
  node("line", { x1: x(clock), x2: x(clock), y1: 13, y2: 190, stroke: "#f0d58e", "stroke-width": 2 });
  return svg;
}
function lanes(active) {
  const lanes = document.createElement("div");
  lanes.className = "lanes";
  for (let actor = 0; actor < 4; actor++) {
    const lane = document.createElement("div");
    lane.className = "lane";
    const title = document.createElement("h3");
    title.textContent = `Individual ${actor}`;
    const status = document.createElement("p");
    status.className = "muted";
    status.textContent = active.some(i => i.record.commitment.participants.includes(actor)) ? "Committed after starts at this tick" : "No live commitment after starts";
    lane.append(title, status);
    for (const item of active.filter(i => i.record.commitment.participants.includes(actor))) {
      const c = item.record.commitment;
      lane.append(link(`${item.record.tick}–${item.end} · ${work(c)} #${c.id}`, `transitions[${item.index}]`, item.record));
    }
    lanes.append(lane);
  }
  return lanes;
}
function choices(boundary, boundaryIndex, observer, owner) {
  if (!boundary) {
    card(byId("people"), "No ordinary decision at this tick", ["This tick lies inside an exclusive atomic inquiry or at the endpoint. No intermediate decision input is invented."]);
    return;
  }
  if (observer) {
    card(byId("people"), "Actual physical state", [
      `Cache stock ${boundary.stock}. Completed events ${boundary.events}; meals ${boundary.meals}; physical claims ${boundary.grants}.`,
      ...boundary.agents.map(a => `Individual ${a.id}: food ${a.food}, hunger ${a.hunger}.`)
    ]);
  }
  const selected = observer ? boundary.choices : ConcurrentModel.ownedChoices(trial, clock, owner);
  if (!selected.length) card(byId("choices"), "No ordinary choice recorded", ["A reply or committed activity can occupy this boundary. No free decision is invented."]);
  for (const d of selected) {
    const own = d.input.legacy.own, memory = d.input.retained;
    const query = d.input.legacy.query.decision.selected;
    const queryLine = query !== "Pause" ? [`Local query candidate: concern ${query.Ask.concern}, source ${query.Ask.source}, ${query.Ask.strategy}. Exclusive inquiry lane ${d.input.inquiry_lane_free ? "available" : "blocked"}; this candidate is not an executed inquiry.`] : [];
    const box = card(byId("choices"), `Individual ${own.id} · recorded local choice`, [
      `Own food ${own.food}; hunger ${own.hunger}.`,
      memory ? `Own receipt ${memory.receipt}: stock ${memory.observation.stock} observed at ${memory.observation.observed_at}; own takes ${memory.own_takes}; retained upper bound ${Math.max(0, memory.observation.stock - memory.own_takes)}.` : "No retained environmental observation: unknown, not zero.",
      `Public availability: ${d.input.legacy.people.map(p => `${p.id} ${p.available ? "free" : "busy"}`).join(", ")}.`,
      `Candidates: ${d.candidates.map(c => `${c.action} ${c.eligible ? c.score : "ineligible"}`).join(" · ")}.`,
      ...queryLine,
      `Selected ${d.selected}${d.selected === "Invite" ? " → " + JSON.stringify(d.contact) : ""}.`
    ]);
    const index = boundary.choices.indexOf(d), path = `boundaries[${boundaryIndex}].choices[${index}]`;
    box.append(link("Original local input", path + ".input", d.input), link("Original recorded choice", path, d));
  }
}
function events() {
  const host = byId("events");
  for (const [n, event] of base.events.entries()) {
    if (event.tick !== clock) continue;
    const bootstrap = n < trial.bootstrap_events;
    const box = card(host, `${bootstrap ? "Bootstrap history · " : ""}Scene ${event.scene} · event ${event.id} · individual ${event.decision.actor}: ${event.decision.selected}`, [
      `Participants ${event.participants.join(" ↔ ")}. Inventory ${event.balances_before.join(" / ")} → ${event.balances_after.join(" / ")}.`,
      `Transferred ${event.transferred}; outcome ${event.outcome || "still active"}.`,
      ...(bootstrap ? ["Pre-period historical evidence. The declared initial physical intervention follows bootstrap, before the comparison starts."] : [])
    ]);
    box.append(link("Original resource event", `final_state.base.base.base.base.base.events[${n}]`, event));
  }
  for (const [n, grant] of trial.grants.entries()) {
    if (grant.tick !== clock) continue;
    const box = card(host, `Claim #${grant.commitment} · individual ${grant.owner}: ${grant.outcome}`, [`Stock ${grant.stock_before} → ${grant.stock_after}; own food ${grant.food_before} → ${grant.food_after}.`]);
    box.append(link("Original physical result", `grants[${n}]`, grant));
  }
  for (const [n, meal] of base.cognition.consumption.entries()) {
    if (meal.tick !== clock) continue;
    const box = card(host, `Individual ${meal.actor} · actual meal`, [`Food ${meal.food_before} → ${meal.food_after}; hunger ${meal.hunger_before} → ${meal.hunger_after}.`]);
    box.append(link("Original meal", `final_state.base.base.base.base.base.cognition.consumption[${n}]`, meal));
  }
  for (const [n, need] of trial.needs.entries()) {
    if (need.tick !== clock) continue;
    const box = card(host, "One shared need pulse", [`Hunger ${need.before.join(" / ")} → ${need.after.join(" / ")}. One elapsed-time charge for all four people.`]);
    box.append(link("Original need record", `needs[${n}]`, need));
  }
  for (const [n, transition] of trial.transitions.entries()) {
    if (!transition.completed || transition.tick !== clock) continue;
    const c = transition.commitment;
    const box = card(host, `${work(c)} #${c.id} · actual completion/release`, [`Participants ${c.participants.join(" / ")}; started ${c.start}; released ${transition.tick}; deadline/bound ${c.deadline}.`]);
    box.append(link("Original release", `transitions[${n}]`, transition));
  }
  for (const [n, information] of base.cognition.information.entries()) {
    if (information.tick !== clock) continue;
    const box = card(host, `Information receipt ${information.id} · listener ${information.listener}`, [`Source ${information.speaker}; event ${information.event}; retained support ${information.before ? information.before.support : "unknown"} → ${information.after.support}.`]);
    box.append(link("Original information receipt", `final_state.base.base.base.base.base.cognition.information[${n}]`, information));
  }
  if (!host.children.length) host.textContent = "No physical operation recorded at this tick.";
}
function consent() {
  for (const [n, admission] of trial.admissions.entries()) {
    if (admission.tick !== clock) continue;
    const invitation = admission.invitation;
    const box = card(byId("consent"), `${invitation.proposer} → ${invitation.recipient} · ${invitation.kind}: ${admission.reason}`, [`Proposal source boundary ${admission.source}; expires ${invitation.expires}; commitment ${admission.commitment === null ? "none" : admission.commitment}.`]);
    box.append(link("Original admission", `admissions[${n}]`, admission), link("Original sending choices", `boundaries[${admission.source}].choices`, trial.boundaries[admission.source].choices));
    const pending = trial.boundaries.flatMap(b => b.pending).find(p => p.source === admission.source && p.invitation.proposer === invitation.proposer && p.response);
    if (pending) {
      const r = pending.response;
      box.append(link("Original recipient reply", `boundaries[${r.boundary}].responses[${r.response}]`, trial.boundaries[r.boundary].responses[r.response]));
    }
  }
  for (const [n, boundary] of trial.boundaries.entries()) {
    for (const [j, response] of boundary.responses.entries()) {
      if (response.input.own.now !== clock) continue;
      const box = card(byId("consent"), `Individual ${response.input.own.own.id} · independent reply`, [`Selected ${JSON.stringify(response.selected)}. Candidates: ${response.candidates.map(c => JSON.stringify(c.reply) + " " + c.score).join(" · ")}.`]);
      box.append(link("Original recipient decision", `boundaries[${n}].responses[${j}]`, response));
    }
  }
}
function inspect() {
  if (!trial) return;
  byId("reference").value = "";
  byId("original").textContent = "Choose an original record in the current perspective.";
  clock = Math.max(trial.start, Math.min(trial.end, Number(byId("tick").value)));
  byId("tick").value = clock;
  const observer = byId("lens").value === "observer", owner = Number(byId("owner").value);
  const boundaryIndex = trial.boundaries.findIndex(b => b.tick === clock), boundary = trial.boundaries[boundaryIndex];
  const active = intervals().filter(i => i.record.tick <= clock && clock < i.end);
  const statistics = observer ? [[clock, "world tick"], [active.filter(i => work(i.record.commitment) === "Resource").length, "social scenes after starts"], [active.length, "physical commitments"], [trial.config.capacity, "social capacity"]] : [[clock, "recorded world tick"], [owner, "individual perspective"]];
  byId("overview").replaceChildren();
  for (const [value, label] of statistics) { const stat = document.createElement("div");stat.className = "stat";const strong = document.createElement("strong");strong.textContent = value;stat.append(strong, label);byId("overview").append(stat); }
  for (const id of ["timeline", "people", "choices", "events", "consent"]) byId(id).replaceChildren();
  if (observer) byId("timeline").append(gantt(), lanes(active));
  else byId("timeline").textContent = "Actor view uses only this individual's captured decision input. Other commitments, objective stock and future events are hidden.";
  byId("perspective-title").textContent = observer ? "Observer Perspective" : "Actor Perspective";
  choices(boundary, boundaryIndex, observer, owner);
  if (observer) { events(); consent(); }
  else byId("events").textContent = "Other people's records and physical outcomes are available in Observer Perspective. They are not imported into this captured local input.";
  byId("status").textContent = `Original ordinal ${ordinal} · ${trial.config.case} · capacity ${trial.config.capacity} · ticks ${trial.start}–${trial.end} · ${observer ? "Observer" : "Actor"} Perspective`;
}
async function loadRecordedTrial() {
  try {
    byId("open").disabled = true;
    const requestedOrdinal = Number(byId("trial").value);
    byId("status").textContent = "Reading original compressed member…";
    const result = await (await fetch(`/api/trial?ordinal=${requestedOrdinal}`)).json();
    ordinal = result.source.original_ordinal;
    trial = result.trial; source = result.source; base = trial.final_state;
    while (!base.events) base = base.base;
    byId("tick").min = trial.start; byId("tick").max = trial.end; byId("tick").value = trial.start;
    byId("source").textContent = `${source.archive} · original zero-based ordinal ${ordinal} · schema ${source.schema} · member SHA256 ${source.member_sha256} · archive SHA256 ${source.archive_sha256} · source-freeze SHA256 ${source.freeze_sha256}`;
    inspect();
  } catch (error) { byId("status").textContent = "Could not read original evidence: " + error.message; }
  finally { byId("open").disabled = false; }
}
byId("open").onclick = loadRecordedTrial;
byId("jump").onclick = inspect;
byId("lens").onchange = inspect;
byId("owner").onchange = inspect;
byId("previous").onclick = () => { byId("tick").value = clock - 1; inspect(); };
byId("next").onclick = () => { byId("tick").value = clock + 1; inspect(); };
byId("decision").onclick = () => {
  if (!trial) return;
  const owner = Number(byId("owner").value);
  const boundary = trial.boundaries.find(b => b.tick > clock && b.choices.some(d => d.input.legacy.own.id === owner));
  if (boundary) { byId("tick").value = boundary.tick; inspect(); }
};
(async () => {
  try {
    const rows = await (await fetch("/api/catalog")).json();
    for (const [n, config] of rows.entries()) {
      const option = document.createElement("option"); option.value = n;
      option.textContent = `${n} · ${config.case} · capacity ${config.capacity} · seed ${config.seed} · ${config.order} · claim ${config.duration} · horizon ${config.horizon}`;
      byId("trial").append(option);
    }
    byId("trial").value = 1;
    await loadRecordedTrial();
  } catch (error) { byId("status").textContent = error.message; }
})();
