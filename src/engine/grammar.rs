//! Form construction: combine morphemes into full word forms.
//!
//! Allomorphy rules are documented in one home: docs/architecture.md §Алломорфия.
//! The only rule implemented entirely in this file is ъ-insertion (see
//! `needs_hard_sign`); suffix+ending combinations are looked up from
//! SUFFIX_ENDING_TABLE because some suffix classes replace their thematic vowel
//! in certain endings (e.g., -а- → -ё- in the present tense).

/// A pre-defined suffix+ending combination string.
const SUFFIX_ENDING_TABLE: &[(&str, &str, &str)] = &[
    // suffix 0: -а- class
    ("а", "ть", "ать"),
    ("а", "л", "ал"),
    ("а", "ёт", "ёт"),
    // suffix 1: -ну- class
    ("ну", "ть", "нуть"),
    ("ну", "л", "нул"),
    ("ну", "нёт", "нёт"),
    // suffix 2: -е- class (е → и in present 3sg)
    ("е", "ть", "еть"),
    ("е", "л", "ел"),
    ("е", "ит", "ит"),
    // suffix 3: -и- class
    ("и", "ть", "ить"),
    ("и", "л", "ил"),
    ("и", "ит", "ит"),
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

/// Check whether the ending is a present-tense ending (used for present-stem allomorphy).
fn is_present_ending(ending: &str) -> bool {
    matches!(ending, "ёт" | "нёт" | "ит")
}

/// Build a word form from its morpheme components.
///
/// `prefix_form` must already be the resolved allomorph (e.g., "ис" not "из").
/// If `present_stem` is `Some` and the ending is present tense, the present stem
/// is used instead of the root val (suppletive allomorphy like блев → блю).
///
/// # Example
///
/// ```text
/// build_form("вы", "еб", "а", "ть", None) → "выебать"
/// build_form("от", "еб", "а", "ть", None) → "отъебать"
/// build_form("ис", "еб", "ну", "ть", None) → "исебнуть"
/// build_form("", "блев", "а", "ть", Some("блю")) → "блевать"
/// build_form("", "блю", "а", "ёт", Some("блю")) → "блюёт"
/// ```
pub fn build_form(
    prefix: &str,
    root: &str,
    suffix: &str,
    ending: &str,
    present_stem: Option<&str>,
) -> String {
    // Step 1: determine which stem to use (regular vs present-stem allomorph)
    let actual_root = present_stem
        .filter(|_| is_present_ending(ending))
        .unwrap_or(root);

    // Step 2: build the stem (prefix + possible ъ + root)
    let stem = build_stem(prefix, actual_root);

    // Step 3: combine suffix and ending
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
/// Русская орфография: after a prefix ending in a hard consonant — the full set is
/// от, раз, вз, в, с, под, над, пред, об (see the match below) — before a root
/// starting with е, ё, ю, or я, a hard sign (ъ) is inserted.
///
/// However, prefixes that have already been resolved to a voiceless allomorph (e.g.,
/// из- → ис-) do NOT trigger ъ-insertion, even though "с" is a consonant — the
/// allomorph selection already handled the phonotactic adjustment.
fn needs_hard_sign(prefix: &str, root: &str) -> bool {
    // Prefixes that trigger ъ-insertion in standard Russian orthography.
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

    // -а- class tests (existing)

    #[test]
    fn test_build_form_bare_infinitive() {
        assert_eq!(build_form("", "еб", "а", "ть", None), "ебать");
    }

    #[test]
    fn test_build_form_vy_ebat() {
        assert_eq!(build_form("вы", "еб", "а", "ть", None), "выебать");
    }

    #[test]
    fn test_build_form_za_ebat() {
        assert_eq!(build_form("за", "еб", "а", "ть", None), "заебать");
    }

    #[test]
    fn test_build_form_with_hard_sign() {
        // от- + еб- → отъеб- (ъ-insertion)
        assert_eq!(build_form("от", "еб", "а", "ть", None), "отъебать");
    }

    #[test]
    fn test_build_form_with_allomorph_iz() {
        // из- → ис- before еб-
        assert_eq!(build_form("ис", "еб", "а", "ть", None), "исебать");
    }

    #[test]
    fn test_build_form_nu_suffix() {
        assert_eq!(build_form("", "еб", "ну", "ть", None), "ебнуть");
    }

    #[test]
    fn test_build_form_ot_ebnut() {
        assert_eq!(build_form("от", "еб", "ну", "ть", None), "отъебнуть");
    }

    #[test]
    fn test_build_form_vy_ebnul() {
        assert_eq!(build_form("вы", "еб", "ну", "л", None), "выебнул");
    }

    #[test]
    fn test_build_form_pere_ebyot() {
        assert_eq!(build_form("пере", "еб", "а", "ёт", None), "переебёт");
    }

    #[test]
    fn test_build_form_bare_ebnyot() {
        assert_eq!(build_form("", "еб", "ну", "нёт", None), "ебнёт");
    }

    #[test]
    fn test_build_form_pro_ebal() {
        assert_eq!(build_form("про", "еб", "а", "л", None), "проебал");
    }

    // -е- class tests (suffix 2)

    #[test]
    fn test_build_form_pizdet() {
        assert_eq!(build_form("", "пизд", "е", "ть", None), "пиздеть");
    }

    #[test]
    fn test_build_form_zapizdel() {
        assert_eq!(build_form("за", "пизд", "е", "л", None), "запиздел");
    }

    #[test]
    fn test_build_form_pizdit() {
        assert_eq!(build_form("", "пизд", "е", "ит", None), "пиздит");
    }

    #[test]
    fn test_build_form_propizdit() {
        assert_eq!(build_form("про", "пизд", "е", "ит", None), "пропиздит");
    }

    #[test]
    fn test_build_form_blyadet() {
        assert_eq!(build_form("", "бляд", "е", "ть", None), "блядеть");
    }

    // -и- class tests (suffix 3, new)

    #[test]
    fn test_build_form_drochit() {
        // дроч- + и- + ть → дрочить
        assert_eq!(build_form("", "дроч", "и", "ть", None), "дрочить");
    }

    #[test]
    fn test_build_form_zadrochil() {
        // за- + дроч- + и- + л → задрочил
        assert_eq!(build_form("за", "дроч", "и", "л", None), "задрочил");
    }

    #[test]
    fn test_build_form_vydrochit() {
        // вы- + дроч- + и- + ит → выдрочит
        assert_eq!(build_form("вы", "дроч", "и", "ит", None), "выдрочит");
    }

    #[test]
    fn test_build_form_mudit() {
        // муд- + и- + ть → мудить
        assert_eq!(build_form("", "муд", "и", "ть", None), "мудить");
    }

    #[test]
    fn test_build_form_zamudil() {
        // за- + муд- + и- + л → замудил
        assert_eq!(build_form("за", "муд", "и", "л", None), "замудил");
    }

    // Present-stem allomorphy tests (блев → блю)

    #[test]
    fn test_build_form_blevat() {
        // блев- + а- + ть (infinitive, regular stem) → блевать
        assert_eq!(build_form("", "блев", "а", "ть", Some("блю")), "блевать");
    }

    #[test]
    fn test_build_form_bleval() {
        // блев- + а- + л (past, regular stem) → блевал
        assert_eq!(build_form("", "блев", "а", "л", Some("блю")), "блевал");
    }

    #[test]
    fn test_build_form_blyuet() {
        // блев- + а- + ёт (present): the engine must select present_stem блю-
        // over the dictionary stem блев-. Passing root="блев" (not "блю") is what
        // actually exercises that selection.
        assert_eq!(build_form("", "блев", "а", "ёт", Some("блю")), "блюёт");
    }

    #[test]
    fn test_build_form_zablevat() {
        // за- + блев- + а- + ть (infinitive) → заблевать
        assert_eq!(
            build_form("за", "блев", "а", "ть", Some("блю")),
            "заблевать"
        );
    }

    #[test]
    fn test_build_form_zabliuet() {
        // за- + блев- + а- + ёт (present): present_stem selected under a prefix.
        assert_eq!(build_form("за", "блев", "а", "ёт", Some("блю")), "заблюёт");
    }

    // Hard sign tests with new prefixes

    #[test]
    fn test_build_form_ob_ebat() {
        // об- + еб- + а- + ть → объебать (ъ before е)
        assert_eq!(build_form("об", "еб", "а", "ть", None), "объебать");
    }

    #[test]
    fn test_build_form_o_pizdit() {
        // о- (from об-) + пизд- + и- + ть → опиздить
        assert_eq!(build_form("о", "пизд", "и", "ть", None), "опиздить");
    }

    #[test]
    fn test_build_form_raz_ebat() {
        // раз- + еб- + а- + ть → разъебать
        assert_eq!(build_form("раз", "еб", "а", "ть", None), "разъебать");
    }

    #[test]
    fn test_build_form_s_ebat() {
        // с- + еб- + а- + ть → съебать
        assert_eq!(build_form("с", "еб", "а", "ть", None), "съебать");
    }

    // Assert existing forms remain unchanged

    #[test]
    fn test_build_form_srat() {
        assert_eq!(build_form("", "ср", "а", "ть", None), "срать");
    }

    #[test]
    fn test_build_form_ssat() {
        assert_eq!(build_form("", "сс", "а", "ть", None), "ссать");
    }

    #[test]
    fn test_build_form_khuyanut() {
        assert_eq!(build_form("", "хуй", "ну", "ть", None), "хуйнуть");
    }

    #[test]
    fn test_build_form_zakhuyanut() {
        assert_eq!(build_form("за", "хуй", "ну", "ть", None), "захуйнуть");
    }
}
