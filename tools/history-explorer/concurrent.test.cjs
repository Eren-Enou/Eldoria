"use strict";
const assert = require("node:assert/strict");
const fs = require("node:fs");
const model = require("./concurrent-model.js");
const trial = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const tick = trial.start;
const original = model.ownedChoices(trial, tick, 0);
assert.equal(original.length, 1);
const poisoned = structuredClone(trial);
poisoned.final_state = { objective_truth: "invented future", private_partner_food: 999 };
poisoned.boundaries[0].agents = [{ id: 0, food: 9999 }];
poisoned.boundaries[0].stock = 99999;
poisoned.memories = [{ forgotten: "observer audit alone" }];
assert.deepEqual(model.ownedChoices(poisoned, tick, 0), original);
poisoned.boundaries[0].choices[0].input.retained = null;
assert.equal(model.ownedChoices(poisoned, tick, 0)[0].input.retained, null);
assert.deepEqual(model.ownedChoices(trial, trial.end, 0), []);
assert.equal(model.ownedChoices(trial, tick, 3)[0].input.legacy.own.id, 3);
const intervals = model.intervals(trial);
assert.ok(intervals.every(i => i.end > i.record.tick));
assert.ok(intervals.some(a => intervals.some(b => a.record.commitment.id !== b.record.commitment.id &&
  typeof a.record.commitment.work === "object" && typeof b.record.commitment.work === "object" &&
  Math.max(a.record.tick, b.record.tick) < Math.min(a.end, b.end))));
const missing = structuredClone(trial);
missing.transitions.splice(missing.transitions.findIndex(r => r.completed), 1);
assert.throws(() => model.intervals(missing), /Missing or duplicated/);
console.log("022 actual-source actor isolation, unavailable-input, own-slot, four-ID and overlapping-interval checks passed");
