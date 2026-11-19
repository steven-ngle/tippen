use std::{error::Error, fs::File, path::Path};

use rand::seq::IndexedRandom;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Quote {
    pub quote: String,
    pub source: String,
}

pub struct RoundResult {
    pub correct: bool,
    pub elapsed: f64,
    pub wpm: f64,
}

pub fn load_quotes() -> std::result::Result<Vec<Quote>, Box<dyn Error>> {
    let file = File::open(Path::new("quotes/quotes.json"))?;
    Ok(serde_json::from_reader(file)?)
}

pub fn choose_quote(quotes: &[Quote]) -> Option<Quote> {
    quotes.choose(&mut rand::rng()).cloned()
}

pub fn evaluate(expected: &str, typed: &str, elapsed: f64) -> RoundResult {
    let characters = typed.chars().count();
    let wpm = if elapsed > 0.0 {
        (characters as f64 / 5.0) / (elapsed / 60.0)
    } else {
        0.0
    };

    RoundResult {
        correct: typed == expected,
        elapsed,
        wpm,
    }
}

#[cfg(test)]
mod tests {
    use super::evaluate;

    #[test]
    fn evaluate_checks_exact_text_and_calculates_wpm() {
        let result = evaluate("hello", "hello", 60.0);
        assert!(result.correct);
        assert_eq!(result.wpm, 1.0);

        let result = evaluate("hello", "hell", 60.0);
        assert!(!result.correct);
        assert_eq!(result.wpm, 0.8);
    }

    #[test]
    fn evaluate_handles_zero_elapsed_time() {
        let result = evaluate("quote", "quote", 0.0);
        assert!(result.correct);
        assert_eq!(result.wpm, 0.0);
    }
}
