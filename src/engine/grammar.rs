//! Form construction: combine morphemes into full word forms with allomorphy rules.
//!
//! Allomorphy rules for the walking skeleton:
//!
//! 1. **ъ-insertion**: a prefix ending in a consonant followed by a root starting
//!    with a soft vowel (е, ё, ю, я) inserts the hard sign (ъ) between them.
//!    Example: от- + еб- → отъеб-
//!
//! 2. **из-/ис- alternation**: the prefix из- uses its allomorph ис- before
//!    voiceless consonants and before root еб- (colloquial form).
//!
//! 3. **Suffix+ending combinations** are looked up from a table rather than
//!    concatenated directly, because some suffix classes replace their thematic
//!    vowel in certain endings (e.g., -а- → -ё- in present tense).

/// A pre-defined suffix+ending combination string.
const SUFFIX_ENDING_TABLE: &[(&str, &str, &str)] = &[
    // suffix, ending → combined string
    ("а", "ть", "ать"),
    ("а", "л", "ал"),
    ("а", "ёт", "ёт"),
    ("ну", "ть", "нуть"),
    ("ну", "л", "нул"),
    ("ну", "нёт", "нёт"),
];

/// Combine suffix value and ending value into the actual suffix+ending string.
///
/// e.g., ("а", "ть") → "ать", ("а", "ёт") → "ёт"
fn combine_suffix_ending(suffix: &str, ending: &str) -> &'static str {
    SUFFIX_ENDING_TABLE
        .iter()
        .find(|(s, e, _)| *s == suffix && *e == ending)
        .map(|(_, _, combo)| *combo)
        .expect("unknown suffix+ending combination")
}

/// Build a word form from its morpheme components.
///
/// `prefix_form` must already be the resolved allomorph (e.g., "ис" not "из").
///
/// # Example
///
/// ```text
/// build_form("вы", "еб", "а", "ть") → "выебать"
/// build_form("от", "еб", "а", "ть") → "отъебать"
/// build_form("ис", "еб", "ну", "ть") → "исебнуть"
/// ```
pub fn build_form(prefix: &str, root: &str, suffix: &str, ending: &str) -> String {
    // Step 1: build the stem (prefix + possible ъ + root)
    let stem = build_stem(prefix, root);

    // Step 2: combine suffix and ending
    let se = combine_suffix_ending(suffix, ending);

    format!("{stem}{se}")
}

/// Build the stem: apply ъ-insertion between prefix and root.
///
/// `prefix` must already be in its resolved allomorph form.
fn build_stem(prefix: &str, root: &str) -> String {
    if prefix.is_empty() {
        return root.to_string();
    }
    if needs_hard_sign(prefix, root) {
        format!("{prefix}ъ{root}")
    } else {
        format!("{prefix}{root}")
    }
}

/// Check whether the prefix+root boundary requires a hard sign (ъ).
///
/// Русская орфография: after prefixes ending in a hard consonant (от-, под-, раз-,
/// вз-, с-, etc.) before roots starting with е, ё, ю, or я, a hard sign (ъ) is inserted.
///
/// However, prefixes that have already been resolved to a voiceless allomorph (e.g.,
/// из- → ис-) do NOT trigger ъ-insertion, even though "с" is a consonant — the
/// allomorph selection already handled the phonotactic adjustment.
fn needs_hard_sign(prefix: &str, root: &str) -> bool {
    // Prefixes that trigger ъ-insertion in standard Russian orthography.
    // As new prefixes are added, they should be listed here (notably includes
    // от-, раз-, вз-, в-, с-, под-, над-, пред-, об-).
    if !matches!(
        prefix,
        "от" | "раз" | "вз" | "в" | "с" | "под" | "над" | "пред" | "об"
    ) {
        return false;
    }
    let first_r = root.chars().next();
    matches!(first_r, Some('е' | 'ё' | 'ю' | 'я'))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_form_bare_infinitive() {
        assert_eq!(build_form("", "еб", "а", "ть"), "ебать");
    }

    #[test]
    fn test_build_form_vy_ebat() {
        assert_eq!(build_form("вы", "еб", "а", "ть"), "выебать");
    }

    #[test]
    fn test_build_form_za_ebat() {
        assert_eq!(build_form("за", "еб", "а", "ть"), "заебать");
    }

    #[test]
    fn test_build_form_with_hard_sign() {
        // от- + еб- → отъеб- (ъ-insertion)
        assert_eq!(build_form("от", "еб", "а", "ть"), "отъебать");
    }

    #[test]
    fn test_build_form_with_allomorph_iz() {
        // из- → ис- before еб-
        assert_eq!(build_form("ис", "еб", "а", "ть"), "исебать");
    }

    #[test]
    fn test_build_form_nu_suffix() {
        assert_eq!(build_form("", "еб", "ну", "ть"), "ебнуть");
    }

    #[test]
    fn test_build_form_ot_ebnut() {
        // от- + еб- → отъеб- + нуть → отъебнуть
        assert_eq!(build_form("от", "еб", "ну", "ть"), "отъебнуть");
    }

    #[test]
    fn test_build_form_vy_ebnul() {
        // Past m.sg: вы- + еб- + ну- + л
        assert_eq!(build_form("вы", "еб", "ну", "л"), "выебнул");
    }

    #[test]
    fn test_build_form_pere_ebyot() {
        // Present 3sg: пере- + еб- + -ё- + т
        assert_eq!(build_form("пере", "еб", "а", "ёт"), "переебёт");
    }

    #[test]
    fn test_build_form_bare_ebnyot() {
        // Present 3sg -ну-: еб- + н- + ёт
        assert_eq!(build_form("", "еб", "ну", "нёт"), "ебнёт");
    }

    #[test]
    fn test_build_form_pro_ebal() {
        // Past: про- + еб- + а- + л
        assert_eq!(build_form("про", "еб", "а", "л"), "проебал");
    }
}
