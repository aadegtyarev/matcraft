# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
