use rand::seq::SliceRandom;

use crate::engine::morphemes::{EVAL_NOUNS, Gender, INTERJECTIONS, ROOTS};

/// Main entry point: generates `count` obscene phrases incorporating `topic`.
///
/// Count is clamped to 1..=100. The topic string becomes the noun in every phrase.
/// Phrases are built from the full combinatorial pool (all roots × all templates:
/// AdjectiveNoun, Interjection, Evaluation), shuffled, and sampled without
/// replacement — cycling only when count exceeds the pool size. This guarantees
/// maximum diversity: for count ≤ pool size, every phrase is unique.
pub fn generate(topic: &str, count: usize) -> Vec<String> {
    let count = count.clamp(1, 100);
    let mut rng = rand::rng();
    let gender = guess_gender(topic);

    // Build the full phrase pool — every combination of root, template,
    // interjection, and eval noun that can produce a distinct phrase.
    let mut pool: Vec<String> = Vec::new();

    // AdjectiveNoun template: each root's adjective + topic
    for root in ROOTS {
        let adj = format_adjective(root, gender);
        pool.push(format!("{} {}", adj, topic));
    }

    // Interjection template: each interjection + topic
    for interj in INTERJECTIONS {
        pool.push(format!("{}, {}!", interj, topic));
    }

    // Evaluation template: each eval noun + topic
    for eval in EVAL_NOUNS {
        pool.push(format!("{} — {}", topic, eval));
    }

    // Shuffle the pool so each invocation produces a different order
    pool.shuffle(&mut rng);

    // Sample without replacement, cycling if count exceeds pool size
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        result.push(pool[i % pool.len()].clone());
    }

    result
}

/// Guess Russian noun gender from its ending.
///
/// Rules:
/// - Ends with а, я → Feminine
/// - Ends with о, е → Neuter
/// - Ends with ы, и → Plural
/// - Ends with consonant, й → Masculine
/// - Ends with ь → Masculine (default; ~50% of ь-nouns are feminine, but
///   obscene-adjacent ь-words like "грязь" are uncommon in this context)
///
/// For multi-word topics, the last word determines gender.
pub fn guess_gender(word: &str) -> Gender {
    // For multi-word topics, use the last word
    let word = word.split_whitespace().last().unwrap_or(word);

    let last = match word.chars().last() {
        Some(c) => c,
        None => return Gender::Masculine,
    };

    match last {
        'а' | 'я' => Gender::Feminine,
        'о' | 'е' => Gender::Neuter,
        'ы' | 'и' => Gender::Plural,
        'й' => Gender::Masculine,
        'ь' => Gender::Masculine,
        _ => Gender::Masculine,
    }
}

/// Build an adjective from a root's stem and gender-appropriate ending.
fn format_adjective(root: &crate::engine::morphemes::Root, gender: Gender) -> String {
    let idx = match gender {
        Gender::Masculine => 0,
        Gender::Feminine => 1,
        Gender::Neuter => 2,
        Gender::Plural => 3,
    };
    format!("{}{}", root.adjective_stem, root.adjective_endings[idx])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_gender_masculine() {
        assert_eq!(guess_gender("космос"), Gender::Masculine);
        assert_eq!(guess_gender("дятел"), Gender::Masculine);
        assert_eq!(guess_gender("автомобиль"), Gender::Masculine);
        assert_eq!(guess_gender("орёл"), Gender::Masculine);
        assert_eq!(guess_gender("чай"), Gender::Masculine);
    }

    #[test]
    fn test_guess_gender_feminine() {
        assert_eq!(guess_gender("философия"), Gender::Feminine);
        assert_eq!(guess_gender("птичка"), Gender::Feminine);
        assert_eq!(guess_gender("рыба"), Gender::Feminine);
        assert_eq!(guess_gender("земля"), Gender::Feminine);
        assert_eq!(guess_gender("вода"), Gender::Feminine);
    }

    #[test]
    fn test_guess_gender_neuter() {
        assert_eq!(guess_gender("море"), Gender::Neuter);
        assert_eq!(guess_gender("солнце"), Gender::Neuter);
        assert_eq!(guess_gender("окно"), Gender::Neuter);
    }

    #[test]
    fn test_guess_gender_plural() {
        assert_eq!(guess_gender("птички"), Gender::Plural);
        assert_eq!(guess_gender("автомобили"), Gender::Plural);
        assert_eq!(guess_gender("коты"), Gender::Plural);
    }

    #[test]
    fn test_generate_returns_requested_count() {
        let result = generate("тест", 5);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_generate_phrases_contain_topic() {
        let result = generate("космос", 20);
        for phrase in &result {
            assert!(phrase.contains("космос"), "Phrase '{}' does not contain topic", phrase);
        }
    }

    #[test]
    fn test_generate_returns_diverse_phrases() {
        let result = generate("тест", 10);
        let unique: std::collections::HashSet<&str> = result.iter().map(|s| s.as_str()).collect();
        // Pool size is ~17 (4 roots + 7 interjections + 6 eval nouns).
        // With shuffle + sampling without replacement, count ≤ pool size
        // guarantees every phrase is unique, so 10/10 is the expectation.
        assert!(unique.len() >= 8, "Only {} unique phrases out of 10", unique.len());
    }

    #[test]
    fn test_count_capped_at_100() {
        let result = generate("тест", 500);
        assert!(result.len() <= 100);
    }

    #[test]
    fn test_count_zero_gives_one() {
        let result = generate("тест", 0);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_multi_word_topic() {
        let result = generate("красная площадь", 5);
        for phrase in &result {
            assert!(phrase.contains("красная площадь"), "Phrase does not contain multi-word topic");
        }
    }

    #[test]
    fn test_non_cyrillic_topic() {
        let result = generate("programming", 3);
        assert_eq!(result.len(), 3);
        for phrase in &result {
            assert!(phrase.contains("programming"));
        }
    }
}
