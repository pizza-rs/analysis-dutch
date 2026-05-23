#![cfg_attr(not(feature = "std"), no_std)]
//! Dutch language analysis for Pizza search engine.
//!
//! Provides a Dutch analyzer with stemming and stop words.
//!
//! # Components
//!
//! - [`DutchStemFilter`] — Dutch suffix-stripping stemmer
//! - [`DutchStopFilter`] — Dutch stop words filter
extern crate alloc;
mod stem;
mod stop;

pub mod register;

pub use register::register_all;
pub use stem::DutchStemFilter;
pub use stop::DutchStopFilter;
