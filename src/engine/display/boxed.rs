//! `random` / `root-of-the-day` formatter: an enriched summary box plus a full
//! breakdown block below it (D4 — the block goes under the box, not inside, so
//! the box arithmetic is untouched).

use crate::engine::morpheme::{RootData, VerbForm, suffix_display, suffix_gloss};

use super::form_block::format_form_block;
use super::{domain_inline, wrap_text};

/// Format a random-root display: an enriched summary box, then `Пример разбора:`
/// with a full breakdown block for one exemplary form.
///
/// The box holds the root's Russian gloss, domain · productivity, suffix classes
/// (with glosses), sample forms, and the linguistic note. `example` is one
/// Common infinitive as a `VerbForm`; `None` (nominal root, or a verbal root with
/// no attested form) prints an honest note instead of inventing an example.
pub fn format_random(rd: &RootData, sample_forms: &[&str], example: Option<&VerbForm>) -> String {
    let mut out = String::new();

    // Box width: 46 chars total (content field = 40). See the width arithmetic
    // note preserved from the pre-split implementation; every content line is
    // wrapped at `content_width` so nothing overflows the frame.
    let content_width = 40;

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

    // Root name line (Russian gloss)
    let gloss_str = rd.gloss_ru.map(|g| format!("«{g}»")).unwrap_or_default();
    push_content(&mut out, &format!("Корень: {}-  {}", rd.name, gloss_str));

    // Separator
    out.push_str(&format!("╠{}╣\n", "═".repeat(content_width + 4)));

    // Domain · productivity
    push_content(
        &mut out,
        &format!(
            "Домен: {} · продуктивность {}",
            domain_inline(rd.domain),
            rd.productivity
        ),
    );

    // Suffix classes with glosses
    let suffix_strs: Vec<String> = rd
        .suffix_indices
        .iter()
        .map(|&idx| format!("{} ({})", suffix_display(idx), suffix_gloss(idx)))
        .collect();
    let suffix_classes = if suffix_strs.is_empty() {
        "Нет (именной корень)".to_string()
    } else {
        suffix_strs.join(", ")
    };
    push_content(&mut out, &format!("Суффиксальные классы: {suffix_classes}"));

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

    // Linguistic note
    push_content(&mut out, "Заметка:");
    push_content(&mut out, rd.linguistic_note);

    // Bottom border
    out.push_str(&format!("╚{}╝\n", "═".repeat(content_width + 4)));

    // Breakdown block below the box (outside the frame).
    match example {
        Some(vf) => {
            out.push_str("\nПример разбора:\n");
            out.push_str(&format_form_block(rd, vf));
        }
        None => {
            out.push_str("\nПример разбора: нет засвидетельствованных форм уровня common.\n");
        }
    }

    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::paradigm::example_form;
    use crate::engine::roots::root_data;

    #[test]
    fn test_format_random_contains_root_name() {
        let rd = root_data("еб").unwrap();
        let example = example_form(rd);
        let output = format_random(rd, &["ебать", "ебнуть"], example.as_ref());
        assert!(output.contains("еб-"), "root name: {output}");
        assert!(output.contains("═"), "box borders");
        assert!(output.contains("Заметка:"), "note section");
        assert!(
            output.contains("«совокупляться»"),
            "Russian gloss: {output}"
        );
    }

    #[test]
    fn test_format_random_has_example_block() {
        let rd = root_data("еб").unwrap();
        let example = example_form(rd);
        let output = format_random(rd, &["ебать"], example.as_ref());
        assert!(
            output.contains("Пример разбора:"),
            "example section: {output}"
        );
        assert!(
            output.contains("разбор   :"),
            "form block present: {output}"
        );
    }

    #[test]
    fn test_format_random_empty_forms() {
        let rd = root_data("манд").unwrap();
        let output = format_random(rd, &[], None);
        assert!(output.contains("именной корень"), "{output}");
        assert!(
            output.contains("нет засвидетельствованных форм"),
            "no-example honest note: {output}"
        );
    }

    #[test]
    fn test_format_random_verb_root_no_samples() {
        // хуй- has verb classes but no Common forms post-grounding.
        let rd = root_data("хуй").unwrap();
        let output = format_random(rd, &[], None);
        assert!(output.contains("не указаны"), "{output}");
    }

    #[test]
    fn test_format_random_group_5() {
        let rd = root_data("сиповк").unwrap();
        let output = format_random(rd, &[], None);
        assert!(output.contains("именной корень"), "{output}");
    }

    #[test]
    fn test_format_random_box_lines_equal_width() {
        // The box is aligned only if every frame line — top/bottom border,
        // separator, blank line, and all content lines — has the same char count.
        // The breakdown block below the box is NOT part of the frame, so the width
        // check is applied only to lines that begin with a box-drawing character.
        for name in ["еб", "манд", "хуй", "сиповк", "говн", "хар", "елд"] {
            let rd = root_data(name).unwrap();
            let example = example_form(rd);
            for samples in [Vec::new(), vec!["ебать", "ебнуть"]] {
                let output = format_random(rd, &samples, example.as_ref());
                let widths: Vec<usize> = output
                    .lines()
                    .filter(|l| {
                        l.starts_with('║')
                            || l.starts_with('╔')
                            || l.starts_with('╠')
                            || l.starts_with('╚')
                    })
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
}
