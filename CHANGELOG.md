# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.1] — 2026-07-06

### Fixed

- **`--mode` is now accepted in any position** (before OR after the subcommand). The flag was
  documented as «Глобальный флаг» but the code did not mark it `global = true`, so
  `matcraft <subcommand> --mode <X>` failed with `unexpected argument '--mode'`. Now matches
  the documented contract. Backwards-compatible (the pre-subcommand form still works). (#35)

## [0.9.0] — 2026-07-06

### Fixed

- **Fill-vowel (беглая -о-) morphonology** at the prefix–root boundary for сра-, сса-, жр-:
  vowelless prefixes now insert -о- instead of producing consonant pile-ups —
  ссрать→**сосрать**, всссать→**взоссать**, сжрать→**сожрать**, оссать→**обоссать**, etc.
  No triple-consonant forms remain. Lexically scoped (only these 3 roots take the fill vowel);
  incidentally realizes обо-/ото- for them. (#28)
- **говн- takes об-** with the о-prefix: `оговнить` → **обговнить** (the real attested form),
  attestation raised to Common. Only говн- — other о-/об- roots keep о-. (resolves the #24
  case for this root; the general о-/об- irregularity stays a documented simplification)

### Notes

- Edge fill-vowel forms (воссать, восрать, вожрать, …) are marked `possible` — derivable but
  not verified against the source (raw volumes unavailable). See `docs/decisions/cluster-fill-vowel.md`.

## [0.8.0] — 2026-07-06

### Changed

- **Research-grade output across all commands.** Every form is now shown as a full
  breakdown block — attestation level, meaning, morpheme decomposition (приставка /
  корень / суффикс / окончание, each glossed), domain·productivity, and a line
  explaining *why* the attestation level. Output is fully Russian. (#29)
  - `explore` — enriched table (endings as columns; past/present forms now visible)
    with a morpheme legend; a `--suffix` filter adds full breakdown blocks.
  - `generate` — one full breakdown block per form (**no longer one line per form** —
    `generate | wc -l` output shape changed; there is no machine-output contract).
  - `list-roots` — enriched per-root line (Russian gloss, domain, productivity, type,
    a flag for verbal roots with no source-attested forms).
  - `random` / `root-of-the-day` — enriched summary box + a breakdown block.

### Added

- `RootData.gloss_ru` (Russian root glosses, all 35) and ending labels/glosses.

### Internal

- `src/engine/display.rs` split into a cohesive `src/engine/display/` submodule.

## [0.7.1] — 2026-07-06

### Fixed

- Attestation honesty: `оговнить` (говн-) and `одристать` (дрист-) are now `Possible`,
  not `Common`/`Rare` — the engine builds these via a simplified о-/об- rule, and the
  source does not attest the simplified forms (the real derivatives use об-/обо-). Notes
  aligned to the built forms. No generated word string changed.

### Documentation

- Added `docs/decisions/o-ob-allomorphy.md` recording that the о-/об- prefix rule is the
  preposition rule applied as a conscious simplification (the verbal prefix is irregular
  allomorphy), and that the обо- allomorph is a known engine gap. (#24)

## [0.7.0] — 2026-07-06

### Added

- `explore --suffix` now accepts the space form for hyphen-leading values
  (`--suffix -ну-`), not only `--suffix=-ну-`. (#23)
- `generate` on a valid noun-only root now prints an informative message
  ("именной корень «<root>-»: глагольных форм нет") and exits 0, instead of
  silently producing no output. (#22)

### Fixed

- `explore` on a verbal root whose suffix filter matches nothing now reports
  "Нет форм по заданному фильтру." instead of a false "чисто именной корень"
  label (a honesty gap made reachable by the `--suffix` space form).

## [0.6.1] — 2026-07-05

### Fixed

- Attestation notes now match the engine-built form (`прохарить`, `оговнить` — were
  `прокарить`/`обговнить`). Honesty fix, no behavior change.
- `random` / `root-of-the-day` output box is now aligned for every root (long content
  lines are wrapped; previously the frame burst for noun-only roots and was off by one).
- Documentation currency: threat model lists all five subcommands; stale "walking skeleton"
  phase labels and an outdated root-count figure removed. (v0.6.0 audit fixes)

## [0.6.0] — 2026-07-05

### Added

- **`root-of-the-day` subcommand** — a deterministic "root of the day": the same root
  is returned throughout one calendar day (UTC) and changes daily, seeded from the date.
  Distinct from `random`, which stays fresh-random per invocation. Honors `--mode`. (#5)

## [0.5.1] — 2026-07-05

### Added

- CLI integration test coverage for `main.rs` via `assert_cmd` (19 end-to-end tests
  exercising command dispatch, exit codes, error output, and `--mode`). Internal
  quality — no user-facing change. (#13)

## [0.5.0] — 2026-07-05

### Added

- **Root expansion from 9 to 35 roots**, grounded on the Plutser-Sarno synthesis.
  Nuclear (7): еб-, пизд-, хуй-, бляд-, муд-, манд-, елд-.
  Excretory (7): сра-, сса-, перд-, бзд-, дрист-, говн-, жоп-.
  Peripheral (21): дроч-, трах-, жр-, хер-, хар-, блев-, залуп-, пидор-, курв-,
  сиповк-, секел-, поц-, молофь-, минж-, целк-, королёвк-, кун-, сперм-, менстр-,
  минет-, гондон-.
- **Two-axis taxonomy grounded on the source:** `Domain` (Nuclear / Excretory /
  Peripheral, source §1) and `ProductivityClass` (A–E, source §2). Productivity is
  now a visible attribute in `list-roots` and the `explore` header.
- **Source-grounded attestation.** Common/Rare levels trace to the Plutser-Sarno
  synthesis; analogical extrapolation is honestly marked `possible`. Methodology and
  the full inventory live in `docs/decisions/plutser-sarno-taxonomy.md`.
- **New suffix class: -и- (suffix index 3).** Supports -ить verbs (дрочить, мудить,
  херить). Three endings: infinitive -ить, past m.sg -ил, present 3sg -ит.
- **`--mode {classic|full}` CLI flag.** Classic (default) shows the 9 backward-
  compatible roots (nuclear ∪ excretory≤B); full shows all 35. Direct
  `explore <root>` works regardless of mode.
- **Domain grouping** in `list-roots --mode full`: output grouped by semantic domain
  with headers (Ядро / Экскреторная / Периферия). Noun-only roots marked "(только именной)".
- **Root allomorphy support:** `present_stem` field on `RootData` enables stem
  alternations (e.g., блев- → present stem блю- for блюёт).
- **Prefix expansion from 9 to 18:** added в-, вз-/вс-, о-/об-, по-, под-, при-,
  раз-/рас-, с-, у-.
- **Prefix allomorph selection** for раз-/рас- and вз-/вс- (voice assimilation)
  and о-/об- (consonant-deletion before consonant-initial roots).

### Changed

- муд- root: suffix class changed from -а- to -и- (actual verb forms are мудить,
  not *мудать). Attestation data updated accordingly.
- пизд- root: added -и- suffix class (пиздить), in addition to existing
  -е- (пиздеть) and -ну- (пиздануть) classes.
- перд- root: corrected to the -е- class (пердеть) per source §3; the previous -а-
  class built the non-word *пердать.
- `RootData` now carries `domain` (Domain), `productivity` (ProductivityClass), and
  `present_stem` (Option<&str>) instead of a single `group` field.
- `ParadigmResult` now carries `root_domain` and `root_productivity`.
- Root inventory (`ROOTS`, `all_roots()`, `root_data()`) and attestation tables live
  together in `src/engine/roots/` — one home per root; `morpheme.rs` holds types and
  morpheme inventories only.
- `list_roots()`, `generate()`, `random_root()` now take a `mode` parameter.
- `format_list_roots()` now takes a `mode` parameter for mode-aware display.
- Prefix count increased from 9 to 18.
- `wrap_text` now measures line width in characters, not bytes, so Cyrillic notes in
  `random` wrap at the intended column.
- Version bumped to 0.5.0.

### Documentation

- `docs/decisions/plutser-sarno-taxonomy.md`: new one-home reference for the root
  inventory, semantic domains, productivity classes, and attestation methodology.
- `README.md`: added "Режимы" section, primary-source citations (tom 1/2, введение
  864–870), and source-grounded attestation methodology.
- `docs/architecture.md`: Domain/ProductivityClass model, corrected root sets and
  module map, SUFFIXES = 4, attestation grounded on the reference doc.
- `docs/threat-model.md`: added `random` and `--mode` surfaces.
- `CHANGELOG.md`: v0.5.0 entry.

## [0.4.0] — 2026-07-05

### Added

- **Root expansion from 1 to 9 roots.** Added 8 new roots: сра-, сса-, пизд-, хуй-,
  бляд-, муд-, манд-, елд-. Each with attestation data, meaning notes, and
  linguistic notes for the `random` subcommand.
- **New suffix class: -е-/-и- (class II).** Supports verbs with -еть/-ить theme
  (e.g., пиздеть, блядеть). Three endings: infinitive -еть, past m.sg -ел,
  present 3sg -ит.
- **`matcraft random` subcommand.** Prints a randomly selected root with its
  gloss, suffix classes, sample formed words, and a linguistic note in a boxed
  display.
- Root `val` field in `RootData` struct enabling accurate form construction for
  roots where the citation form differs from the stem (e.g., сра- vs stem ср-).

### Changed

- `RootData` now includes `val` and `linguistic_note` fields.
- Suffix table extended to 3 entries, ending table extended to 5 entries.
- `format_explore` handles roots with no verb forms (e.g., манд-, елд-) with
  a descriptive message.
- Version bumped to 0.4.0.

### Documentation

- `README.md`: added "Источники" section citing Plutser-Sarno and native speaker
  intuition methodology.
- `docs/architecture.md`: updated root inventory table to 9 entries, added
  -е-/-и- suffix class section, updated module map.

## [0.3.0] — 2026-07-05

### Changed

- **Fundamental rewrite — morphological paradigm engine.** The product changes from
  "random phrase generator" to "morphological paradigm explorer".
- CLI: replaced `--topic`/`--count` flat flags with three subcommands:
  `explore <ROOT>`, `generate [--root R] [--count N]`, `list-roots`
- Engine: 5 modules (mod, morpheme, grammar, paradigm, display) instead of 3
- Data model: `Attestation` enum (Common, Rare, Possible, Impossible) instead
  of plain phrase templates
- Architecture docs fully rewritten for new module structure

### Added

- Root еб- with 9 prefixes (bare + вы-, до-, за-, из-/ис-, на-, от-/ото-, пере-, про-)
- 2 suffix classes: -а- (imperfective) and -ну- (semelfactive)
- 3 endings per suffix class: infinitive, past m.sg, present 3sg
- Full combinatorial exploration: `matcraft explore еб-` shows all 18 combinations
  in a grouped table with attestation levels
- Attestation default+exceptions model: unlisted combinations = Possible
- Form construction with allomorphy rules (ъ-insertion, из-/ис- alternation)
- Random generation from the combinatorial pool: `matcraft generate`
- Error handling: invalid root shows available roots
- 35 unit tests covering data model, form construction, paradigm builder,
  display formatting, and CLI dispatch

### Removed

- Old phrase template engine (AdjectiveNoun, Interjection, Evaluation templates)
- `--topic` flag and gender-guessing heuristic
- 4 old roots (пизд-, хуй-, еб-, бляд-) — replaced by new morphological data model
- Old template-based generator tests

## [0.2.0] — 2026-07-05

### Changed

- Full generator rewrite: topic selects a semantic category instead of a morpheme root
- `--topic` now accepts any non-empty string (previously: 4 predefined roots)

### Added

- Morphological engine: 4 roots, ~14 prefixes, suffixes, combinatorics
- Phrase templates: adjective+noun, interjection+topic, topic—evaluation
- Adjective gender agreement via ending heuristic
- Optional `--count` flag (default 1, capped at 100)
- Modular structure: engine/ (morphemes, generator)

## [0.1.0] — 2026-07-05

### Added

- Initial project structure and CLI skeleton
- `matcraft --topic <T>` — basic generation from four topics (хуй, пизда, ебать, блядь)
- CI via GitHub Actions (cargo build, clippy, test)
