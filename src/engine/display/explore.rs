//! `explore` formatter: the scannable overview table for a full paradigm, plus
//! per-form breakdown blocks when the result is narrowed by `--suffix`.

use crate::engine::morpheme::{
    ParadigmResult, VerbForm, suffix_display, suffix_gloss, suffix_index_for_val,
};
use crate::engine::roots::all_roots;

use super::form_block::format_form_block;
use super::{count_attestation, domain_inline, format_attestation};

// Column widths (in characters) for the overview table. Form columns are never
// cut (D2); only the meaning column is truncated to keep the table ≤ ~90 cols.
const W_PREFIX: usize = 11;
const W_INF: usize = 16;
const W_PAST: usize = 15;
const W_PRESENT: usize = 14;
const W_ATT: usize = 10;
const W_MEANING: usize = 34;

/// Format an explore result.
///
/// Full paradigm → header + morpheme legend + a scannable overview table (one
/// row per prefix×suffix, endings as columns) + tallies. A `--suffix`-narrowed
/// result additionally appends a full breakdown block per matching combination.
pub fn format_explore(result: &ParadigmResult) -> String {
    let mut out = String::new();

    // --- Header ---
    let mut header = format!("Корень: {}-", result.root_name);
    if let Some(gloss) = result.root_gloss_ru {
        header.push_str(&format!("  «{gloss}»"));
    }
    out.push_str(&header);
    out.push('\n');
    out.push_str(&format!(
        "  Домен: {} · продуктивность {}\n",
        domain_inline(result.root_domain),
        result.root_productivity
    ));

    // --- No verb forms: noun-only vs non-matching filter (unchanged honesty) ---
    if result.forms.is_empty() {
        let rd = all_roots().iter().find(|r| r.name == result.root_name);
        let is_noun_only = rd.map(|r| r.suffix_indices.is_empty()).unwrap_or(true);
        if is_noun_only {
            out.push_str("  Глагольная парадигма отсутствует, чисто именной корень.\n");
            out.push_str("  Именная деривация в текущей версии движка не реализована.\n");
            if let Some(rd) = rd {
                out.push_str(&format!("  Заметка: {}\n", rd.linguistic_note));
            }
        } else {
            out.push_str("  Нет форм по заданному фильтру.\n");
        }
        return out;
    }

    // Linguistic note in the header (previously not shown in explore).
    if let Some(rd) = all_roots().iter().find(|r| r.name == result.root_name) {
        out.push_str(&format!("  Заметка: {}\n", rd.linguistic_note));
    }
    out.push('\n');

    let form_refs: Vec<&VerbForm> = result.forms.iter().collect();

    // --- Combination tally ---
    let combos = unique_combos(&form_refs);
    let prefix_count = unique_prefixes(&form_refs).len();
    let suffix_count = unique_suffixes(&form_refs).len();
    out.push_str(&format!(
        "Всего комбинаций: {} префиксов × {} суффиксов = {} форм\n\n",
        prefix_count,
        suffix_count,
        combos.len()
    ));

    // --- Morpheme legend (once, so table rows need not repeat glosses) ---
    out.push_str(&format_legend(&form_refs));
    out.push('\n');

    // --- Overview table, grouped by suffix ---
    let groups = group_by_suffix(&form_refs);

    let (mut oc, mut orr, mut op, mut oi) = (0usize, 0usize, 0usize, 0usize);

    for (suffix_val_str, group) in &groups {
        let idx = suffix_index_for_val(suffix_val_str);
        out.push_str(&format!(
            "═══ Суффикс {} ({}) ═══\n\n",
            suffix_display(idx),
            suffix_gloss(idx)
        ));
        out.push_str(&table_header());

        let (mut c, mut r, mut p, mut i) = (0usize, 0usize, 0usize, 0usize);
        for prefix in unique_prefixes(group) {
            let row_forms: Vec<&VerbForm> = group
                .iter()
                .filter(|f| f.prefix_display == prefix)
                .copied()
                .collect();
            let inf = form_str(&row_forms, "ть");
            let past = form_str(&row_forms, "л");
            let present = present_form_str(&row_forms);
            // Attestation/note are per (prefix, suffix) — read from any ending.
            let rep = row_forms[0];
            count_attestation(&mut c, &mut r, &mut p, &mut i, rep.attestation);
            let note = rep.note.unwrap_or("—");
            out.push_str(&format!(
                "  {:<wp$} | {:<wi$} | {:<wpa$} | {:<wpr$} | {:<wa$} | {}\n",
                prefix,
                inf,
                past,
                present,
                format_attestation(rep.attestation),
                truncate(note, W_MEANING),
                wp = W_PREFIX,
                wi = W_INF,
                wpa = W_PAST,
                wpr = W_PRESENT,
                wa = W_ATT,
            ));
        }

        out.push_str(&format!(
            "\n  Итого: common ({c}), rare ({r}), possible ({p}), impossible ({i})\n\n"
        ));
        oc += c;
        orr += r;
        op += p;
        oi += i;
    }

    out.push_str(&format!(
        "Всего: common ({oc}), rare ({orr}), possible ({op}), impossible ({oi})\n"
    ));

    // --- Narrowed result: append a full breakdown block per combination ---
    if result.suffix_filter.is_some() {
        out.push('\n');
        out.push_str("Разбор форм:\n\n");
        let rd = all_roots()
            .iter()
            .find(|r| r.name == result.root_name)
            .expect("result root exists in inventory");
        let mut first = true;
        for (_suffix, group) in &groups {
            for prefix in unique_prefixes(group) {
                let row_forms: Vec<&VerbForm> = group
                    .iter()
                    .filter(|f| f.prefix_display == prefix)
                    .copied()
                    .collect();
                // Representative = infinitive.
                if let Some(vf) = row_forms.iter().copied().find(|f| f.ending_val == "ть") {
                    if !first {
                        out.push('\n');
                    }
                    first = false;
                    out.push_str(&format_form_block(rd, vf));
                }
            }
        }
    }

    out
}

fn table_header() -> String {
    let mut s = format!(
        "  {:<wp$} | {:<wi$} | {:<wpa$} | {:<wpr$} | {:<wa$} | {}\n",
        "Префикс",
        "Инфинитив",
        "Прош. -л",
        "Наст. 3л",
        "Аттест.",
        "Значение",
        wp = W_PREFIX,
        wi = W_INF,
        wpa = W_PAST,
        wpr = W_PRESENT,
        wa = W_ATT,
    );
    s.push_str(&format!(
        "  {}-+-{}-+-{}-+-{}-+-{}-+-{}\n",
        "-".repeat(W_PREFIX),
        "-".repeat(W_INF),
        "-".repeat(W_PAST),
        "-".repeat(W_PRESENT),
        "-".repeat(W_ATT),
        "-".repeat(8),
    ));
    s
}

/// The morpheme legend: suffixes (with glosses), endings (as columns), prefixes.
fn format_legend(forms: &[&VerbForm]) -> String {
    let mut out = String::from("Легенда:\n");

    let suffixes: Vec<String> = unique_suffixes(forms)
        .iter()
        .map(|&sv| {
            let idx = suffix_index_for_val(sv);
            format!("{} ({})", suffix_display(idx), suffix_gloss(idx))
        })
        .collect();
    out.push_str(&format!("  суффиксы: {}\n", suffixes.join(", ")));

    // Endings, by column meaning; present forms vary by suffix class.
    let mut present: Vec<&str> = Vec::new();
    for f in forms {
        if is_present(f.ending_val) && !present.contains(&f.ending_val) {
            present.push(f.ending_val);
        }
    }
    let present_disp: Vec<String> = present.iter().map(|e| format!("-{e}")).collect();
    out.push_str(&format!(
        "  окончания (колонки): Инфинитив -ть · Прош. -л · Наст. 3 л. {}\n",
        present_disp.join("/")
    ));

    let prefixes = unique_prefixes(forms);
    out.push_str(&format!("  префиксы: {}\n", prefixes.join(", ")));
    out
}

// --- small helpers over the form list ---

fn is_present(ending_val: &str) -> bool {
    matches!(ending_val, "ёт" | "нёт" | "ит")
}

/// The word for the ending `ending_val` among `row_forms`, or "—".
fn form_str(row_forms: &[&VerbForm], ending_val: &str) -> String {
    row_forms
        .iter()
        .find(|f| f.ending_val == ending_val)
        .map(|f| f.form.clone())
        .unwrap_or_else(|| "—".to_string())
}

/// The present-tense word among `row_forms` (any of ёт/нёт/ит), or "—".
fn present_form_str(row_forms: &[&VerbForm]) -> String {
    row_forms
        .iter()
        .find(|f| is_present(f.ending_val))
        .map(|f| f.form.clone())
        .unwrap_or_else(|| "—".to_string())
}

fn unique_prefixes<'a>(forms: &[&'a VerbForm]) -> Vec<&'a str> {
    let mut seen: Vec<&str> = Vec::new();
    for f in forms {
        if !seen.contains(&f.prefix_display) {
            seen.push(f.prefix_display);
        }
    }
    seen
}

fn unique_suffixes<'a>(forms: &[&'a VerbForm]) -> Vec<&'a str> {
    let mut seen: Vec<&str> = Vec::new();
    for f in forms {
        if !seen.contains(&f.suffix_val) {
            seen.push(f.suffix_val);
        }
    }
    seen
}

fn unique_combos<'a>(forms: &[&'a VerbForm]) -> Vec<(&'a str, &'a str)> {
    let mut seen: Vec<(&str, &str)> = Vec::new();
    for f in forms {
        let key = (f.prefix_display, f.suffix_val);
        if !seen.contains(&key) {
            seen.push(key);
        }
    }
    seen
}

fn group_by_suffix<'a>(forms: &[&'a VerbForm]) -> Vec<(&'a str, Vec<&'a VerbForm>)> {
    let mut groups: Vec<(&str, Vec<&VerbForm>)> = Vec::new();
    for f in forms {
        if let Some(idx) = groups.iter().position(|(k, _)| *k == f.suffix_val) {
            groups[idx].1.push(f);
        } else {
            groups.push((f.suffix_val, vec![f]));
        }
    }
    groups
}

/// Truncate to `max` characters (char-based, for Cyrillic), adding an ellipsis.
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let head: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{head}…")
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::paradigm::explore;

    #[test]
    fn test_format_explore_header_has_russian_gloss() {
        let result = explore("еб", None).expect("еб should be valid");
        let output = format_explore(&result);
        assert!(output.contains("Корень: еб-"), "root name: {output}");
        assert!(
            output.contains("«совокупляться»"),
            "header shows the Russian gloss, not English: {output}"
        );
        assert!(
            !output.contains("fuck, copulate"),
            "English gloss must not appear in output: {output}"
        );
    }

    #[test]
    fn test_format_explore_shows_domain_and_productivity() {
        let result = explore("еб", None).expect("еб should be valid");
        let output = format_explore(&result);
        assert!(output.contains("Домен: ядро"), "{output}");
        assert!(output.contains("продуктивность A"), "{output}");
    }

    #[test]
    fn test_format_explore_has_legend_and_note() {
        let result = explore("еб", None).expect("еб should be valid");
        let output = format_explore(&result);
        assert!(output.contains("Легенда:"), "legend present: {output}");
        assert!(
            output.contains("имперфектив"),
            "suffix gloss in legend: {output}"
        );
        assert!(
            output.contains("Заметка:"),
            "linguistic note shown: {output}"
        );
    }

    #[test]
    fn test_format_explore_has_ending_columns() {
        let result = explore("еб", None).expect("еб should be valid");
        let output = format_explore(&result);
        assert!(output.contains("Инфинитив"), "{output}");
        assert!(output.contains("Прош. -л"), "past column: {output}");
        assert!(output.contains("Наст. 3л"), "present column: {output}");
        // The actual past/present forms are now visible (were hidden before).
        assert!(output.contains("ебал"), "past form ебал visible: {output}");
        assert!(
            output.contains("ебёт"),
            "present form ебёт visible: {output}"
        );
    }

    #[test]
    fn test_format_explore_contains_suffix_sections() {
        let result = explore("еб", None).expect("еб should be valid");
        let output = format_explore(&result);
        assert!(output.contains("Суффикс"), "{output}");
    }

    #[test]
    fn test_format_explore_filtered_appends_form_blocks() {
        // Scenario 1: explore сса --suffix -ну- → short table + block for осснуть.
        let result = explore("сса", Some("ну")).expect("сса should be valid");
        let output = format_explore(&result);
        assert!(output.contains("Разбор форм:"), "detail section: {output}");
        assert!(
            output.contains("осснуть  ·  possible"),
            "осснуть block: {output}"
        );
        assert!(
            output.contains("разбор   : о-/об- + сса- + -ну- + -ть"),
            "{output}"
        );
        assert!(output.contains("корень сса- «мочиться»"), "{output}");
    }

    #[test]
    fn test_format_explore_full_paradigm_has_no_detail_blocks() {
        // No filter → overview table only, no per-form breakdown blocks.
        let result = explore("еб", None).expect("еб should be valid");
        let output = format_explore(&result);
        assert!(
            !output.contains("Разбор форм:"),
            "full paradigm must stay a table, no detail blocks: {output}"
        );
    }

    #[test]
    fn test_format_explore_empty_forms_message() {
        let result = explore("манд", None).expect("манд should be valid");
        let output = format_explore(&result);
        assert!(
            output.contains("Глагольная парадигма отсутствует"),
            "{output}"
        );
        assert!(!output.contains("v0.5"), "message must be version-neutral");
    }

    #[test]
    fn test_format_explore_verbal_root_empty_filter_not_noun_only() {
        let result = explore("еб", Some("бред")).expect("еб should be valid");
        assert!(result.forms.is_empty());
        let output = format_explore(&result);
        assert!(output.contains("Нет форм по заданному фильтру"), "{output}");
        assert!(!output.contains("именной корень"), "{output}");
    }

    #[test]
    fn test_format_explore_group_5_noun_only() {
        let result = explore("сиповк", None).expect("сиповк should be valid");
        let output = format_explore(&result);
        assert!(
            output.contains("Глагольная парадигма отсутствует"),
            "{output}"
        );
    }

    #[test]
    fn test_format_explore_for_sra_contains_sra() {
        let result = explore("сра", None).expect("сра should be valid");
        let output = format_explore(&result);
        assert!(output.contains("сра-"), "{output}");
    }

    #[test]
    fn test_format_explore_for_pizd_has_ei_section() {
        let result = explore("пизд", None).expect("пизд should be valid");
        let output = format_explore(&result);
        assert!(output.contains("-е-"), "{output}");
        assert!(output.contains("пиздеть"), "{output}");
        assert!(output.contains("-и-"), "{output}");
    }

    #[test]
    fn test_format_explore_droch_has_i_section() {
        let result = explore("дроч", None).expect("дроч should be valid");
        let output = format_explore(&result);
        assert!(output.contains("-и-"), "{output}");
        assert!(output.contains("дрочить"), "{output}");
    }
}
