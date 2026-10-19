(function (root, factory) {
  if (typeof module === "object" && module.exports) module.exports = factory();
  else root.ConcurrentModel = factory();
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";
  function ownedChoices(trial, tick, owner) {
    const boundary = trial.boundaries.find(b => b.tick === tick);
    return boundary ? boundary.choices.filter(d => d.input.legacy.own.id === owner) : [];
  }
  function intervals(trial) {
    return trial.transitions.flatMap((record, index) => {
      if (record.completed) return [];
      const completions = trial.transitions.filter(r => r.completed && r.commitment.id === record.commitment.id);
      if (completions.length !== 1) throw new Error("Missing or duplicated original release");
      return [{ record, index, end: completions[0].tick }];
    });
  }
  return { ownedChoices, intervals };
});
