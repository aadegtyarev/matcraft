# Stack decision

## Problem

Choose a programming language and toolchain for implementing matcraft — an
offline CLI tool that generates Russian obscene language by morphological rules.

## Alternatives considered

### Rust

- **Unicode / Cyrillic:** Native UTF-8 support; `\p{Cyrillic}` in regex.
- **Tool distribution:** `cargo install` produces a standalone binary with no
  runtime dependency.
- **Morphology ecosystem:** No existing generation crate exists — must be custom.
- **CLI ergonomics:** clap 4 (derive) — mature, widely used.
- **Binary size:** ~5 MB (stripped).
- **Cross-platform:** Linux, macOS, Windows.

### Python

- **Unicode / Cyrillic:** Native UTF-8, good support.
- **Tool distribution:** `pip install` — requires Python + venv on the target
  system.
- **Morphology ecosystem:** pymorphy2 is an *analysis* library (tagging,
  lemmatization), not a *generation* engine. No help for this use case.
- **CLI ergonomics:** argparse / click — mature.
- **Binary size:** N/A (script, no standalone binary).
- **Cross-platform:** Linux, macOS, Windows.

### Key research finding

Neither language has a crate or library that implements generation of Russian
mat by morphological rules. The implementation would be novel either way. The
morphological rules for mat are well-documented academically (4 roots, ~14
prefixes, ~5 suffixes, interfixes, postfix -ся) and are simple enough (finite
combinatorial rules, not ML) that either language works.

## Recommendation

**Rust.** The deciding factors:

1. **Standalone binary distribution** matches the "offline CLI tool" spec
   without a Python runtime dependency.
2. **Finite, well-understood rule set** — Rust's compile-time checks and
   pattern matching are a net benefit for correctness.
3. **Ecosystem maturity** — clap 4 and rand are battle-tested dependencies.

### Edition

Rust edition 2024 (stable since Rust 1.85.0, Feb 2025). Current stable
toolchain (1.96.0, May 2026) fully supports it.

### Dependencies

| Crate | Version | Purpose |
| --- | --- | --- |
| `clap` | `4.6` (with `derive` feature) | CLI argument parsing |
| `rand` | `0.9` | Random morpheme selection |

## Decision

**Operator:** Alexander Degtyarev

**Date:** 2026-07-05

**Decision:** Use Rust with edition 2024, clap 4.6 (derive), rand 0.9.
