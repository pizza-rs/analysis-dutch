//! Dutch stemmer.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Dutch light stemmer — removes common Dutch suffixes.
///
/// Handles plurals (-en, -s), diminutives (-tje, -pje, etc.),
/// and common derivational suffixes.
#[derive(Clone, Debug, Default)]
pub struct DutchStemFilter;

impl DutchStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for DutchStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 4 {
            return (false, None);
        }

        if let Some(stemmed) = stem_dutch(text) {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_dutch(word: &str) -> Option<String> {
    let mut s = word.to_string();
    let len = s.len();

    // Diminutive suffixes (most specific first)
    if len > 5 {
        if s.ends_with("etjes") || s.ends_with("pjes") {
            let trim = if s.ends_with("etjes") { 5 } else { 4 };
            s.truncate(len - trim);
            return Some(s);
        }
        if s.ends_with("tjes") {
            s.truncate(len - 4);
            return Some(s);
        }
    }
    if len > 4 && s.ends_with("tje") {
        s.truncate(len - 3);
        return Some(s);
    }

    // -heid suffix (abstract nouns)
    if s.len() > 5 && s.ends_with("heid") {
        s.truncate(s.len() - 4);
        return Some(s);
    }

    // -ing suffix
    if s.len() > 4 && s.ends_with("ing") {
        s.truncate(s.len() - 3);
        return Some(s);
    }

    // -lijk suffix
    if s.len() > 5 && s.ends_with("lijk") {
        s.truncate(s.len() - 4);
        return Some(s);
    }

    // -baar suffix
    if s.len() > 5 && s.ends_with("baar") {
        s.truncate(s.len() - 4);
        return Some(s);
    }

    // Plural -en (most common Dutch plural)
    if s.len() > 4 && s.ends_with("en") {
        s.truncate(s.len() - 2);
        // Handle doubled consonant: "katten" → "katt" → "kat"
        let bytes = s.as_bytes();
        let blen = bytes.len();
        if blen >= 2 && bytes[blen - 1] == bytes[blen - 2] && bytes[blen - 1].is_ascii_alphabetic() {
            s.truncate(blen - 1);
        }
        return Some(s);
    }

    // Plural -s
    if s.len() > 3 && s.ends_with('s') {
        s.truncate(s.len() - 1);
        return Some(s);
    }

    // Final -e
    if s.len() > 3 && s.ends_with('e') {
        s.truncate(s.len() - 1);
        return Some(s);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural_en() {
        let f = DutchStemFilter::new();
        let mut token = Token::new("katten", 0, 6, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "kat");
    }

    #[test]
    fn test_plural_s() {
        let f = DutchStemFilter::new();
        let mut token = Token::new("boeks", 0, 5, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "boek");
    }

    #[test]
    fn test_heid() {
        let f = DutchStemFilter::new();
        let mut token = Token::new("vrijheid", 0, 8, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "vrij");
    }

    #[test]
    fn test_short_word() {
        let f = DutchStemFilter::new();
        let mut token = Token::new("de", 0, 2, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "de");
    }
}
