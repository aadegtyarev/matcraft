//! Root inventory and attestation tables, organised by Plutser-Sarno group files.
//!
//! Each `groupN.rs` is the single home for its roots: their `RootData` literals
//! and their attestation tables live together. This module aggregates them into
//! the global inventory and dispatches attestation lookups.

mod group1;
mod group2;
mod group3;
mod group4;
mod group5;

use std::sync::LazyLock;

use crate::engine::morpheme::{Attestation, RootData};

/// The full 35-root inventory, concatenated from the group files.
static ROOTS: LazyLock<Vec<RootData>> = LazyLock::new(|| {
    [
        group1::GROUP_1,
        group2::GROUP_2,
        group3::GROUP_3,
        group4::GROUP_4,
        group5::GROUP_5,
    ]
    .concat()
});

/// All roots in the inventory.
pub fn all_roots() -> &'static [RootData] {
    &ROOTS
}

/// Look up a root's data by name.
pub fn root_data(name: &str) -> Option<&'static RootData> {
    ROOTS.iter().find(|r| r.name == name)
}

/// Look up attestation and note for a given root, prefix index, and suffix index.
///
/// Returns `(Attestation, Option<note>)`. Unlisted combinations default to
/// (Possible, None) — linguistically honest: unattested ≠ impossible.
pub fn lookup_attestation(
    root: &str,
    prefix_idx: usize,
    suffix_idx: usize,
) -> (Attestation, Option<&'static str>) {
    let table = match root {
        // Nuclear (манд-, елд- are noun-only → Possible default)
        "еб" => group1::ROOT_EB_ATTEST,
        "пизд" => group1::ROOT_PIZD_ATTEST,
        "хуй" => group1::ROOT_KHUY_ATTEST,
        "бляд" => group1::ROOT_BLYAD_ATTEST,
        "муд" => group1::ROOT_MUD_ATTEST,
        // Excretory core
        "сра" => group2::ROOT_SRA_ATTEST,
        "сса" => group2::ROOT_SSA_ATTEST,
        // Group 3 verbs (peripheral + excretory)
        "дроч" => group3::ROOT_DROCH_ATTEST,
        "трах" => group3::ROOT_TRAKH_ATTEST,
        "жр" => group3::ROOT_ZHR_ATTEST,
        "хер" => group3::ROOT_KHER_ATTEST,
        "перд" => group3::ROOT_PERD_ATTEST,
        "хар" => group3::ROOT_KHAR_ATTEST,
        "блев" => group3::ROOT_BLEV_ATTEST,
        "бзд" => group3::ROOT_BZD_ATTEST,
        "дрист" => group3::ROOT_DRIST_ATTEST,
        // Group 4 nominal-with-verb-potential
        "залуп" => group4::ROOT_ZALUP_ATTEST,
        "жоп" => group4::ROOT_ZHOP_ATTEST,
        "говн" => group4::ROOT_GOVN_ATTEST,
        "пидор" => group4::ROOT_PIDOR_ATTEST,
        "курв" => group4::ROOT_KURV_ATTEST,
        // Noun-only roots — no attestation tables, all default to Possible
        _ => return (Attestation::Possible, None),
    };
    table
        .iter()
        .find(|(p, s, _, _)| *p == prefix_idx && *s == suffix_idx)
        .map(|(_, _, a, n)| (*a, *n))
        .unwrap_or((Attestation::Possible, None))
}

// ---------------------------------------------------------------------------
// Tests — root inventory and attestation (moved here with the data they cover)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::morpheme::{Domain, ProductivityClass};

    #[test]
    fn test_root_data_eb() {
        let rd = root_data("еб").expect("еб should be a known root");
        assert_eq!(rd.name, "еб");
        assert_eq!(rd.gloss, Some("fuck, copulate"));
        assert_eq!(rd.domain, Domain::Nuclear);
        assert_eq!(rd.productivity, ProductivityClass::A);
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
    fn test_all_roots_has_35() {
        assert_eq!(all_roots().len(), 35);
    }

    #[test]
    fn test_all_roots_contains_all() {
        for name in &[
            "еб",
            "пизд",
            "хуй",
            "бляд",
            "муд",
            "манд",
            "елд",
            "сра",
            "сса",
            "дроч",
            "трах",
            "жр",
            "хер",
            "перд",
            "хар",
            "блев",
            "бзд",
            "дрист",
            "залуп",
            "жоп",
            "говн",
            "пидор",
            "курв",
            "сиповк",
            "секел",
            "поц",
            "молофь",
            "минж",
            "целк",
            "королёвк",
            "кун",
            "сперм",
            "менстр",
            "минет",
            "гондон",
        ] {
            assert!(
                all_roots().iter().any(|r| r.name == *name),
                "all_roots should contain '{}'",
                name
            );
        }
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

    #[test]
    fn test_all_roots_have_russian_gloss() {
        // gloss_ru is the meaning shown in every command's output — it must be
        // present and non-empty for all 35 roots (rich-output D1). The field is
        // Option only so the form block can defend against a hypothetical gap.
        for rd in all_roots() {
            let gloss = rd
                .gloss_ru
                .unwrap_or_else(|| panic!("Root '{}' should have a Russian gloss", rd.name));
            assert!(
                !gloss.trim().is_empty(),
                "Root '{}' Russian gloss must not be blank",
                rd.name
            );
        }
    }

    #[test]
    fn test_domain_counts_match_source() {
        // Source §1: 7 nuclear, 7 excretory, 21 peripheral.
        let nuclear = all_roots()
            .iter()
            .filter(|r| r.domain == Domain::Nuclear)
            .count();
        let excretory = all_roots()
            .iter()
            .filter(|r| r.domain == Domain::Excretory)
            .count();
        let peripheral = all_roots()
            .iter()
            .filter(|r| r.domain == Domain::Peripheral)
            .count();
        assert_eq!(nuclear, 7, "nuclear domain should have 7 roots");
        assert_eq!(excretory, 7, "excretory domain should have 7 roots");
        assert_eq!(peripheral, 21, "peripheral domain should have 21 roots");
    }

    #[test]
    fn test_root_data_sra() {
        let rd = root_data("сра").expect("сра should be a known root");
        assert_eq!(rd.name, "сра");
        assert_eq!(rd.val, "ср");
        assert_eq!(rd.gloss, Some("shit, excrete"));
        assert_eq!(rd.domain, Domain::Excretory);
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
        assert!(rd.suffix_indices.contains(&3));
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
        assert_eq!(rd.suffix_indices, &[3]);
    }

    #[test]
    fn test_root_data_mand_noun_only() {
        let rd = root_data("манд").expect("манд should be a known root");
        assert_eq!(rd.name, "манд");
        assert!(rd.suffix_indices.is_empty());
    }

    #[test]
    fn test_root_data_eld_noun_only() {
        let rd = root_data("елд").expect("елд should be a known root");
        assert_eq!(rd.name, "елд");
        assert!(rd.suffix_indices.is_empty());
    }

    #[test]
    fn test_root_data_perd_is_e_class() {
        // Source §3: перд- is the -е- class (пердеть), not -а- (which builds *пердать).
        let rd = root_data("перд").expect("перд should be a known root");
        assert!(
            rd.suffix_indices.contains(&2),
            "перд- should have -е- class"
        );
        assert!(
            !rd.suffix_indices.contains(&0),
            "перд- must not have -а- class (would build non-word пердать)"
        );
        assert_eq!(rd.domain, Domain::Excretory);
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
    fn test_lookup_attestation_khuy_nu_possible() {
        // Honesty gate: хуй- -ну- forms are a surrogate (source §3 attests -а-/-и-
        // via unbuildable -j-), so they are Possible, not Common.
        let (att, _note) = lookup_attestation("хуй", 0, 1);
        assert_eq!(att, Attestation::Possible);
    }

    #[test]
    fn test_class_e_roots_never_common() {
        // Source §2 class E ("possible, не attested"): залуп-/пидор-/курв- must not
        // carry Common/Rare on any form.
        for name in ["залуп", "пидор", "курв"] {
            for p in 0..18 {
                for s in 0..4 {
                    let (att, _) = lookup_attestation(name, p, s);
                    assert_eq!(
                        att,
                        Attestation::Possible,
                        "class-E root '{}' ({},{}) must be Possible",
                        name,
                        p,
                        s
                    );
                }
            }
        }
    }

    #[test]
    fn test_lookup_attestation_blyad_ei() {
        let (att, _note) = lookup_attestation("бляд", 0, 2);
        assert_eq!(att, Attestation::Rare);
    }

    #[test]
    fn test_lookup_attestation_mud_common() {
        let (att, note) = lookup_attestation("муд", 0, 3);
        assert_eq!(att, Attestation::Common);
        assert_eq!(note, Some("медлить, делать ерунду"));
    }

    #[test]
    fn test_lookup_attestation_perd_e_class() {
        // перд- -е- class: пердеть = common on the bare form.
        let (att, note) = lookup_attestation("перд", 0, 2);
        assert_eq!(att, Attestation::Common);
        assert_eq!(note, Some("испускать газы"));
    }

    #[test]
    fn test_lookup_attestation_mand_possible() {
        // манд is noun-only; any lookup defaults to Possible.
        let (att, _note) = lookup_attestation("манд", 0, 0);
        assert_eq!(att, Attestation::Possible);
    }

    #[test]
    fn test_lookup_attestation_eld_possible() {
        let (att, _note) = lookup_attestation("елд", 0, 0);
        assert_eq!(att, Attestation::Possible);
    }

    #[test]
    fn test_takes_fill_vowel_only_three_roots() {
        // The fluid vowel -о- is lexically conditioned (issue #28): exactly сра-,
        // сса-, жр- carry it — no other root, whatever its surface cluster.
        for name in ["сра", "сса", "жр"] {
            assert!(
                root_data(name).expect("root exists").takes_fill_vowel,
                "{name}- must take the fluid vowel"
            );
        }
        for name in ["еб", "блев", "дроч", "трах", "бзд", "дрист", "говн"]
        {
            assert!(
                !root_data(name).expect("root exists").takes_fill_vowel,
                "{name}- must NOT take the fluid vowel"
            );
        }
    }
}
