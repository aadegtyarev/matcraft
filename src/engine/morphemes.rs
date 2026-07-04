#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
    Plural,
}

pub struct Root {
    #[allow(dead_code)]
    pub name: &'static str,
    pub adjective_stem: &'static str,
    /// Endings in order: [masculine, feminine, neuter, plural]
    pub adjective_endings: &'static [&'static str; 4],
}

pub const ROOTS: &[Root] = &[
    Root {
        name: "пизд",
        adjective_stem: "пиздат",
        adjective_endings: &["ый", "ая", "ое", "ые"],
    },
    Root {
        name: "хуй",
        adjective_stem: "хуёв",
        adjective_endings: &["ый", "ая", "ое", "ые"],
    },
    Root {
        name: "еб",
        adjective_stem: "ебанут",
        adjective_endings: &["ый", "ая", "ое", "ые"],
    },
    Root {
        name: "бляд",
        adjective_stem: "блядск",
        adjective_endings: &["ий", "ая", "ое", "ие"],
    },
];

pub const INTERJECTIONS: &[&str] = &[
    "охуеть",
    "пиздец",
    "ёбаный стыд",
    "ни хуя себе",
    "ёбаный в рот",
    "ебись оно конём",
    "ёбаный насос",
];

pub const EVAL_NOUNS: &[&str] = &[
    "пиздец",
    "хуйня",
    "залупа",
    "блядство",
    "мудачьё",
    "пиздопроёбина",
];
