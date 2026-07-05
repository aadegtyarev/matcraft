//! Output formatting for the paradigm explorer, root list, and random display.

use crate::engine::morpheme::{
    Attestation, Domain, Mode, ParadigmResult, RootData, suffix_display, suffix_gloss,
    suffix_index_for_val,
};
use crate::engine::paradigm::list_roots;
use crate::engine::roots::all_roots;

/// Format an explore result as a human-readable table.
///
/// Produces a grouped table:
///
/// ```text
/// Корень: еб- 'fuck, copulate'
///   Домен: ядро · продуктивность A
///
/// ═══ Суффикс -а- (имперфектив) ═══
/// ...
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

    // Domain + productivity (source §1 / §2)
    out.push_str(&format!(
        "  Домен: {} · продуктивность {}\n",
        domain_inline(result.root_domain),
        result.root_productivity
    ));

    // Handle roots with no verb forms (noun-only roots, or empty suffix_indices)
    if result.forms.is_empty() {
        out.push_str("  Глагольная парадигма отсутствует, чисто именной корень.\n");
        out.push_str("  Именная деривация в текущей версии движка не реализована.\n");
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
        let suffix_section_idx = groups.iter().position(|(k, _)| *k == vf.suffix_val);
        if let Some(idx) = suffix_section_idx {
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
        let idx = suffix_index_for_val(suffix_val_str);
        let section_title = format!(
            "═══ Суффикс {} ({}) ═══",
            suffix_display(idx),
            suffix_gloss(idx)
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

/// Format the root list output for a given mode.
pub fn format_list_roots(mode: Mode) -> String {
    let roots = list_roots(mode);
    let mut out = String::new();

    match mode {
        Mode::Classic => {
            out.push_str(&format!("Доступные корни ({}):\n", roots.len()));
            for name in &roots {
                if let Some(rd) = all_roots().iter().find(|r| r.name == *name) {
                    out.push_str(&format_root_line(rd));
                }
            }
        }
        Mode::Full => {
            out.push_str(&format!("Все корни ({}):\n", roots.len()));
            // Group by semantic domain (source §1), in inventory order.
            for domain in [Domain::Nuclear, Domain::Excretory, Domain::Peripheral] {
                out.push_str(&format!("  --- {} ---\n", domain_list_header(domain)));
                for rd in all_roots().iter().filter(|r| r.domain == domain) {
                    out.push_str(&format_root_line(rd));
                }
            }
        }
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

    if matches!(mode, Mode::Classic) {
        out.push_str(
            "  Используйте --mode full для всех 35 корней (включая дрочить, жрать, говно, ...)\n",
        );
    }

    out
}

/// One list line for a root: name, gloss, productivity class, and noun-only marker.
fn format_root_line(rd: &RootData) -> String {
    let gloss_str = rd.gloss.map(|g| format!("'{}'", g)).unwrap_or_default();
    let noun_marker = if rd.suffix_indices.is_empty() {
        " (только именной)"
    } else {
        ""
    };
    format!(
        "  {}-   {} [{}]{}\n",
        rd.name, gloss_str, rd.productivity, noun_marker
    )
}

/// Domain header for the full-list grouping.
fn domain_list_header(domain: Domain) -> &'static str {
    match domain {
        Domain::Nuclear => "Ядро",
        Domain::Excretory => "Экскреторная",
        Domain::Peripheral => "Периферия",
    }
}

/// Domain name inline in the explore header.
fn domain_inline(domain: Domain) -> &'static str {
    match domain {
        Domain::Nuclear => "ядро",
        Domain::Excretory => "экскреторная",
        Domain::Peripheral => "периферия",
    }
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

    // Box width: 46 chars total. The content field is `content_width` chars; the
    // border row is `content_width + 4` (2 frame chars + 2 padding). A content row
    // is `║` + 2 leading spaces + a field padded to `content_width + 1` + 1 trailing
    // space + `║` = 1 + 2 + (content_width + 1) + 1 + 1 = content_width + 6, matching
    // the border. Every content line is wrapped at `content_width` so nothing longer
    // than the field overflows the frame (the note, name, suffix, and example lines
    // alike).
    let content_width = 40;

    // Push a content line (or lines), wrapping at the field width so no text
    // overflows the frame.
    let push_content = |out: &mut String, text: &str| {
        for line in wrap_text(text, content_width) {
            out.push_str(&format!(
                "║  {:<width$} ║\n",
                line,
                width = content_width + 1
            ));
        }
    };

    // Top border
    out.push_str(&format!("╔{}╗\n", "═".repeat(content_width + 4)));

    // Root name line
    let gloss_str = rd.gloss.map(|g| format!("'{}'", g)).unwrap_or_default();
    let name_line = format!("Корень: {}-  {}", rd.name, gloss_str);
    push_content(&mut out, &name_line);

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
    push_content(
        &mut out,
        &format!("Суффиксальные классы: {}", suffix_classes),
    );

    // Sample forms
    if sample_forms.is_empty() && rd.suffix_indices.is_empty() {
        push_content(&mut out, "Примеры: именной корень (нет глагольных форм)");
    } else if sample_forms.is_empty() {
        push_content(&mut out, "Примеры: не указаны");
    } else {
        push_content(&mut out, &format!("Примеры: {}", sample_forms.join(", ")));
    }

    // Blank line
    out.push_str(&format!("║{:<width$}║\n", "", width = content_width + 4));

    // Linguistic note header
    push_content(&mut out, "Заметка:");

    // Wrap and print the note
    push_content(&mut out, rd.linguistic_note);

    // Bottom border
    out.push_str(&format!("╚{}╝\n", "═".repeat(content_width + 4)));

    out
}

/// Simple word-wrapping: split text at word boundaries to fit `line_width`.
///
/// Widths are measured in characters, not bytes, so Cyrillic text (2 bytes per
/// letter in UTF-8) wraps at the intended column rather than half of it.
fn wrap_text(text: &str, line_width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.chars().count() + word.chars().count() + 1 > line_width && !current.is_empty() {
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::paradigm::explore;
    use crate::engine::roots::root_data;

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
    fn test_format_explore_shows_domain_and_productivity() {
        let result = explore("еб", None).expect("еб should be valid");
        let output = format_explore(&result);
        assert!(
            output.contains("Домен: ядро"),
            "Output should name the domain"
        );
        assert!(
            output.contains("продуктивность A"),
            "еб- should show productivity A"
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
    fn test_format_list_roots_classic_contains_eb() {
        let output = format_list_roots(Mode::Classic);
        assert!(output.contains("еб"), "Output should contain еб root");
        assert!(
            output.contains("корней"),
            "Output should contain root count in Russian"
        );
        assert!(output.contains("9"), "Classic mode should show 9 roots");
    }

    #[test]
    fn test_format_list_roots_full_contains_35() {
        let output = format_list_roots(Mode::Full);
        assert!(output.contains("35"), "Full mode should show 35 roots");
    }

    #[test]
    fn test_format_list_roots_full_contains_domain_headers() {
        let output = format_list_roots(Mode::Full);
        assert!(output.contains("Ядро"), "Should show Nuclear domain header");
        assert!(
            output.contains("Экскреторная"),
            "Should show Excretory domain header"
        );
        assert!(
            output.contains("Периферия"),
            "Should show Peripheral domain header"
        );
    }

    #[test]
    fn test_format_list_roots_shows_productivity() {
        let output = format_list_roots(Mode::Full);
        assert!(
            output.contains("[A]"),
            "Full list should show productivity class markers"
        );
    }

    #[test]
    fn test_format_explore_empty_forms_message() {
        let result = explore("манд", None).expect("манд should be valid");
        let output = format_explore(&result);
        assert!(
            output.contains("Глагольная парадигма отсутствует"),
            "Output should show noun-only message"
        );
        assert!(
            !output.contains("v0.5"),
            "Noun-only message must be version-neutral"
        );
    }

    #[test]
    fn test_format_explore_group_5_noun_only() {
        let result = explore("сиповк", None).expect("сиповк should be valid");
        let output = format_explore(&result);
        assert!(
            output.contains("Глагольная парадигма отсутствует"),
            "Noun-only root should show noun-only message"
        );
    }

    #[test]
    fn test_format_random_contains_root_name() {
        let rd = root_data("еб").unwrap();
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
        let rd = root_data("манд").unwrap();
        let samples: Vec<&str> = Vec::new();
        let output = format_random(rd, &samples);
        assert!(
            output.contains("именной корень"),
            "Empty samples for noun-only root should show message"
        );
    }

    #[test]
    fn test_format_random_verb_root_no_samples() {
        // хуй- has verb classes but no Common forms post-grounding; `random` passes
        // empty samples and the display must show "не указаны", not crash.
        let rd = root_data("хуй").unwrap();
        let samples: Vec<&str> = Vec::new();
        let output = format_random(rd, &samples);
        assert!(
            output.contains("не указаны"),
            "verb root with no sample forms should show 'не указаны'"
        );
    }

    #[test]
    fn test_format_random_group_5() {
        let rd = root_data("сиповк").unwrap();
        let samples: Vec<&str> = Vec::new();
        let output = format_random(rd, &samples);
        assert!(
            output.contains("именной корень"),
            "Noun-only root should show noun-only message"
        );
    }

    #[test]
    fn test_format_random_box_lines_equal_width() {
        // The box is only aligned if every rendered line — top/bottom border,
        // separator, blank line, and all content lines — has the same char count.
        // This pins the box arithmetic against regression; the `contains`-based
        // tests above never check width. Both the empty-samples branch (whose
        // "именной корень" message is longer than the field) and the populated
        // branch are exercised, across noun-only and verb roots.
        for name in ["еб", "манд", "хуй", "сиповк", "говн", "хар", "елд"] {
            let rd = root_data(name).unwrap();
            for samples in [Vec::new(), vec!["ебать", "ебнуть"]] {
                let output = format_random(rd, &samples);
                let widths: Vec<usize> = output
                    .lines()
                    .filter(|l| !l.is_empty())
                    .map(|l| l.chars().count())
                    .collect();
                let first = widths[0];
                assert!(
                    widths.iter().all(|&w| w == first),
                    "all box lines must share one width for root {name} \
                     (samples={samples:?}): got {widths:?}"
                );
            }
        }
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
            // Measured in characters (the fix): a Cyrillic line must fit the
            // char-column width, not the byte width.
            assert!(
                line.chars().count() <= 20,
                "Each line should fit width in characters"
            );
        }
    }

    #[test]
    fn test_wrap_text_fills_to_char_width() {
        // Regression for the byte-vs-char bug: with char counting, a line of
        // Cyrillic words packs close to the width instead of ~half of it.
        let text = "аб вг де жз ик лм но пр ст уф хц чш";
        let result = wrap_text(text, 20);
        assert!(
            result.iter().any(|l| l.chars().count() > 10),
            "char-based wrapping should pack more than ~10 chars per line"
        );
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
            output.contains("-е-"),
            "Output should contain -е- suffix class"
        );
        assert!(
            output.contains("пиздеть"),
            "Output should contain пиздеть form"
        );
        assert!(
            output.contains("-и-"),
            "Output should contain -и- suffix class"
        );
    }

    #[test]
    fn test_format_list_roots_full_shows_noun_only_marker() {
        let output = format_list_roots(Mode::Full);
        assert!(
            output.contains("только именной"),
            "Full mode list should show noun-only marker for noun-only roots"
        );
    }

    #[test]
    fn test_format_explore_droch_has_i_section() {
        let result = explore("дроч", None).expect("дроч should be valid");
        let output = format_explore(&result);
        assert!(
            output.contains("-и-"),
            "Output should contain -и- suffix section"
        );
        assert!(
            output.contains("дрочить"),
            "Output should contain дрочить form"
        );
    }
}
