use super::*;
use ic_host_process::tool::ExecutionFailure;
use std::process::Stdio;

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
    let baseline = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let alias = baseline.parent().unwrap().join("./Cargo.toml");
    let before = std::fs::read(&baseline).unwrap();
    let error = run("cohorts", Some(&alias), Some(&baseline)).unwrap_err();
    assert!(error.to_string().contains("must not overwrite"));
    assert_eq!(std::fs::read(baseline).unwrap(), before);
}
