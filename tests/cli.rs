use std::process::{Command, Output};

use rstest::rstest;

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_dollar-cmd"))
        .args(args)
        .output()
        .expect("failed to spawn the binary under test")
}

fn exit_code(output: &Output) -> i32 {
    output
        .status
        .code()
        .expect("process terminated by a signal")
}

#[test]
fn positive_executes_leading_argument_as_command() {
    let output = run(&["echo", "hello"]);

    assert_eq!(exit_code(&output), 0);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello\n");
}

#[test]
fn positive_executes_arguments_after_option_terminator() {
    let output = run(&["--", "echo", "hello"]);

    assert_eq!(exit_code(&output), 0);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello\n");
}

#[test]
fn positive_passes_second_option_terminator_to_the_command() {
    let output = run(&["echo", "--", "-al"]);

    assert_eq!(exit_code(&output), 0);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "-- -al\n");
}

#[test]
fn positive_treats_help_after_option_terminator_as_a_command_name() {
    let output = run(&["--", "--help"]);

    assert_eq!(exit_code(&output), 127);
    assert!(output.stdout.is_empty());
}

#[rstest]
#[case::positive_long("--help")]
#[case::positive_short("-h")]
fn positive_prints_help_to_stdout(#[case] flag: &str) {
    let output = run(&[flag]);

    assert_eq!(exit_code(&output), 0);
    assert!(!output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[rstest]
#[case::positive_long("--version")]
#[case::positive_short("-V")]
fn positive_prints_version_to_stdout(#[case] flag: &str) {
    let output = run(&[flag]);

    assert_eq!(exit_code(&output), 0);
    assert!(!output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[rstest]
#[case::negative_no_arguments(&[])]
#[case::negative_option_terminator_only(&["--"])]
fn negative_prints_usage_when_no_command_is_given(#[case] args: &[&str]) {
    let output = run(args);

    assert_eq!(exit_code(&output), 2);
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}

#[rstest]
#[case::negative_long_option("--foo")]
#[case::negative_short_option("-x")]
fn negative_treats_unknown_option_as_a_command_name(#[case] arg: &str) {
    let output = run(&[arg]);

    assert_eq!(exit_code(&output), 127);
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}

#[test]
fn negative_reports_command_not_found() {
    let output = run(&["dollar-cmd-no-such-command"]);

    assert_eq!(exit_code(&output), 127);
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}

#[test]
fn negative_reports_permission_denied() {
    let output = run(&["/etc/hosts"]);

    assert_eq!(exit_code(&output), 126);
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}
