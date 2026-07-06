//! Group 5: 12 purely nominal peripheral roots (no verb paradigm).
//!
//! Roots: сиповк-, секел-, поц-, молофь-, минж-, целк-, королёвк-,
//!        кун-, сперм-, менстр-, минет-, гондон-.
//!
//! All have empty `suffix_indices`; attestation defaults to Possible in `mod.rs`.

use crate::engine::morpheme::{Domain, ProductivityClass, RootData};

/// Root definitions for the purely nominal periphery (source §2 class E).
pub const GROUP_5: &[RootData] = &[
    RootData {
        name: "сиповк",
        val: "сиповк",
        gloss: Some("female genitals type (vulg.)"),
        gloss_ru: Some("тип женских гениталий"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень из классификации Плуцера-Сарно. \
            Обозначает один из типов женских гениталий (с высоким входом). \
            Чисто номинативный корень, без глагольных форм.",
    },
    RootData {
        name: "секел",
        val: "секел",
        gloss: Some("female genitals type (vulg.)"),
        gloss_ru: Some("тип женских гениталий"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень из пласта обсценной анатомической \
            лексики. Обозначает тип женских гениталий с поперечным разрезом. \
            Не образует глагольных форм.",
    },
    RootData {
        name: "поц",
        val: "поц",
        gloss: Some("dick (vulg.)"),
        gloss_ru: Some("член (вульг.); придурок"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень из идиша (פּאָץ — 'член'). \
            В русском мате — универсальное презрительное обозначение мужчины. \
            Не образует глагольных форм. Заимствование XX века.",
    },
    RootData {
        name: "молофь",
        val: "молофь",
        gloss: Some("sperm (vulg.)"),
        gloss_ru: Some("сперма (вульг.)"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень со значением 'сперма'. \
            Относится к соматическому слою мата. Глагольных форм не образует. \
            Устаревающий корень, редко встречается в современной речи.",
    },
    RootData {
        name: "минж",
        val: "минж",
        gloss: Some("female genitals (vulg.)"),
        gloss_ru: Some("женские гениталии (вульг.)"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень, обозначающий женские гениталии. \
            Заимствован из французского (la moniche) через уголовное арго. \
            Не образует глагольных форм.",
    },
    RootData {
        name: "целк",
        val: "целк",
        gloss: Some("hymen"),
        gloss_ru: Some("девственная плева"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень со значением 'девственная плева'. \
            Относится к анатомическому слою мата. Не образует глагольных форм. \
            Связан с 'целка' — уничижительное обозначение девственницы.",
    },
    RootData {
        name: "королёвк",
        val: "королёвк",
        gloss: Some("female genitals type (vulg.)"),
        gloss_ru: Some("тип женских гениталий"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень из классификации Плуцера-Сарно. \
            Обозначает тип женских гениталий с крупными половыми губами. \
            Не образует глагольных форм. От 'королёк' — сорт сладкого апельсина.",
    },
    RootData {
        name: "кун",
        val: "кун",
        gloss: Some("cunnilingus"),
        gloss_ru: Some("куннилингус"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень, обозначающий куннилингус. \
            Заимствование из немецкого через медицинский дискурс. \
            Не образует глагольных форм.",
    },
    RootData {
        name: "сперм",
        val: "сперм",
        gloss: Some("sperm"),
        gloss_ru: Some("сперма"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень со значением 'сперма'. \
            От греческого σπέρμα через медицинскую терминологию. \
            Не образует глагольных форм. Стилистически нейтральный корень.",
    },
    RootData {
        name: "менстр",
        val: "менстр",
        gloss: Some("menstruation"),
        gloss_ru: Some("менструация"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень со значением 'менструация'. \
            От латинского menstruum. Не образует глагольных форм. \
            Относится к соматическому слою, на границе медицинской \
            и обсценной лексики.",
    },
    RootData {
        name: "минет",
        val: "минет",
        gloss: Some("blowjob"),
        gloss_ru: Some("минет, фелляция"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень, обозначающий фелляцию. \
            Заимствование из французского (la minette, 'кошечка'). \
            Не образует глагольных форм. Преимущественно именной.",
    },
    RootData {
        name: "гондон",
        val: "гондон",
        gloss: Some("condom"),
        gloss_ru: Some("презерватив"),
        suffix_indices: &[],
        domain: Domain::Peripheral,
        productivity: ProductivityClass::E,
        present_stem: None,
        linguistic_note: "Именной корень со значением 'презерватив'. \
            Французское заимствование (condom). В переносном значении — \
            'противный, надоедливый человек'. Не образует глагольных форм.",
    },
];
