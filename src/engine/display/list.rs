//! `list-roots` formatter: an enriched one-line-per-root listing.

use crate::engine::morpheme::{Attestation, Domain, Mode, RootData, suffix_display};
use crate::engine::paradigm::{explore, list_roots};
use crate::engine::roots::all_roots;

use super::{domain_inline, domain_list_header};

/// Format the root list for a given mode.
///
/// Classic → a flat list; full → grouped by semantic domain. Each line carries
/// the root, its Russian gloss, domain, productivity class, verb/noun type, and
/// an honest "no attested forms" flag for verbal roots whose paradigm is all
/// `possible` post-grounding.
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

/// One enriched list line for a root.
fn format_root_line(rd: &RootData) -> String {
    let gloss = rd.gloss_ru.map(|g| format!("«{g}»")).unwrap_or_default();

    let type_str = if rd.suffix_indices.is_empty() {
        "только именной".to_string()
    } else {
        let classes: Vec<&str> = rd
            .suffix_indices
            .iter()
            .map(|&i| suffix_display(i))
            .collect();
        format!("глагольный ({})", classes.join(", "))
    };

    // Honesty (§7): a verbal root whose whole paradigm is Possible has no
    // source-attested form — flag it so the reader is not misled.
    let flag = if !rd.suffix_indices.is_empty() && !has_attested_form(rd.name) {
        "  (нет засвидетельствованных форм)"
    } else {
        ""
    };

    format!(
        "  {}-  {}  {} · {} · {}{}\n",
        rd.name,
        gloss,
        domain_inline(rd.domain),
        rd.productivity,
        type_str,
        flag,
    )
}

/// Whether the root has at least one Common or Rare form (source-attested).
fn has_attested_form(name: &str) -> bool {
    explore(name, None)
        .map(|r| {
            r.forms
                .iter()
                .any(|f| matches!(f.attestation, Attestation::Common | Attestation::Rare))
        })
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_list_roots_classic_contains_eb() {
        let output = format_list_roots(Mode::Classic);
        assert!(output.contains("еб"), "Output should contain еб root");
        assert!(output.contains("корней"), "root count word in Russian");
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
        assert!(output.contains("Ядро"), "Nuclear header");
        assert!(output.contains("Экскреторная"), "Excretory header");
        assert!(output.contains("Периферия"), "Peripheral header");
    }

    #[test]
    fn test_format_list_roots_shows_productivity_and_type() {
        // Format migration: productivity now sits in the "домен · класс · тип"
        // triple rather than a "[A]" marker.
        let output = format_list_roots(Mode::Full);
        assert!(
            output.contains("ядро · A · глагольный"),
            "еб- line shows domain · class · verbal type: {output}"
        );
    }

    #[test]
    fn test_format_list_roots_shows_russian_gloss_not_english() {
        let output = format_list_roots(Mode::Classic);
        assert!(
            output.contains("«совокупляться»"),
            "Russian gloss shown: {output}"
        );
        assert!(
            !output.contains("fuck, copulate"),
            "English gloss must not leak into output: {output}"
        );
    }

    #[test]
    fn test_format_list_roots_full_shows_noun_only_marker() {
        let output = format_list_roots(Mode::Full);
        assert!(
            output.contains("только именной"),
            "noun-only roots marked: {output}"
        );
    }

    #[test]
    fn test_format_list_roots_flags_unattested_verbal_root() {
        // хуй- is verbal (-ну- surrogate) but every form is Possible post-grounding
        // → must carry the honest no-attested-forms flag.
        let output = format_list_roots(Mode::Full);
        assert!(
            output.contains("нет засвидетельствованных форм"),
            "an all-possible verbal root should be flagged: {output}"
        );
    }
}
