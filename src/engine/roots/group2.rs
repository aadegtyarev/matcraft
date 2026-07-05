//! Group 2: the two most productive excretory roots (сра-, сса-).
//!
//! Root data plus attestation tables. See `Attestation` in morpheme.rs.

use crate::engine::morpheme::{Attestation, Domain, ProductivityClass, RootData};

/// Root definitions for the highly productive excretory roots (source §2 class B).
pub const GROUP_2: &[RootData] = &[
    RootData {
        name: "сра",
        val: "ср",
        gloss: Some("shit, excrete"),
        suffix_indices: &[0, 1], // -а- and -ну-
        domain: Domain::Excretory,
        productivity: ProductivityClass::B,
        present_stem: None,
        linguistic_note: "Экскреторный корень. По классификации Плуцера-Сарно \
            относится к отдельному домену (не сексуальный мат), но по продуктивности \
            не уступает корню еб-. В современном русском образует десятки глагольных форм.",
    },
    RootData {
        name: "сса",
        val: "сс",
        gloss: Some("piss, urinate"),
        suffix_indices: &[0, 1], // -а- and -ну-
        domain: Domain::Excretory,
        productivity: ProductivityClass::B,
        present_stem: None,
        linguistic_note: "Экскреторный корень 'мочиться'. Менее продуктивен, чем сра-, \
            но образует ряд ярких метафор: зассать ('испугаться'), \
            обоссать ('раскритиковать').",
    },
];

// Root сра- — -а- and -ну- classes
pub const ROOT_SRA_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
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

// Root сса- — -а- and -ну- classes
pub const ROOT_SSA_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
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
