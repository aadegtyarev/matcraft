//! End-to-end CLI integration tests for the `matcraft` binary.
//!
//! These run the real built binary via `assert_cmd` (`CARGO_BIN_EXE_matcraft`),
//! exercising the wiring in `main.rs` that unit tests cannot reach: command
//! dispatch, `std::process::exit` codes, clap parsing, and the trailing-hyphen
//! strip. Expected exit codes are anchored to `docs/contracts/cli.md`:
//! 0 = success, 1 = application error, 2 = clap parse error.
//!
//! Error branches assert on **stderr + exit code** (not stdout): the contract
//! sends errors to stderr, and asserting stdout would pass vacuously.

use assert_cmd::Command;
use predicates::prelude::*;

/// A fresh `Command` for the built binary.
fn matcraft() -> Command {
    Command::cargo_bin("matcraft").expect("binary `matcraft` should be built by cargo test")
}

/// Count the lines the command printed to stdout (one form per line).
fn stdout_line_count(args: &[&str]) -> usize {
    let output = matcraft().args(args).output().expect("command should run");
    assert!(output.status.success(), "expected success for {args:?}");
    String::from_utf8_lossy(&output.stdout).lines().count()
}

// ---------------------------------------------------------------------------
// A. Audit-named untested branches (core of the task)
// ---------------------------------------------------------------------------

/// A1: Explore dispatch, Ok path — header, productivity class, suffix section.
#[test]
fn explore_known_root_renders_paradigm() {
    matcraft()
        .args(["explore", "еб"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Корень: еб-"))
        .stdout(predicate::str::contains("продуктивность A"))
        .stdout(predicate::str::contains("Суффикс"));
}

/// A2: trailing-hyphen strip on Explore — `еб-` normalises to `еб`.
#[test]
fn explore_strips_trailing_hyphen() {
    matcraft()
        .args(["explore", "еб-"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Корень: еб-"));
}

/// A3: root-not-found on Explore → exit 1, message on stderr.
#[test]
fn explore_unknown_root_fails_on_stderr() {
    matcraft()
        .args(["explore", "zzz"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::is_empty().not());
}

/// A4: Explore of a purely nominal root — no verbal paradigm message.
#[test]
fn explore_nominal_only_root_reports_no_verbal_paradigm() {
    matcraft()
        .args(["explore", "манд"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Глагольная парадигма отсутствует"));
}

/// A5: `count > 100` guard → exit 1, message on stderr.
#[test]
fn generate_count_over_100_fails_on_stderr() {
    matcraft()
        .args(["generate", "--count", "101"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("не может превышать 100"));
}

/// A6: guard boundary — exactly 100 is allowed and prints 100 lines.
#[test]
fn generate_count_100_is_allowed() {
    assert_eq!(stdout_line_count(&["generate", "--count", "100"]), 100);
}

/// A7: `--count 0` is internally clamped to 1..=100 → one line (documents that
/// the clamp is distinct from the `> 100` guard).
#[test]
fn generate_count_zero_clamps_to_one() {
    assert_eq!(stdout_line_count(&["generate", "--count", "0"]), 1);
}

/// A8: root-not-found on Generate → exit 1, message on stderr.
#[test]
fn generate_unknown_root_fails_on_stderr() {
    matcraft()
        .args(["generate", "--root", "zzz"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("не найден"));
}

/// A9: trailing-hyphen strip on Generate — `еб-` resolves and prints one line.
#[test]
fn generate_strips_trailing_hyphen() {
    assert_eq!(
        stdout_line_count(&["generate", "--root", "еб-", "--count", "1"]),
        1
    );
}

/// A10: Random dispatch (classic) — path does not panic and renders a box.
///
/// Coverage limit (test-methodology): the sample-form assembly filter in
/// `main.rs` (`ending_val == "ть" && attestation == Common`, `.take(3)`)
/// cannot be pinned deterministically here — `random` picks a root at random
/// and no CLI seed exists. This asserts dispatch + frame rendering only; the
/// filter itself is unit-covered by `test_format_random_*` in `display.rs`.
#[test]
fn random_classic_renders_box() {
    matcraft()
        .args(["random"])
        .assert()
        .success()
        .stdout(predicate::str::contains("╔"))
        .stdout(predicate::str::contains("Заметка:"));
}

/// A11: Random dispatch in full mode — same box-rendering guarantee.
/// (Same coverage limit as A10.)
#[test]
fn random_full_renders_box() {
    matcraft()
        .args(["--mode", "full", "random"])
        .assert()
        .success()
        .stdout(predicate::str::contains("╔"))
        .stdout(predicate::str::contains("Заметка:"));
}

/// Random never panics across repeated runs (no form assertions — see A10).
#[test]
fn random_never_panics_over_repeated_runs() {
    for _ in 0..10 {
        matcraft().args(["random"]).assert().success();
    }
}

// ---------------------------------------------------------------------------
// B. Happy-path smoke
// ---------------------------------------------------------------------------

/// B1: `list-roots` (classic) reports the 9-root count.
#[test]
fn list_roots_classic_reports_nine() {
    matcraft()
        .args(["list-roots"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Доступные корни (9)"));
}

/// B2: `--mode full list-roots` reports the 35-root count and domain headers.
///
/// The classic↔full distinction is asserted by the header **count** `(35)` and
/// the domain headings — NOT by a root substring: the classic hint line
/// literally contains "дрочить", so `contains("дроч")` would pass vacuously in
/// classic mode. `(35)` and the domain headers appear only in full mode.
#[test]
fn list_roots_full_reports_thirtyfive_and_domains() {
    matcraft()
        .args(["--mode", "full", "list-roots"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Все корни (35)"))
        .stdout(predicate::str::contains("Ядро"))
        .stdout(predicate::str::contains("Экскреторная"))
        .stdout(predicate::str::contains("Периферия"));
}

/// B3: Explore of the -е-/-и- class root surfaces both conjugation stems.
#[test]
fn explore_e_i_class_root() {
    matcraft()
        .args(["explore", "пизд"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-е-"))
        .stdout(predicate::str::contains("пиздеть"))
        .stdout(predicate::str::contains("-и-"));
}

/// B4: `generate --root еб --count 3` prints exactly 3 lines.
#[test]
fn generate_root_count_three() {
    assert_eq!(
        stdout_line_count(&["generate", "--root", "еб", "--count", "3"]),
        3
    );
}

// ---------------------------------------------------------------------------
// C. clap parse errors (exit 2 by contract)
// ---------------------------------------------------------------------------

/// C1: an unknown `--mode` value is rejected by clap's value_enum → exit 2.
#[test]
fn invalid_mode_value_exits_2() {
    matcraft()
        .args(["--mode", "bogus", "list-roots"])
        .assert()
        .failure()
        .code(2);
}

/// C2: a missing subcommand is rejected by clap → exit 2.
#[test]
fn missing_subcommand_exits_2() {
    matcraft().assert().failure().code(2);
}

/// C3: `--version` smoke — exit 0, version string on stdout.
#[test]
fn version_flag_smoke() {
    matcraft()
        .args(["--version"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0.5"));
}
