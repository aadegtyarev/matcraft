//! Output formatting for the paradigm explorer, root list, and random display.

use crate::engine::morpheme::{
    Attestation, ParadigmResult, RootData, all_roots, suffix_display, suffix_gloss,
};
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

    // Handle roots with no verb forms (e.g., манд-, елд-)
    if result.forms.is_empty() {
        out.push_str("  Нет глагольных форм для данного корня.\n");
        out.push_str("  Полная парадигма включает именную деривацию (отложено до v0.5+).\n");
        let rd = all_roots().iter().find(|r| r.name == result.root_name);
        if let Some(rd) = rd {
            out.push_str(&format!("  Заметка: {}\n", rd.linguistic_note));
        }
        return out;
    }

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
        let section_title = format!(
            "═══ Суффикс {} ({}) ═══",
            suffix_display(get_suffix_index(suffix_val_str)),
            gloss
        );
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

            count_attestation(
                &mut common,
                &mut rare,
                &mut possible,
                &mut impossible,
                vf.attestation,
            );

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
        let gloss_str = rd
            .and_then(|r| r.gloss)
            .map(|g| format!("'{}'", g))
            .unwrap_or_default();
        out.push_str(&format!("  {}-   {}\n", root_name, gloss_str));
    }
    out.push('\n');
    let root_form = if roots.len() == 1 {
        "корень"
    } else if roots.len() < 5 {
        "корня"
    } else {
        "корней"
    };
    out.push_str(&format!("  Всего: {} {}\n", roots.len(), root_form));
    out.push_str("  Команда: matcraft explore <корень> для полной парадигмы\n");

    out
}

// ---------------------------------------------------------------------------
// Random display
// ---------------------------------------------------------------------------

/// Format a random root display with linguistic note.
///
/// Produces a boxed output like:
///
/// ```text
/// ╔══════════════════════════════════════════╗
/// ║  Корень: еб- ...                         ║
/// ╠══════════════════════════════════════════╣
/// ║  Суффиксальные классы: -а-, -ну-         ║
/// ║  Примеры: заебать, ебнуть, выебать       ║
/// ║                                           ║
/// ║  Заметка:                                ║
/// ║  Самый продуктивный матерный корень.     ║
/// ║  ...                                      ║
/// ╚══════════════════════════════════════════╝
/// ```
pub fn format_random(rd: &RootData, sample_forms: &[&str]) -> String {
    let mut out = String::new();

    // Box width: 44 chars (2 border + 40 content + 2 spaces)
    let content_width = 40;

    // Top border
    out.push_str(&format!("╔{}╗\n", "═".repeat(content_width + 4)));

    // Root name line
    let gloss_str = rd.gloss.map(|g| format!("'{}'", g)).unwrap_or_default();
    let name_line = format!("Корень: {}-  {}", rd.name, gloss_str);
    out.push_str(&format!(
        "║  {:<width$} ║\n",
        name_line,
        width = content_width + 2
    ));

    // Separator
    out.push_str(&format!("╠{}╣\n", "═".repeat(content_width + 4)));

    // Suffix classes
    let suffix_strs: Vec<String> = rd
        .suffix_indices
        .iter()
        .map(|&idx| suffix_display(idx).to_string())
        .collect();
    let suffix_classes = if suffix_strs.is_empty() {
        "Нет (именной корень)".to_string()
    } else {
        suffix_strs.join(", ")
    };
    let suffix_line = format!("Суффиксальные классы: {}", suffix_classes);
    out.push_str(&format!(
        "║  {:<width$} ║\n",
        suffix_line,
        width = content_width + 2
    ));

    // Sample forms
    if sample_forms.is_empty() {
        out.push_str(&format!(
            "║  {:<width$} ║\n",
            "Примеры: не указаны",
            width = content_width + 2
        ));
    } else {
        let examples = format!("Примеры: {}", sample_forms.join(", "));
        out.push_str(&format!(
            "║  {:<width$} ║\n",
            examples,
            width = content_width + 2
        ));
    }

    // Blank line
    out.push_str(&format!("║{:<width$}║\n", "", width = content_width + 4));

    // Linguistic note header
    out.push_str(&format!(
        "║  {:<width$} ║\n",
        "Заметка:",
        width = content_width + 2
    ));

    // Wrap and print the note
    let wrapped = wrap_text(rd.linguistic_note, content_width);
    for line in &wrapped {
        out.push_str(&format!(
            "║  {:<width$} ║\n",
            line,
            width = content_width + 2
        ));
    }

    // Bottom border
    out.push_str(&format!("╚{}╝\n", "═".repeat(content_width + 4)));

    out
}

/// Simple word-wrapping: split text at word boundaries to fit `line_width`.
fn wrap_text(text: &str, line_width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 > line_width && !current.is_empty() {
            result.push(current.clone());
            current.clear();
        }
        if current.is_empty() {
            current.push_str(word);
        } else {
            current.push(' ');
            current.push_str(word);
        }
    }
    if !current.is_empty() {
        result.push(current);
    }
    if result.is_empty() {
        result.push(String::new());
    }
    result
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
        "е" => 2,
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
        assert!(
            output.contains("common"),
            "Output should contain attestation labels"
        );
    }

    #[test]
    fn test_format_explore_contains_suffix_sections() {
        let result = explore("еб", None).expect("еб should be valid");
        let output = format_explore(&result);
        assert!(
            output.contains("Суффикс"),
            "Output should contain suffix sections"
        );
    }

    #[test]
    fn test_format_list_roots_contains_eb() {
        let output = format_list_roots();
        assert!(output.contains("еб"), "Output should contain еб root");
        assert!(
            output.contains("корней"),
            "Output should contain root count in Russian"
        );
    }

    #[test]
    fn test_format_explore_empty_forms_message() {
        // манд- has empty suffix_indices → no forms
        let result = explore("манд", None).expect("манд should be valid");
        let output = format_explore(&result);
        assert!(
            output.contains("Нет глагольных форм"),
            "Output should show empty message"
        );
    }

    #[test]
    fn test_format_random_contains_root_name() {
        let rd = crate::engine::morpheme::root_data("еб").unwrap();
        let samples = vec!["ебать", "ебнуть"];
        let output = format_random(rd, &samples);
        assert!(
            output.contains("еб-"),
            "Random output should contain root name"
        );
        assert!(
            output.contains("═"),
            "Random output should have box borders"
        );
        assert!(
            output.contains("Заметка:"),
            "Random output should have note section"
        );
    }

    #[test]
    fn test_format_random_empty_forms() {
        let rd = crate::engine::morpheme::root_data("манд").unwrap();
        let samples: Vec<&str> = Vec::new();
        let output = format_random(rd, &samples);
        assert!(
            output.contains("не указаны"),
            "Empty samples should show fallback"
        );
    }

    #[test]
    fn test_wrap_text_short() {
        let result = wrap_text("Короткий текст", 40);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "Короткий текст");
    }

    #[test]
    fn test_wrap_text_long() {
        let text = "а б в г д е ё ж з и й к л м н о п р с т у ф х ц ч ш щ ъ ы ь э ю я";
        let result = wrap_text(text, 20);
        assert!(
            result.len() > 1,
            "Long text should be split into multiple lines"
        );
        for line in &result {
            assert!(line.len() <= 20, "Each line should fit width");
        }
    }

    #[test]
    fn test_format_explore_for_sra_contains_sra() {
        let result = explore("сра", None).expect("сра should be valid");
        let output = format_explore(&result);
        assert!(
            output.contains("сра-"),
            "Output should contain root name сра-"
        );
    }

    #[test]
    fn test_format_explore_for_pizd_has_ei_section() {
        let result = explore("пизд", None).expect("пизд should be valid");
        let output = format_explore(&result);
        assert!(
            output.contains("-е-/-и-"),
            "Output should contain -е-/-и- suffix class"
        );
        assert!(
            output.contains("пиздеть"),
            "Output should contain пиздеть form"
        );
    }

    #[test]
    fn test_format_list_roots_shows_all_9() {
        let output = format_list_roots();
        assert!(output.contains("9 корней"), "Output should say '9 корней'");
        assert!(output.contains("еб"), "Output should contain еб");
        assert!(output.contains("сра"), "Output should contain сра");
        assert!(output.contains("пизд"), "Output should contain пизд");
        assert!(output.contains("хуй"), "Output should contain хуй");
    }
}
