//! Group 3: 9 verb roots (peripheral + excretory).
//!
//! Roots: дроч-, трах-, жр-, хер-, перд-, хар-, блев-, бзд-, дрист-.
//! Root data plus attestation tables. See `Attestation` in morpheme.rs.

use crate::engine::morpheme::{Attestation, Domain, ProductivityClass, RootData};

/// Root definitions for the verb roots split across periphery and excretory domains.
pub const GROUP_3: &[RootData] = &[
    RootData {
        name: "дроч",
        val: "дроч",
        gloss: Some("masturbate"),
        suffix_indices: &[3], // -и- (дрочить)
        domain: Domain::Peripheral,
        productivity: ProductivityClass::D,
        present_stem: None,
        linguistic_note: "Глагольный корень со значением 'заниматься мастурбацией'. \
            Образует формы II спряжения (дрочить, дрочу, дрочишь). \
            В переносном значении — 'надоедать, изводить повторами'.",
    },
    RootData {
        name: "трах",
        val: "трах",
        gloss: Some("copulate, fuck"),
        suffix_indices: &[0], // -а- (трахать)
        domain: Domain::Peripheral,
        productivity: ProductivityClass::D,
        present_stem: None,
        linguistic_note: "Глагольный корень, обозначающий половой акт. \
            Спрягается по I классу (-а-): трахать, трахаю, трахает. \
            Омонимичен звукоподражательному 'трах' (удар, грохот).",
    },
    RootData {
        name: "жр",
        val: "жр",
        gloss: Some("eat greedily, devour"),
        suffix_indices: &[0], // -а- (жрать)
        domain: Domain::Peripheral,
        productivity: ProductivityClass::D,
        present_stem: None,
        linguistic_note: "Глагольный корень 'есть жадно, много'. \
            От праславянского *žerti. Нестандартное спряжение с чередованием: \
            жрать, жру, жрёт. Переносно: 'проживать, тратить впустую'.",
    },
    RootData {
        name: "хер",
        val: "хер",
        gloss: Some("spoil, cancel"),
        suffix_indices: &[3], // -и- (херить)
        domain: Domain::Peripheral,
        productivity: ProductivityClass::D,
        present_stem: None,
        linguistic_note: "Глагольный корень от существительного 'хер' \
            (название буквы Х, эвфемизм 'хуй'). Глагол херить значит \
            'портить, вычёркивать, отменять'. Относится к новейшему слою мата \
            (переосмысление XX века).",
    },
    RootData {
        name: "перд",
        val: "перд",
        gloss: Some("fart"),
        suffix_indices: &[2, 1], // -е- (пердеть), -ну- (перднуть)
        domain: Domain::Excretory,
        productivity: ProductivityClass::C,
        present_stem: None,
        linguistic_note: "Корень со значением 'испускать кишечные газы'. \
            От праславянского *pьrděti. Два глагольных класса: длительное пердеть \
            (II спряжение) и однократное перднуть. Имеет широкий ряд \
            метафорических значений.",
    },
    RootData {
        name: "хар",
        val: "хар",
        gloss: Some("copulate roughly"),
        suffix_indices: &[3], // -и- (харить)
        domain: Domain::Peripheral,
        productivity: ProductivityClass::D,
        present_stem: None,
        linguistic_note: "Глагольный корень с общим значением грубого действия. \
            Спрягается по II спряжению: харить, харю, харишь. Семантический диапазон \
            широк: от полового акта до интенсивной работы. Этимология неясна.",
    },
    RootData {
        name: "блев",
        val: "блев",
        gloss: Some("vomit"),
        suffix_indices: &[0], // -а- (блевать); present stem блю-
        domain: Domain::Peripheral,
        productivity: ProductivityClass::D,
        present_stem: Some("блю"),
        linguistic_note: "Глагольный корень 'извергать рвоту'. \
            От праславянского *bljьvati. Характеризуется историческим чередованием \
            в основе: инфинитив блевать, основа настоящего времени блю- (блюю, блюёт). \
            Переносно: 'выдавать из себя'.",
    },
    RootData {
        name: "бзд",
        val: "бзд",
        gloss: Some("fart silently"),
        suffix_indices: &[2], // -е- (бздеть)
        domain: Domain::Excretory,
        productivity: ProductivityClass::C,
        present_stem: None,
        linguistic_note: "Корень со значением 'испускать газы беззвучно'. \
            Спрягается по II классу (-е-): бздеть, бзжу, бздишь. \
            Переносное значение развилось в сторону страха: 'бздеть' = 'бояться, трусить'.",
    },
    RootData {
        name: "дрист",
        val: "дрист",
        gloss: Some("have diarrhea"),
        suffix_indices: &[0], // -а- (дристать)
        domain: Domain::Excretory,
        productivity: ProductivityClass::C,
        present_stem: None,
        linguistic_note: "Корень со значением 'страдать поносом'. \
            От праславянского *dristati. Спрягается по I классу (-а-): \
            дристать, дрищу, дрищешь. В переносном значении — \
            'делать что-то некачественное, жидкое'.",
    },
];

// Root дроч- — -и- class (дрочить)
pub const ROOT_DROCH_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 3, Attestation::Common, Some("заниматься мастурбацией")),
    (1, 3, Attestation::Rare, Some("оттрахать интенсивно")),
    (3, 3, Attestation::Common, Some("измучить повторами")),
    (5, 3, Attestation::Rare, Some("довести до маразма")),
    (6, 3, Attestation::Common, Some("кончить; закончить")),
    (8, 3, Attestation::Common, Some("истратить на бесполезное")),
    (9, 3, Attestation::Rare, Some("добиться изнурением")),
    (
        14,
        3,
        Attestation::Common,
        Some("добавить; приделать наспех"),
    ),
    (15, 3, Attestation::Common, Some("довести до изнеможения")),
    (16, 3, Attestation::Rare, Some("содрать курсовую (студ.)")),
];

// Root трах- — -а- class (трахать)
pub const ROOT_TRAKH_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 0, Attestation::Common, Some("совершать половой акт")),
    (
        1,
        0,
        Attestation::Common,
        Some("вытрахать; износить одеяло"),
    ),
    (3, 0, Attestation::Common, Some("затрахать; измучить")),
    (5, 0, Attestation::Common, Some("натрахать; получить удов.")),
    (6, 0, Attestation::Common, Some("оттрахать качественно")),
    (7, 0, Attestation::Rare, Some("перетрахать многих")),
    (8, 0, Attestation::Common, Some("протрахать всё время")),
    (15, 0, Attestation::Common, Some("растрахать; расшевелить")),
];

// Root жр- — -а- class (жрать)
pub const ROOT_ZHR_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 0, Attestation::Common, Some("есть жадно, много")),
    (1, 0, Attestation::Rare, Some("выжрать; выпить всё")),
    (2, 0, Attestation::Possible, None),
    (3, 0, Attestation::Common, Some("зажраться; обнаглеть")),
    (
        5,
        0,
        Attestation::Common,
        Some("нажраться; наесться до отвала"),
    ),
    (6, 0, Attestation::Rare, Some("отожраться; потолстеть")),
    (7, 0, Attestation::Rare, Some("пережрать; переесть")),
    (
        8,
        0,
        Attestation::Common,
        Some("прожрать; истратить на еду"),
    ),
    (12, 0, Attestation::Rare, Some("пожрать немного")),
];

// Root хер- — -и- class (херить). хер- is absent from source §3/§5 entirely; its
// productivity class D is an extrapolation. No form is source-attested → all Possible
// (honesty gate), notes kept as projected meaning.
pub const ROOT_KHER_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 3, Attestation::Possible, Some("портить; отменять")),
    (1, 3, Attestation::Possible, Some("выхерить; удалить")),
    (3, 3, Attestation::Possible, Some("зачеркнуть; отменить")),
    (5, 3, Attestation::Possible, Some("нахерить; навредить")),
    (
        7,
        3,
        Attestation::Possible,
        Some("перехерить; испортить всё"),
    ),
    (8, 3, Attestation::Possible, Some("прохерить; упустить")),
    (16, 3, Attestation::Possible, Some("схерить; свести на нет")),
];

// Root перд- — -е- (пердеть) and -ну- (перднуть) classes; source §3 marks -е-, not -а-
pub const ROOT_PERD_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    // -е- class
    (0, 2, Attestation::Common, Some("испускать газы")),
    (1, 2, Attestation::Rare, Some("выпердеть; резко выдохнуть")),
    (
        3,
        2,
        Attestation::Common,
        Some("засорить воздух; испортить"),
    ),
    (5, 2, Attestation::Rare, Some("напердеть; наполнить газами")),
    (6, 2, Attestation::Common, Some("отпердеть; закончить")),
    (8, 2, Attestation::Rare, Some("пропердеть; провалить")),
    (12, 2, Attestation::Rare, Some("попердеть немного")),
    // -ну- class
    (0, 1, Attestation::Common, Some("испустить газ однократно")),
    (3, 1, Attestation::Rare, Some("засорить резко")),
];

// Root хар- — -и- class (харить)
pub const ROOT_KHAR_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (
        0,
        3,
        Attestation::Common,
        Some("совокупляться; делать грубо"),
    ),
    (
        3,
        3,
        Attestation::Common,
        Some("захарить; заиметь пол. контакт"),
    ),
    (
        5,
        3,
        Attestation::Rare,
        Some("нахарить; совершить мн. актов"),
    ),
    (8, 3, Attestation::Common, Some("прохарить насквозь")),
    (11, 3, Attestation::Rare, Some("охарить; обмануть")),
    (
        15,
        3,
        Attestation::Common,
        Some("расхарить; разбудить/разогреть"),
    ),
];

// Root блев- — -а- class (блевать); present_stem "блю"
pub const ROOT_BLEV_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 0, Attestation::Common, Some("извергать рвоту")),
    (1, 0, Attestation::Rare, Some("выблевать; извергнуть")),
    (
        3,
        0,
        Attestation::Common,
        Some("заблевать; испачкать рвотой"),
    ),
    (5, 0, Attestation::Common, Some("наблевать; оставить рвоту")),
    (6, 0, Attestation::Rare, Some("отблевать; перестать")),
    (
        8,
        0,
        Attestation::Common,
        Some("проблевать; пропустить из-за рвоты"),
    ),
    (11, 0, Attestation::Rare, Some("облевать; облить рвотой")),
    (16, 0, Attestation::Rare, Some("сблевать; вырвать")),
];

// Root бзд- — -е- class (бздеть)
pub const ROOT_BZD_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 2, Attestation::Common, Some("испускать газы беззвучно")),
    (3, 2, Attestation::Common, Some("испугаться, струсить")),
    (5, 2, Attestation::Rare, Some("набздеть; навонять")),
    (7, 2, Attestation::Common, Some("перестараться от страха")),
    (
        8,
        2,
        Attestation::Rare,
        Some("пробздеть; упустить из трусости"),
    ),
];

// Root дрист- — -а- class (дристать)
pub const ROOT_DRIST_ATTEST: &[(usize, usize, Attestation, Option<&str>)] = &[
    (0, 0, Attestation::Common, Some("страдать поносом")),
    (3, 0, Attestation::Rare, Some("задристать; испачкать")),
    (5, 0, Attestation::Rare, Some("надристать; нагадить")),
    (11, 0, Attestation::Rare, Some("обдристать; обгадить")),
];
