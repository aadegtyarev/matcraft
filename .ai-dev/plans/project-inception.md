# Plan: Project inception — matcraft

**Status:** Approved
**Decisions:** Rust (edition 2024, clap 4.6, rand 0.9) · flat CLI (`matcraft --topic <T>`) · MIT license

## Guarantee

This inception guarantees that the project has a named, recorded set of
day-zero decisions — stack, environment, ops, license, architecture —
before any feature code is written, so every subsequent change builds on a
documented foundation rather than accumulating implicit choices.

## Behaviour

**Before:** Empty project tree — only `CLAUDE.md`, `AGENTS.md`, `.gitignore`,
and `.ai-dev/` infrastructure.

**After:** The following exist:

| What | Path | Content |
| --- | --- | --- |
| Stack decision | `docs/decisions/stack.md` | Researched recommendation + Operator's recorded choice |
| Architecture | `docs/architecture.md` | System structure, modules, data model, unknowns marked `[?]` |
| Deployment | `docs/deployment.md` | Deploy path, failure visibility, secrets policy |
| Threat model | `docs/threat-model.md` | CLI OSS sketch — minimal, no user data |
| License | `LICENSE` | Operator chooses |
| README | `README.md` | In Russian (`docLanguage: ru`) — what, how to install, where to get help |
| Cargo project | `Cargo.toml`, `src/main.rs` | Walking skeleton: one CLI invocation with flags produces output |
| CI | `.github/workflows/ci.yml` | `cargo build` + `cargo clippy` + `cargo test` on push/PR |
| Quality tooling | `src/quality/tools.json` | Registry of build-beat quality tools |
| Changelog | `CHANGELOG.md` | Standard keepachangelog format |

**What stays the same:** No existing files are modified. The `.gitignore` is
updated to add Rust build artifacts.

## Scope

**In scope:**
- Stack research and decision recording
- All documentation files listed above
- Walking skeleton: `cargo run -- --topic <X>` prints a generated word
- GitHub Actions CI configuration
- Quality-tool registry

**Out of scope (explicitly deferred to feature plans):**
- Full morphological rule engine (all 4 roots with all affixes)
- Multiple topics with distinct generation behaviours
- Any advanced generation (weighted morpheme selection, multi-word output,
  grammatical agreement)
- Tests for morphological logic (the skeleton has minimal logic)
- Release/publish to crates.io (the deploy path is documented, not executed)
- Any interactive mode, piping, or rich output

## Structural choice

### Language and packaging: Rust vs Python

| Axis | Rust | Python |
| --- | --- | --- |
| Unicode / Cyrillic | Native UTF-8, `\p{Cyrillic}` in regex | Native UTF-8, good support |
| Tool distribution | `cargo install` — standalone binary, no runtime | `pip install` — requires Python + venv |
| Morphology ecosystem | No existing generation crate (custom build) | pymorphy2 is analysis-only, not generation |
| CLI ergonomics | clap 4 (derive) — mature | argparse / click — mature |
| Binary size | ~5 MB (stripped) | N/A (script) |
| Cross-platform | Linux, macOS, Windows | Linux, macOS, Windows |

**Key finding from research:** No existing Rust or Python crate implements
generation of Russian mat by morphological rules. This is a novel
implementation either way. pymorphy2 is an *analysis* library (tagging,
lemmatization), not a *generation* engine — it wouldn't help. The
morphological rules for mat are well-documented academically (4 roots,
~14 prefixes, ~5 suffixes, interfixes, postfix -ся) and are simple enough
(finite combinatorial rules, not ML) that Rust's type system helps enforce
correctness.

**Recommendation: Rust.** The deciding factors: standalone binary distribution
matches the "offline CLI tool" spec without a Python runtime dependency; the
rule set is finite and well-understood, so Rust's compile-time checks and
pattern matching are a net benefit.

**Fork: Rust edition.** Edition 2024 (stable since Rust 1.85.0, Feb 2025) is
the latest. On current stable (1.96.0, May 2026) it is mature. Use edition
2024.

### CLI framework: clap 4 with derive

structopt is deprecated and merged into clap. Use `clap = { version = "4.6",
features = ["derive"] }`.

### Quality tools: cargo build + clippy + test

Standard Rust toolchain. `src/quality/tools.json` registers these as the
build-beat checks.

### CI: GitHub Actions

Single job: `cargo build --all-targets`, `cargo clippy -- -D warnings`,
`cargo test`.

### Dependency versions (pinned)

| Crate | Version | Purpose |
| --- | --- | --- |
| `clap` | `4.6` with `derive` | CLI argument parsing |
| `rand` | `0.9` | Random selection of morphemes |

No other dependencies for the walking skeleton. `regex` may come in the first
feature but is not needed for the skeleton.

## Product questions

### Who is this for

Russian-speaking developers and enthusiasts who want to generate topical
obscene language programmatically, without invoking an LLM or scraping a
dictionary. The product brief (`docs/product.md`) does not yet exist (this is
inception, not feature work), but the project intent is a tool for a
programming-linguistics audience — people who understand what "morphological
rules" means and find the combinatorial creativity interesting, not
end-users looking for a ready insult.

### What user pain

Developer-linguists who want to generate mat in code currently have no
offline, rule-based, non-LLM tool. The alternatives are: a static list
(finite, boring), an LLM (heavy, online, not deterministic), or a
hand-written script per use case (duplicated effort).

### What breaks if we DON'T build it

Nothing. This is a hobby/educational project. The cost of not building it is
zero — no user base is depending on it.

### Is this the right bet

The only alternative to inception is to start coding without a documented
foundation — which is exactly what the protocol exists to prevent. The
project inception is mandatory, not a choice.

### The cheapest test that would tell us

The walking skeleton succeeds: `cargo run -- --topic хуй` prints a generated
word. If the skeleton compiles and runs, the stack decision is validated.

## Elicitation (applied to the draft)

Two techniques from the catalog:

**1. Pre-mortem — "6 months from now, the project failed. Why?"**
- The morphological engine's rules are published academically but implementing
  them to produce diverse, non-repetitive output is harder than expected.
- The walking skeleton was too thin — no real value until multiple rules exist.
- Mitigation: the skeleton proves the CLI skeleton works. The first real
  feature MUST follow promptly with the full root set and affix combinatorics.

**2. Red-vs-blue — "Why not Python?"**
- Counter: Python adds a runtime dependency (`pip install` → needs Python).
  pymorphy2 doesn't help (it's analysis, not generation). For a finite
  rule-based generator, Rust's performance and distribution advantages
  outweigh Python's prototyping speed. The morphological rules here are
  well-understood and finite — no ML exploration needed where Python's
  ecosystem shines.

**Hold — nothing surfaced that changes the plan.** The stack choice is solid;
the skeleton scope is appropriate for inception.

## Plan adversary (probe before finalizing)

**What breaks?** The most plausible failure: `clap 4.6` updates before the
Builder runs, introducing a minor API change. Mitigation: use `"4.6"` as the
version (minimum 4.6.x, not `"4"` which could pull 4.7 with breaking changes).
The derive API is stable across 4.x minors, but pinning to 4.6 avoids 4.6→4.7
ambiguity.

**What is missing?** The plan does not specify:
- The exact CLI command shape (subcommand or flag-only?)
- The `.gitignore` additions for Rust (target/, Cargo.lock for binaries)
- CI for `cargo fmt` (code style consistency)

All addressed below in the work items.

**Fuzzy expected values tightened:**
- "walking skeleton" → concrete: `cargo run -- --topic хуй` prints one
  generated word. The word is produced by picking one of the 4 roots and
  applying a random prefix + suffix.
- "basic CLI" → concrete: `matcraft generate --topic <TOPIC>` or just
  `matcraft <TOPIC>`.

**Hidden structural fork surfaced:** CLI shape. See structural choice below.

### CLI shape decision (Operator call)

Two options for the walking skeleton:

**Option A (flat):** `matcraft --topic <TOPIC>` — single-command, no subcommands.
Simpler, fewer args to parse. Scales poorly when more features arrive.

**Option B (subcommand):** `matcraft generate --topic <TOPIC>` — subcommand
structure. More extensible (add `list-topics`, `explain`, etc. later). Slightly
more complex skeleton.

**Recommendation: Option A (flat)** for the skeleton. It is the smallest thing
that works. A subcommand like `generate` can be added when the first real
feature adds a second command. The Operator approves the CLI shape.

## Verification scenario

**Primary integration layer:** CLI invocation in the terminal.

```
cd /home/adegtyarev/Develop/Hobby/matcraft
cargo build --quiet
./target/debug/matcraft --topic хуй
```

**Expected result:** The tool prints a single generated word (e.g.,
"захуярить", "охуеть", "хуякнул") and exits with code 0.

**Error case:**
```
./target/debug/matcraft --topic unknown
```
Expected: prints an error in Russian listing available topics, exits with code 1.

**Help case:**
```
./target/debug/matcraft --help
```
Expected: prints usage information with --topic flag and available topics.

## Security surface

### Attack surface
**None.** The tool takes a `--topic` string argument. No network, no file I/O,
no user data. The topic argument is validated against a fixed enum.

### Secrets & credentials
**None.** OSS CLI, no secrets, no API keys, no tokens.

### Trust boundaries
**None.** No untrusted input crosses any boundary. The topic argument is
validated by clap's enum parser.

### Injection & unsafe ops
**None.** No shell execution, no SQL, no template engine, no deserialization.
Safe string concatenation for word formation.

### Fail-open vs fail-closed
**Fail-closed by design.** Topic validation: unknown topic → error, never
fall back to a default topic silently.

### Data & privacy exposure
**None.** No data collected, stored, or transmitted.

### AuthZ / AuthN
**N/A.** Local CLI tool, no access control.

### Supply chain
- `clap`: 573M+ downloads, 17K+ dependents, actively maintained, source on
  GitHub (clap-rs/clap). Trusted.
- `rand`: 525M+ downloads, official rust-random organization. Trusted.

### Isolation / identity invariant
**N/A.** No per-user or visibility surface.

## Unfamiliar interface (research documentation)

### Rust edition 2024 (source: blog.rust-lang.org)
- Stabilized in Rust 1.85.0 (Feb 2025). Current stable is 1.96.0 (May 2026).
- Changes: `impl Trait` in return position, `gen` blocks, `let` chains, new
  prelude. No breaking changes that affect this project.
- Confidence: high. Verified against blog.rust-lang.org and rust-lang.org
  releases page. Date: 2026-07-05.

### clap derive API (source: docs.rs/clap/latest)
- Use `#[derive(Parser)]` with `#[command()]` and `#[arg()]` attributes.
- No more structopt — deprecated, merged into clap.
- Confidence: high. Verified against docs.rs. Date: 2026-07-05.

### rand `SliceRandom::choose` + `choose_weighted` (source: docs.rs/rand/0.9)
- `use rand::seq::SliceRandom;` for slice random selection.
- `choose(&mut rng)` — uniform random element.
- `choose_weighted(&mut rng, |item| item.weight)` — weighted.
- `rng.gen()` — random value of standard types.
- Confidence: high. Verified against docs.rs. Date: 2026-07-05.

### Russian mat morphology (source: academic papers, Wikipedia)
- 4 roots: хуй, пизд-, еб-, бляд-
- Prefixes: в-/вз-/вы-/до-/за-/на-/от-/по-/под-/пере-/при-/про-/раз-/у-
- Suffixes: -и-/ну-/ану-/е-/ова-
- Postfix: -ся/-сь (reflexive)
- Confidence: medium. Linguistic descriptions from academic sources are
  authoritative. The morphological rules are well-documented but the precise
  combinatorics (which prefix works with which root) need implementation
  research. This is marked `[?]` in architecture.md.

## Docs (files to create)

| File | Audience | Language | Content |
| --- | --- | --- | --- |
| `README.md` | End users | Russian | What, install, help |
| `docs/architecture.md` | Developers | Russian | System structure, modules, unknowns `[?]` |
| `docs/decisions/stack.md` | Developers | English (machine-facing) | Researched alternatives, recommendation, decision |
| `docs/deployment.md` | Operators | Russian | Deploy path, failure visibility |
| `docs/threat-model.md` | Developers/Reviewers | Russian | Threat enumeration (minimal) |
| `CHANGELOG.md` | End users | English (machine-facing) | keepachangelog format |
| `LICENSE` | Legal | English | Chosen license text |

## Estimate

**Complexity:** Medium (non-trivial). Multiple files, research integration,
architectural decisions.

**Time bucket:** 2-4 hours (the inception category per estimation guidelines).

**Risk factors:**
- No existing tests to break (greenfield).
- One unresolved design decision: CLI shape (flat vs subcommand) — surfaced
  to Operator for approval above; this is a one-line choice, not a blocker.
- Stack recommendation is researched, not assumed — the research evidence is
  recorded in this plan.

## Visual form

- **Stack decision** (`docs/decisions/stack.md`): table-based comparison
  (Rust vs Python) with a prose recommendation and the Operator's recorded
  choice.
- **Architecture** (`docs/architecture.md`): normal prose with structural
  headings. No ASCII diagrams needed at this stage — the system is a single
  binary with two layers (CLI + engine).
- **README**: prose with installation code blocks. In Russian.
- **Threat model**: enumerated table (surface, mitigation, file:line) per the
  threat-model module format.

## Work items (Builder instructions)

### 1. Resolve pending decisions with Operator

Before building, the Orchestrator must get the Operator's approval on:
- **Stack:** Rust with clap 4 + rand 0.9 (recommended above). Confirm.
- **License:** choose one: MIT (permissive, standard for Rust OSS), Apache 2.0
  (Rust ecosystem default), or GPLv3 (copyleft). The Builder seeds the chosen
  license.
- **CLI shape:** flat (`--topic <T>`) vs subcommand (`generate --topic <T>`).
  Recommendation: flat for skeleton.
- **Plan approval:** read the plan, confirm.

### 2. Create project files

**2a. Project root configuration**

- `Cargo.toml` — Rust 2024 edition, MSRV 1.85. Dependencies:
  - `clap = { version = "4.6", features = ["derive"] }`
  - `rand = "0.9"`
  - `name = "matcraft"`, `version = "0.1.0"`
- Update `.gitignore` — add `/target/`, `Cargo.lock` (binary project, commit
  lockfile for reproducibility — actually for a binary the convention is to
  commit Cargo.lock, so do NOT ignore it).

Wait — check Rust convention: binary crates commit `Cargo.lock`, library
crates don't. matcraft is a binary crate. The .gitignore should NOT ignore
`Cargo.lock`. Update `.gitignore` to only add `/target/`.

**2b. Source code**

- `src/main.rs` — CLI skeleton:
  - `clap` derive struct with `--topic` argument
  - Topic enum: `Хуй`, `Пизда`, `Ебать`, `Блядь` (Cyrillic in code, but
    code comments in English per invariant 5; the enum variants are machine
    grammar → English transliterations `Huy`, `Pizda`, `Yebat`, `Blyad` or
    simply use the Russian words as `&str` matched to a static list)
  - Match topic to a hardcoded list of 2-3 morpheme combinations per topic
  - Pick one randomly via `rand::seq::SliceRandom::choose`
  - Print the result

**2c. Documentation**

- `README.md` in Russian:
  - What is matcraft (генератор мата по морфологическим правилам)
  - How to install: `cargo install matcraft`
  - How to use: `matcraft --topic <ТЕМА>`
  - Where to get help: GitHub Issues
- `CHANGELOG.md` — standard keepachangelog, 0.1.0 entry: "Начальная
  структура проекта и CLI-скелет"

**2d. Decision and architecture docs**

- `docs/decisions/stack.md` — full research narrative in English:
  - Problem statement
  - Alternatives considered (with evidence)
  - Recommendation: Rust
  - Operator's recorded decision (filled in after approval)
- `docs/architecture.md` in Russian — seeded content:
  - Purpose and scope
  - Stack summary
  - System layers: CLI (clap) → Engine (morphological rules)
  - Module structure (planned): `engine/` (roots, affixes, combinatorics)
  - Data model: morpheme types, combination rules
  - Unknowns marked `[?]`: full affix compatibility matrix, topic taxonomy
  - Environment: Linux/macOS/Win, offline, no budget
  - Future: crates.io publish, rule externalization
- `docs/deployment.md` in Russian:
  - Deploy path: `cargo publish` → crates.io → `cargo install matcraft`
  - Prerequisites: Rust toolchain, crates.io account, API token
  - No secrets to manage
  - Failure visibility: GitHub Issues, crates.io bug tracker
  - No backups (no user data)
- `docs/threat-model.md` in Russian — threat enumeration:
  - Attack surface: none (single topic enum argument)
  - Secrets: none
  - Trust boundaries: none
  - Injection: none
  - Data privacy: none
  - Auth: none
  - Identified risk: output may be offensive (by design — the product's
    purpose). Mitigation: README states the tool generates obscene language.
    This is a feature, not a vulnerability.

**2e. License**

- `LICENSE` — copy the chosen license text.

**2f. CI**

- `.github/workflows/ci.yml`:
  ```yaml
  on: [push, pull_request]
  name: CI
  jobs:
    check:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4
        - uses: actions-rust-lang/setup-rust-toolchain@v1
        - run: cargo build --all-targets
        - run: cargo clippy -- -D warnings
        - run: cargo test
  ```

**2g. Quality tool registry**

- `src/quality/tools.json` — register `cargo build`, `cargo clippy`,
  `cargo test` as build-beat tools. Follow the format from the tools.json
  template (one entry per tool with `what`, `command`, `beat`, `init`).

### 3. Verify the skeleton

```sh
cargo build
cargo clippy -- -D warnings
./target/debug/matcraft --help
./target/debug/matcraft --topic хуй     # should print a word
./target/debug/matcraft --topic unknown  # should error
cargo test
```

All must pass. The Builder hands back the working tree uncommitted.

### 4. Not in scope for this plan

- `docs/product.md` — not created. Product discovery did not run (this is
  inception, which predates product discovery). If the Operator wants a
  product brief, it is a separate follow-up task.
- Any `docs/contracts/` files — not yet. Contracts come with feature-level
  work.
- `.github/dependabot.yml` — deferred. Not a day-zero block.

## Progress note

```
feature: project-inception
state: plan-draft
blocked-by: operator-approval
next: after approval → Builder creates all files → verify → Reviewer → ship
remaining decisions: stack (recommended: Rust), license (needs Operator choice),
  CLI shape (recommended: flat --topic)
```
