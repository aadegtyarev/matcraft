//! Morphological data: types, morpheme inventories, attestation tables, meaning notes.
//!
//! All attestation levels are marked as **native speaker intuition** — not corpus-grounded.
//! See the topological model design in docs/architecture.md for details.

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// How attested a form is.
///
/// All levels are marked as **native speaker intuition** — not corpus-grounded.
/// See the attestation model in docs/architecture.md for details.
///
/// - `Common` — widely used in real speech, any native speaker knows it
/// - `Rare` — attested in use but not common; some speakers may not use it
/// - `Possible` — grammatically valid by the rules of Russian, but not attested in real use
/// - `Impossible` — blocked by a phonological, morphological, or semantic constraint
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attestation {
    Common,
    Rare,
    Possible,
    Impossible,
}

/// Root data: a root plus the suffix classes it can take.
#[derive(Clone, Copy, Debug)]
pub struct RootData {
    pub name: &'static str,
    pub gloss: Option<&'static str>,
    /// Indices into the SUFFIXES table that this root can combine with.
    pub suffix_indices: &'static [usize],
}

/// A single verb form combining a prefix, root, suffix, and ending.
#[derive(Clone, Debug)]
pub struct VerbForm {
    #[allow(dead_code)]
    pub prefix_val: &'static str,
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
    pub root_gloss: Option<&'static str>,
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
}

/// All prefixes for the walking skeleton.
///
/// Index positions are load-bearing — attestation tables reference them by index.
const PREFIXES: &[PrefixEntry] = &[
    // 0: bare (no prefix)
    PrefixEntry {
        val: "",
        display: "(без)",
        allomorphs: &[],
    },
    // 1
    PrefixEntry {
        val: "вы",
        display: "вы-",
        allomorphs: &[],
    },
    // 2
    PrefixEntry {
        val: "до",
        display: "до-",
        allomorphs: &[],
    },
    // 3
    PrefixEntry {
        val: "за",
        display: "за-",
        allomorphs: &[],
    },
    // 4: из-/ис-
    PrefixEntry {
        val: "из",
        display: "из-/ис-",
        allomorphs: &["ис"],
    },
    // 5
    PrefixEntry {
        val: "на",
        display: "на-",
        allomorphs: &[],
    },
    // 6: от-/ото-
    PrefixEntry {
        val: "от",
        display: "от-/ото-",
        allomorphs: &["ото"],
    },
    // 7
    PrefixEntry {
        val: "пере",
        display: "пере-",
        allomorphs: &[],
    },
    // 8
    PrefixEntry {
        val: "про",
        display: "про-",
        allomorphs: &[],
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

/// Suffix classes for the walking skeleton.
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

// ---------------------------------------------------------------------------
// Ending data
// ---------------------------------------------------------------------------

struct EndingEntry {
    val: &'static str,
    /// Which suffix class(es) this ending applies to (indices into SUFFIXES).
    applicable_to: &'static [usize],
}

/// Endings for the walking skeleton.
const ENDINGS: &[EndingEntry] = &[
    // 0: infinitive
    EndingEntry { val: "ть", applicable_to: &[0, 1] },
    // 1: past masculine singular
    EndingEntry { val: "л", applicable_to: &[0, 1] },
    // 2: present 3sg
    EndingEntry { val: "ёт", applicable_to: &[0] },
    // 3: present 3sg (-нёт for -ну-)
    EndingEntry { val: "нёт", applicable_to: &[1] },
];

pub fn ending_val(idx: usize) -> &'static str {
    ENDINGS[idx].val
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
// Root data
// ---------------------------------------------------------------------------

/// Root data for the walking skeleton.
const ROOTS: &[RootData] = &[RootData {
    name: "еб",
    gloss: Some("fuck, copulate"),
    suffix_indices: &[0, 1], // -а- and -ну-
}];

pub fn root_data(name: &str) -> Option<&'static RootData> {
    ROOTS.iter().find(|r| r.name == name)
}

pub fn all_roots() -> &'static [RootData] {
    ROOTS
}

// ---------------------------------------------------------------------------
// Attestation + meaning notes
// ---------------------------------------------------------------------------

/// Attestation and note entries for root "еб".
///
/// Format: (prefix_idx, suffix_idx, attestation, note).
/// Any combination NOT in this table defaults to Attestation::Possible with no note.
const ROOT_EB_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // bare + -а-
    (0, 0, Attestation::Common, Some("совершать половой акт")),
    // bare + -ну-
    (0, 1, Attestation::Common, Some("ударить; сделать глупость")),
    // вы- + -а-
    (1, 0, Attestation::Common, Some("износить, испортить")),
    // вы- + -ну-
    (1, 1, Attestation::Rare, Some("выбросить, выкинуть")),
    // до- + -а-
    (2, 0, Attestation::Rare, Some("довести до оргазма")),
    // до- + -ну-
    (2, 1, Attestation::Possible, None),
    // за- + -а-
    (3, 0, Attestation::Common, Some("измучить (перен.)")),
    // за- + -ну-
    (3, 1, Attestation::Rare, Some("ударить, стукнуть")),
    // из- + -а-
    (4, 0, Attestation::Rare, Some("интенсив, избить (?)")),
    // из- + -ну-
    (4, 1, Attestation::Possible, None),
    // на- + -а-
    (5, 0, Attestation::Common, Some("обмануть")),
    // на- + -ну-
    (5, 1, Attestation::Rare, Some("надуть, обмануть")),
    // от- + -а-
    (6, 0, Attestation::Common, Some("оттрахать")),
    // от- + -ну-
    (6, 1, Attestation::Possible, None),
    // пере- + -а-
    (7, 0, Attestation::Common, Some("перетрахать(ся)")),
    // пере- + -ну-
    (7, 1, Attestation::Possible, None),
    // про- + -а-
    (8, 0, Attestation::Common, Some("упустить, просрать")),
    // про- + -ну-
    (8, 1, Attestation::Rare, Some("пробить, проломить")),
];

/// Lookup attestation and note for a given root, prefix index, and suffix index.
///
/// Returns (Attestation, Option<note>). Unlisted combinations default to
/// (Possible, None) — linguistically honest: unattested ≠ impossible.
pub fn lookup_attestation(root: &str, prefix_idx: usize, suffix_idx: usize) -> (Attestation, Option<&'static str>) {
    match root {
        "еб" => ROOT_EB_ATTEST
            .iter()
            .find(|(p, s, _, _)| *p == prefix_idx && *s == suffix_idx)
            .map(|(_, _, a, n)| (*a, *n))
            .unwrap_or((Attestation::Possible, None)),
        _ => (Attestation::Possible, None),
    }
}

// ---------------------------------------------------------------------------
// Prefix allomorph selection
// ---------------------------------------------------------------------------

/// Select the correct allomorph of a prefix before the given root.
///
/// Rules:
/// - из-/ис-: use "ис" before voiceless consonants and before root "еб" (colloquial form);
///   otherwise use "из".
/// - All other prefixes: return the primary form (ъ-insertion is handled in build_stem).
pub fn select_prefix_allomorph<'a>(prefix_val: &'a str, allomorphs: &[&str], root: &str) -> &'a str {
    if allomorphs.is_empty() {
        return prefix_val;
    }
    match prefix_val {
        "из" => {
            let first = root.chars().next().unwrap_or(' ');
            // Before root еб-, colloquial usage prefers ис-
            // Also before any voiceless consonant.
            if matches!(first, 'е' | 'ё' | 'п' | 'с' | 'т' | 'к' | 'х' | 'ц' | 'ч' | 'ш' | 'щ') {
                "ис"
            } else {
                "из"
            }
        }
        _ => prefix_val,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_count() {
        assert_eq!(prefix_count(), 9);
    }

    #[test]
    fn test_root_data_eb() {
        let rd = root_data("еб").expect("еб should be a known root");
        assert_eq!(rd.name, "еб");
        assert_eq!(rd.gloss, Some("fuck, copulate"));
    }

    #[test]
    fn test_root_data_unknown() {
        assert!(root_data("unknown").is_none());
    }

    #[test]
    fn test_all_roots_contains_eb() {
        assert!(all_roots().iter().any(|r| r.name == "еб"));
    }

    #[test]
    fn test_lookup_attestation_common() {
        // bare + -а- = common
        let (att, note) = lookup_attestation("еб", 0, 0);
        assert_eq!(att, Attestation::Common);
        assert_eq!(note, Some("совершать половой акт"));
    }

    #[test]
    fn test_lookup_attestation_default_possible() {
        // A non-existent combination defaults to Possible
        // Use a high prefix index that doesn't exist
        let (att, note) = lookup_attestation("еб", 99, 99);
        assert_eq!(att, Attestation::Possible);
        assert!(note.is_none());
    }

    #[test]
    fn test_lookup_attestation_unknown_root() {
        let (att, note) = lookup_attestation("unknown", 0, 0);
        assert_eq!(att, Attestation::Possible);
        assert!(note.is_none());
    }

    #[test]
    fn test_select_allomorph_iz_before_eb() {
        assert_eq!(select_prefix_allomorph("из", &["ис"], "еб"), "ис");
    }

    #[test]
    fn test_select_allomorph_no_allomorphs() {
        assert_eq!(select_prefix_allomorph("вы", &[], "еб"), "вы");
    }

    #[test]
    fn test_endings_for_suffix_a() {
        let endings = endings_for_suffix(0);
        // -а- suffix has 3 endings: ть, л, ёт
        assert!(endings.contains(&0)); // ть
        assert!(endings.contains(&1)); // л
        assert!(endings.contains(&2)); // ёт
        assert_eq!(endings.len(), 3);
    }

    #[test]
    fn test_endings_for_suffix_nu() {
        let endings = endings_for_suffix(1);
        // -ну- suffix has 3 endings: ть, л, нёт
        assert!(endings.contains(&0)); // ть
        assert!(endings.contains(&1)); // л
        assert!(endings.contains(&3)); // нёт
        assert_eq!(endings.len(), 3);
    }
}
