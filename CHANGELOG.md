# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
