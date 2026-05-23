//! Register Dutch analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use pizza_engine::analysis::{
    Analyzer, AnalysisFactory, LowercaseNormalizer, Normalizer, StandardTokenizer, TokenFilter,
    Tokenizer,
};

use crate::{DutchStemFilter, DutchStopFilter};

/// Register Dutch token filters and the `"dutch"` analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("dutch_stem", Box::new(DutchStemFilter::new()));
    factory.register_token_filter("dutch_stop", Box::new(DutchStopFilter::new()));

    let normalizers: Vec<Box<dyn Normalizer>> = vec![Box::new(LowercaseNormalizer::new())];
    let tokenizer: Box<dyn Tokenizer> = Box::new(StandardTokenizer::new());
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(DutchStopFilter::new()),
        Box::new(DutchStemFilter::new()),
    ];
    factory.register_analyzer("dutch", Analyzer::new(normalizers, tokenizer, filters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("dutch_stem").is_some());
        assert!(factory.get_token_filter("dutch_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("dutch").is_some());
    }

    #[test]
    fn test_analyzer_pipeline() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("dutch").unwrap();
        let mut input = String::from("De kat is in het huis");
        let tokens = analyzer.analyze_and_return_tokens(&mut input);
        assert!(!tokens.iter().any(|t| t.term == "de"));
        assert!(!tokens.iter().any(|t| t.term == "is"));
        assert!(!tokens.iter().any(|t| t.term == "in"));
        assert!(!tokens.iter().any(|t| t.term == "het"));
        assert!(tokens.len() >= 2);
    }
}
