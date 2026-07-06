//! Paradigm builder: combinatorial expansion, form construction, attestation lookup.
//!
//! Main entry points:
//! - `explore(root, suffix_filter)` — builds the full combinatorial paradigm for a root
//! - `generate(mode, root_opt, count)` — generates random forms from the pool
//! - `list_roots(mode)` — returns available roots filtered by mode
//! - `random_root(mode)` — returns a random root from those available in mode
//! - `root_of_the_day(mode, day_index)` — deterministic root, stable within a day index
//! - `sample_forms(rd)` — up to 3 Common infinitives for the box display

use rand::prelude::IndexedRandom;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use crate::engine::grammar::build_form;
use crate::engine::morpheme::{
    Attestation, ExploreError, Mode, ParadigmResult, RootData, VerbForm, ending_val,
    endings_for_suffix, prefix_allomorphs, prefix_count, prefix_display, prefix_fill_form,
    prefix_val, select_prefix_allomorph, suffix_display, suffix_val,
};
use crate::engine::roots::{all_roots, lookup_attestation, root_data};

/// Explore the full paradigm for a given root.
///
/// Returns all morphological combinations (prefix × suffix × ending) with
/// attestation levels and meaning notes. If `suffix_filter` is `Some`, only
/// combinations with that suffix value are returned.
///
/// Unlisted combinations default to `Attestation::Possible` — linguistically
/// honest: unattested ≠ impossible.
pub fn explore(
    root_name: &str,
    suffix_filter: Option<&str>,
) -> Result<ParadigmResult, ExploreError> {
    let rd = root_data(root_name).ok_or_else(|| {
        let available: Vec<&str> = all_roots().iter().map(|r| r.name).collect();
        ExploreError::RootNotFound {
            root: root_name.to_string(),
            available,
        }
    })?;

    let mut forms = Vec::new();

    // Iterate over all prefix × suffix × ending combinations
    for prefix_idx in 0..prefix_count() {
        let p_val = prefix_val(prefix_idx);
        let p_display = prefix_display(prefix_idx);
        let allomorphs = prefix_allomorphs(prefix_idx);

        for &suffix_idx in rd.suffix_indices {
            let s_val = suffix_val(suffix_idx);
            let s_display = suffix_display(suffix_idx);

            // Apply suffix filter if provided
            if let Some(filter) = suffix_filter {
                // Accept filter if it matches the suffix value or display form
                if s_val != filter && s_display != filter {
                    continue;
                }
            }

            // Resolve the prefix surface before the root. A yer-final prefix
            // (fill_form is Some) before a fluid-vowel root (сра-/сса-/жр-) takes
            // its беглая -о- form (со-, изо-, разо-…), which cancels devoicing and,
            // ending in a vowel, blocks ъ-insertion — so it takes precedence over
            // the ordinary voiceless allomorph. Otherwise the standard selection
            // applies. One home for the rule: docs/architecture.md §Алломорфия.
            let prefix_form = match prefix_fill_form(prefix_idx) {
                Some(fill) if rd.takes_fill_vowel => fill,
                _ => select_prefix_allomorph(p_val, allomorphs, rd.val, rd.o_takes_ob),
            };

            // Get endings for this suffix class
            let end_indices = endings_for_suffix(suffix_idx);

            for &end_idx in &end_indices {
                let e_val = ending_val(end_idx);

                // Look up attestation and note
                let (att, note) = lookup_attestation(rd.name, prefix_idx, suffix_idx);

                // Build the full word form with present-stem allomorphy if applicable
                let form = build_form(prefix_form, rd.val, s_val, e_val, rd.present_stem);

                forms.push(VerbForm {
                    prefix_display: p_display,
                    suffix_val: s_val,
                    ending_val: e_val,
                    form,
                    attestation: att,
                    note,
                });
            }
        }
    }

    Ok(ParadigmResult {
        root_name: rd.name,
        root_gloss_ru: rd.gloss_ru,
        root_domain: rd.domain,
        root_productivity: rd.productivity,
        suffix_filter: suffix_filter.map(|s| s.to_string()),
        forms,
    })
}

/// A generated form paired with its root, so `generate` output can be rendered
/// as a full breakdown block (each form carries its own root/domain/gloss).
#[derive(Clone, Debug)]
pub struct GeneratedForm {
    pub root: &'static RootData,
    pub form: VerbForm,
}

/// Generate random forms.
///
/// If `root_name` is `None`, picks from all available roots in the given mode.
/// Returns `count` randomly selected forms, each paired with its root so the
/// caller can render a full breakdown block. Count is clamped to 1..=100.
pub fn generate(mode: Mode, root_name: Option<&str>, count: usize) -> Vec<GeneratedForm> {
    let count = count.clamp(1, 100);
    let mut rng = rand::rng();

    // Build the form pool based on root filter
    let roots: Vec<&str> = match root_name {
        Some(name) => {
            if root_data(name).is_some() {
                vec![name]
            } else {
                return Vec::new();
            }
        }
        None => list_roots(mode),
    };

    let mut pool: Vec<GeneratedForm> = Vec::new();

    for &root in &roots {
        let Some(rd) = root_data(root) else { continue };
        if let Ok(result) = explore(root, None) {
            // Only include forms with attestation != Impossible
            for vf in result.forms {
                if vf.attestation != Attestation::Impossible {
                    pool.push(GeneratedForm { root: rd, form: vf });
                }
            }
        }
    }

    if pool.is_empty() {
        return Vec::new();
    }

    pool.shuffle(&mut rng);

    // Cycle through the shuffled pool when count exceeds pool size
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        result.push(pool[i % pool.len()].clone());
    }

    result
}

/// List available roots filtered by mode.
pub fn list_roots(mode: Mode) -> Vec<&'static str> {
    all_roots()
        .iter()
        .filter(|r| mode.includes(r))
        .map(|r| r.name)
        .collect()
}

/// Pick one root from the mode's pool using the supplied RNG.
///
/// Shared by `random_root` (fresh `rand::rng()`) and `root_of_the_day`
/// (seeded `StdRng`), so the pool-selection logic lives in one place.
fn select_root<R: Rng + ?Sized>(mode: Mode, rng: &mut R) -> &'static RootData {
    let roots: Vec<&RootData> = all_roots().iter().filter(|r| mode.includes(r)).collect();
    roots.choose(rng).expect("at least one root must exist")
}

/// Select a random root from those available in the given mode.
pub fn random_root(mode: Mode) -> &'static RootData {
    select_root(mode, &mut rand::rng())
}

/// Select the deterministic "root of the day" for the given day index.
///
/// The same `day_index` always yields the same root (within a fixed `rand`
/// major version): the index seeds a `StdRng`, which drives the same pool
/// selection as `random_root`. `StdRng`'s algorithm is not guaranteed portable
/// across `rand` major versions, so a future bump may reshuffle the day→root
/// mapping — this does not break the contract, which promises "one root within
/// a day", not "day X → root Y forever" (see `docs/contracts/cli.md`).
pub fn root_of_the_day(mode: Mode, day_index: u64) -> &'static RootData {
    select_root(mode, &mut StdRng::seed_from_u64(day_index))
}

/// Assemble up to 3 Common infinitive sample forms for a root's box display.
///
/// Filters the root's paradigm for infinitives (ending `ть`) with `Common`
/// attestation and takes the first three. Returns an empty `Vec` on an explore
/// error or a root with no such forms (e.g. a purely nominal root). Shared by
/// the `random` and `root-of-the-day` commands so the assembly is not copied.
pub fn sample_forms(rd: &RootData) -> Vec<String> {
    match explore(rd.name, None) {
        Ok(result) => result
            .forms
            .iter()
            .filter(|f| f.ending_val == "ть" && f.attestation == Attestation::Common)
            .take(3)
            .map(|f| f.form.clone())
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// One exemplary form for the `random` / `root-of-the-day` breakdown block.
///
/// The first Common infinitive of the root, as a full `VerbForm` (unlike
/// `sample_forms`, which returns only the word strings for the summary box).
/// Returns `None` for a root with no Common infinitive (a nominal root, or a
/// verbal root whose forms are all Possible post-grounding) — the display then
/// prints an honest "no attested example" note rather than inventing one.
pub fn example_form(rd: &RootData) -> Option<VerbForm> {
    explore(rd.name, None)
        .ok()?
        .forms
        .into_iter()
        .find(|f| f.ending_val == "ть" && f.attestation == Attestation::Common)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[path = "paradigm_tests.rs"]
mod tests;
