//! Group 4: 5 nominal roots with verb potential (залуп-, жоп-, говн-, пидор-, курв-).
//!
//! Root data plus attestation tables. See `Attestation` in morpheme.rs.

use crate::engine::morpheme::{Attestation, Domain, ProductivityClass, RootData};

/// Root definitions for the nominal-with-verb-potential roots.
pub const GROUP_4: &[RootData] = &[
    RootData {
        name: "залуп",
        val: "залуп",
        gloss: Some("foreskin, glans"),
        gloss_ru: Some("головка члена, крайняя плоть"),
        suffix_indices: &[3], // -и- (залупиться)
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        takes_fill_vowel: false,
        o_takes_ob: false,
        linguistic_note: "Именной корень, обозначающий головку полового члена \
            и крайнюю плоть. В глагольных формах (залупиться) означает \
            'обнажить головку'. По Плуцеру-Сарно — номинатив с глагольным потенциалом.",
    },
    RootData {
        name: "жоп",
        val: "жоп",
        gloss: Some("ass, buttocks"),
        gloss_ru: Some("ягодицы, зад"),
        suffix_indices: &[3], // -и- (зажопить)
        domain: Domain::Excretory,
        productivity: ProductivityClass::D,
        present_stem: None,
        takes_fill_vowel: false,
        o_takes_ob: false,
        linguistic_note: "Именной корень со значением 'ягодицы, зад'. \
            В глагольных формах (зажопить, прижопить) развивает значение \
            'присвоить, не поделиться'. Один из самых частотных корней \
            в бытовом обсценном лексиконе.",
    },
    RootData {
        name: "говн",
        val: "говн",
        gloss: Some("shit, excrement"),
        gloss_ru: Some("экскременты, кал"),
        suffix_indices: &[3], // -и- (говнить)
        domain: Domain::Excretory,
        productivity: ProductivityClass::D,
        present_stem: None,
        takes_fill_vowel: false,
        o_takes_ob: true,
        linguistic_note: "Именной корень, обозначающий экскременты. \
            В глагольных формах (говнить, заговнить) — 'портить, делать плохо'. \
            Чрезвычайно продуктивен в именной деривации (говно, говённый, говнище).",
    },
    RootData {
        name: "пидор",
        val: "пидор",
        gloss: Some("faggot (derog.)"),
        gloss_ru: Some("гомосексуал (презр.)"),
        suffix_indices: &[3], // -и- (пидорить)
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        takes_fill_vowel: false,
        o_takes_ob: false,
        linguistic_note: "Именной корень, сокращение от 'педераст'. \
            Глагольные формы (пидорить, запидорить) развивают значение \
            'анально изнасиловать; испортить'. Обладает сильной обсценной окраской.",
    },
    RootData {
        name: "курв",
        val: "курв",
        gloss: Some("whore, slut"),
        gloss_ru: Some("распутница"),
        suffix_indices: &[3], // -и- (курвиться)
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        takes_fill_vowel: false,
        o_takes_ob: false,
        linguistic_note: "Именной корень со значением 'проститутка, \
            распутная женщина'. От латинского curva. Глагольные формы \
            (курвиться, закурвиться) означают 'вести себя распутно'. \
            Преимущественно именной.",
    },
];

// Root залуп- — -и- class (залупиться). Source §2 class E: "possible, не attested" →
// every form Possible (honesty gate), notes kept as projected meaning.
pub const ROOT_ZALUP_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 3, Attestation::Possible, Some("обнажить головку члена")),
    (
        3,
        3,
        Attestation::Possible,
        Some("залупиться; обнажиться (груб.)"),
    ),
    (5, 3, Attestation::Possible, Some("налупить; натянуть кожу")),
    (16, 3, Attestation::Possible, Some("слупиться")),
];

// Root жоп- — -и- class (зажопить)
pub const ROOT_ZHOP_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 3, Attestation::Common, Some("экономить; присваивать")),
    (3, 3, Attestation::Common, Some("зажопить; не поделиться")),
    (5, 3, Attestation::Rare, Some("нажопить; накопить")),
    (14, 3, Attestation::Common, Some("прижопить; присвоить")),
];

// Root говн- — -и- class (говнить)
pub const ROOT_GOVN_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 3, Attestation::Common, Some("портить; делать плохо")),
    (
        3,
        3,
        Attestation::Common,
        Some("заговнить; испачкать/испортить"),
    ),
    (5, 3, Attestation::Rare, Some("наговнить; наделать дерьма")),
    (8, 3, Attestation::Rare, Some("провонять; обратить в говно")),
    // о-/об- + говн-: говн- lexically selects the об- allomorph (o_takes_ob), so the
    // engine now builds обговнить — the real attested colloquial derivative (Operator-
    // confirmed Common), not the earlier simplified оговнить. This resolves говн-'s
    // #24 case; the general о-/об- irregularity for OTHER roots stays a documented
    // simplification. See docs/decisions/o-ob-allomorphy.md.
    (
        11,
        3,
        Attestation::Common,
        Some("обговнить; испортить, испачкать"),
    ),
];

// Root пидор- — -и- class (пидорить). Source §2 class E: "possible, не attested" →
// every form Possible (honesty gate), notes kept as projected meaning.
pub const ROOT_PIDOR_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (
        0,
        3,
        Attestation::Possible,
        Some("заниматься анальным сексом (акт.)"),
    ),
    (
        3,
        3,
        Attestation::Possible,
        Some("запидорить; ударить; испортить"),
    ),
    (
        5,
        3,
        Attestation::Possible,
        Some("напидорить; натворить дел"),
    ),
    (
        8,
        3,
        Attestation::Possible,
        Some("пропидорить; анально изнасиловать"),
    ),
    (14, 3, Attestation::Possible, Some("припидорить; приделать")),
    (
        16,
        3,
        Attestation::Possible,
        Some("спидорить; сделать наскоро"),
    ),
];

// Root курв- — -и- class (курвиться). Source §2 class E: "possible, не attested" →
// every form Possible (honesty gate), notes kept as projected meaning.
pub const ROOT_KURV_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 3, Attestation::Possible, Some("вести себя распутно")),
    (
        3,
        3,
        Attestation::Possible,
        Some("закурвиться; стать проституткой"),
    ),
    (5, 3, Attestation::Possible, Some("накурвить; нагулять")),
    (
        8,
        3,
        Attestation::Possible,
        Some("прокурвиться; прогулять с муж."),
    ),
    (
        15,
        3,
        Attestation::Possible,
        Some("раскурвиться; стать развратной"),
    ),
];
