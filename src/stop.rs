//! Dutch stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Default Dutch stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
        "aan", "al", "alles", "als", "altijd", "andere", "ben", "bij", "daar", "dan", "dat", "de",
        "der", "deze", "die", "dit", "doch", "doen", "door", "dus", "een", "eens", "en", "er",
        "ge", "geen", "geweest", "haar", "had", "heb", "hebben", "heeft", "hem", "het", "hier",
        "hij", "hoe", "hun", "iemand", "iets", "ik", "in", "is", "ja", "je", "kan", "kon",
        "kunnen", "maar", "me", "meer", "men", "met", "mij", "mijn", "moet", "na", "naar", "niet",
        "niets", "nog", "nu", "of", "om", "omdat", "onder", "ons", "ook", "op", "over", "reeds",
        "te", "tegen", "toch", "toen", "tot", "u", "uit", "uw", "van", "veel", "voor", "want",
        "waren", "was", "wat", "werd", "wezen", "wie", "wil", "worden", "wordt", "zal", "ze",
        "zelf", "zich", "zij", "zijn", "zo", "zonder", "zou",
    ];
    words.iter().copied().collect()
});

/// Removes Dutch stop words from the token stream.
#[derive(Clone, Debug)]
pub struct DutchStopFilter {
    stop_words: HashSet<String>,
}

impl Default for DutchStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl DutchStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for DutchStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 101);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = DutchStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = DutchStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = DutchStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
