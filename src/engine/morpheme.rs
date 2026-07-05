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
    /// Value used in form construction (e.g., "ср" for root "сра").
    /// Usually the same as `name`, but may differ for roots where the
    /// surface form includes a thematic vowel.
    pub val: &'static str,
    pub gloss: Option<&'static str>,
    /// Indices into the SUFFIXES table that this root can combine with.
    pub suffix_indices: &'static [usize],
    /// Linguistic note (2-4 sentences in Russian) for the random subcommand.
    pub linguistic_note: &'static str,
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
    // 2
    SuffixEntry {
        val: "е",
        display: "-е-/-и-",
        gloss: "II спряжения",
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
    EndingEntry {
        val: "ть",
        applicable_to: &[0, 1, 2],
    },
    // 1: past masculine singular
    EndingEntry {
        val: "л",
        applicable_to: &[0, 1, 2],
    },
    // 2: present 3sg
    EndingEntry {
        val: "ёт",
        applicable_to: &[0],
    },
    // 3: present 3sg (-нёт for -ну-)
    EndingEntry {
        val: "нёт",
        applicable_to: &[1],
    },
    // 4: present 3sg (-ит for -е-/-и-)
    EndingEntry {
        val: "ит",
        applicable_to: &[2],
    },
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
///
/// NS intuition, 2026-07 — Attestation levels are native-speaker intuition, not corpus-grounded.
const ROOTS: &[RootData] = &[
    RootData {
        name: "еб",
        val: "еб",
        gloss: Some("fuck, copulate"),
        suffix_indices: &[0, 1], // -а- and -ну-
        linguistic_note: "Самый продуктивный матерный корень. От праславянского *jebati. \
            В современном русском образует более 100 глагольных и именных дериватов. \
            Едва ли не каждое действие можно описать глаголом на еб-.",
    },
    RootData {
        name: "сра",
        val: "ср",
        gloss: Some("shit, excrete"),
        suffix_indices: &[0, 1], // -а- and -ну-
        linguistic_note: "Скатологический корень. По классификации Плуцера-Сарно, \
            относится к отдельному домену (не классический мат), но по продуктивности \
            не уступает корню еб-. В современном русском образует десятки глагольных форм.",
    },
    RootData {
        name: "сса",
        val: "сс",
        gloss: Some("piss, urinate"),
        suffix_indices: &[0, 1], // -а- and -ну-
        linguistic_note: "Скатологический корень 'мочиться'. Менее продуктивен, чем сра-, \
            но образует ряд ярких метафор: зассать ('испугаться'), \
            обоссать ('раскритиковать').",
    },
    RootData {
        name: "пизд",
        val: "пизд",
        gloss: Some("female genitals (cunt)"),
        suffix_indices: &[2, 1], // -е-/-и- (пиздеть) and -ну- (пиздануть)
        linguistic_note: "Женский корень мата. Образует два семантических ряда: \
            'говорить ерунду' (пиздеть, класс -е-) и 'бить/красть' (пиздить, класс -и-). \
            Один из немногих корней с двумя продуктивными глагольными классами.",
    },
    RootData {
        name: "хуй",
        val: "хуй",
        gloss: Some("penis (dick)"),
        suffix_indices: &[1], // -ну- only; хуярить (class -и-) deferred
        linguistic_note: "Ключевой именной корень русского мата. Этимология спорна: \
            от праиндоевропейского *ks-u- (хвоя), монгольского khui, или латинского huic. \
            Первый том словаря Плуцера-Сарно (500+ стр.) посвящён исключительно этому корню.",
    },
    RootData {
        name: "бляд",
        val: "бляд",
        gloss: Some("whore, prostitute"),
        suffix_indices: &[2], // -е-/-и- (блядеть); -ова- deferred
        linguistic_note: "От древнерусского блѧдь — 'обман, ерунда, прелюбодейка'. \
            Преимущественно именной корень (блядь, блядский, блядство). \
            Глагольные формы (блядовать) используют суффикс -ова-, не включённый \
            в текущую версию движка.",
    },
    RootData {
        name: "муд",
        val: "муд",
        gloss: Some("testicles"),
        suffix_indices: &[0], // -а- class (approximate; actual forms use -и-)
        linguistic_note: "Корень со значением 'testiculi'. В современном русском \
            глагольные формы (мудить) означают 'медлить, заниматься ерундой'. \
            Преимущественно именной: мудак, мудило.",
    },
    RootData {
        name: "манд",
        val: "манд",
        gloss: Some("female genitals (archaic)"),
        suffix_indices: &[], // verb forms minimal; noun root
        linguistic_note: "Архаичный корень со значением 'женские гениталии'. \
            В XIX веке — одно из сильнейших ругательств. К XXI веку практически \
            утратил обсценную силу. Глагольные формы (мандить) крайне редки.",
    },
    RootData {
        name: "елд",
        val: "елд",
        gloss: Some("penis (archaic)"),
        suffix_indices: &[], // verb forms minimal; noun root
        linguistic_note: "Архаичный корень со значением 'мужской член'. \
            Как и манд-, практически утратил обсценную силу в современном языке. \
            Глагольные формы (елдить) маргинальны.",
    },
];

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

// ---------------------------------------------------------------------------
// Attestation: root сра-
// ---------------------------------------------------------------------------

const ROOT_SRA_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // -а- class
    (0, 0, Attestation::Common, Some("испражняться")),
    (1, 0, Attestation::Common, Some("извергнуть")),
    (2, 0, Attestation::Possible, None),
    (3, 0, Attestation::Common, Some("загрязнить, испортить")),
    (4, 0, Attestation::Possible, None),
    (5, 0, Attestation::Common, Some("причинить неприятности")),
    (6, 0, Attestation::Rare, Some("выбраниться; отделаться")),
    (7, 0, Attestation::Rare, Some("переполнить, перенервничать")),
    (8, 0, Attestation::Common, Some("упустить, потерять")),
    // -ну- class
    (0, 1, Attestation::Rare, Some("однократно испражниться")),
    (3, 1, Attestation::Possible, None),
    (5, 1, Attestation::Possible, None),
];

// ---------------------------------------------------------------------------
// Attestation: root сса-
// ---------------------------------------------------------------------------

const ROOT_SSA_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // -а- class
    (0, 0, Attestation::Common, Some("мочиться")),
    (1, 0, Attestation::Possible, None),
    (2, 0, Attestation::Possible, None),
    (3, 0, Attestation::Common, Some("испугаться")),
    (4, 0, Attestation::Possible, None),
    (5, 0, Attestation::Common, Some("наполнить мочой")),
    (6, 0, Attestation::Rare, Some("отделаться страхом")),
    (7, 0, Attestation::Possible, None),
    (8, 0, Attestation::Common, Some("опоздать, упустить")),
    // -ну- class
    (0, 1, Attestation::Rare, Some("помочиться однократно")),
];

// ---------------------------------------------------------------------------
// Attestation: root пизд-
// ---------------------------------------------------------------------------

const ROOT_PIZD_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // -е-/-и- class (пиздеть type)
    (0, 2, Attestation::Common, Some("говорить ерунду")),
    (2, 2, Attestation::Rare, Some("договориться до абсурда")),
    (3, 2, Attestation::Common, Some("начать врать")),
    (4, 2, Attestation::Possible, None),
    (8, 2, Attestation::Common, Some("проболтаться; пропустить")),
    // -ну- class (пиздануть)
    (0, 1, Attestation::Common, Some("ударить; соврать")),
    (3, 1, Attestation::Rare, Some("ударить с силой")),
    (5, 1, Attestation::Possible, None),
    (8, 1, Attestation::Rare, Some("пробить насквозь")),
];

// ---------------------------------------------------------------------------
// Attestation: root хуй-
// ---------------------------------------------------------------------------

const ROOT_KHUY_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // -ну- class
    (0, 1, Attestation::Common, Some("ударить")),
    (1, 1, Attestation::Possible, None),
    (2, 1, Attestation::Possible, None),
    (3, 1, Attestation::Common, Some("забить, пренебречь")),
    (4, 1, Attestation::Possible, None),
    (5, 1, Attestation::Rare, Some("навредить")),
    (6, 1, Attestation::Possible, None),
    (7, 1, Attestation::Possible, None),
    (8, 1, Attestation::Rare, Some("промахнуться")),
];

// ---------------------------------------------------------------------------
// Attestation: root бляд-
// ---------------------------------------------------------------------------

const ROOT_BLYAD_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // -е-/-и- class
    (
        0,
        2,
        Attestation::Rare,
        Some("говорить 'блядь' как слово-паразит"),
    ),
    (3, 2, Attestation::Rare, Some("начать материться")),
    (5, 2, Attestation::Possible, None),
    (
        8,
        2,
        Attestation::Common,
        Some("провести время в распутстве; потратить зря"),
    ),
];

// ---------------------------------------------------------------------------
// Attestation: root муд-
// ---------------------------------------------------------------------------

const ROOT_MUD_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // -а- class (approximate; actual forms use -и- theme: мудить, намудить, etc.)
    (0, 0, Attestation::Common, Some("медлить, делать ерунду")),
    (1, 0, Attestation::Possible, None),
    (2, 0, Attestation::Possible, None),
    (3, 0, Attestation::Rare, Some("задержать, затянуть")),
    (4, 0, Attestation::Possible, None),
    (5, 0, Attestation::Common, Some("наделать глупостей")),
    (6, 0, Attestation::Rare, Some("отделаться")),
    (7, 0, Attestation::Rare, Some("перестараться")),
    (8, 0, Attestation::Possible, None),
];

// ---------------------------------------------------------------------------
// Attestation: root манд-
// ---------------------------------------------------------------------------

const ROOT_MAND_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // Verb forms essentially nonexistent; preserved for completeness
    (
        0,
        0,
        Attestation::Rare,
        Some("совершать половой акт (арх.)"),
    ),
];

// ---------------------------------------------------------------------------
// Attestation: root елд-
// ---------------------------------------------------------------------------

const ROOT_ELD_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // Verb forms essentially nonexistent; preserved for completeness
    (0, 0, Attestation::Rare, Some("заниматься ерундой (арх.)")),
];

/// Lookup attestation and note for a given root, prefix index, and suffix index.
///
/// Returns `(Attestation, Option<note>)`. Unlisted combinations default to
/// (Possible, None) — linguistically honest: unattested ≠ impossible.
///
/// NS intuition, 2026-07 — All levels are marked as native-speaker intuition,
/// not corpus-grounded. See docs/architecture.md for the attestation model.
pub fn lookup_attestation(
    root: &str,
    prefix_idx: usize,
    suffix_idx: usize,
) -> (Attestation, Option<&'static str>) {
    match root {
        "еб" => ROOT_EB_ATTEST
            .iter()
            .find(|(p, s, _, _)| *p == prefix_idx && *s == suffix_idx)
            .map(|(_, _, a, n)| (*a, *n))
            .unwrap_or((Attestation::Possible, None)),
        "сра" => ROOT_SRA_ATTEST
            .iter()
            .find(|(p, s, _, _)| *p == prefix_idx && *s == suffix_idx)
            .map(|(_, _, a, n)| (*a, *n))
            .unwrap_or((Attestation::Possible, None)),
        "сса" => ROOT_SSA_ATTEST
            .iter()
            .find(|(p, s, _, _)| *p == prefix_idx && *s == suffix_idx)
            .map(|(_, _, a, n)| (*a, *n))
            .unwrap_or((Attestation::Possible, None)),
        "пизд" => ROOT_PIZD_ATTEST
            .iter()
            .find(|(p, s, _, _)| *p == prefix_idx && *s == suffix_idx)
            .map(|(_, _, a, n)| (*a, *n))
            .unwrap_or((Attestation::Possible, None)),
        "хуй" => ROOT_KHUY_ATTEST
            .iter()
            .find(|(p, s, _, _)| *p == prefix_idx && *s == suffix_idx)
            .map(|(_, _, a, n)| (*a, *n))
            .unwrap_or((Attestation::Possible, None)),
        "бляд" => ROOT_BLYAD_ATTEST
            .iter()
            .find(|(p, s, _, _)| *p == prefix_idx && *s == suffix_idx)
            .map(|(_, _, a, n)| (*a, *n))
            .unwrap_or((Attestation::Possible, None)),
        "муд" => ROOT_MUD_ATTEST
            .iter()
            .find(|(p, s, _, _)| *p == prefix_idx && *s == suffix_idx)
            .map(|(_, _, a, n)| (*a, *n))
            .unwrap_or((Attestation::Possible, None)),
        "манд" => ROOT_MAND_ATTEST
            .iter()
            .find(|(p, s, _, _)| *p == prefix_idx && *s == suffix_idx)
            .map(|(_, _, a, n)| (*a, *n))
            .unwrap_or((Attestation::Possible, None)),
        "елд" => ROOT_ELD_ATTEST
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
pub fn select_prefix_allomorph<'a>(
    prefix_val: &'a str,
    allomorphs: &[&str],
    root: &str,
) -> &'a str {
    if allomorphs.is_empty() {
        return prefix_val;
    }
    match prefix_val {
        "из" => {
            let first = root.chars().next().unwrap_or(' ');
            // Before root еб-, colloquial usage prefers ис-
            // Also before any voiceless consonant.
            if matches!(
                first,
                'е' | 'ё' | 'п' | 'с' | 'т' | 'к' | 'х' | 'ц' | 'ч' | 'ш' | 'щ'
            ) {
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
    fn test_endings_for_suffix_ei() {
        let endings = endings_for_suffix(2);
        // -е-/-и- suffix has 3 endings: ть, л, ит
        assert!(endings.contains(&0)); // ть (infinitive → еть)
        assert!(endings.contains(&1)); // л (past → ел)
        assert!(endings.contains(&4)); // ит (present 3sg)
        assert_eq!(endings.len(), 3);
    }

    #[test]
    fn test_root_data_sra() {
        let rd = root_data("сра").expect("сра should be a known root");
        assert_eq!(rd.name, "сра");
        assert_eq!(rd.val, "ср");
        assert_eq!(rd.gloss, Some("shit, excrete"));
    }

    #[test]
    fn test_root_data_ssa() {
        let rd = root_data("сса").expect("сса should be a known root");
        assert_eq!(rd.name, "сса");
        assert_eq!(rd.val, "сс");
    }

    #[test]
    fn test_root_data_pizd() {
        let rd = root_data("пизд").expect("пизд should be a known root");
        assert_eq!(rd.name, "пизд");
        assert!(rd.suffix_indices.contains(&2));
        assert!(rd.suffix_indices.contains(&1));
    }

    #[test]
    fn test_root_data_khuy() {
        let rd = root_data("хуй").expect("хуй should be a known root");
        assert_eq!(rd.name, "хуй");
        assert_eq!(rd.suffix_indices, &[1]);
    }

    #[test]
    fn test_root_data_blyad() {
        let rd = root_data("бляд").expect("бляд should be a known root");
        assert_eq!(rd.name, "бляд");
    }

    #[test]
    fn test_root_data_mud() {
        let rd = root_data("муд").expect("муд should be a known root");
        assert_eq!(rd.name, "муд");
    }

    #[test]
    fn test_root_data_mand() {
        let rd = root_data("манд").expect("манд should be a known root");
        assert_eq!(rd.name, "манд");
        assert!(rd.suffix_indices.is_empty());
    }

    #[test]
    fn test_root_data_eld() {
        let rd = root_data("елд").expect("елд should be a known root");
        assert_eq!(rd.name, "елд");
        assert!(rd.suffix_indices.is_empty());
    }

    #[test]
    fn test_all_roots_contains_all() {
        for name in &[
            "еб", "сра", "сса", "пизд", "хуй", "бляд", "муд", "манд", "елд",
        ] {
            assert!(
                all_roots().iter().any(|r| r.name == *name),
                "all_roots should contain '{}'",
                name
            );
        }
    }

    #[test]
    fn test_all_roots_has_9() {
        assert_eq!(all_roots().len(), 9);
    }

    #[test]
    fn test_lookup_attestation_sra_common() {
        let (att, note) = lookup_attestation("сра", 0, 0);
        assert_eq!(att, Attestation::Common);
        assert_eq!(note, Some("испражняться"));
    }

    #[test]
    fn test_lookup_attestation_ssa_common() {
        let (att, note) = lookup_attestation("сса", 0, 0);
        assert_eq!(att, Attestation::Common);
        assert_eq!(note, Some("мочиться"));
    }

    #[test]
    fn test_lookup_attestation_pizd_ei() {
        let (att, note) = lookup_attestation("пизд", 0, 2);
        assert_eq!(att, Attestation::Common);
        assert_eq!(note, Some("говорить ерунду"));
    }

    #[test]
    fn test_lookup_attestation_khuy_nu() {
        let (att, note) = lookup_attestation("хуй", 0, 1);
        assert_eq!(att, Attestation::Common);
        assert_eq!(note, Some("ударить"));
    }

    #[test]
    fn test_lookup_attestation_blyad_ei() {
        let (att, _note) = lookup_attestation("бляд", 0, 2);
        assert_eq!(att, Attestation::Rare);
    }

    #[test]
    fn test_lookup_attestation_mud_common() {
        let (att, note) = lookup_attestation("муд", 0, 0);
        assert_eq!(att, Attestation::Common);
        assert_eq!(note, Some("медлить, делать ерунду"));
    }

    #[test]
    fn test_lookup_attestation_mand_possible() {
        // манд has very limited verb forms; unknown prefix×suffix should be Possible
        let (att, _note) = lookup_attestation("манд", 3, 1);
        assert_eq!(att, Attestation::Possible);
    }

    #[test]
    fn test_lookup_attestation_eld_possible() {
        let (att, _note) = lookup_attestation("елд", 5, 0);
        assert_eq!(att, Attestation::Possible);
    }

    #[test]
    fn test_all_roots_have_linguistic_notes() {
        for rd in all_roots() {
            assert!(
                !rd.linguistic_note.is_empty(),
                "Root '{}' should have a linguistic note",
                rd.name
            );
        }
    }

    #[test]
    fn test_all_roots_have_val() {
        for rd in all_roots() {
            assert!(!rd.val.is_empty(), "Root '{}' should have a val", rd.name);
        }
    }
}
