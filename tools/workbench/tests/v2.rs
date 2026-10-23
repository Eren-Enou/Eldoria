use woi_workbench::{
    config::{Configuration, Effort, Mode, Preparation, PreparationPreset, SCENARIOS, Valuation},
    execution, history,
    storage::Store,
};

fn store() -> Store {
    let mut s = Store::open(&woi_workbench::repository()).unwrap();
    s.root = s.root.join("acceptance-tests");
    std::fs::create_dir_all(&s.root).unwrap();
    s
}
fn config(mode: Mode, valuation: Valuation) -> Configuration {
    Configuration {
        schema_version: 2,
        mode,
        valuation: Some(valuation),
        preparation: (mode == Mode::MaterialPreparation).then(Preparation::default),
        ..Default::default()
    }
}
#[test]
fn legacy_json_is_compatible_and_new_modes_are_explicit() {
    let legacy =
        r#"{"mode":"ContinuousWorld","scenario":"paired","seed":42,"ticks":192,"period":11}"#;
    let old: Configuration = serde_json::from_str(legacy).unwrap();
    assert_eq!(old, Configuration::default());
    for mode in [Mode::ActivityValuation, Mode::MaterialPreparation] {
        let c = config(mode, Valuation::MealRelief);
        c.validate().unwrap();
        let roundtrip: Configuration =
            serde_json::from_value(serde_json::to_value(&c).unwrap()).unwrap();
        assert_eq!(c, roundtrip);
        let mut wrong = c.clone();
        wrong.schema_version = 1;
        assert!(wrong.validate().is_err());
        wrong = c.clone();
        wrong.valuation = None;
        assert!(wrong.validate().is_err());
        let mut unknown = serde_json::to_value(c).unwrap();
        unknown["command"] = "cmd.exe".into();
        assert!(serde_json::from_value::<Configuration>(unknown).is_err());
    }
    let mut wrong = old;
    wrong.valuation = Some(Valuation::Baseline);
    assert!(wrong.validate().is_err());
    let mut wrong = config(Mode::MaterialPreparation, Valuation::Baseline);
    wrong.preparation.as_mut().unwrap().raw_initial = 17;
    assert!(wrong.validate().is_err());
    let mut unknown =
        serde_json::to_value(config(Mode::MaterialPreparation, Valuation::Baseline)).unwrap();
    unknown["preparation"]["future_relevance"] = true.into();
    assert!(serde_json::from_value::<Configuration>(unknown).is_err());
}

#[test]
#[ignore = "explicit genuine023/024/025 runner, replay, schema and physical acceptance"]
fn genuine_modes_and_material_ledger() {
    let s = store();
    let mut ids = Vec::new();
    for valuation in [Valuation::Baseline, Valuation::MealRelief] {
        let c = config(Mode::ActivityValuation, valuation);
        let m = execution::run(&s, c.clone(), None).unwrap();
        assert_eq!(m.status, "Completed", "{:?}", m.diagnostic);
        let summary = execution::inspect(&s, &m.id).unwrap();
        assert_eq!(summary.experiment, "024");
        assert!(summary.food_accounting_matches);
        assert_eq!(summary.progressions.len(), 32);
        for p in &summary.progressions {
            let v = p.valuation.as_ref().expect("exact valuation reference");
            assert_eq!(
                v.selection.selected,
                summary.decisions[p.decision_index].selected
            );
            let mut best = None;
            for term in &v.selection.terms {
                let bonus = if valuation == Valuation::MealRelief
                    && term.eligible
                    && term.action == "Consume"
                {
                    summary.decisions[p.decision_index]
                        .input
                        .legacy
                        .own
                        .hunger
                        .min(40) as i64
                } else {
                    0
                };
                assert_eq!(term.meal_relief, Some(bonus));
                assert_eq!(term.final_score, term.base_score.map(|x| x + bonus));
                if term.eligible && best.is_none_or(|(_, score)| term.final_score.unwrap() > score)
                {
                    best = Some((term.action.as_str(), term.final_score.unwrap()));
                }
            }
            assert_eq!(best.unwrap().0, v.selection.selected);
        }
        let replay = execution::reproduce(&s, &m.id).unwrap();
        assert_eq!(replay.exact_match, Some(true));
        ids.push(m.id);
    }
    let pair = woi_workbench::comparison::compare(&s, &ids[0], &ids[1]).unwrap();
    assert_eq!(pair.classification, "Matched exploratory comparison");
    assert_eq!(pair.differences.len(), 1);
    assert_eq!(pair.differences[0].field, "valuation");
    for (preset, raw_initial, effort) in [
        (PreparationPreset::KnownRaw, 6, Effort::Short),
        (PreparationPreset::KnownRaw, 1, Effort::Short),
        (PreparationPreset::KnownRaw, 16, Effort::Long),
        (PreparationPreset::UnobservedRaw, 0, Effort::Medium),
        (PreparationPreset::UnobservedRaw, 16, Effort::Long),
        (PreparationPreset::NoAccess, 16, Effort::Short),
    ] {
        let mut c = config(Mode::MaterialPreparation, Valuation::MealRelief);
        c.preparation = Some(Preparation {
            preset,
            raw_initial,
            effort,
        });
        let m = execution::run(&s, c.clone(), None).unwrap();
        assert_eq!(m.status, "Completed", "{:?}", m.diagnostic);
        let summary = execution::inspect(&s, &m.id).unwrap();
        assert!(summary.food_accounting_matches);
        let material = summary.material.unwrap();
        assert_eq!(material.initial_raw, u64::from(raw_initial));
        assert_eq!(material.final_escrow, 0);
        assert_eq!(material.prepared as u64, material.final_waste);
        if preset == PreparationPreset::KnownRaw && raw_initial == 6 && effort == Effort::Short {
            assert_eq!(material.prepared, 3);
            assert_eq!(
                material.failed, 1,
                "Preserve the real full-effort failed attempt"
            );
        }
        if preset == PreparationPreset::NoAccess {
            assert_eq!(material.prepared, 0);
            assert_eq!(material.final_raw, u64::from(raw_initial));
        }
        let replay = execution::reproduce(&s, &m.id).unwrap();
        assert_eq!(replay.exact_match, Some(true));
        // Independent full recorded envelope check: failure still finishes at the
        // original deadline; no shortened effort, generated meal attribution or
        // probe substituted for genuine executed work.
        let path = s.run(&m.id).unwrap().join("history.jsonl");
        let d: serde_json::Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        for op in d["material"]["operations"].as_array().unwrap() {
            if op["outcome"] == "Failed" || op["outcome"] == "Prepared" {
                let p = d["material"]["processes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|p| p["commitment"] == op["commitment"])
                    .unwrap();
                assert_eq!(op["tick"], p["finish_at"]);
                let expected = c.preparation.as_ref().unwrap().settings();
                assert_eq!(
                    p["finish_at"].as_u64().unwrap() - p["start"].as_u64().unwrap(),
                    expected["extraction_ticks"].as_u64().unwrap()
                        + expected["preparation_ticks"].as_u64().unwrap()
                );
            }
        }
        let mut corrupt = d.clone();
        corrupt["material"]["final_waste"] = 999.into();
        std::fs::write(&path, serde_json::to_vec(&corrupt).unwrap()).unwrap();
        assert!(execution::inspect(&s, &m.id).unwrap_err().contains("hash"));
        assert!(!history::read(&path, &c).unwrap().food_accounting_matches);
        std::fs::write(&path, serde_json::to_vec(&d).unwrap()).unwrap(); // test-owned copy only
    }
    // Every exposed024 scenario is a genuine runner case, with both policies.
    for scenario in SCENARIOS {
        for valuation in [Valuation::Baseline, Valuation::MealRelief] {
            let mut c = config(Mode::ActivityValuation, valuation);
            c.scenario = scenario.into();
            c.ticks = 64;
            let m = execution::run(&s, c, None).unwrap();
            assert_eq!(m.status, "Completed", "{scenario}: {:?}", m.diagnostic);
        }
    }
    // A differing seed is a recorded difference, never a matched experiment.
    let mut c = config(Mode::ActivityValuation, Valuation::Baseline);
    c.seed = 7;
    let m = execution::run(&s, c, None).unwrap();
    assert!(
        woi_workbench::comparison::compare(&s, &ids[0], &m.id)
            .unwrap()
            .classification
            .starts_with("Recorded difference")
    );
}

fn http(port: u16, path: &str, host: &str) -> String {
    use std::io::{Read, Write};
    let mut stream = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(60)))
        .unwrap();
    write!(stream, "GET {path} HTTP/1.0\r\nHost: {host}\r\n\r\n").unwrap();
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).unwrap();
    String::from_utf8(bytes).unwrap()
}
#[test]
#[ignore = "real selected-run Python viewer compatibility; no browser or frozen file writes"]
fn selected_viewer_has_only_exact_verified_source() {
    let s = store();
    let app = s.repo.join("tools/history-explorer/app.js");
    let before = execution::hash(&app).unwrap();
    for c in [
        Configuration::default(),
        config(Mode::ActivityValuation, Valuation::MealRelief),
        config(Mode::MaterialPreparation, Valuation::MealRelief),
    ] {
        let m = execution::run(&s, c.clone(), None).unwrap();
        assert_eq!(m.status, "Completed", "{:?}", m.diagnostic);
        let path = s.run(&m.id).unwrap().join("history.jsonl");
        let original = execution::hash(&path).unwrap();
        let v = woi_workbench::viewer::launch(&s, &m.id).unwrap();
        let port = v.url.rsplit(':').next().unwrap().parse().unwrap();
        let host = format!("127.0.0.1:{port}");
        let catalog = http(port, "/api/catalog", &host);
        assert!(catalog.starts_with("HTTP/1.0 200"));
        let d: serde_json::Value =
            serde_json::from_str(catalog.split_once("\r\n\r\n").unwrap().1).unwrap();
        let entries = d["entries"].as_array().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0]["experiment"], c.mode.experiment());
        let id = entries[0]["id"].as_str().unwrap();
        let trial = http(port, &format!("/api/trial?id={id}"), &host);
        assert!(trial.starts_with("HTTP/1.0 200"));
        let d: serde_json::Value =
            serde_json::from_str(trial.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(d["source"]["id"], id);
        assert!(http(port, "/", &host).contains("/workbench-start.js"));
        assert!(http(port, "/workbench-start.js", &host).contains("select.options.length === 1"));
        assert!(http(port, "/api/catalog", "outside.invalid").starts_with("HTTP/1.0 403"));
        assert!(
            http(port, "/api/trial?id=../../experiments", "127.0.0.1:1")
                .starts_with("HTTP/1.0 403")
        );
        drop(v);
        assert_eq!(execution::hash(&path).unwrap(), original);
        // A stale/mutated history cannot launch, even with old completed metadata.
        std::fs::write(&path, b"{}\n").unwrap();
        assert!(
            woi_workbench::viewer::arguments(&s, &m.id, 8766)
                .unwrap_err()
                .contains("hash")
        );
    }
    assert_eq!(execution::hash(&app).unwrap(), before);
}

#[test]
#[ignore = "real024/025 public duration, reporting and seed boundary acceptance"]
fn new_modes_execute_public_bounds() {
    let s = store();
    for (mode, valuation) in [
        (Mode::ActivityValuation, Valuation::Baseline),
        (Mode::ActivityValuation, Valuation::MealRelief),
        (Mode::MaterialPreparation, Valuation::MealRelief),
    ] {
        for (ticks, period, seed) in [(1, 128, u64::MAX), (1024, 1, 0)] {
            let mut c = config(mode, valuation);
            c.ticks = ticks;
            c.period = period;
            c.seed = seed;
            let m = execution::run(&s, c, None).unwrap();
            assert_eq!(
                m.status, "Completed",
                "{mode:?}/{ticks}: {:?}",
                m.diagnostic
            );
            let summary = execution::inspect(&s, &m.id).unwrap();
            assert_eq!(summary.end - summary.start, ticks + 33);
            assert!(summary.food_accounting_matches);
        }
    }
}

#[test]
#[ignore = "genuine mode output required for schema/valuation corruption controls"]
fn new_mode_reader_rejects_substitution_and_missing_recorded_terms() {
    let s = store();
    for mode in [Mode::ActivityValuation, Mode::MaterialPreparation] {
        let c = config(mode, Valuation::MealRelief);
        let m = execution::run(&s, c.clone(), None).unwrap();
        assert_eq!(m.status, "Completed", "{:?}", m.diagnostic);
        let directory = s.run(&m.id).unwrap();
        let original: serde_json::Value =
            serde_json::from_slice(&std::fs::read(directory.join("history.jsonl")).unwrap())
                .unwrap();
        for mutation in 0..4 {
            let mut d = original.clone();
            match mutation {
                0 => d["history"]["schema"] = "experiment-023-physical-history/v1".into(),
                1 => d
                    .as_object_mut()
                    .unwrap()
                    .remove("valuations")
                    .map(|_| ())
                    .unwrap(),
                2 => d["valuations"][0]["selection"]["terms"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("final_score")
                    .map(|_| ())
                    .unwrap(),
                3 => d["valuations"][0]["policy"] = "Baseline".into(),
                _ => unreachable!(),
            }
            let path = directory.join(format!("corrupt-{mutation}.jsonl"));
            std::fs::write(&path, serde_json::to_vec(&d).unwrap()).unwrap();
            assert!(
                history::read(&path, &c).is_err(),
                "{mode:?}: mutation{mutation}"
            );
        }
        assert!(
            execution::inspect(&s, &m.id).is_ok(),
            "Original exploratory output untouched"
        );
    }
}

#[test]
#[ignore = "every exposed corrected025 scenario under both existing valuation policies"]
fn corrected025_cases_execute_with_both_policies() {
    let s = store();
    for scenario in SCENARIOS {
        for valuation in [Valuation::Baseline, Valuation::MealRelief] {
            let mut c = config(Mode::MaterialPreparation, valuation);
            c.scenario = scenario.into();
            c.ticks = 64;
            let m = execution::run(&s, c, None).unwrap();
            assert_eq!(
                m.status, "Completed",
                "{scenario}/{valuation:?}: {:?}",
                m.diagnostic
            );
            let summary = execution::inspect(&s, &m.id).unwrap();
            assert_eq!(summary.experiment, "025");
            assert!(summary.food_accounting_matches);
        }
    }
}
