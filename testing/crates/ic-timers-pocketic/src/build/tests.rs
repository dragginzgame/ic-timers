use super::*;
use ic_host_process::tool::ExecutionFailure;
use std::process::Stdio;

#[test]
fn subject_discovery_rejects_empty_unrelated_and_failed_producers() {
    let watchdog = REQUIRED_WATCHDOG_SUBJECTS
        .iter()
        .map(|name| format!("{name}: test\n"))
        .collect::<String>();
    let missing = watchdog.lines().skip(1).collect::<Vec<_>>().join("\n");
    let cohort = format!("{COHORT_SUBJECT}: test\n");
    for (mode, listing, admitted) in [
        ("watchdog", "", false),
        ("watchdog", "report::tests::host_only: test\n", false),
        ("watchdog", "tests::recovery: test\n", false),
        ("watchdog", watchdog.as_str(), true),
        ("watchdog", missing.as_str(), false),
        ("cohorts", "", false),
        ("cohorts", "cohorts::renamed_subject: test\n", false),
        ("cohorts", cohort.as_str(), true),
        ("watchdog", "tests::fake: benchmark\n", false),
    ] {
        let mut command = Command::new("sh");
        command.args([
            "-c",
            "case \"${2:-}\" in --ignored) exit 0;; esac; printf '%s' \"$1\"",
            "fixture",
            listing,
        ]);
        assert_eq!(discover_subjects(&mut command, mode).is_ok(), admitted);
    }
    let mut ignored = Command::new("sh");
    ignored.args(["-c", "printf '%s' \"$1\"", "fixture", &cohort]);
    assert!(
        discover_subjects(&mut ignored, "cohorts")
            .unwrap_err()
            .to_string()
            .contains("ignored")
    );
    // A valid-looking listing on stdout cannot hide Cargo/compiler failure.
    let mut command = Command::new("sh");
    command.args(["-c", "printf '%s' \"$1\"; exit 7", "fixture", &cohort]);
    let error = discover_subjects(&mut command, "cohorts").unwrap_err();
    let Some(ToolError::Execution(error)) = error.downcast_ref::<ToolError>() else {
        panic!("expected retained producer failure: {error}");
    };
    assert_eq!(error.evidence.status.unwrap().code(), Some(7));
    assert_eq!(error.evidence.stdout, cohort.as_bytes());
}

#[test]
fn qualification_deadline_covers_descendants_and_preserves_failure() {
    let mut command = Command::new("sh");
    command
        .args(["-c", "sleep 60 & wait"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let error = qualify(&mut command, Duration::from_millis(200)).unwrap_err();
    let ToolError::Execution(error) = error else {
        panic!("expected execution failure")
    };
    assert!(matches!(error.failure, ExecutionFailure::TimedOut));
    assert!(
        error.cleanup.is_none(),
        "process group cleanup failed: {error:?}"
    );

    let mut command = Command::new("sh");
    command
        .args(["-c", "printf 'fixture failure' >&2; exit 7"])
        .stderr(Stdio::piped());
    let error = qualify(&mut command, Duration::from_secs(10)).unwrap_err();
    let ToolError::Execution(error) = error else {
        panic!("expected execution failure")
    };
    assert!(matches!(error.failure, ExecutionFailure::ExitStatus));
    assert_eq!(error.evidence.status.unwrap().code(), Some(7));
    assert_eq!(error.evidence.stderr, b"fixture failure");
}

#[test]
fn baseline_output_alias_is_rejected_before_any_build_or_write() {
    let fixture =
        ic_host_fs::testing::FixtureDirectory::new(&std::env::temp_dir(), "timer-baseline-alias")
            .unwrap();
    let baseline = fixture.path().join("baseline.json");
    let hard_link = fixture.path().join("hard-link.json");
    let symlink = fixture.path().join("symlink.json");
    let before = b"reviewed baseline";
    std::fs::write(&baseline, before).unwrap();
    std::fs::hard_link(&baseline, &hard_link).unwrap();
    std::os::unix::fs::symlink(&baseline, &symlink).unwrap();
    for alias in [fixture.path().join("./baseline.json"), hard_link, symlink] {
        let error = run("cohorts", Some(&alias), Some(&baseline)).unwrap_err();
        assert!(
            matches!(
                error.downcast_ref::<ic_host_fs::durable::NonAliasingWriteError>(),
                Some(ic_host_fs::durable::NonAliasingWriteError::Alias { .. })
            ),
            "{error}"
        );
        assert_eq!(std::fs::read(&baseline).unwrap(), before);
    }
}
