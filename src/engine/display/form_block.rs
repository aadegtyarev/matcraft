//! The house-style "full breakdown block" for a single form.
//!
//! One home for the explanatory block reused by `generate`, `explore` (under a
//! `--suffix` filter), `random`, and `root-of-the-day`. Every displayed morpheme
//! gloss is pulled from the morpheme/root tables via accessors — never hardcoded
//! here — so the block can never drift from the data.

use crate::engine::morpheme::{
    RootData, VerbForm, ending_by_val, ending_display, ending_label, prefix_display,
    suffix_display, suffix_gloss, suffix_index_for_val,
};

use super::{attestation_reason, domain_inline, format_attestation};

/// Placeholder shown for a form without a source note (`possible` forms).
const NO_NOTE: &str = "— (в источнике не засвидетельствовано)";

/// Format a single form as the house-style explanatory block:
///
/// ```text
/// обосснуть  ·  possible
///   значение : — (в источнике не засвидетельствовано)
///   разбор   : о-/об- + сса- + -ну- + -ть
///   морфемы  : приставка «о-/об-» · корень сса- «мочиться» · суффикс -ну- (однократный) · инфинитив -ть
///   уровень  : possible — словообразовательно возможна по аналогии, в источнике не засвидетельствована
///   домен    : экскреторная · продуктивность B
/// ```
///
/// The bare prefix (`(без)`) is omitted from both the `разбор` and `морфемы`
/// lines — never printed as `приставка «(без)»`. A root with no `gloss_ru`
/// prints `корень <name>-` without a gloss (honesty: no invented meaning).
pub fn format_form_block(rd: &RootData, vf: &VerbForm) -> String {
    let token = format_attestation(vf.attestation);
    let suffix_idx = suffix_index_for_val(vf.suffix_val);
    let ending_idx = ending_by_val(vf.ending_val);
    let is_bare = vf.prefix_display == prefix_display(0);

    // разбор: prefix + root- + suffix + ending, ` + `-joined (prefix dropped if bare)
    let mut razbor: Vec<String> = Vec::new();
    if !is_bare {
        razbor.push(vf.prefix_display.to_string());
    }
    razbor.push(format!("{}-", rd.name));
    razbor.push(suffix_display(suffix_idx).to_string());
    razbor.push(ending_display(ending_idx).to_string());
    let razbor = razbor.join(" + ");

    // морфемы: labelled segments, ` · `-joined (prefix segment dropped if bare)
    let mut morphemes: Vec<String> = Vec::new();
    if !is_bare {
        morphemes.push(format!("приставка «{}»", vf.prefix_display));
    }
    morphemes.push(match rd.gloss_ru {
        Some(g) => format!("корень {}- «{}»", rd.name, g),
        None => format!("корень {}-", rd.name),
    });
    morphemes.push(format!(
        "суффикс {} ({})",
        suffix_display(suffix_idx),
        suffix_gloss(suffix_idx)
    ));
    morphemes.push(ending_label(ending_idx).to_string());
    let morphemes = morphemes.join(" · ");

    let znachenie = vf.note.unwrap_or(NO_NOTE);

    let mut out = String::new();
    out.push_str(&format!("{}  ·  {}\n", vf.form, token));
    out.push_str(&format!("  значение : {znachenie}\n"));
    out.push_str(&format!("  разбор   : {razbor}\n"));
    out.push_str(&format!("  морфемы  : {morphemes}\n"));
    out.push_str(&format!(
        "  уровень  : {} — {}\n",
        token,
        attestation_reason(vf.attestation)
    ));
    out.push_str(&format!(
        "  домен    : {} · продуктивность {}\n",
        domain_inline(rd.domain),
        rd.productivity
    ));
    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::morpheme::Attestation;
    use crate::engine::paradigm::explore;
    use crate::engine::roots::root_data;

    /// Find the first form matching a (prefix_display, suffix_val, ending_val)
    /// triple in a root's paradigm.
    fn find_form(root: &str, prefix_display: &str, suffix_val: &str, ending_val: &str) -> VerbForm {
        explore(root, None)
            .expect("root explores")
            .forms
            .into_iter()
            .find(|f| {
                f.prefix_display == prefix_display
                    && f.suffix_val == suffix_val
                    && f.ending_val == ending_val
            })
            .expect("form should exist")
    }

    #[test]
    fn test_form_block_matches_operator_mockup() {
        // о-/об- + сса- + -ну- + -ть, Possible. Since #28 the fluid vowel makes
        // о- surface as обо- (обосснуть) — the display block is what is under test
        // here, not the morphonology; the form string is the vehicle.
        let rd = root_data("сса").unwrap();
        let vf = find_form("сса", "о-/об-", "ну", "ть");
        assert_eq!(vf.form, "обосснуть", "sanity: engine builds обосснуть");
        let block = format_form_block(rd, &vf);

        assert!(block.contains("обосснуть  ·  possible"), "header: {block}");
        assert!(
            block.contains("значение : — (в источнике не засвидетельствовано)"),
            "possible form has no source note: {block}"
        );
        assert!(
            block.contains("разбор   : о-/об- + сса- + -ну- + -ть"),
            "разбор: {block}"
        );
        assert!(
            block.contains(
                "морфемы  : приставка «о-/об-» · корень сса- «мочиться» · \
                 суффикс -ну- (однократный) · инфинитив -ть"
            ),
            "морфемы: {block}"
        );
        assert!(
            block.contains("уровень  : possible — словообразовательно возможна по аналогии"),
            "уровень explains why it is possible: {block}"
        );
        assert!(
            block.contains("домен    : экскреторная · продуктивность B"),
            "домен: {block}"
        );
    }

    #[test]
    fn test_form_block_28_corrected_form_is_self_explanatory() {
        // #28 fixed: вз-/вс- + сса- + -ну- + -нёт now surfaces with the fluid vowel
        // as взосснёт (no triple consonant). It is still an unverified edge form
        // (Possible), so the block must mark it possible + explain non-attestation,
        // making it self-explanatory — that self-explanation is what is under test.
        let rd = root_data("сса").unwrap();
        let vf = find_form("сса", "вз-/вс-", "ну", "нёт");
        assert_eq!(
            vf.form, "взосснёт",
            "sanity: engine builds the corrected form"
        );
        let block = format_form_block(rd, &vf);
        assert!(block.contains("взосснёт  ·  possible"), "{block}");
        assert!(
            block.contains("в источнике не засвидетельствована"),
            "{block}"
        );
    }

    #[test]
    fn test_form_block_bare_prefix_omits_prefix_segment() {
        // bare ебать: no "приставка …" segment, разбор starts at the root.
        let rd = root_data("еб").unwrap();
        let vf = find_form("еб", "(без)", "а", "ть");
        assert_eq!(vf.form, "ебать");
        let block = format_form_block(rd, &vf);
        assert!(
            block.contains("разбор   : еб- + -а- + -ть"),
            "bare разбор omits the prefix term: {block}"
        );
        assert!(
            !block.contains("приставка"),
            "bare form must not print a приставка segment: {block}"
        );
        assert!(
            !block.contains("(без)"),
            "bare form must not print «(без)»: {block}"
        );
        assert!(block.contains("корень еб- «совокупляться»"), "{block}");
    }

    #[test]
    fn test_form_block_common_level() {
        let rd = root_data("еб").unwrap();
        let vf = find_form("еб", "(без)", "а", "ть");
        assert_eq!(vf.attestation, Attestation::Common);
        let block = format_form_block(rd, &vf);
        assert!(block.contains("ебать  ·  common"), "{block}");
        assert!(
            block.contains("уровень  : common — широко засвидетельствована"),
            "{block}"
        );
        // A common form carries a source note, not the placeholder.
        assert!(
            block.contains("значение : совершать половой акт"),
            "{block}"
        );
    }

    #[test]
    fn test_form_block_rare_level() {
        // сра- + от-/ото- + -а- = Rare per group2 table.
        let rd = root_data("сра").unwrap();
        let vf = find_form("сра", "от-/ото-", "а", "ть");
        assert_eq!(vf.attestation, Attestation::Rare);
        let block = format_form_block(rd, &vf);
        assert!(block.contains("  ·  rare"), "{block}");
        assert!(
            block.contains("уровень  : rare — засвидетельствована редко"),
            "{block}"
        );
    }

    #[test]
    fn test_form_block_past_and_present_endings_labelled() {
        let rd = root_data("еб").unwrap();
        let past = find_form("еб", "(без)", "а", "л");
        assert!(
            format_form_block(rd, &past).contains("прош. время, м. р. ед. ч. -л"),
            "past ending label"
        );
        let present = find_form("еб", "(без)", "а", "ёт");
        assert!(
            format_form_block(rd, &present).contains("наст. время, 3 л. ед. ч. -ёт"),
            "present ending label"
        );
    }

    #[test]
    fn test_form_block_no_gloss_ru_root_prints_bare_root() {
        // Defensive: a hypothetical root without gloss_ru prints "корень x-"
        // without «…». Constructed directly since all 35 roots currently have one.
        use crate::engine::morpheme::{Domain, ProductivityClass};
        let rd = RootData {
            name: "x",
            val: "x",
            gloss: None,
            gloss_ru: None,
            suffix_indices: &[0],
            domain: Domain::Peripheral,
            productivity: ProductivityClass::E,
            present_stem: None,
            takes_fill_vowel: false,
            o_takes_ob: false,
            linguistic_note: "",
        };
        let vf = VerbForm {
            prefix_display: "вы-",
            suffix_val: "а",
            ending_val: "ть",
            form: "выхать".to_string(),
            attestation: Attestation::Possible,
            note: None,
        };
        let block = format_form_block(&rd, &vf);
        assert!(block.contains("корень x-"), "{block}");
        assert!(!block.contains("корень x- «"), "no gloss quotes: {block}");
    }
}
