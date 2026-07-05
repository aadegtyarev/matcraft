//! Paradigm builder: combinatorial expansion, form construction, attestation lookup.
//!
//! Main entry points:
//! - `explore(root, suffix_filter)` — builds the full combinatorial paradigm for a root
//! - `generate(mode, root_opt, count)` — generates random forms from the pool
//! - `list_roots(mode)` — returns available roots filtered by mode
//! - `random_root(mode)` — returns a random root from those available in mode
//! - `root_of_the_day(mode, day_index)` — deterministic root, stable within a day index
//! - `sample_forms(rd)` — up to 3 Common infinitives for the box display

use rand::prelude::IndexedRandom;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use crate::engine::grammar::build_form;
use crate::engine::morpheme::{
    Attestation, ExploreError, Mode, ParadigmResult, RootData, VerbForm, ending_val,
    endings_for_suffix, prefix_allomorphs, prefix_count, prefix_display, prefix_val,
    select_prefix_allomorph, suffix_display, suffix_val,
};
use crate::engine::roots::{all_roots, lookup_attestation, root_data};

/// Explore the full paradigm for a given root.
///
/// Returns all morphological combinations (prefix × suffix × ending) with
/// attestation levels and meaning notes. If `suffix_filter` is `Some`, only
/// combinations with that suffix value are returned.
///
/// Unlisted combinations default to `Attestation::Possible` — linguistically
/// honest: unattested ≠ impossible.
pub fn explore(
    root_name: &str,
    suffix_filter: Option<&str>,
) -> Result<ParadigmResult, ExploreError> {
    let rd = root_data(root_name).ok_or_else(|| {
        let available: Vec<&str> = all_roots().iter().map(|r| r.name).collect();
        ExploreError::RootNotFound {
            root: root_name.to_string(),
            available,
        }
    })?;

    let mut forms = Vec::new();

    // Iterate over all prefix × suffix × ending combinations
    for prefix_idx in 0..prefix_count() {
        let p_val = prefix_val(prefix_idx);
        let p_display = prefix_display(prefix_idx);
        let allomorphs = prefix_allomorphs(prefix_idx);

        for &suffix_idx in rd.suffix_indices {
            let s_val = suffix_val(suffix_idx);
            let s_display = suffix_display(suffix_idx);

            // Apply suffix filter if provided
            if let Some(filter) = suffix_filter {
                // Accept filter if it matches the suffix value or display form
                if s_val != filter && s_display != filter {
                    continue;
                }
            }

            // Resolve prefix allomorph before the root
            let prefix_form = select_prefix_allomorph(p_val, allomorphs, rd.val);

            // Get endings for this suffix class
            let end_indices = endings_for_suffix(suffix_idx);

            for &end_idx in &end_indices {
                let e_val = ending_val(end_idx);

                // Look up attestation and note
                let (att, note) = lookup_attestation(rd.name, prefix_idx, suffix_idx);

                // Build the full word form with present-stem allomorphy if applicable
                let form = build_form(prefix_form, rd.val, s_val, e_val, rd.present_stem);

                forms.push(VerbForm {
                    prefix_display: p_display,
                    suffix_val: s_val,
                    ending_val: e_val,
                    form,
                    attestation: att,
                    note,
                });
            }
        }
    }

    Ok(ParadigmResult {
        root_name: rd.name,
        root_gloss: rd.gloss,
        root_domain: rd.domain,
        root_productivity: rd.productivity,
        forms,
    })
}

/// Generate random forms.
///
/// If `root_name` is `None`, picks from all available roots in the given mode.
/// Returns `count` randomly selected full-form strings.
/// Count is clamped to 1..=100.
pub fn generate(mode: Mode, root_name: Option<&str>, count: usize) -> Vec<String> {
    let count = count.clamp(1, 100);
    let mut rng = rand::rng();

    // Build the form pool based on root filter
    let roots: Vec<&str> = match root_name {
        Some(name) => {
            if root_data(name).is_some() {
                vec![name]
            } else {
                return Vec::new();
            }
        }
        None => list_roots(mode),
    };

    let mut pool: Vec<String> = Vec::new();

    for &root in &roots {
        if let Ok(result) = explore(root, None) {
            // Only include forms with attestation != Impossible
            for vf in result.forms {
                if vf.attestation != Attestation::Impossible {
                    pool.push(vf.form);
                }
            }
        }
    }

    if pool.is_empty() {
        return Vec::new();
    }

    pool.shuffle(&mut rng);

    // Sample without replacement, cycling if count exceeds pool size
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        result.push(pool[i % pool.len()].clone());
    }

    result
}

/// List available roots filtered by mode.
pub fn list_roots(mode: Mode) -> Vec<&'static str> {
    all_roots()
        .iter()
        .filter(|r| mode.includes(r))
        .map(|r| r.name)
        .collect()
}

/// Pick one root from the mode's pool using the supplied RNG.
///
/// Shared by `random_root` (fresh `rand::rng()`) and `root_of_the_day`
/// (seeded `StdRng`), so the pool-selection logic lives in one place.
fn select_root<R: Rng + ?Sized>(mode: Mode, rng: &mut R) -> &'static RootData {
    let roots: Vec<&RootData> = all_roots().iter().filter(|r| mode.includes(r)).collect();
    roots.choose(rng).expect("at least one root must exist")
}

/// Select a random root from those available in the given mode.
pub fn random_root(mode: Mode) -> &'static RootData {
    select_root(mode, &mut rand::rng())
}

/// Select the deterministic "root of the day" for the given day index.
///
/// The same `day_index` always yields the same root (within a fixed `rand`
/// major version): the index seeds a `StdRng`, which drives the same pool
/// selection as `random_root`. `StdRng`'s algorithm is not guaranteed portable
/// across `rand` major versions, so a future bump may reshuffle the day→root
/// mapping — this does not break the contract, which promises "one root within
/// a day", not "day X → root Y forever" (see `docs/contracts/cli.md`).
pub fn root_of_the_day(mode: Mode, day_index: u64) -> &'static RootData {
    select_root(mode, &mut StdRng::seed_from_u64(day_index))
}

/// Assemble up to 3 Common infinitive sample forms for a root's box display.
///
/// Filters the root's paradigm for infinitives (ending `ть`) with `Common`
/// attestation and takes the first three. Returns an empty `Vec` on an explore
/// error or a root with no such forms (e.g. a purely nominal root). Shared by
/// the `random` and `root-of-the-day` commands so the assembly is not copied.
pub fn sample_forms(rd: &RootData) -> Vec<String> {
    match explore(rd.name, None) {
        Ok(result) => result
            .forms
            .iter()
            .filter(|f| f.ending_val == "ть" && f.attestation == Attestation::Common)
            .take(3)
            .map(|f| f.form.clone())
            .collect(),
        Err(_) => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explore_returns_all_combinations() {
        // 18 prefixes × 2 suffixes (еб has -а-, -ну-) = 36 combos
        let result = explore("еб", None).expect("еб should be a valid root");
        assert_eq!(result.root_name, "еб");

        let mut seen: Vec<(&str, &str)> = Vec::new();
        for vf in &result.forms {
            let key = (vf.prefix_display, vf.suffix_val);
            if !seen.contains(&key) {
                seen.push(key);
            }
        }
        // 18 prefixes × 2 suffixes = 36 combinations
        assert_eq!(seen.len(), 36, "Expected 36 prefix×suffix combinations");
    }

    #[test]
    fn test_explore_invalid_root_error() {
        let result = explore("unknown", None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            ExploreError::RootNotFound { root, available } => {
                assert_eq!(root, "unknown");
                assert!(available.contains(&"еб"));
            }
        }
    }

    #[test]
    fn test_explore_with_suffix_filter() {
        let result = explore("еб", Some("ну")).expect("еб should be valid");
        for vf in &result.forms {
            assert_eq!(vf.suffix_val, "ну", "All forms should have -ну- suffix");
        }
        assert!(!result.forms.is_empty());
    }

    #[test]
    fn test_generate_returns_count() {
        let forms = generate(Mode::Full, Some("еб"), 5);
        assert_eq!(forms.len(), 5);
    }

    #[test]
    fn test_generate_no_root_returns_count() {
        let forms = generate(Mode::Full, None, 3);
        assert_eq!(forms.len(), 3);
    }

    #[test]
    fn test_generate_form_is_valid() {
        let forms = generate(Mode::Full, Some("еб"), 10);
        for form in &forms {
            assert!(!form.is_empty(), "Generated form should not be empty");
            assert!(
                form.contains("еб"),
                "Form '{}' should contain root 'еб'",
                form
            );
        }
    }

    #[test]
    fn test_generate_count_capped_at_100() {
        let forms = generate(Mode::Full, Some("еб"), 500);
        assert!(forms.len() <= 100);
    }

    #[test]
    fn test_generate_count_zero_gives_one() {
        let forms = generate(Mode::Full, Some("еб"), 0);
        assert_eq!(forms.len(), 1);
    }

    #[test]
    fn test_list_roots_classic_contains_eb() {
        let roots = list_roots(Mode::Classic);
        assert!(
            roots.contains(&"еб"),
            "list_roots(Classic) should contain 'еб'"
        );
    }

    #[test]
    fn test_list_roots_classic_contains_9() {
        let roots = list_roots(Mode::Classic);
        assert_eq!(roots.len(), 9, "Classic mode should have 9 roots");
    }

    #[test]
    fn test_list_roots_full_contains_35() {
        let roots = list_roots(Mode::Full);
        assert_eq!(roots.len(), 35, "Full mode should have 35 roots");
    }

    #[test]
    fn test_list_roots_full_contains_all_classic() {
        let classic = list_roots(Mode::Classic);
        let full = list_roots(Mode::Full);
        for name in &classic {
            assert!(full.contains(name), "Full mode should contain '{}'", name);
        }
    }

    #[test]
    fn test_explore_known_forms_exist() {
        let result = explore("еб", None).expect("еб should be valid");
        let forms: Vec<&str> = result.forms.iter().map(|f| f.form.as_str()).collect();

        assert!(forms.contains(&"ебать"), "ебать should be in the paradigm");
        assert!(
            forms.contains(&"выебать"),
            "выебать should be in the paradigm"
        );
        assert!(
            forms.contains(&"заебать"),
            "заебать should be in the paradigm"
        );
        assert!(
            forms.contains(&"отъебать"),
            "отъебать should be in the paradigm"
        );
        assert!(
            forms.contains(&"исебать"),
            "исебать should be in the paradigm"
        );
        assert!(
            forms.contains(&"ебнуть"),
            "ебнуть should be in the paradigm"
        );
    }

    #[test]
    fn test_explore_sra_returns_combinations() {
        let result = explore("сра", None).expect("сра should be a valid root");
        assert_eq!(result.root_name, "сра");
        // 18 prefixes × 2 suffixes = 36 combos
        let mut seen: Vec<(&str, &str)> = Vec::new();
        for vf in &result.forms {
            let key = (vf.prefix_display, vf.suffix_val);
            if !seen.contains(&key) {
                seen.push(key);
            }
        }
        assert_eq!(
            seen.len(),
            36,
            "Expected 36 prefix×suffix combinations for сра-"
        );
    }

    #[test]
    fn test_explore_ssa_returns_combinations() {
        let result = explore("сса", None).expect("сса should be a valid root");
        assert_eq!(result.root_name, "сса");
        let mut seen: Vec<(&str, &str)> = Vec::new();
        for vf in &result.forms {
            let key = (vf.prefix_display, vf.suffix_val);
            if !seen.contains(&key) {
                seen.push(key);
            }
        }
        assert_eq!(
            seen.len(),
            36,
            "Expected 36 prefix×suffix combinations for сса-"
        );
    }

    #[test]
    fn test_explore_pizd_has_all_suffixes() {
        let result = explore("пизд", None).expect("пизд should be a valid root");
        assert_eq!(result.root_name, "пизд");
        let forms: Vec<&str> = result.forms.iter().map(|f| f.form.as_str()).collect();
        // -е- class forms
        assert!(forms.contains(&"пиздеть"), "пиздеть should be in paradigm");
        assert!(forms.contains(&"пиздел"), "пиздел should be in paradigm");
        assert!(forms.contains(&"пиздит"), "пиздит should be in paradigm");
        // -ну- class
        assert!(
            forms.contains(&"пизднуть"),
            "пизднуть should be in paradigm"
        );
        // -и- class (new)
        assert!(forms.contains(&"пиздить"), "пиздить should be in paradigm");
        assert!(forms.contains(&"пиздил"), "пиздил should be in paradigm");
    }

    #[test]
    fn test_explore_pizd_suffix_filter_e() {
        let result = explore("пизд", Some("е")).expect("пизд should be valid");
        for vf in &result.forms {
            assert_eq!(vf.suffix_val, "е", "All forms should have -е- suffix");
        }
    }

    #[test]
    fn test_explore_khuy_has_nu_only() {
        let result = explore("хуй", None).expect("хуй should be a valid root");
        assert_eq!(result.root_name, "хуй");
        for vf in &result.forms {
            assert_eq!(vf.suffix_val, "ну", "хуй- only has -ну- class");
        }
        let forms: Vec<&str> = result.forms.iter().map(|f| f.form.as_str()).collect();
        assert!(forms.contains(&"хуйнуть"), "хуйнуть should be in paradigm");
    }

    #[test]
    fn test_explore_blyad_has_ei_class() {
        let result = explore("бляд", None).expect("бляд should be a valid root");
        assert_eq!(result.root_name, "бляд");
        for vf in &result.forms {
            assert_eq!(vf.suffix_val, "е", "бляд- only has -е- class");
        }
        let forms: Vec<&str> = result.forms.iter().map(|f| f.form.as_str()).collect();
        assert!(forms.contains(&"блядеть"), "блядеть should be in paradigm");
    }

    #[test]
    fn test_explore_mand_returns_empty() {
        let result = explore("манд", None).expect("манд should be a valid root");
        assert_eq!(result.root_name, "манд");
        assert!(result.forms.is_empty(), "манд- should have no verb forms");
    }

    #[test]
    fn test_explore_eld_returns_empty() {
        let result = explore("елд", None).expect("елд should be a valid root");
        assert_eq!(result.root_name, "елд");
        assert!(result.forms.is_empty(), "елд- should have no verb forms");
    }

    #[test]
    fn test_explore_ei_forms_use_correct_endings() {
        let result = explore("пизд", None).expect("пизд should be valid");
        let infinitive: Vec<&str> = result
            .forms
            .iter()
            .filter(|f| f.ending_val == "ть" && f.suffix_val == "е")
            .map(|f| f.form.as_str())
            .collect();
        assert!(!infinitive.is_empty(), "should have infinitive forms");
        assert!(infinitive.iter().any(|f| f.ends_with("деть")));

        let past: Vec<&str> = result
            .forms
            .iter()
            .filter(|f| f.ending_val == "л" && f.suffix_val == "е")
            .map(|f| f.form.as_str())
            .collect();
        assert!(!past.is_empty(), "should have past forms");
        assert!(past.iter().any(|f| f.ends_with("дел")));

        let present: Vec<&str> = result
            .forms
            .iter()
            .filter(|f| f.ending_val == "ит" && f.suffix_val == "е")
            .map(|f| f.form.as_str())
            .collect();
        assert!(!present.is_empty(), "should have present 3sg forms");
        assert!(present.iter().any(|f| f.ends_with("дит")));
    }

    #[test]
    fn test_explore_pizd_i_class() {
        // -и- class forms for пизд-: пиздить, пиздил, пиздит
        let result = explore("пизд", None).expect("пизд should be valid");
        let forms: Vec<&str> = result.forms.iter().map(|f| f.form.as_str()).collect();
        assert!(forms.contains(&"пиздить"), "пиздить (bare -и- inf)");
        assert!(forms.contains(&"пиздил"), "пиздил (bare -и- past)");
        let present: Vec<&str> = result
            .forms
            .iter()
            .filter(|f| f.suffix_val == "и" && f.ending_val == "ит")
            .map(|f| f.form.as_str())
            .collect();
        // Some present form should exist
        assert!(!present.is_empty());
    }

    #[test]
    fn test_generate_from_sra() {
        let forms = generate(Mode::Full, Some("сра"), 3);
        assert_eq!(forms.len(), 3);
        for form in &forms {
            assert!(
                form.contains("ср"),
                "Form '{}' should contain root 'ср'",
                form
            );
        }
    }

    #[test]
    fn test_generate_from_pizd() {
        let forms = generate(Mode::Full, Some("пизд"), 3);
        assert_eq!(forms.len(), 3);
        for form in &forms {
            assert!(
                form.contains("пизд"),
                "Form '{}' should contain 'пизд'",
                form
            );
        }
    }

    #[test]
    fn test_random_root_returns_valid() {
        let rd = random_root(Mode::Full);
        let all = all_roots();
        assert!(
            all.iter().any(|r| r.name == rd.name),
            "random_root should return a known root"
        );
    }

    #[test]
    fn test_random_root_classic_only_classic_roots() {
        // In Classic mode, random should only ever return a Classic-visible root.
        let rd = random_root(Mode::Classic);
        assert!(
            Mode::Classic.includes(rd),
            "random_root(Classic) should only return roots visible in Classic mode"
        );
    }

    #[test]
    fn test_list_roots_classic_exact_composition() {
        // The classic set is derived from the taxonomy (nuclear ∪ excretory≤B),
        // and must be exactly these 9 roots — not just a count of 9.
        let mut roots = list_roots(Mode::Classic);
        roots.sort_unstable();
        let mut expected = vec![
            "еб", "бляд", "хуй", "пизд", "муд", "манд", "елд", "сра", "сса",
        ];
        expected.sort_unstable();
        assert_eq!(roots, expected, "classic composition must be exactly the 9");
    }

    #[test]
    fn test_classic_attested_roots_have_common_infinitives() {
        // Scenario 7 (random in classic): the source-attested classic roots expose
        // ≥1 Common infinitive, so `random` can show examples. хуй-/бляд- (source
        // gaps: -ну- surrogate / "?" base) and манд-/елд- (noun-only) legitimately
        // have none post-grounding — the honesty gate forbids faking Common there,
        // and `random` handles them gracefully (see display tests).
        for name in ["еб", "пизд", "сра", "сса"] {
            let result = explore(name, None).expect("root explores");
            let has_common_inf = result
                .forms
                .iter()
                .any(|f| f.ending_val == "ть" && f.attestation == Attestation::Common);
            assert!(
                has_common_inf,
                "attested classic root '{}' should have ≥1 Common infinitive",
                name
            );
        }
    }

    #[test]
    fn test_explore_blev_present_stem_end_to_end() {
        // The present-stem allomorphy (блев → блю) must surface in the full
        // paradigm, not just in build_form: блюёт should be among блев-'s forms.
        let result = explore("блев", None).expect("блев should be valid");
        let forms: Vec<&str> = result.forms.iter().map(|f| f.form.as_str()).collect();
        assert!(
            forms.contains(&"блюёт"),
            "explore(блев) should contain present form блюёт (present_stem блю)"
        );
        assert!(
            forms.contains(&"блевать"),
            "explore(блев) should contain infinitive блевать (dictionary stem)"
        );
    }

    #[test]
    fn test_explore_new_roots() {
        // Check that new Group 3+ roots produce forms
        for root in &["дроч", "трах", "жр", "хер", "перд", "хар"] {
            let result = explore(root, None).unwrap_or_else(|_| panic!("{root} should be valid"));
            assert!(!result.forms.is_empty(), "{} should have verb forms", root);
        }
    }

    #[test]
    fn test_explore_group_4_has_forms() {
        for root in &["залуп", "жоп", "говн", "пидор", "курв"] {
            let result = explore(root, None).unwrap_or_else(|_| panic!("{root} should be valid"));
            assert!(!result.forms.is_empty(), "{} should have verb forms", root);
        }
    }

    #[test]
    fn test_explore_group_5_no_forms() {
        for root in &[
            "сиповк",
            "секел",
            "поц",
            "молофь",
            "минж",
            "целк",
            "королёвк",
            "кун",
            "сперм",
            "менстр",
            "минет",
            "гондон",
        ] {
            let result = explore(root, None).unwrap_or_else(|_| panic!("{root} should be valid"));
            assert!(
                result.forms.is_empty(),
                "{} should have no verb forms (noun-only)",
                root
            );
        }
    }

    #[test]
    fn test_explore_pizd_suffix_i_forms() {
        // пизд- with -и- suffix should generate valid forms
        let result = explore("пизд", Some("и")).expect("пизд should be valid");
        assert!(!result.forms.is_empty(), "пизд + -и- should have forms");
        for vf in &result.forms {
            assert_eq!(vf.suffix_val, "и");
        }
    }

    #[test]
    fn test_root_of_the_day_is_stable_for_fixed_index() {
        // The core contract: a fixed day index always yields the same root.
        assert_eq!(
            root_of_the_day(Mode::Full, 12345).name,
            root_of_the_day(Mode::Full, 12345).name,
            "same day index must yield the same root"
        );
    }

    #[test]
    fn test_root_of_the_day_varies_across_indices() {
        // Not a constant: across 0..100 indices we see more than one root.
        // Can't require pairwise distinctness — only 9/35 roots exist, so
        // adjacent indices may collide. We assert "not always the same root".
        let names: std::collections::HashSet<&str> = (0..100)
            .map(|i| root_of_the_day(Mode::Full, i).name)
            .collect();
        assert!(names.len() > 1, "root of the day must not be constant");
    }

    #[test]
    fn test_root_of_the_day_classic_only_classic_roots() {
        // Mode invariant (mirror of test_random_root_classic_only_classic_roots).
        for i in 0..50 {
            let rd = root_of_the_day(Mode::Classic, i);
            assert!(
                Mode::Classic.includes(rd),
                "root_of_the_day(Classic, {i}) must be visible in Classic mode"
            );
        }
    }

    #[test]
    fn test_root_of_the_day_returns_valid_root() {
        let rd = root_of_the_day(Mode::Full, 42);
        assert!(
            all_roots().iter().any(|r| r.name == rd.name),
            "root_of_the_day should return a known root"
        );
    }

    #[test]
    fn test_sample_forms_verb_root_nonempty_and_contains_root() {
        let rd = root_data("еб").expect("еб should be a valid root");
        let samples = sample_forms(rd);
        assert!(!samples.is_empty(), "еб- should yield Common infinitives");
        for form in &samples {
            assert!(
                form.contains("еб"),
                "sample form '{form}' should contain root 'еб'"
            );
        }
    }

    #[test]
    fn test_sample_forms_nominal_root_empty() {
        // манд- is purely nominal — no verb forms, so no samples (mirrors the
        // inline logic's contract).
        let rd = root_data("манд").expect("манд should be a valid root");
        assert!(
            sample_forms(rd).is_empty(),
            "nominal root манд- should yield no sample forms"
        );
    }
}
