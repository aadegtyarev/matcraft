//! Morphological types and morpheme inventories: prefixes, suffixes, endings,
//! and prefix-allomorph selection.
//!
//! Root inventory and attestation tables live in `roots/` (one home per root).

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// How attested a form is, grounded on the Plutser-Sarno synthesis.
///
/// The three source marks (common / rare / possible) and their mapping onto
/// this 4-level enum are documented in one home:
/// `docs/decisions/plutser-sarno-taxonomy.md`.
///
/// - `Common` — widely attested in the source's corpus work.
/// - `Rare` — occasional, dialectal, or a single attestation.
/// - `Possible` — word-formation-plausible by analogy, not attested in the source.
/// - `Impossible` — blocked by a phonological, morphological, or semantic constraint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attestation {
    Common,
    Rare,
    Possible,
    Impossible,
}

/// Semantic domain per Plutser-Sarno (source §1).
///
/// See `docs/decisions/plutser-sarno-taxonomy.md` for the full typology and counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Domain {
    /// Nuclear mat core (7 roots).
    Nuclear,
    /// Excretory domain (7 roots) — an autonomous taboo system.
    Excretory,
    /// Periphery: sexual and nominal roots (21 roots).
    Peripheral,
}

/// Productivity class per Plutser-Sarno (source §2), from A (highest) to E (minimal).
///
/// Ordering follows declaration order, so `A < B < C < D < E`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProductivityClass {
    A,
    B,
    C,
    D,
    E,
}

impl std::fmt::Display for ProductivityClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ProductivityClass::A => "A",
            ProductivityClass::B => "B",
            ProductivityClass::C => "C",
            ProductivityClass::D => "D",
            ProductivityClass::E => "E",
        };
        write!(f, "{s}")
    }
}

/// Root display mode for list-roots.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum Mode {
    /// Classic mode: show the 9 backward-compatible core roots.
    #[default]
    Classic,
    /// Full mode: show all 35 roots.
    Full,
}

impl Mode {
    /// Whether this mode includes a given root.
    ///
    /// Classic = the whole nuclear domain plus the most productive excretory
    /// roots (productivity ≤ B) — exactly the 9 roots kept for backward
    /// compatibility (ядро 7 + ср-, сс-).
    pub fn includes(&self, rd: &RootData) -> bool {
        match self {
            Mode::Classic => {
                rd.domain == Domain::Nuclear
                    || (rd.domain == Domain::Excretory && rd.productivity <= ProductivityClass::B)
            }
            Mode::Full => true,
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Classic => write!(f, "classic"),
            Mode::Full => write!(f, "full"),
        }
    }
}

/// Root data: a root plus its semantic domain, productivity, and verb classes.
#[derive(Clone, Copy, Debug)]
pub struct RootData {
    pub name: &'static str,
    /// Value used in form construction (e.g., "ср" for root "сра").
    /// Usually the same as `name`, but may differ for roots where the
    /// surface form includes a thematic vowel.
    pub val: &'static str,
    /// English gloss — a dev-facing reference only, deliberately never shown in
    /// user output (the tool is Russian); the displayed meaning is `gloss_ru`.
    /// Read only by tests, so it is dead in a release build by design.
    #[allow(dead_code)]
    pub gloss: Option<&'static str>,
    /// Russian gloss (1–4 words) — the meaning shown in every command's output.
    /// Faithful to `gloss` and the source taxonomy, never invented. `None` means
    /// no clear Russian gloss; the form block then prints `корень <name>-` bare.
    pub gloss_ru: Option<&'static str>,
    /// Indices into the SUFFIXES table that this root can combine with.
    pub suffix_indices: &'static [usize],
    /// Semantic domain (source §1).
    pub domain: Domain,
    /// Productivity class (source §2).
    pub productivity: ProductivityClass,
    /// Present-tense stem for roots with an irregular present stem
    /// (e.g., "блю" for root "блев"). None for most roots.
    pub present_stem: Option<&'static str>,
    /// Whether this root triggers the fluid vowel -о- (беглая гласная) after a
    /// yer-final prefix (изо-, со-, разо-…). A lexically conditioned morphonological
    /// property (historically weak-yer roots), NOT derivable from the surface
    /// cluster: `true` only for сра-, сса-, жр-. See docs/decisions/cluster-fill-vowel.md.
    pub takes_fill_vowel: bool,
    /// Whether this root selects the об- allomorph of the о-/об- prefix even before
    /// a consonant (обговнить, not оговнить). A lexical override of the general
    /// simplification (об- only before a vowel): `true` only for говн-, whose real
    /// derivative is обговнять/обговнить. Distinct from the fluid-vowel обо- path
    /// (говн- inserts no vowel — г is a single consonant). See
    /// docs/decisions/o-ob-allomorphy.md.
    pub o_takes_ob: bool,
    /// Linguistic note (2-4 sentences in Russian) for the random subcommand.
    pub linguistic_note: &'static str,
}

/// A single verb form combining a prefix, root, suffix, and ending.
#[derive(Clone, Debug)]
pub struct VerbForm {
    pub prefix_display: &'static str,
    pub suffix_val: &'static str,
    pub ending_val: &'static str,
    /// The fully constructed word (e.g., "выебать").
    pub form: String,
    pub attestation: Attestation,
    pub note: Option<&'static str>,
}

/// Result of exploring a root's paradigm.
#[derive(Clone, Debug)]
pub struct ParadigmResult {
    pub root_name: &'static str,
    /// Russian gloss for display (the English `RootData.gloss` is dev-facing and
    /// never shown, so it is not carried on the result).
    pub root_gloss_ru: Option<&'static str>,
    pub root_domain: Domain,
    pub root_productivity: ProductivityClass,
    /// The `--suffix` filter that produced this result, if any. Drives the
    /// display: a filtered (narrow) result gets per-form breakdown blocks; the
    /// full paradigm gets the scannable overview table only.
    pub suffix_filter: Option<String>,
    pub forms: Vec<VerbForm>,
}

/// Error type for explore().
#[derive(Clone, Debug)]
pub enum ExploreError {
    RootNotFound {
        root: String,
        available: Vec<&'static str>,
    },
}

impl std::fmt::Display for ExploreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExploreError::RootNotFound { root, available } => {
                write!(
                    f,
                    "Корень '{}' не найден. Доступные корни: {}. Используйте `matcraft list-roots` для полного списка.",
                    root,
                    available.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for ExploreError {}

// ---------------------------------------------------------------------------
// Prefix data
// ---------------------------------------------------------------------------

/// A prefix with display name and allomorphs.
struct PrefixEntry {
    /// Value used in form construction (e.g., "из").
    val: &'static str,
    /// Display name in tables (e.g., "из-/ис-", or "(без)" for bare).
    display: &'static str,
    allomorphs: &'static [&'static str],
    /// The fluid-vowel (беглая гласная -о-) allomorph of this yer-final prefix,
    /// used before a `takes_fill_vowel` root (сра-, сса-, жр-). Carries the VOICED
    /// base — the fluid -о- cancels devoicing (изо-, not исо-; взо-, not всо-;
    /// разо-, not рас+о — source: изостлать vs исстелю). `None` for prefixes that
    /// never take the fluid vowel (prefixes ending in a vowel, and bare).
    /// See docs/decisions/cluster-fill-vowel.md.
    fill_form: Option<&'static str>,
}

/// All prefixes in the inventory.
///
/// Index positions are load-bearing — attestation tables reference them by index.
const PREFIXES: &[PrefixEntry] = &[
    // 0: bare (no prefix)
    PrefixEntry {
        val: "",
        display: "(без)",
        allomorphs: &[],
        fill_form: None,
    },
    PrefixEntry {
        val: "вы",
        display: "вы-",
        allomorphs: &[],
        fill_form: None,
    },
    PrefixEntry {
        val: "до",
        display: "до-",
        allomorphs: &[],
        fill_form: None,
    },
    PrefixEntry {
        val: "за",
        display: "за-",
        allomorphs: &[],
        fill_form: None,
    },
    // 4: из-/ис-/изо-
    PrefixEntry {
        val: "из",
        display: "из-/ис-",
        allomorphs: &["ис"],
        fill_form: Some("изо"),
    },
    PrefixEntry {
        val: "на",
        display: "на-",
        allomorphs: &[],
        fill_form: None,
    },
    // 6: от-/ото-
    PrefixEntry {
        val: "от",
        display: "от-/ото-",
        allomorphs: &["ото"],
        fill_form: Some("ото"),
    },
    PrefixEntry {
        val: "пере",
        display: "пере-",
        allomorphs: &[],
        fill_form: None,
    },
    PrefixEntry {
        val: "про",
        display: "про-",
        allomorphs: &[],
        fill_form: None,
    },
    // 9: в-/во-
    PrefixEntry {
        val: "в",
        display: "в-",
        allomorphs: &[],
        fill_form: Some("во"),
    },
    // 10: вз-/вс-/взо-
    PrefixEntry {
        val: "вз",
        display: "вз-/вс-",
        allomorphs: &["вс"],
        fill_form: Some("взо"),
    },
    // 11: о-/об-/обо-
    PrefixEntry {
        val: "о",
        display: "о-/об-",
        allomorphs: &["об"],
        fill_form: Some("обо"),
    },
    PrefixEntry {
        val: "по",
        display: "по-",
        allomorphs: &[],
        fill_form: None,
    },
    // 13: под-/подо-
    PrefixEntry {
        val: "под",
        display: "под-",
        allomorphs: &[],
        fill_form: Some("подо"),
    },
    PrefixEntry {
        val: "при",
        display: "при-",
        allomorphs: &[],
        fill_form: None,
    },
    // 15: раз-/рас-/разо-
    PrefixEntry {
        val: "раз",
        display: "раз-/рас-",
        allomorphs: &["рас"],
        fill_form: Some("разо"),
    },
    // 16: с-/со-
    PrefixEntry {
        val: "с",
        display: "с-",
        allomorphs: &[],
        fill_form: Some("со"),
    },
    PrefixEntry {
        val: "у",
        display: "у-",
        allomorphs: &[],
        fill_form: None,
    },
];

pub fn prefix_val(idx: usize) -> &'static str {
    PREFIXES[idx].val
}

pub fn prefix_display(idx: usize) -> &'static str {
    PREFIXES[idx].display
}

pub fn prefix_allomorphs(idx: usize) -> &'static [&'static str] {
    PREFIXES[idx].allomorphs
}

/// The fluid-vowel (беглая -о-) form of a prefix, if it is yer-final.
///
/// `Some("изо")`, `Some("со")`, … for the 8 yer-final prefixes; `None` otherwise.
/// Consulted only when the root `takes_fill_vowel` (сра-/сса-/жр-); see
/// `docs/architecture.md` §Алломорфия.
pub fn prefix_fill_form(idx: usize) -> Option<&'static str> {
    PREFIXES[idx].fill_form
}

pub fn prefix_count() -> usize {
    PREFIXES.len()
}

// ---------------------------------------------------------------------------
// Suffix data
// ---------------------------------------------------------------------------

struct SuffixEntry {
    val: &'static str,
    display: &'static str,
    gloss: &'static str,
}

/// Suffix classes in the inventory.
const SUFFIXES: &[SuffixEntry] = &[
    // 0
    SuffixEntry {
        val: "а",
        display: "-а-",
        gloss: "имперфектив",
    },
    // 1
    SuffixEntry {
        val: "ну",
        display: "-ну-",
        gloss: "однократный",
    },
    // 2
    SuffixEntry {
        val: "е",
        display: "-е-",
        gloss: "II спряжение (-е- основа)",
    },
    // 3
    SuffixEntry {
        val: "и",
        display: "-и-",
        gloss: "II спряжение (-и- основа)",
    },
];

pub fn suffix_val(idx: usize) -> &'static str {
    SUFFIXES[idx].val
}

pub fn suffix_display(idx: usize) -> &'static str {
    SUFFIXES[idx].display
}

pub fn suffix_gloss(idx: usize) -> &'static str {
    SUFFIXES[idx].gloss
}

/// Reverse-lookup a suffix index from its value string.
///
/// Derived from SUFFIXES so it can never drift from the table. Panics on an
/// unknown value — a programmer error, since the only callers pass a value that
/// came out of SUFFIXES in the first place.
pub fn suffix_index_for_val(val: &str) -> usize {
    SUFFIXES
        .iter()
        .position(|s| s.val == val)
        .expect("unknown suffix val")
}

// ---------------------------------------------------------------------------
// Ending data
// ---------------------------------------------------------------------------

struct EndingEntry {
    val: &'static str,
    /// Display form of the ending (e.g. "-ть", "-ёт").
    display: &'static str,
    /// Human-readable grammatical label (Russian) for the form block.
    label: &'static str,
    /// Which suffix class(es) this ending applies to (indices into SUFFIXES).
    applicable_to: &'static [usize],
}

/// Verb endings by class.
const ENDINGS: &[EndingEntry] = &[
    // 0: infinitive
    EndingEntry {
        val: "ть",
        display: "-ть",
        label: "инфинитив -ть",
        applicable_to: &[0, 1, 2, 3],
    },
    // 1: past masculine singular
    EndingEntry {
        val: "л",
        display: "-л",
        label: "прош. время, м. р. ед. ч. -л",
        applicable_to: &[0, 1, 2, 3],
    },
    // 2: present 3sg
    EndingEntry {
        val: "ёт",
        display: "-ёт",
        label: "наст. время, 3 л. ед. ч. -ёт",
        applicable_to: &[0],
    },
    // 3: present 3sg (-нёт for -ну-)
    EndingEntry {
        val: "нёт",
        display: "-нёт",
        label: "наст. время, 3 л. ед. ч. -нёт",
        applicable_to: &[1],
    },
    // 4: present 3sg (-ит for -е- and -и-)
    EndingEntry {
        val: "ит",
        display: "-ит",
        label: "наст. время, 3 л. ед. ч. -ит",
        applicable_to: &[2, 3],
    },
];

pub fn ending_val(idx: usize) -> &'static str {
    ENDINGS[idx].val
}

pub fn ending_display(idx: usize) -> &'static str {
    ENDINGS[idx].display
}

pub fn ending_label(idx: usize) -> &'static str {
    ENDINGS[idx].label
}

/// Reverse-lookup an ending index from its value string.
///
/// Derived from ENDINGS so it can never drift from the table (mirror of
/// `suffix_index_for_val`; needed because `VerbForm` stores `ending_val`, not an
/// index). Panics on an unknown value — a programmer error, since callers pass a
/// value that came out of ENDINGS in the first place.
pub fn ending_by_val(val: &str) -> usize {
    ENDINGS
        .iter()
        .position(|e| e.val == val)
        .expect("unknown ending val")
}

/// Returns all ending indices that apply to a given suffix index.
pub fn endings_for_suffix(suffix_idx: usize) -> Vec<usize> {
    ENDINGS
        .iter()
        .enumerate()
        .filter(|(_, e)| e.applicable_to.contains(&suffix_idx))
        .map(|(i, _)| i)
        .collect()
}

// ---------------------------------------------------------------------------
// Prefix allomorph selection
// ---------------------------------------------------------------------------

/// Select the correct allomorph of a prefix before the given root.
///
/// Rules:
/// - из-/ис-: use "ис" before voiceless consonants and before root "еб" (colloquial form);
///   otherwise use "из".
/// - вз-/вс-: use "вс" before voiceless consonants; otherwise "вз".
/// - раз-/рас-: use "рас" before voiceless consonants; otherwise "раз".
/// - о-/об-: use "об" before vowels; otherwise "о" — UNLESS the root sets
///   `o_takes_ob` (lexical override, говн- → обговнить), in which case "об" is
///   used before the consonant too.
/// - All other prefixes: return the primary form (ъ-insertion is handled in build_stem).
pub fn select_prefix_allomorph<'a>(
    prefix_val: &'a str,
    allomorphs: &[&str],
    root: &str,
    o_takes_ob: bool,
) -> &'a str {
    if allomorphs.is_empty() {
        return prefix_val;
    }
    let first = root.chars().next().unwrap_or(' ');
    // Voiceless consonants in Russian
    let is_voiceless = matches!(first, 'п' | 'с' | 'т' | 'к' | 'х' | 'ц' | 'ч' | 'ш' | 'щ');
    let is_vowel = matches!(
        first,
        'а' | 'е' | 'ё' | 'и' | 'о' | 'у' | 'ы' | 'э' | 'ю' | 'я'
    );
    match prefix_val {
        "из" | "вз" | "раз" => {
            let voiceless_form = if prefix_val == "из" {
                "ис"
            } else if prefix_val == "вз" {
                "вс"
            } else {
                "рас"
            };
            // из-/ис- uses ис- before any е/ё-starting root (effectively only еб- in the current inventory)
            if prefix_val == "из" && matches!(first, 'е' | 'ё') {
                return "ис";
            }
            if is_voiceless {
                voiceless_form
            } else {
                prefix_val
            }
        }
        "о" => {
            if is_vowel || o_takes_ob {
                "об"
            } else {
                prefix_val
            }
        }
        _ => prefix_val,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[path = "morpheme_tests.rs"]
mod tests;
