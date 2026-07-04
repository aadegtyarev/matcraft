# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
