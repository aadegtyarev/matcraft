use super::*;

#[test]
fn test_prefix_count() {
    assert_eq!(prefix_count(), 18);
}

#[test]
fn test_select_allomorph_iz_before_eb() {
    assert_eq!(select_prefix_allomorph("из", &["ис"], "еб", false), "ис");
}

#[test]
fn test_select_allomorph_no_allomorphs() {
    assert_eq!(select_prefix_allomorph("вы", &[], "еб", false), "вы");
}

#[test]
fn test_select_allomorph_o_takes_ob_forces_ob_before_consonant() {
    // говн- (o_takes_ob=true): об- even before a consonant → обговн-.
    assert_eq!(select_prefix_allomorph("о", &["об"], "говн", true), "об");
    // Any other root (o_takes_ob=false) keeps о- before a consonant → охар-, одроч-.
    assert_eq!(select_prefix_allomorph("о", &["об"], "хар", false), "о");
    assert_eq!(select_prefix_allomorph("о", &["об"], "дроч", false), "о");
    // The vowel rule is untouched: об- still applies before a vowel regardless.
    assert_eq!(select_prefix_allomorph("о", &["об"], "еб", false), "об");
}

#[test]
fn test_prefix_fill_form_yer_final_are_voiced() {
    // The 8 yer-final prefixes carry the VOICED fill base (devoicing cancelled).
    assert_eq!(prefix_fill_form(4), Some("изо")); // из-
    assert_eq!(prefix_fill_form(6), Some("ото")); // от-
    assert_eq!(prefix_fill_form(9), Some("во")); // в-
    assert_eq!(prefix_fill_form(10), Some("взо")); // вз-
    assert_eq!(prefix_fill_form(11), Some("обо")); // о-
    assert_eq!(prefix_fill_form(13), Some("подо")); // под-
    assert_eq!(prefix_fill_form(15), Some("разо")); // раз-
    assert_eq!(prefix_fill_form(16), Some("со")); // с-
}

#[test]
fn test_prefix_fill_form_none_for_vowel_and_bare_prefixes() {
    assert_eq!(prefix_fill_form(0), None); // bare
    assert_eq!(prefix_fill_form(1), None); // вы-
    assert_eq!(prefix_fill_form(3), None); // за-
    assert_eq!(prefix_fill_form(5), None); // на-
    assert_eq!(prefix_fill_form(17), None); // у-
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
    // -е- suffix has 3 endings: ть, л, ит
    assert!(endings.contains(&0)); // ть (infinitive → еть)
    assert!(endings.contains(&1)); // л (past → ел)
    assert!(endings.contains(&4)); // ит (present 3sg)
    assert_eq!(endings.len(), 3);
}

#[test]
fn test_suffix_index_for_val() {
    assert_eq!(suffix_index_for_val("а"), 0);
    assert_eq!(suffix_index_for_val("ну"), 1);
    assert_eq!(suffix_index_for_val("е"), 2);
    assert_eq!(suffix_index_for_val("и"), 3);
}

#[test]
fn test_ending_by_val_round_trips() {
    // ending_by_val must invert ending_val for every entry, so the display
    // block can go from a VerbForm's stored ending_val back to the table.
    for idx in 0..ENDINGS.len() {
        assert_eq!(ending_by_val(ending_val(idx)), idx);
    }
}

#[test]
fn test_ending_display_and_label() {
    assert_eq!(ending_display(ending_by_val("ть")), "-ть");
    assert_eq!(ending_label(ending_by_val("ть")), "инфинитив -ть");
    assert_eq!(ending_display(ending_by_val("л")), "-л");
    assert_eq!(
        ending_label(ending_by_val("л")),
        "прош. время, м. р. ед. ч. -л"
    );
    assert_eq!(ending_display(ending_by_val("ёт")), "-ёт");
    assert_eq!(
        ending_label(ending_by_val("ёт")),
        "наст. время, 3 л. ед. ч. -ёт"
    );
    assert_eq!(ending_display(ending_by_val("нёт")), "-нёт");
    assert_eq!(
        ending_label(ending_by_val("нёт")),
        "наст. время, 3 л. ед. ч. -нёт"
    );
    assert_eq!(ending_display(ending_by_val("ит")), "-ит");
    assert_eq!(
        ending_label(ending_by_val("ит")),
        "наст. время, 3 л. ед. ч. -ит"
    );
}

#[test]
#[should_panic(expected = "unknown ending val")]
fn test_ending_by_val_unknown_panics() {
    ending_by_val("щщ");
}

#[test]
fn test_productivity_ordering() {
    assert!(ProductivityClass::A < ProductivityClass::B);
    assert!(ProductivityClass::B < ProductivityClass::C);
    assert!(ProductivityClass::D < ProductivityClass::E);
}
