//! Regression for the source-audited inherited episodic cap, not the mistaken prose.
use world_of_individuals::{continuous as c, model::MEMORY_CAPACITY};
#[test]
fn long_continuation_preserves_the_actual_sixteen_episode_cap() {
    assert_eq!(MEMORY_CAPACITY, 16);
    for mode in [c::Mode::Windowed, c::Mode::Continuous] {
        let config = c::matrix()
            .into_iter()
            .find(|c| c.case == "exhaustion" && c.mode == mode && c.role == "controlled")
            .unwrap();
        let trial = c::trial(config);
        let lengths: Vec<_> = trial
            .history
            .boundaries
            .iter()
            .flat_map(|b| &b.agents)
            .map(|a| a.memories.len())
            .collect();
        assert!(lengths.iter().all(|n| *n <= MEMORY_CAPACITY));
        assert_eq!(lengths.into_iter().max(), Some(MEMORY_CAPACITY));
        trial.history.final_state.validate().unwrap();
    }
}
