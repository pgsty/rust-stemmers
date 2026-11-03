//! This library provides rust implementations for some stemmer algorithms
//! written in the [snowball language](https://snowballstem.org/).
//!
//!
//! All algorithms expect the input to already be lowercased.
//!
//! # Usage
//! ```toml
//! [dependencies]
//! rust-stemmers = { git = "https://github.com/pgsty/rust-stemmers.git", tag = "v1.2.1" }
//! ```
//!
//! ```rust
//! extern crate rust_stemmers;
//!
//! use rust_stemmers::{Algorithm, Stemmer};
//!
//! fn main() {
//!    let en_stemmer = Stemmer::create(Algorithm::English);
//!    assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");
//! }
//! ```
use std::borrow::Cow;

mod snowball;

use snowball::algorithms;
use snowball::SnowballEnv;

/// Enum of all supported algorithms.
/// Check the [Snowball-Website](https://snowballstem.org/) for details.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Algorithm {
    Arabic,
    Armenian,
    Basque,
    Catalan,
    Danish,
    Dutch,
    English,
    Estonian,
    Finnish,
    French,
    German,
    Greek,
    Hindi,
    Hungarian,
    Indonesian,
    Irish,
    Italian,
    Lithuanian,
    Nepali,
    Norwegian,
    Porter,
    Portuguese,
    Romanian,
    Russian,
    Serbian,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
    Yiddish,
}

/// Wraps a usable interface around the actual stemmer implementation.
#[derive(Clone)]
pub struct Stemmer {
    stemmer: fn(&mut SnowballEnv) -> bool,
}

impl Stemmer {
    /// Create a new stemmer from an algorithm
    pub fn create(lang: Algorithm) -> Self {
        match lang {
            Algorithm::Arabic => Stemmer {
                stemmer: algorithms::arabic_stemmer::stem,
            },
            Algorithm::Armenian => Stemmer {
                stemmer: algorithms::armenian_stemmer::stem,
            },
            Algorithm::Basque => Stemmer {
                stemmer: algorithms::basque_stemmer::stem,
            },
            Algorithm::Catalan => Stemmer {
                stemmer: algorithms::catalan_stemmer::stem,
            },
            Algorithm::Danish => Stemmer {
                stemmer: algorithms::danish_stemmer::stem,
            },
            Algorithm::Dutch => Stemmer {
                stemmer: algorithms::dutch_stemmer::stem,
            },
            Algorithm::English => Stemmer {
                stemmer: algorithms::english_stemmer::stem,
            },
            Algorithm::Estonian => Stemmer {
                stemmer: algorithms::estonian_stemmer::stem,
            },
            Algorithm::Finnish => Stemmer {
                stemmer: algorithms::finnish_stemmer::stem,
            },
            Algorithm::French => Stemmer {
                stemmer: algorithms::french_stemmer::stem,
            },
            Algorithm::German => Stemmer {
                stemmer: algorithms::german_stemmer::stem,
            },
            Algorithm::Greek => Stemmer {
                stemmer: algorithms::greek_stemmer::stem,
            },
            Algorithm::Hindi => Stemmer {
                stemmer: algorithms::hindi_stemmer::stem,
            },
            Algorithm::Hungarian => Stemmer {
                stemmer: algorithms::hungarian_stemmer::stem,
            },
            Algorithm::Indonesian => Stemmer {
                stemmer: algorithms::indonesian_stemmer::stem,
            },
            Algorithm::Irish => Stemmer {
                stemmer: algorithms::irish_stemmer::stem,
            },
            Algorithm::Italian => Stemmer {
                stemmer: algorithms::italian_stemmer::stem,
            },
            Algorithm::Lithuanian => Stemmer {
                stemmer: algorithms::lithuanian_stemmer::stem,
            },
            Algorithm::Nepali => Stemmer {
                stemmer: algorithms::nepali_stemmer::stem,
            },
            Algorithm::Norwegian => Stemmer {
                stemmer: algorithms::norwegian_stemmer::stem,
            },
            Algorithm::Porter => Stemmer {
                stemmer: algorithms::porter_stemmer::stem,
            },
            Algorithm::Portuguese => Stemmer {
                stemmer: algorithms::portuguese_stemmer::stem,
            },
            Algorithm::Romanian => Stemmer {
                stemmer: algorithms::romanian_stemmer::stem,
            },
            Algorithm::Russian => Stemmer {
                stemmer: algorithms::russian_stemmer::stem,
            },
            Algorithm::Serbian => Stemmer {
                stemmer: algorithms::serbian_stemmer::stem,
            },
            Algorithm::Spanish => Stemmer {
                stemmer: algorithms::spanish_stemmer::stem,
            },
            Algorithm::Swedish => Stemmer {
                stemmer: algorithms::swedish_stemmer::stem,
            },
            Algorithm::Tamil => Stemmer {
                stemmer: algorithms::tamil_stemmer::stem,
            },
            Algorithm::Turkish => Stemmer {
                stemmer: algorithms::turkish_stemmer::stem,
            },
            Algorithm::Yiddish => Stemmer {
                stemmer: algorithms::yiddish_stemmer::stem,
            },
        }
    }

    /// Stem a single word
    /// Please note, that the input is expected to be all lowercase (if that is applicable).
    pub fn stem<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut env = SnowballEnv::create(input);
        (self.stemmer)(&mut env);
        env.get_current()
    }
}
