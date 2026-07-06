//! Output formatting, split by command into cohesive submodules.
//!
//! The shared house-style breakdown block (`form_block`) is reused by every
//! command; per-command formatters live in `explore`, `list`, `boxed`. Shared
//! low-level helpers (attestation tokens, domain names, word-wrap) live here so
//! each submodule pulls from one home.

use crate::engine::morpheme::{Attestation, Domain};

mod boxed;
mod explore;
mod form_block;
mod list;

pub use boxed::format_random;
pub use explore::format_explore;
pub use form_block::format_form_block;
pub use list::format_list_roots;

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Attestation token as shown to the user.
///
/// Kept English (`common`/`rare`/`possible`/`impossible`) per the CLI contract
/// and the Operator's mockup — these are the source's own marks.
fn format_attestation(att: Attestation) -> &'static str {
    match att {
        Attestation::Common => "common",
        Attestation::Rare => "rare",
        Attestation::Possible => "possible",
        Attestation::Impossible => "impossible",
    }
}

/// One-line explanation of *why* a form carries its attestation level.
///
/// Single home for these phrasings (used by the form block's `уровень` line).
fn attestation_reason(att: Attestation) -> &'static str {
    match att {
        Attestation::Common => "широко засвидетельствована в корпусе (синтез Плуцера-Сарно)",
        Attestation::Rare => {
            "засвидетельствована редко — диалектная, окказиональная или единичная фиксация"
        }
        Attestation::Possible => {
            "словообразовательно возможна по аналогии, в источнике не засвидетельствована"
        }
        Attestation::Impossible => {
            "заблокирована фонологическим, морфологическим или семантическим ограничением"
        }
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

/// Domain name inline (lowercase), for the explore header, form block, and box.
fn domain_inline(domain: Domain) -> &'static str {
    match domain {
        Domain::Nuclear => "ядро",
        Domain::Excretory => "экскреторная",
        Domain::Peripheral => "периферия",
    }
}

/// Domain header (capitalised) for the full-list grouping.
fn domain_list_header(domain: Domain) -> &'static str {
    match domain {
        Domain::Nuclear => "Ядро",
        Domain::Excretory => "Экскреторная",
        Domain::Peripheral => "Периферия",
    }
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
// Tests for the shared helpers
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_attestation_reason_distinct_per_level() {
        // The four "why" phrasings must be distinct — a form block relies on the
        // level's reason being specific to it.
        let reasons = [
            attestation_reason(Attestation::Common),
            attestation_reason(Attestation::Rare),
            attestation_reason(Attestation::Possible),
            attestation_reason(Attestation::Impossible),
        ];
        for i in 0..reasons.len() {
            for j in (i + 1)..reasons.len() {
                assert_ne!(reasons[i], reasons[j], "reasons must be distinct");
            }
        }
    }
}
