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

/// Count the breakdown blocks the command printed to stdout.
///
/// `generate` now prints a multi-line breakdown block per form (not one line per
/// form), so blocks are counted by their stable per-block marker `разбор   :`
/// — one occurrence per rendered form.
fn stdout_block_count(args: &[&str]) -> usize {
    let output = matcraft().args(args).output().expect("command should run");
    assert!(output.status.success(), "expected success for {args:?}");
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| l.contains("разбор   :"))
        .count()
}

// ---------------------------------------------------------------------------
// A. Audit-named untested branches (core of the task)
// ---------------------------------------------------------------------------

/// A1: Explore dispatch, Ok path — header (root, Russian gloss, productivity),
/// morpheme legend, suffix section, and the new ending columns.
#[test]
fn explore_known_root_renders_paradigm() {
    matcraft()
        .args(["explore", "еб"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Корень: еб-"))
        .stdout(predicate::str::contains("«совокупляться»"))
        .stdout(predicate::str::contains("продуктивность A"))
        .stdout(predicate::str::contains("Легенда:"))
        .stdout(predicate::str::contains("Инфинитив"))
        .stdout(predicate::str::contains("Наст. 3л"))
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

/// A6: guard boundary — exactly 100 is allowed and prints 100 breakdown blocks.
#[test]
fn generate_count_100_is_allowed() {
    assert_eq!(stdout_block_count(&["generate", "--count", "100"]), 100);
}

/// A7: `--count 0` is internally clamped to 1..=100 → one block (documents that
/// the clamp is distinct from the `> 100` guard).
#[test]
fn generate_count_zero_clamps_to_one() {
    assert_eq!(stdout_block_count(&["generate", "--count", "0"]), 1);
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

/// A9: trailing-hyphen strip on Generate — `еб-` resolves and prints one block.
#[test]
fn generate_strips_trailing_hyphen() {
    assert_eq!(
        stdout_block_count(&["generate", "--root", "еб-", "--count", "1"]),
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
        .stdout(predicate::str::contains("Заметка:"))
        .stdout(predicate::str::contains("Пример разбора:"));
}

/// A11: Random dispatch in full mode — same box + example-block guarantee.
/// (Same coverage limit as A10.)
#[test]
fn random_full_renders_box() {
    matcraft()
        .args(["--mode", "full", "random"])
        .assert()
        .success()
        .stdout(predicate::str::contains("╔"))
        .stdout(predicate::str::contains("Заметка:"))
        .stdout(predicate::str::contains("Пример разбора:"));
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

/// B4: `generate --root еб --count 3` prints exactly 3 breakdown blocks.
#[test]
fn generate_root_count_three() {
    assert_eq!(
        stdout_block_count(&["generate", "--root", "еб", "--count", "3"]),
        3
    );
}

/// B5 (#22): a valid noun-only root has no verbal pool — informative message on
/// stdout, exit 0 (valid input, not an error).
#[test]
fn generate_noun_only_root_reports_message_and_exits_zero() {
    matcraft()
        .args(["generate", "--root", "манд"])
        .assert()
        .success()
        .stdout(predicate::str::contains("именной корень"))
        .stdout(predicate::str::contains("глагольных форм нет"));
}

/// B6 (#22 regression): a valid verb root still emits forms and never the
/// noun-only message — guards against the noun-only branch swallowing verb roots.
#[test]
fn generate_verb_root_still_produces_forms() {
    let out = matcraft()
        .args(["generate", "--root", "еб"])
        .output()
        .expect("command should run");
    assert!(out.status.success(), "verb root generate must succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("глагольных форм нет"),
        "verb root must not report the noun-only message"
    );
    assert!(
        stdout.trim().contains("еб"),
        "generated verb-root form should contain the root 'еб'"
    );
}

/// B7 (#23): `--suffix -ну-` (space syntax, leading hyphen) must parse and yield
/// the same output as the `--suffix=-ну-` (equals) form.
#[test]
fn explore_suffix_leading_hyphen_space_and_equals_agree() {
    let space = matcraft()
        .args(["explore", "еб", "--suffix", "-ну-"])
        .output()
        .expect("command should run");
    assert!(
        space.status.success(),
        "space-form `--suffix -ну-` should parse and succeed"
    );

    let equals = matcraft()
        .args(["explore", "еб", "--suffix=-ну-"])
        .output()
        .expect("command should run");
    assert!(
        equals.status.success(),
        "equals-form `--suffix=-ну-` should succeed"
    );

    assert_eq!(
        space.stdout, equals.stdout,
        "space and equals `--suffix` syntaxes must produce identical output"
    );
}

/// B8 (#23 honesty): a verbal root under a non-matching hyphen-prefixed filter
/// exits 0 with a "no forms for this filter" message — NOT a false noun-only
/// label. `allow_hyphen_values` made this path reachable; it must stay honest.
#[test]
fn explore_verbal_root_nonmatching_filter_is_not_noun_only() {
    matcraft()
        .args(["explore", "еб", "--suffix", "-бред-"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Нет форм по заданному фильтру"))
        .stdout(predicate::str::contains("именной корень").not());
}

// ---------------------------------------------------------------------------
// D. root-of-the-day (deterministic within a day)
// ---------------------------------------------------------------------------

/// D1: `root-of-the-day` (classic) renders the same box + example as `random`.
#[test]
fn root_of_the_day_classic_renders_box() {
    matcraft()
        .args(["root-of-the-day"])
        .assert()
        .success()
        .stdout(predicate::str::contains("╔"))
        .stdout(predicate::str::contains("Заметка:"))
        .stdout(predicate::str::contains("Пример разбора:"));
}

/// D2: `--mode full root-of-the-day` — same box + example-block guarantee.
#[test]
fn root_of_the_day_full_renders_box() {
    matcraft()
        .args(["--mode", "full", "root-of-the-day"])
        .assert()
        .success()
        .stdout(predicate::str::contains("╔"))
        .stdout(predicate::str::contains("Заметка:"))
        .stdout(predicate::str::contains("Пример разбора:"));
}

/// D3: determinism within a day — two invocations produce byte-identical stdout.
///
/// Test-methodology honesty: a theoretical ~1e-11 flake exists if the two
/// (milliseconds-apart) runs straddle a UTC midnight boundary exactly. This is
/// negligible; the hard determinism guarantee lives in the unit test
/// `test_root_of_the_day_is_stable_for_fixed_index`. This test checks the
/// end-to-end dispatch (clap → SystemTime → engine → print) plus equality.
#[test]
fn root_of_the_day_is_deterministic_within_a_day() {
    let first = matcraft()
        .args(["root-of-the-day"])
        .output()
        .expect("command should run");
    let second = matcraft()
        .args(["root-of-the-day"])
        .output()
        .expect("command should run");
    assert!(first.status.success());
    assert!(second.status.success());
    assert_eq!(
        first.stdout, second.stdout,
        "two same-day invocations must produce byte-identical stdout"
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

/// C3: `--version` smoke — exit 0, `matcraft <crate version>` on stdout.
///
/// Version-agnostic: asserts against `CARGO_PKG_VERSION` (known to the test
/// binary at compile time) rather than a hardcoded number, so a version bump
/// never breaks this test.
#[test]
fn version_flag_smoke() {
    matcraft()
        .args(["--version"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("matcraft "))
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}
