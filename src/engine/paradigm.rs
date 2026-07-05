//! Paradigm builder: combinatorial expansion, form construction, attestation lookup.
//!
//! Main entry points:
//! - `explore(root, suffix_filter)` — builds the full combinatorial paradigm for a root
//! - `generate(root_opt, count)` — generates random forms from the pool
//! - `list_roots()` — returns available roots

use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;

use crate::engine::grammar::build_form;
use crate::engine::morpheme::{
    Attestation, ExploreError, ParadigmResult, VerbForm, all_roots, ending_val, endings_for_suffix,
    lookup_attestation, prefix_allomorphs, prefix_count, prefix_display, prefix_val, root_data,
    select_prefix_allomorph, suffix_display, suffix_val,
};

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

                // Build the full word form
                let form = build_form(prefix_form, rd.val, s_val, e_val);

                forms.push(VerbForm {
                    prefix_val: prefix_form,
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
        forms,
    })
}

/// Generate random forms.
///
/// If `root_name` is `None`, picks from all available roots.
/// Returns `count` randomly selected full-form strings.
/// Count is clamped to 1..=100.
pub fn generate(root_name: Option<&str>, count: usize) -> Vec<String> {
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
        None => all_roots().iter().map(|r| r.name).collect(),
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

/// List available roots.
pub fn list_roots() -> Vec<&'static str> {
    all_roots().iter().map(|r| r.name).collect()
}

/// Select a random root from the available ones.
pub fn random_root() -> &'static crate::engine::morpheme::RootData {
    let roots = all_roots();
    let mut rng = rand::rng();
    roots
        .choose(&mut rng)
        .expect("at least one root must exist")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explore_returns_all_combinations() {
        // 9 prefixes × 2 suffixes = 18 combinations × 3 endings each
        // But the walking skeleton asks for 18 forms (infinitive only check):
        // Actually we output ALL endings per combination, so more than 18.
        // The plan says 9 prefixes × 2 suffixes = the combinatorial space.
        // Let's check that we have at least the infinitive for each combination.
        let result = explore("еб", None).expect("еб should be a valid root");
        assert_eq!(result.root_name, "еб");

        // Count unique prefix+suffix combinations
        let mut seen: Vec<(&str, &str)> = Vec::new();
        for vf in &result.forms {
            let key = (vf.prefix_display, vf.suffix_val);
            if !seen.contains(&key) {
                seen.push(key);
            }
        }
        // 9 prefixes × 2 suffixes = 18 combinations
        assert_eq!(seen.len(), 18, "Expected 18 prefix×suffix combinations");
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
        // 9 prefixes × 1 suffix × 3 endings = 27
        assert!(!result.forms.is_empty());
    }

    #[test]
    fn test_generate_returns_count() {
        let forms = generate(Some("еб"), 5);
        assert_eq!(forms.len(), 5);
    }

    #[test]
    fn test_generate_no_root_returns_count() {
        let forms = generate(None, 3);
        assert_eq!(forms.len(), 3);
    }

    #[test]
    fn test_generate_form_is_valid() {
        let forms = generate(Some("еб"), 10);
        // Every form should be a non-empty string
        for form in &forms {
            assert!(!form.is_empty(), "Generated form should not be empty");
            // Every form should contain "еб"
            assert!(
                form.contains("еб"),
                "Form '{}' should contain root 'еб'",
                form
            );
        }
    }

    #[test]
    fn test_generate_count_capped_at_100() {
        let forms = generate(Some("еб"), 500);
        assert!(forms.len() <= 100);
    }

    #[test]
    fn test_generate_count_zero_gives_one() {
        let forms = generate(Some("еб"), 0);
        assert_eq!(forms.len(), 1);
    }

    #[test]
    fn test_list_roots_contains_eb() {
        let roots = list_roots();
        assert!(roots.contains(&"еб"), "list_roots should contain 'еб'");
    }

    #[test]
    fn test_explore_known_forms_exist() {
        let result = explore("еб", None).expect("еб should be valid");
        let forms: Vec<&str> = result.forms.iter().map(|f| f.form.as_str()).collect();

        // Check specific known forms exist
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
    fn test_list_roots_returns_9() {
        let roots = list_roots();
        assert_eq!(roots.len(), 9);
    }

    #[test]
    fn test_list_roots_contains_all_new_roots() {
        let roots = list_roots();
        for name in &[
            "еб", "сра", "сса", "пизд", "хуй", "бляд", "муд", "манд", "елд",
        ] {
            assert!(roots.contains(name), "list_roots should contain '{}'", name);
        }
    }

    #[test]
    fn test_explore_sra_returns_combinations() {
        let result = explore("сра", None).expect("сра should be a valid root");
        assert_eq!(result.root_name, "сра");
        // 9 prefixes × 2 suffixes = 18 combos
        let mut seen: Vec<(&str, &str)> = Vec::new();
        for vf in &result.forms {
            let key = (vf.prefix_display, vf.suffix_val);
            if !seen.contains(&key) {
                seen.push(key);
            }
        }
        assert_eq!(
            seen.len(),
            18,
            "Expected 18 prefix×suffix combinations for сра-"
        );
    }

    #[test]
    fn test_explore_ssa_returns_combinations() {
        let result = explore("сса", None).expect("сса should be a valid root");
        assert_eq!(result.root_name, "сса");
        // 9 prefixes × 2 suffixes = 18 combos
        let mut seen: Vec<(&str, &str)> = Vec::new();
        for vf in &result.forms {
            let key = (vf.prefix_display, vf.suffix_val);
            if !seen.contains(&key) {
                seen.push(key);
            }
        }
        assert_eq!(
            seen.len(),
            18,
            "Expected 18 prefix×suffix combinations for сса-"
        );
    }

    #[test]
    fn test_explore_pizd_has_ei_class() {
        let result = explore("пизд", None).expect("пизд should be a valid root");
        assert_eq!(result.root_name, "пизд");
        // Should contain -е-/-и- class forms (пиздеть, пиздел, пиздит)
        let forms: Vec<&str> = result.forms.iter().map(|f| f.form.as_str()).collect();
        assert!(forms.contains(&"пиздеть"), "пиздеть should be in paradigm");
        assert!(forms.contains(&"пиздел"), "пиздел should be in paradigm");
        assert!(forms.contains(&"пиздит"), "пиздит should be in paradigm");
        // Should contain -ну- class forms (eng: пизднуть, actual: пиздануть with epenthetic vowel)
        assert!(
            forms.contains(&"пизднуть"),
            "пизднуть should be in paradigm"
        );
    }

    #[test]
    fn test_explore_khuy_has_nu_only() {
        let result = explore("хуй", None).expect("хуй should be a valid root");
        assert_eq!(result.root_name, "хуй");
        // Only -ну- class forms
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
        // Only -е-/-и- class forms (блядеть)
        for vf in &result.forms {
            assert_eq!(vf.suffix_val, "е", "бляд- only has -е-/-и- class");
        }
        let forms: Vec<&str> = result.forms.iter().map(|f| f.form.as_str()).collect();
        assert!(forms.contains(&"блядеть"), "блядеть should be in paradigm");
    }

    #[test]
    fn test_explore_mand_returns_empty() {
        let result = explore("манд", None).expect("манд should be a valid root");
        assert_eq!(result.root_name, "манд");
        // Empty suffix_indices → no forms generated
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
        // -е-/-и- class: infinitive -еть, past -ел, present 3sg -ит
        let result = explore("пизд", None).expect("пизд should be valid");
        let infinitive: Vec<&str> = result
            .forms
            .iter()
            .filter(|f| f.ending_val == "ть" && f.suffix_val == "е")
            .map(|f| f.form.as_str())
            .collect();
        assert!(!infinitive.is_empty(), "should have infinitive forms");
        // At least one form should end in -деть
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
    fn test_generate_from_sra() {
        let forms = generate(Some("сра"), 3);
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
        let forms = generate(Some("пизд"), 3);
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
        let rd = random_root();
        // Must be one of the known roots
        let all = all_roots();
        assert!(
            all.iter().any(|r| r.name == rd.name),
            "random_root should return a known root"
        );
    }

    #[test]
    fn test_explore_ei_class_suffix_filter() {
        let result = explore("пизд", Some("е")).expect("пизд should be valid");
        for vf in &result.forms {
            assert_eq!(vf.suffix_val, "е", "All forms should have -е- suffix");
        }
    }
}
