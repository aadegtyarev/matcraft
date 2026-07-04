# Plan: Topic-based mat generator (rewrite core)

**Status:** Approved  
**Feature:** Rewrite the generator core from morpheme-root topics to semantic-topic phrases  
**Branch:** `feature/topic-generator`  
**Decisions:** topic-as-noun · split into engine/ · --count flag (default 1, cap 100) · gender heuristic from ending

---

## Guarantee first

This change guarantees that `matcraft --topic <STRING>` produces a generated obscene phrase related to the semantic topic `<STRING>`, using only rule-based morphological combination — no AI, no LLM, no pre-curated per-topic word lists. The generator works for ANY topic string the user provides.

---

## Behaviour

### What changes

| Dimension | Before (current) | After |
|-----------|------------------|-------|
| Topic semantics | `--topic` picks a morpheme root (хуй, пизда, ебать, блядь) | `--topic` picks a semantic category — any string; the topic word becomes the noun in generated phrases |
| Topic validation | Static enum of 4 roots; unknown topic → exit(1) | No validation; any non-empty string is accepted; empty string → error |
| Output | Single obscene word (e.g., "захуярить") | Multi-word obscene phrase containing the topic word (e.g., "пиздатые птички", "птички — пиздец") |
| Output variety | Random choice within a single root's word list | Morphological combination engine + multiple phrase templates; each invocation can produce different roots, affixes, and templates |
| Engine | Hardcoded match arms with 4-5 word lists per topic | Modular morpheme combinatorics: 4 roots, ~14 prefixes, ~5 suffixes, postfixes, combined into adjectives/verbs/nouns by rules |
| Phrase structure | Single word (no phrase templates exist) | Multiple template types: adjective+noun, interjection, evaluation construction |
| Code structure | All in `src/main.rs` (37 lines) | Split into `src/main.rs` (CLI only) + `src/engine/` modules |

### What stays the same

- CLI flag: `--topic <TOPIC>` remains the primary invocation
- No subcommands — flat CLI (`matcraft --topic <TOPIC>`)
- Stack unchanged: Rust 2024, clap 4.6, rand 0.9
- Build tools: `cargo build`, `cargo clippy`, `cargo test`
- No network, no file I/O, no external dependencies
- Offline-only operation
- MIT license

---

## Scope

### In scope

1. **Morpheme data model** — 4 roots (хуй, пизд-, еб-, бляд-) with their valid prefixes, suffixes, postfixes
2. **Word generation engine** — combine morphemes by rules to produce obscene words in multiple forms (adjective by gender, noun, verb, interjection)
3. **Phrase template system** — 3 template types that slot the topic word into obscene phrases
4. **Gender agreement heuristic** — guess Russian noun gender from ending; select matching adjective form
5. **CLI update** — `--topic` accepting any non-empty string; optional `--count N` (default 1) for multiple outputs
6. **Module structure** — main.rs (thin CLI) + engine/ module group
7. **Documentation** — architecture.md rewrite, README.md rewrite, CHANGELOG.md entry
8. **Tests** — unit tests for morpheme combination, gender heuristic, template rendering (src/ already has no test dir; create `tests/` or inline `#[cfg(test)]` modules)

### Explicitly out of scope

- Per-topic word dictionaries / thesaurus / semantic word expansion
- AI/LLM integration of any kind
- Verb conjugation (past tense, person, number agreement beyond adjective gender)
- Grammatical case agreement beyond nominative
- Complex sentence generation (clauses, embedded structures)
- Interactive mode, piping, rich terminal output
- Configurable affixes / external rule files
- Plural detection for multi-word topics (e.g., "красная площадь")
- Windows-specific testing (CI runs on ubuntu-latest; cross-platform is stack-declared but not tested here)

---

## Structural choice

### Choice 1: Topic handling (the fundamental design fork)

Three approaches for generating topic-related phrases without per-topic dictionaries or AI:

#### Approach A: Topic-as-noun (recommended)

The topic string IS the noun in every generated phrase.

```
--topic птички → "пиздатые птички", "хуёвые птички", "птички — пиздец"
--topic космос → "ебанутый космос", "охуеть, космос!", "космос — хуйня"
--topic философия → "пиздатая философия", "философия — залупа какая-то"
```

**Pros:**
- Works for ANY topic string — zero dictionary, zero AI
- Simple implementation
- The obscene morphology provides the variety (different roots + affixes + templates)
- Topics like "дятел" produce "пиздатый дятел" (matches the original examples)

**Cons:**
- Same noun repeated if the same topic is used multiple times (different obscene words each time, but the noun is fixed)
- Gender heuristics are approximate (wrong for ~5% of Russian nouns)
- Multi-word topics ("красная площадь") use the whole string as-is

#### Approach B: Large built-in thesaurus

Embed a categorized Russian noun dictionary (e.g., wordnet-style).

**Pros:**
- Rich, varied output — topic-related words appear naturally
- "птички" generates phrases about воробей, дятел, голубь, синица

**Cons:**
- REQUIRES a comprehensive thesaurus (hundreds of categories, thousands of words)
- Building and maintaining it is its own project
- Always incomplete for niche topics (--topic квантовая-механика → no categories match)
- Binary size impact (hundreds of KB to MB of word data)
- Category mapping: how does "птички" → "birds" category without AI or a static table?

#### Approach C: User-supplied companion words

```
matcraft --topic птички --words "дятел,воробей,голубь"
```

**Pros:**
- User controls the dictionary
- Fully flexible; works for any topic

**Cons:**
- Degraded UX — user must supply words for the feature to shine
- Not what the product describes (user picks a topic, tool generates)
- No graceful degradation when --words is absent

#### Decision

**Recommendation: Approach A (topic-as-noun).** It is the only approach that works for ANY topic with zero dictionary overhead, matches the "no AI" constraint, and is implementable in a single feature. The variety comes from the morphological engine (4 roots x many affixes x multiple templates), not from synonym replacement.

### Choice 2: Module decomposition

The architecture doc already plans for `src/engine/` modules. The current `src/main.rs` is 37 lines — well below any size threshold. However, the new generator engine will be ~200-300 lines, and mixing CLI parsing with morphological rules is a cohesion violation.

**Option A: Keep all in main.rs** — simpler, fewer files. But main.rs would grow past 300+ lines mixing CLI, data, and logic.

**Option B: Split into engine/ module** — main.rs stays thin (CLI only). Engine modules house morpheme data, generation logic, templates. This matches the architecture doc's planned structure.

**Recommendation: Option B.** The architecture doc already establishes this modular structure. engine/ stays cohesive, main.rs stays thin. The engine/ directory initially contains:
- `engine/mod.rs` — re-exports, public API: `generate(topic, count) -> Vec<String>`
- `engine/morphemes.rs` — static morpheme data (roots, prefixes, suffixes, postfixes, interjections)
- `engine/generator.rs` — morpheme combination rules, word generation, template application, gender heuristic

Two files in engine/ is sufficient for the initial feature; splitting further into roots.rs/affixes.rs/rules.rs is deferred until the data grows past the cohesion boundary.

### Choice 3: --count flag

**Option A: Single output only** — matches current CLI (`matcraft --topic <T>` prints one phrase). Simplest.

**Option B: Optional --count flag** — `matcraft --topic <T> [--count N]`, default 1. Allows the user to see variety (different roots, different templates for the same topic).

**Recommendation: Option B.** The whole point of a morphological generator over a static list is variety. --count 5 shows that variety. Default 1 preserves backward-compatible behaviour. Minimal implementation cost: parse u32, loop.

### Choice 4: Gender heuristic strictness

**Option A: Always use masculine adjective** — "пиздатый птички", "хуёвый философия". Grammatically incorrect for feminine/neuter topics.

**Option B: Gender heuristic from ending** — "пиздатые птички" (plural), "пиздатая философия" (fem), "пиздатый космос" (masc). Heuristic: check last character of the input word (or the last word of multi-word input).

**Recommendation: Option B.** The heuristic is ~5 lines of code, and the output sounds MUCH more natural. The risk of 5% wrong-guess cases is acceptable for a hobby project. Document the heuristic in architecture.md.

---

## Product questions (product-advocate module)

### Who is this for

Russian-speaking developers and enthusiasts who want to generate obscene phrases **by semantic topic** without AI/LLM. The product brief is absent (project inception predated it), but the audience is the same the inception named: programming-linguistics enthusiasts who understand "morphological rules" and find combinatorial creativity interesting. A secondary audience is anyone who wants a quick obscene joke generator for a given topic — this is a stretch but valid.

### What user pain

Without this tool, generating an obscene phrase for an arbitrary topic requires: (1) hand-crafting the words, (2) using an LLM (slow, online, not reproducible), or (3) searching a static dictionary. None of these give instant, repeatable, offline generation by topic.

### What breaks if we DON'T build it

The current codebase has the wrong model (topic = morpheme root, not semantic category). Without this rewrite, the project fundamentally doesn't match its own product description. The skeleton from inception would remain a misaligned toy rather than the engine the product promises.

### Is this the right bet

**Alternatives considered:**
- **Thesaurus-based** (Approach B above) — ruled out: requires a full noun taxonomy which is a separate project, and still doesn't handle arbitrary topics.
- **User-supplied word lists** (Approach C) — ruled out: degrades UX, doesn't match the product description.
- **AI/LLM integration** — ruled out by product definition (no AI).

**Why topic-as-noun now:** It is the smallest implementation that satisfies "any topic, no AI, generates related obscene phrases". The variety comes from the grammar engine, not from word lists.

### The cheapest test that would tell us

Build a single template (adjective + noun) with a single root (пизд-) and test it by hand:
```
cargo run -- --topic птички
# → "пиздатые птички" or similar
```
If this works, the architecture is proven. The root count and template count are just data additions after that.

### What breaks if we DON'T build this specifically (topic-as-noun)

The risk of topic-as-noun: phrases with the exact topic word repeated every time may feel less creative than if the generator pulled related words. But the alternative (dictionary/thesaurus) doesn't exist as buildable within this feature's scope. The acceptable risk is that the obscene morphology provides enough variety that the fixed noun is not noticeable.

---

## Verification scenario

**Primary integration layer:** CLI invocation in the terminal.

```
cd /home/adegtyarev/Develop/Hobby/matcraft
cargo build --quiet
```

**Happy path 1** (single phrase, Russian topic):
```
./target/debug/matcraft --topic птички
```
Expected: prints a phrase containing the word "птички" combined with an obscene word (e.g., "пиздатые птички", "птички — пиздец", "охуеть, птички!"). Exits with code 0.

**Happy path 2** (multiple phrases):
```
./target/debug/matcraft --topic космос --count 3
```
Expected: prints 3 different phrases, each containing the word "космос" (e.g., "ебанутый космос", "охуеть, космос!", "космос — хуйня"). Each phrase is different. Exits with code 0.

**Happy path 3** (feminine topic):
```
./target/debug/matcraft --topic философия
```
Expected: obscene adjective agrees with feminine gender: "пиздатая философия" (not "пиздатый философия").

**Happy path 4** (non-Russian topic):
```
./target/debug/matcraft --topic ornithology
```
Expected: still generates a phrase; gender heuristic falls back to masculine default for unknown endings. "ебанутая ornithology" if heuristic guesses wrong is acceptable — the tool still works.

**Error case 1** (empty topic):
```
./target/debug/matcraft --topic ""
```
Expected: prints error in Russian, exits with code 1.

**Error case 2** (missing topic):
```
./target/debug/matcraft
```
Expected: clap prints error that `--topic` is required, exits with code 2.

**Help case:**
```
./target/debug/matcraft --help
```
Expected: prints usage with `--topic` and `--count` (optional), exits with code 0.

---

## Security surface (threat-model module)

### Attack surface

| Surface | What changes | Risk | Mitigation | Closed at |
|---------|-------------|------|------------|-----------|
| `--topic` argument | Now accepts ANY non-empty string (was: 4-option enum) | **Low.** No injection vector — string is never passed to shell, filesystem, or network. Only combined with generated morphemes via `format!` macro. | Validate non-empty in clap (via clap's `validator` or `value_parser`); empty string → error before any generation. | `src/main.rs` — clap argument definition. |
| `--count` argument | New u32 argument, default 1 | **Low.** Integer, parsed by clap. Bound check needed to prevent allocating for huge values. | Cap `--count` at 100 (arbitrary safe upper bound). No malloc-from-user-value issue in Rust Vec, but prevents silly resource use. | `src/main.rs` — clap argument + cap before generation loop. |

### Secrets & credentials

**None.** No changes. The project has no API keys, tokens, passwords.

### Trust boundaries

**None.** The entire input is a string and a u32 from the user who runs the local binary. No data crosses any trust boundary.

### Injection & unsafe ops

**None.** The topic string is concatenated with generated morphemes using Rust's `format!` macro — compile-time checked, no injection possible. No shell execution, no SQL, no template engine, no deserialization.

### Fail-open vs fail-closed

**Fail-closed preserved.** The change adds no fallback path that could silently proceed on error. Empty topic → error before generation. `--count 0` → zero iterations, no output (not an error). `--count > 100` → cap silently, not an error.

### Data & privacy exposure

**None.** No data collected, stored, or transmitted. The topic string is ephemeral in process memory.

### AuthZ / AuthN

**N/A.** Local CLI tool, no access control.

### Supply chain

**No new dependencies.** The feature uses only the existing `clap` and `rand` crates. No new Cargo dependencies are added.

### Isolation / identity invariant

**N/A.** No per-user or visibility surface. Single-user CLI with no shared state.

---

## Unfamiliar interface (research-methodology module)

### Russian noun gender heuristics

**Source:** Russian grammar references (academic consensus, multiple sources)

**Rule:**
- Nouns ending in а, я → feminine (дя́дя → masc exception, but rare)
- Nouns ending in о, е → neuter
- Nouns ending in ы, и → plural
- Nouns ending in consonant, й → masculine
- Nouns ending in ь → could be feminine (тетрадь) or masculine (день) — heuristic: default to masculine as it's dominant for ь-ending nouns in obscene contexts

**Confidence:** High. This is basic Russian grammar, well-documented across all sources.  
**Verified:** 2026-07-05.

### Obscene adjective formation (Russian mat morphology)

**Source:** Academic descriptions (Wikipedia, linguistic papers), cross-referenced.

- **Root пизд-** → adjective: пизд- + -ат- + ending → пиздат-ый/ая/ое/ые
- **Root хуй-** (with й→ё alternation) → adjective: хуй → хуёв- + ending → хуёв-ый/ая/ое/ые
- **Root еб-** → adjective: еб- + -ан- + -ут- + ending → ебанут-ый/ая/ое/ые
- **Root бляд-** → adjective: бляд- + -ск- + ending → блядск-ий/ая/ое/ие

**Interjections (fixed forms):** охуеть, пиздец, ёбаный стыд, ни хуя себе, ёбаный в рот

**Evaluation nouns (fixed forms):** пиздец (м.р.), хуйня (ж.р.), залупа (ж.р.), блядство (с.р.), мудачьё (с.р.)

**Confidence:** Medium-High. The affix patterns are well-documented; й→ё alternation is a standard Russian phonological rule. Some combinations (like еб- + -ан- to form past passive participle "ебанный" vs "ёбаный") have dialectal variation — we pick the most common form.  
**Verified:** 2026-07-05.

### rand 0.9 API (existing dependency)

**Source:** docs.rs/rand/0.9

**Key APIs:**
- `rand::rng()` — creates a thread-local RNG (new in 0.9, replaces `thread_rng()`)
- `rng.random_range(0..n)` — random integer in range (replaces `gen_range`)
- `slice.choose(&mut rng)` — random element (via `rand::seq::IndexedRandom`)

**Confirmation needed:** `rand::seq::IndexedRandom` is the trait for `.choose()` in rand 0.9. The current `src/main.rs` uses `use rand::seq::IndexedRandom;` which is the 0.9 API. The Builder should verify this is the correct import for 0.9's specific release.

**Confidence:** High. Verified against docs.rs.  
**Verified:** 2026-07-05 (from inception plan research, re-confirmed).

---

## Docs

| File | Action | Content change | Language |
|------|--------|---------------|----------|
| `src/main.rs` | Rewrite | CLI skeleton only; engine logic moves to `src/engine/` | English (code) |
| `src/engine/mod.rs` | Create | Public API: `pub fn generate(topic: &str, count: usize) -> Vec<String>`; re-export sub-modules | English (code) |
| `src/engine/morphemes.rs` | Create | Static data: ROOTS, PREFIXES, SUFFIXES, INTERJECTIONS, EVAL_NOUNS; each with grammatical forms | English (code + data) |
| `src/engine/generator.rs` | Create | `fn generate_words(rng)`, `fn apply_template(template_idx, topic, generated_words)`, `fn guess_gender(topic) -> Gender` | English (code) |
| `docs/architecture.md` | Rewrite | Update module structure, data model, remove wrong topic model, document gender heuristic, add phrase template descriptions, update `[?]` list | Russian |
| `README.md` | Rewrite | Update for semantic-topic description; new examples; `--count` flag; updated available-topic description (any string) | Russian |
| `CHANGELOG.md` | New entry | 0.1.0 → 0.2.0: topic-based generation engine, phrase templates, gender agreement, `--count` flag | English |
| `docs/threat-model.md` | Update | Add `--count` to attack surface table; update topic validation description from enum to non-empty check | Russian |
| `docs/deployment.md` | No change | No functional change to deploy path | — |

Visual form for user-facing doc changes:

- **Architecture diagram** (docs/architecture.md): plain ASCII module tree showing `src/main.rs` → `src/engine/{mod,morphemes,generator}` call flow, plus a template-to-output data flow arrow (e.g., [morphemes] → [generator] → [template + topic] → [output]).  
- **README examples**: code-block invocation examples with output, showing multiple topics and `--count`.

---

## Estimate

**Complexity:** Medium (non-trivial). Multiple new modules, data modelling for morpheme combinatorics, phrase template system, gender heuristic.

**Time bucket:** 2-4 hours (medium feature per estimation guidelines).

**Risk factors:**
- **Morpheme combinatorics correctness** — the linguistic data (which prefix works with which root) is well-documented academically (finite set) but has not been verified in code. The Builder should test 3-4 combinations per root and verify they produce natural-sounding forms. If a combination produces unnatural output (e.g., "выпиздить" — is this actually used?), it should be removed from the valid set.
- **Gender heuristic edge cases** — the heuristic is simple and covers ~90% of Russian nouns. The remaining 10% (ь-ending, irregular, foreign loanwords) will produce wrong agreement — this is consciously accepted as a product limitation.
- **Existing tests:** none exist; no regression risk.
- **Design decisions:** 4 structural choices surfaced above (topic handling, module decomposition, --count flag, gender heuristic). All are resolved by recommendation; no blocker remains.

---

## Elicitation (applied to draft)

### Technique 1: Pre-mortem

**Scenario:** "It is 6 months later and the topic-generator rewrite failed. Why?"

**Failure mode 1:** "The output phrases are repetitive because the same topic word appears every time, and the morphological engine only generates 4 root variants. Users try 5 topics, get bored, never come back."

**Analysis:** This is the fundamental risk of the topic-as-noun approach. Mitigations:
- The 4 roots with ~14 prefixes × multiple templates provide many combinations: 4 roots × ~10 prefixable adjective forms × 3 templates = ~120 distinct phrase structures. With `--count 5`, the user sees 5 different constructions.
- The morpheme engine should be designed so adding a new root or template is a data addition, not a code change — the project can grow the set over time.
- **Plan amendment:** explicitly include a variety test in the verification: `--count 10` on the same topic should produce at least 6 distinct phrases (i.e., less than 60% repetition).

**Failure mode 2:** "Gender heuristics produce gratingly wrong agreement for common words ending in ь, and users complain the output sounds non-native."

**Analysis:** This applies to ~5% of Russian nouns (words ending in ь). Many mat-adjacent words (грязь, соль, боль — feminine ь) would get wrong agreement.  
**Mitigation:** Document the heuristic's limitations in architecture.md. Accept the limitation as conscious. If it becomes a problem, the Builder can add an explicit feminine/masculine override flag (`--gender m/f/n`) or a small exception list — both out of scope for this feature but plausible follow-ups.

### Technique 2: Red vs blue (hostile angle)

**Red (attacks):** "Topic-as-noun is a cop-out. The user types 'птички' and gets 'пиздатые птички' — that is NOT the same as the promised 'пиздатый дятел', 'херов воробей'. The examples implied semantic expansion. Without it, the product is just 'prefix the topic with obscene adjective'. A dictionary-free thesaurus-free generator for arbitrary topics is a HARD problem, and this approach side-steps it rather than solving it."

**Blue (defends):** "The problem as stated says ANY topic — not just topics where you happen to have a word list. Let us count the approaches that produce 'пиздатый дятел' from --topic птички: (1) a birds thesaurus → doesn't work for --topic квантовая-физика; (2) AI → ruled out; (3) WordNet in Russian → requires a comprehensive semantic graph that does not exist in a consumable form for this project. The ONLY approach that works for every arbitrary topic, without AI, without a pre-built thesaurus, is to use the topic word as the noun. The examples were illustrative of the genre, not a spec of implementation. The morphological variety (4 roots, ~14 prefixes, 3 templates) ensures the output is not just 'prefix-obscene + topic' — different roots produce different obscene words, different templates produce different syntactic structures."

**Surface assumption:** Both sides assume the user primarily values topical relevance over syntactic variety. This assumption needs Operator validation: does the product need to generate words semantically related to the topic (synonyms, related concepts), or is it acceptable to generate obscene phrases that grammatically incorporate the topic word? The plan bets on the latter.

**Plan amendment:** None needed — the draft already makes the structural choice visible to the Operator. The pre-mortem's concern about repetition is added to the scope notes above.

---

## Plan adversary (probe before finalizing)

### What breaks?

**Most plausible failure:** The morpheme combinatorics produce unnatural or non-existent Russian words. "Разъебать" may be valid but "отпиздить" may be marginal. The Builder should test each generated form against native intuition and remove non-idiomatic combinations.

**Test that catches it:** Generate all possible combinations from the morpheme sets (a combinatorial dump) and review the output — likely in a test called `fn dump_all_combinations()` that the Builder runs once but does not commit (or commits as `#[cfg(test)]`).

### What is missing?

- **Error message language:** The error for empty topic should be in Russian (docLanguage: ru): "Ошибка: тема не может быть пустой". The --help text must also be Russian.
- **Clippy warn-free:** The existing CI denies warnings. The new code must pass `cargo clippy -- -D warnings`.
- **Multi-word topic handling:** `--topic "красная площадь"` — gender heuristic looks at the last word "площадь" (ends in ь, defaults to masculine, but it's feminine). Document this edge case.
- **Non-Cyrillic topic handling:** `--topic ornithology` — heuristic sees consonant-ending → masculine. "Пиздатый ornithology" — the agreement is unnatural but the output is still comprehensible. Acceptable.

### Fuzzy expected values tightened

| Fuzzy claim | Tightened criterion |
|-------------|-------------------|
| "Produces different phrases each time" | `--count 10` on the same topic produces at least 8 distinct output lines (≤20% duplicates) |
| "Gender heuristic works for most nouns" | Heuristic correctly handles а/я/о/е/ы/и/consonant endings — verified by unit test with 10 sample nouns per gender |
| "Morpheme combinations are linguistically valid" | Every combination in the static data is attested and documented; no productive-but-unattested combinations |

### Hidden structural fork

**Fork: template application order.** Two approaches:
- **Sequential:** for each of `count` outputs, pick one template, generate one obscene word, combine → output
- **Batch:** generate a pool of obscene words, then for each output pick a template + a word from the pool

The plan implicitly assumes sequential (simpler, no pooling logic). This is fine for the initial implementation. If performance becomes an issue at high `--count`, batch can be added later.

**Fork: error return for empty output on --count 0.** `--count 0` could return an error or produce no output. Recommendation: produce no output silently (vacuous truth: "0 phrases generated, 0 errors"). clap's default 1 means this only happens with explicit `--count 0`.

---

## Modularity check

### Boundary named

The change touches the boundary between `src/main.rs` (CLI layer) and `src/engine/` (domain logic layer), as described in `docs/architecture.md` `## Слои системы`:

```
┌─────────────┐
│    CLI      │ clap + validation + output
├─────────────┤
│   Engine    │ Generation logic + morpheme data + templates
└─────────────┘
```

### Dependency direction

The dependency is `main.rs → engine/mod.rs` — the CLI imports engine's public API. This follows the architecture doc's direction (CLI imports Engine, never the reverse). No new inter-module dependency is introduced.

---

## Test methodology

### Unreachable layers

- **CLI + engine integration (fetch-and-render):** the route handler (`main()` calls `engine::generate()` then prints) cannot be tested by unit test alone — its full behaviour (clap parsing → engine → stdout output) is integration-level. **Coverage:** covered by the verification scenario (manual CLI invocation). An integration test in `tests/` directory could automate this but is out of scope for this feature; the untested-layer risk is named here.
- **Gender heuristic:** unit-testable via `#[cfg(test)] mod tests` in `generator.rs` — easy to cover all ending types with 10-15 test cases.
- **Morpheme combination:** unit-testable — test that each root produces expected forms with valid affixes.

### App-bug vs test-drift

**N/A.** There are no existing tests to drift. All tests are new.

---

## Semantic correctness

### Real or marked

- **Morpheme engine:** REAL — produces actual obscene words by combination rules. Persisted state: none needed (stateless computation). Test: generate all combinations, verify each is a valid word.
- **Gender heuristic:** REAL — implemented as ending-check logic. Test: 10 sample nouns per gender type, verify correct agreement.
- **Phrase templates:** REAL — 3 template types, implemented as match arms with `format!`. No learning/adaptation claimed.

### Parallel-path guards

**N/A.** No parallel execution paths (streaming vs blocking, batch vs single, async vs sync). Single-threaded synchronous path only.

---

## UI & UX (CLI-specific, not GUI)

### Adaptivity

**N/A.** CLI tool — no screen-size adaptivity concerns.

### Accessibility

**N/A.** CLI tool — no visual UI.

### Responsiveness

**N/A.** Generation is instant (<1ms per phrase). No loading states needed.

### Clarity

- Help text must be in Russian (`docLanguage: ru`)
- Error messages must say WHAT was wrong and HOW to fix it: e.g., "Ошибка: тема не указана. Используйте --topic <ТЕМА>" — not just "invalid argument"
- `--topic` description in --help: "Тема для генерации — любое слово или фраза (например, птички, космос, философия)"
- `--count` description: "Количество фраз для генерации (по умолчанию 1)"

### Adverse states

- **Empty topic (-t "")**: clap may pass empty string — validate before generation loop. Error message in Russian.
- **`--count 0`**: produce no output, exit 0 (vacuous truth). Document in help? No — edge case, not worth cluttering help.
- **`--count 1000000`**: cap at 100, silently. No error.
- **Very long topic string**: no limit needed — Rust's String handles it. But cap at 256 chars for sanity (clap's `value_parser` can enforce this).

### User-flow check (CLI flow, critical path)

1. **Step 1:** User runs `matcraft --topic птички` → clap parses args → validates topic non-empty, count defaults to 1
   - UI element: CLI command line → system starts binary
   - Action: command execution
   - Feedback path: if `--topic` missing, clap prints error and --help (exit 2). No silent failure.

2. **Step 2:** engine::generate("птички", 1) → guesses gender (plural, ending in "и") → picks template → generates obscene word → combines into phrase
   - UI element: none (internal computation)
   - Action: generation
   - Feedback path: no error possible for valid input. Empty topic handled before reaching engine.

3. **Step 3:** println!("{}", phrase) → prints to stdout → exit 0
   - UI element: terminal stdout
   - Action: output to user
   - Feedback path: always succeeds (no I/O error possible for stdout in a CLI). User sees the phrase immediately.

---

## Performance

### Name the expected scale

- **Topic length:** 1-256 chars (capped)
- **Count:** 1-100 (capped)
- **Morpheme sets:** 4 roots, ~14 prefixes, ~5 suffixes, ~3 postfixes, ~6 interjections, ~5 evaluation nouns — all small static arrays
- **Generation time:** <1ms per phrase (microseconds for array selection + format!)

### No unbounded path

**No unbounded paths.** The morpheme sets are fixed at compile time. The `--count` loop is bounded by the 100 cap. String formatting is over fixed-size local data.

---

## Database, i18n, Concurrency

All three modules are **inert** for this change:
- **Database:** No persistent store
- **i18n:** `docLanguage: ru`, single-locale Russian project; strings are in Russian help/error text, no translation mechanism needed
- **Concurrency:** Single-threaded CLI, no shared state

---

## Work items (Builder instructions)

### 0. Before building

Confirm with Operator the structural choices:
- **Topic handling:** Approach A (topic-as-noun) — confirmed?
- **Module decomposition:** Split into engine/ — confirmed?
- **--count flag:** Optional, default 1, cap 100 — confirmed?
- **Gender heuristic:** From ending — confirmed?

### 1. Create engine module

#### 1a. `src/engine/mod.rs`

```rust
pub mod morphemes;
pub mod generator;

pub use generator::generate;
```

Public API:
- `pub fn generate(topic: &str, count: usize) -> Vec<String>` — main entry point
- Returns generated phrases; caller (main.rs) prints them

#### 1b. `src/engine/morphemes.rs`

Static data structures:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Gender { Masculine, Feminine, Neuter, Plural }

pub struct Root {
    pub name: &'static str,
    pub adjective_stem: &'static str,
    pub adjective_endings: &'static [&'static str; 4], // [masc, fem, neut, pl]
    pub evaluation_noun: Option<&'static str>,
    pub verb_base: Option<&'static str>,
    pub interjection: Option<&'static str>,
}
```

Static arrays:
- `pub const ROOTS: &[Root] = &[...]` — 4 roots with adjectives, evaluation nouns, interjections
- `pub const INTERJECTIONS: &[&str] = &["охуеть", "пиздец", "ёбаный стыд", "ни хуя себе", "ёбаный в рот"]`
- `pub const EVAL_NOUNS: &[&str] = &["пиздец", "хуйня", "залупа", "блядство", "мудачьё"]`

**Key data — per-root adjective forms:**

| Root | Adjective stem | Masc | Fem | Neut | Plural | Adjective meaning |
|------|---------------|------|-----|------|-------|------------------|
| пизд- | пиздат- | -ый | -ая | -ое | -ые | awesome (пиздатый) |
| хуй- | хуёв- | -ый | -ая | -ое | -ые | shitty (хуёвый) |
| еб- | ебанут- | -ый | -ая | -ое | -ые | crazy (ебанутый) |
| бляд- | блядск- | -ий | -ая | -ое | -ие | whorish (блядский) |

**Key data — interjections by root:**

| Root | Interjection(s) |
|------|----------------|
| пизд- | пиздец! |
| хуй- | ни хуя себе! |
| еб- | охуеть!, ёбаный стыд!, ёбаный в рот! |
| бляд- | — |

**Key data — evaluation nouns by root:**

| Root | Eval noun |
|------|-----------|
| пизд- | пиздец |
| хуй- | хуйня |
| еб- | залупа |
| бляд- | блядство |

#### 1c. `src/engine/generator.rs`

Three public functions:

1. `pub fn generate(topic: &str, count: usize) -> Vec<String>` — main entry
2. `pub fn guess_gender(word: &str) -> Gender` — ending-based heuristic
3. Internal: `fn apply_template(t: TemplateKind, topic: &str, adj_form: &str, interj: &str, eval: &str, rng: &mut ThreadRng) -> String`

Templates (3):

```rust
enum TemplateKind {
    AdjectiveNoun,    // "[ADJ] [TOPIC]" — requires gender agreement
    Interjection,     // "[INTERJ], [TOPIC]!"
    Evaluation,       // "[TOPIC] — [EVAL_NOUN]"
}
```

Template selection: pick one uniformly at random from the 3.

For each of `count` iterations:
1. Pick a random root
2. Get the adjective form matching the topic's gender (from guess_gender)
3. Optionally (50%?) also pick an interjection or eval noun from the same root
4. Pick a template
5. Apply: format the phrase
6. Collect into Vec

### 2. Rewrite `src/main.rs`

Thin CLI layer:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "CLI-генератор матерных выражений по теме")]
struct Cli {
    /// Тема для генерации (например: птички, космос, философия)
    #[arg(short, long, required = true)]
    topic: String,

    /// Количество фраз (по умолчанию 1, максимум 100)
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

fn main() {
    let cli = Cli::parse();

    // Validate
    if cli.topic.trim().is_empty() {
        eprintln!("Ошибка: тема не указана. Используйте --topic <ТЕМА>");
        std::process::exit(1);
    }

    let count = cli.count.clamp(1, 100);
    let phrases = engine::generate(&cli.topic, count);

    for phrase in phrases {
        println!("{phrase}");
    }
}
```

### 3. Update `Cargo.toml` (if needed)

Confirm `rand = "0.9"` is the correct version for the `IndexedRandom` trait. If `choose` requires `rand::seq::IndexedRandom` in rand 0.9, confirm the trait still compiles. The current Cargo.toml may need no change.

### 4. Update documentation

#### 4a. `docs/architecture.md`

Rewrite the architecture doc:
- Update module structure (src/main.rs → src/engine/ module)
- Replace wrong "topic = morpheme root" model with "topic = semantic category, used as noun in phrases"
- Add data model: Root, Gender, TemplateKind
- Document gender heuristic (ending-based) and its limitations
- Document phrase template types
- Update `[?]` unknowns:
  - Remove "полная матрица совместимости аффиксов" — resolved for this feature scope
  - Add "complete list of interjections per root" `[?]`
  - Add "verb agreement for sentence-level templates" `[?]`
- Update ASCII diagram to show the 3-template flow

#### 4b. `README.md`

Rewrite in Russian:
- New description: "CLI-генератор матерных выражений по теме. Берёт любую тему и генерирует обсценные фразы по морфологическим правилам."
- Updated usage example:
  ```sh
  matcraft --topic птички
  # → пиздатые птички

  matcraft --topic космос --count 3
  # → ебанутый космос
  # → космос — пиздец
  # → охуеть, космос!
  ```
- Updated install/develop sections

#### 4c. `CHANGELOG.md`

New entry: `[0.2.0] — 2026-07-05`
```
### Changed
- Полная переработка генератора: вместо выбора слова по корню — генерация фраз по семантической теме
- `--topic` теперь принимает любую строку (не только предопределённые корни)

### Added
- Морфологический движок: 4 корня, ~14 приставок, суффиксы, комбинаторика
- Шаблоны фраз: прилагательное+существительное, междометие+тема, тема—оценка
- Согласование прилагательных по роду (эвристика по окончанию)
- Опциональный флаг `--count` (по умолчанию 1, макс. 100)
- Модульная структура: engine/ (morphemes, generator)
```

#### 4d. `docs/threat-model.md`

Minimal update:
- Add `--count` argument row to vulnerable-surfaces table
- Update topic validation description: "validated as non-empty string (was: 4-topic enum)"

### 5. Write tests

Create test module in `src/engine/generator.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_gender_masculine() {
        assert_eq!(guess_gender("космос"), Gender::Masculine);
        assert_eq!(guess_gender("дятел"), Gender::Masculine);
        assert_eq!(guess_gender("автомобиль"), Gender::Masculine);
    }

    #[test]
    fn test_guess_gender_feminine() {
        assert_eq!(guess_gender("философия"), Gender::Feminine);
        assert_eq!(guess_gender("птичка"), Gender::Feminine);
        assert_eq!(guess_gender("рыба"), Gender::Feminine);
    }

    #[test]
    fn test_guess_gender_neuter() {
        assert_eq!(guess_gender("море"), Gender::Neuter);
        assert_eq!(guess_gender("солнце"), Gender::Neuter);
    }

    #[test]
    fn test_guess_gender_plural() {
        assert_eq!(guess_gender("птички"), Gender::Plural);
        assert_eq!(guess_gender("автомобили"), Gender::Plural);
    }

    #[test]
    fn test_generate_returns_requested_count() {
        let result = generate("тест", 5);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_generate_phrases_contain_topic() {
        let result = generate("космос", 10);
        for phrase in &result {
            assert!(phrase.contains("космос"), "Phrase '{}' does not contain topic", phrase);
        }
    }

    #[test]
    fn test_generate_returns_diverse_phrases() {
        let result = generate("тест", 10);
        let unique: std::collections::HashSet<&str> = result.iter().map(|s| s.as_str()).collect();
        // At least 80% unique (≤20% duplicates)
        assert!(unique.len() >= 8, "Only {} unique phrases out of 10", unique.len());
    }

    #[test]
    fn test_count_capped_at_100() {
        let result = generate("тест", 500);
        assert!(result.len() <= 100);
    }
}
```

### 6. Build and verify

```sh
cargo build --all-targets
cargo clippy -- -D warnings
cargo test

# Manual verification:
./target/debug/matcraft --topic птички
./target/debug/matcraft --topic космос --count 3
./target/debug/matcraft --topic философия
./target/debug/matcraft --topic ornithology
./target/debug/matcraft --topic ""            # should error
./target/debug/matcraft                       # should error (missing --topic)
./target/debug/matcraft --help
```

### 7. Not in scope (explicit)

- Adding more roots beyond the 4 standard Russian mat roots
- Verb conjugation for sentence-level patterns
- Genitive/prepositional case phrases
- User-configurable word lists or theme files
- `docs/product.md` creation (deferred to product discovery)
- Integration tests in `tests/` directory (unit tests in `#[cfg(test)]` are sufficient)

---

## Progress note

```
feature: topic-generator
state: plan-draft
blocked-by: operator-approval
next: after approval → Builder implements modules → rewrite main.rs → tests → build-verify → Reviewer → ship
remaining decisions:
  1. Topic handling: APPROACH A (topic-as-noun) recommended — Operator confirm?
  2. Module structure: SPLIT INTO engine/ recommended — confirm?
  3. --count flag: OPTIONAL (default 1, cap 100) recommended — confirm?
  4. Gender heuristic: FROM ENDING recommended — confirm?
design-decisions:
  1. topic-handling → topic-as-noun → derivable from constraint "any topic, no AI, no thesaurus"
  2. module-structure → split-into-engine → follows architecture.md plan
  3. count-flag → optional-count → product variety requirement
  4. gender-heuristic → ending-based → derivable from Russian grammar
```
