//! Group 1: 7 nuclear roots (mat core).
//!
//! Roots: еб-, пизд-, хуй-, бляд-, муд-, манд-, елд-.
//! Root data plus attestation tables. See `Attestation` in morpheme.rs.

use crate::engine::morpheme::{Attestation, Domain, ProductivityClass, RootData};

/// Root definitions for the nuclear domain.
pub const GROUP_1: &[RootData] = &[
    RootData {
        name: "еб",
        val: "еб",
        gloss: Some("fuck, copulate"),
        gloss_ru: Some("совокупляться"),
        suffix_indices: &[0, 1], // -а- and -ну-
        domain: Domain::Nuclear,
        productivity: ProductivityClass::A,
        present_stem: None,
        linguistic_note: "Самый продуктивный матерный корень. От праславянского *jebati. \
            В современном русском образует более 100 глагольных и именных дериватов. \
            Едва ли не каждое действие можно описать глаголом на еб-.",
    },
    RootData {
        name: "пизд",
        val: "пизд",
        gloss: Some("female genitals (cunt)"),
        gloss_ru: Some("женские гениталии"),
        suffix_indices: &[2, 1, 3], // -е- (пиздеть), -ну- (пиздануть), -и- (пиздить)
        domain: Domain::Nuclear,
        productivity: ProductivityClass::B,
        present_stem: None,
        linguistic_note: "Женский корень мата. Образует два семантических ряда: \
            'говорить ерунду' (пиздеть, класс -е-) и 'бить/красть' (пиздить, класс -и-). \
            Один из немногих корней с двумя продуктивными глагольными классами.",
    },
    RootData {
        name: "хуй",
        val: "хуй",
        gloss: Some("penis (dick)"),
        gloss_ru: Some("мужской член"),
        suffix_indices: &[1], // -ну- (хуйнуть); хуять/хуячить требуют -j-, движком не строятся
        domain: Domain::Nuclear,
        productivity: ProductivityClass::C,
        present_stem: None,
        linguistic_note: "Ключевой именной корень русского мата. Этимология спорна: \
            от праиндоевропейского *ks-u- (хвоя), монгольского khui, или латинского huic. \
            Первый том словаря Плуцера-Сарно (500+ стр.) посвящён исключительно этому корню.",
    },
    RootData {
        name: "бляд",
        val: "бляд",
        gloss: Some("whore, prostitute"),
        gloss_ru: Some("проститутка; распутство"),
        suffix_indices: &[2], // -е- (блядеть); -ова- (блядовать) движком не строится
        domain: Domain::Nuclear,
        productivity: ProductivityClass::D,
        present_stem: None,
        linguistic_note: "От древнерусского блѧдь — 'обман, ерунда, прелюбодейка'. \
            Преимущественно именной корень (блядь, блядский, блядство). \
            Глагольные формы (блядовать) используют суффикс -ова-, не включённый \
            в текущую версию движка.",
    },
    RootData {
        name: "муд",
        val: "муд",
        gloss: Some("testicles"),
        gloss_ru: Some("яички"),
        suffix_indices: &[3], // -и- (мудить)
        domain: Domain::Nuclear,
        productivity: ProductivityClass::D,
        present_stem: None,
        linguistic_note: "Корень со значением 'testiculi'. В современном русском \
            глагольные формы (мудить) означают 'медлить, заниматься ерундой'. \
            Преимущественно именной: мудак, мудило.",
    },
    RootData {
        name: "манд",
        val: "манд",
        gloss: Some("female genitals (archaic)"),
        gloss_ru: Some("женские гениталии (арх.)"),
        suffix_indices: &[], // verb forms marginal (единичные); treated as noun-only
        domain: Domain::Nuclear,
        productivity: ProductivityClass::D,
        present_stem: None,
        linguistic_note: "Архаичный корень со значением 'женские гениталии'. \
            В XIX веке — одно из сильнейших ругательств. К XXI веку практически \
            утратил обсценную силу. Глагольные формы (мандить) крайне редки.",
    },
    RootData {
        name: "елд",
        val: "елд",
        gloss: Some("penis (archaic)"),
        gloss_ru: Some("мужской член (арх.)"),
        suffix_indices: &[], // purely nominal per source §3
        domain: Domain::Nuclear,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Архаичный корень со значением 'мужской член'. \
            Как и манд-, практически утратил обсценную силу в современном языке. \
            Глагольные формы (елдить) маргинальны.",
    },
];

// Root еб- — most productive mat root; source §4 grants broad common by "pronoun-verb" behaviour
pub const ROOT_EB_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // bare + -а-
    (0, 0, Attestation::Common, Some("совершать половой акт")),
    // bare + -ну-
    (0, 1, Attestation::Common, Some("ударить; сделать глупость")),
    // вы- + -а-
    (1, 0, Attestation::Common, Some("износить, испортить")),
    (1, 1, Attestation::Rare, Some("выбросить, выкинуть")),
    (2, 0, Attestation::Rare, Some("довести до оргазма")),
    (2, 1, Attestation::Possible, None),
    (3, 0, Attestation::Common, Some("измучить (перен.)")),
    (3, 1, Attestation::Rare, Some("ударить, стукнуть")),
    (4, 0, Attestation::Rare, Some("интенсив, избить (?)")),
    (4, 1, Attestation::Possible, None),
    (5, 0, Attestation::Common, Some("обмануть")),
    (5, 1, Attestation::Rare, Some("надуть, обмануть")),
    (6, 0, Attestation::Common, Some("оттрахать")),
    (6, 1, Attestation::Possible, None),
    (7, 0, Attestation::Common, Some("перетрахать(ся)")),
    (7, 1, Attestation::Possible, None),
    (8, 0, Attestation::Common, Some("упустить, просрать")),
    (8, 1, Attestation::Rare, Some("пробить, проломить")),
];

// Root пизд- — has three suffix classes: -е- (пиздеть), -ну- (пиздануть), -и- (пиздить)
pub const ROOT_PIZD_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // -е- class (пиздеть type)
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
    // -и- class (пиздить type)
    (0, 3, Attestation::Common, Some("бить, красть")),
    (1, 3, Attestation::Rare, Some("выгнать, выкинуть")),
    (3, 3, Attestation::Common, Some("ударить; присвоить")),
    (5, 3, Attestation::Rare, Some("навредить")),
    (6, 3, Attestation::Common, Some("избить")),
    (8, 3, Attestation::Common, Some("пропустить; украсть")),
];

// Root хуй- — -ну- (хуйнуть) is a surrogate: source §3 attests only -а- (хуять) and
// -и- (хуячить) via -j-, which the engine cannot build. The -ну- forms are therefore
// unattested by this source → all Possible (honesty gate), notes kept as projected meaning.
pub const ROOT_KHUY_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 1, Attestation::Possible, Some("ударить")),
    (1, 1, Attestation::Possible, None),
    (2, 1, Attestation::Possible, None),
    (3, 1, Attestation::Possible, Some("забить, пренебречь")),
    (4, 1, Attestation::Possible, None),
    (5, 1, Attestation::Possible, Some("навредить")),
    (6, 1, Attestation::Possible, None),
    (7, 1, Attestation::Possible, None),
    (8, 1, Attestation::Possible, Some("промахнуться")),
];

// Root бляд- — -е- only (блядеть); primarily nominal
pub const ROOT_BLYAD_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (
        0,
        2,
        Attestation::Rare,
        Some("говорить 'блядь' как слово-паразит"),
    ),
    (3, 2, Attestation::Rare, Some("начать материться")),
    (5, 2, Attestation::Possible, None),
    // проблядеть: base блядеть is uncertain in §3 ("?") and проблядеть is absent
    // from §5 → Possible, not Common.
    (
        8,
        2,
        Attestation::Possible,
        Some("провести время в распутстве; потратить зря"),
    ),
];

// Root муд- — -и- class (мудить)
pub const ROOT_MUD_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 3, Attestation::Common, Some("медлить, делать ерунду")),
    (3, 3, Attestation::Rare, Some("задержать, затянуть")),
    (5, 3, Attestation::Common, Some("наделать глупостей")),
    (6, 3, Attestation::Rare, Some("отделаться")),
    (7, 3, Attestation::Rare, Some("перестараться")),
];
