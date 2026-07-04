//! Output formatting for the paradigm explorer and root list.

use crate::engine::morpheme::{Attestation, ParadigmResult, suffix_gloss, suffix_display};
use crate::engine::paradigm::list_roots;

/// Format an explore result as a human-readable table.
///
/// Produces a grouped table:
///
/// ```text
/// Корень: еб- 'fuck, copulate'
/// Всего комбинаций: 9 префиксов × 2 суффикса = 18 форм
///
/// ═══ Суффикс -а- (имперфектив) ═══
///
///   Префикс    | Форма (инф)       | Аттестация | Значение
///   -----------+-------------------+------------+----------------------
///   (без)       | ебать             | common     | совершать половой акт
///   вы-         | выебать           | common     | износить, испортить
///   ...
///
///   Итого: common (6), rare (3), possible (0), impossible (0)
///
/// ═══ Суффикс -ну- (однократный) ═══
///   ...
/// ```
pub fn format_explore(result: &ParadigmResult) -> String {
    let mut out = String::new();

    // Header
    let root_display = match result.root_gloss {
        Some(gloss) => format!("Корень: {}- '{}'", result.root_name, gloss),
        None => format!("Корень: {}-", result.root_name),
    };
    out.push_str(&root_display);
    out.push('\n');

    // Count unique prefix×suffix combinations
    let mut combo_count = 0;
    let mut seen_combos: Vec<(&str, &str)> = Vec::new();
    for vf in &result.forms {
        let key = (vf.prefix_display, vf.suffix_val);
        if !seen_combos.contains(&key) {
            seen_combos.push(key);
            combo_count += 1;
        }
    }
    let prefix_count = {
        let mut seen_prefixes: Vec<&str> = Vec::new();
        for vf in &result.forms {
            if !seen_prefixes.contains(&vf.prefix_display) {
                seen_prefixes.push(vf.prefix_display);
            }
        }
        seen_prefixes.len()
    };
    let suffix_count = {
        let mut seen_suffixes: Vec<&str> = Vec::new();
        for vf in &result.forms {
            if !seen_suffixes.contains(&vf.suffix_val) {
                seen_suffixes.push(vf.suffix_val);
            }
        }
        seen_suffixes.len()
    };

    out.push_str(&format!(
        "Всего комбинаций: {} префиксов × {} суффиксов = {} форм\n\n",
        prefix_count, suffix_count, combo_count
    ));

    // Group forms by suffix
    let mut groups: Vec<(&str, Vec<&crate::engine::morpheme::VerbForm>)> = Vec::new();
    for vf in &result.forms {
        // Find the right group
        let group_idx = groups.iter().position(|(k, _)| *k == vf.suffix_val);
        if let Some(idx) = group_idx {
            groups[idx].1.push(vf);
        } else {
            groups.push((vf.suffix_val, vec![vf]));
        }
    }

    let mut overall_common = 0usize;
    let mut overall_rare = 0usize;
    let mut overall_possible = 0usize;
    let mut overall_impossible = 0usize;

    for (suffix_val_str, group) in &groups {
        // Section header — only infinitive-ending forms in the main table
        let gloss = suffix_gloss(get_suffix_index(suffix_val_str));
        let section_title = format!("═══ Суффикс {} ({}) ═══", suffix_display(get_suffix_index(suffix_val_str)), gloss);
        out.push_str(&section_title);
        out.push_str("\n\n");

        // Table header
        out.push_str("  Префикс    | Форма (инф)       | Аттестация | Значение\n");
        out.push_str("  -----------+-------------------+------------+----------------------\n");

        let mut common = 0usize;
        let mut rare = 0usize;
        let mut possible = 0usize;
        let mut impossible = 0usize;

        // Show only infinitive forms in the table (ending = "ть")
        for vf in group.iter().filter(|f| f.ending_val == "ть") {
            let att_str = format_attestation(vf.attestation);
            let note_str = vf.note.unwrap_or("—");

            count_attestation(&mut common, &mut rare, &mut possible, &mut impossible, vf.attestation);

            // Pad columns
            out.push_str(&format!(
                "  {:<12} | {:<17} | {:<10} | {}\n",
                vf.prefix_display, vf.form, att_str, note_str
            ));
        }

        out.push_str(&format!(
            "\n  Итого: common ({}), rare ({}), possible ({}), impossible ({})\n\n",
            common, rare, possible, impossible
        ));

        overall_common += common;
        overall_rare += rare;
        overall_possible += possible;
        overall_impossible += impossible;
    }

    // Overall summary
    out.push_str(&format!(
        "Всего: common ({}), rare ({}), possible ({}), impossible ({})\n",
        overall_common, overall_rare, overall_possible, overall_impossible
    ));

    out
}

/// Format the root list output.
pub fn format_list_roots() -> String {
    let roots = list_roots();
    let mut out = String::new();

    out.push_str(&format!("Доступные корни ({}):\n", roots.len()));
    for root_name in &roots {
        let rd = crate::engine::morpheme::root_data(root_name);
        let gloss_str = rd.and_then(|r| r.gloss).map(|g| format!("'{}'", g)).unwrap_or_default();
        out.push_str(&format!("  {}-   {}\n", root_name, gloss_str));
    }
    out.push('\n');
    let root_form = if roots.len() == 1 { "корень" } else if roots.len() < 5 { "корня" } else { "корней" };
    out.push_str(&format!("  Всего: {} {}\n", roots.len(), root_form));
    out.push_str("  Команда: matcraft explore <корень> для полной парадигмы\n");

    out
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn format_attestation(att: Attestation) -> &'static str {
    match att {
        Attestation::Common => "common",
        Attestation::Rare => "rare",
        Attestation::Possible => "possible",
        Attestation::Impossible => "impossible",
    }
}

fn count_attestation(
    common: &mut usize,
    rare: &mut usize,
    possible: &mut usize,
    impossible: &mut usize,
    att: Attestation,
) {
    match att {
        Attestation::Common => *common += 1,
        Attestation::Rare => *rare += 1,
        Attestation::Possible => *possible += 1,
        Attestation::Impossible => *impossible += 1,
    }
}

/// Get the suffix index for a suffix value string.
fn get_suffix_index(val: &str) -> usize {
    match val {
        "а" => 0,
        "ну" => 1,
        _ => 0,
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
    fn test_format_explore_contains_root() {
        let result = explore("еб", None).expect("еб should be valid");
        let output = format_explore(&result);
        assert!(output.contains("еб-"), "Output should contain root name");
        assert!(output.contains("common"), "Output should contain attestation labels");
    }

    #[test]
    fn test_format_explore_contains_suffix_sections() {
        let result = explore("еб", None).expect("еб should be valid");
        let output = format_explore(&result);
        assert!(output.contains("Суффикс"), "Output should contain suffix sections");
    }

    #[test]
    fn test_format_list_roots_contains_eb() {
        let output = format_list_roots();
        assert!(output.contains("еб"), "Output should contain еб root");
        assert!(output.contains("корень"), "Output should contain root count in Russian");
    }
}
