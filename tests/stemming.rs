extern crate rust_stemmers;

use rust_stemmers::{Algorithm, Stemmer};
use std::borrow::Cow;

const ALGORITHMS: &[Algorithm] = &[
    Algorithm::Arabic,
    Algorithm::Armenian,
    Algorithm::Basque,
    Algorithm::Catalan,
    Algorithm::Danish,
    Algorithm::Dutch,
    Algorithm::English,
    Algorithm::Estonian,
    Algorithm::Finnish,
    Algorithm::French,
    Algorithm::German,
    Algorithm::Greek,
    Algorithm::Hindi,
    Algorithm::Hungarian,
    Algorithm::Indonesian,
    Algorithm::Irish,
    Algorithm::Italian,
    Algorithm::Lithuanian,
    Algorithm::Nepali,
    Algorithm::Norwegian,
    Algorithm::Porter,
    Algorithm::Portuguese,
    Algorithm::Romanian,
    Algorithm::Russian,
    Algorithm::Serbian,
    Algorithm::Spanish,
    Algorithm::Swedish,
    Algorithm::Tamil,
    Algorithm::Turkish,
    Algorithm::Yiddish,
];

#[test]
fn empty_and_non_alphabetic_tokens() {
    for &algorithm in ALGORITHMS {
        let stemmer = Stemmer::create(algorithm);
        for input in &["", "123", "🦀", "中文", "\0"] {
            assert_eq!(stemmer.stem(input), *input, "{:?}: {:?}", algorithm, input);
        }
    }
}

#[test]
fn multilingual_stemming() {
    let cases = &[
        (Algorithm::English, "running", "run"),
        (Algorithm::English, "fruitlessly", "fruitless"),
        (Algorithm::English, "relational", "relat"),
        (Algorithm::French, "chevaux", "cheval"),
        (Algorithm::German, "häuser", "haus"),
        (Algorithm::Spanish, "hablando", "habl"),
        (Algorithm::Portuguese, "casas", "cas"),
        (Algorithm::Russian, "машины", "машин"),
        (Algorithm::Finnish, "talossa", "talo"),
        (Algorithm::Arabic, "الكتب", "كتب"),
    ];
    for &(algorithm, input, expected) in cases {
        assert_eq!(
            Stemmer::create(algorithm).stem(input),
            expected,
            "{:?}: {:?}",
            algorithm,
            input
        );
    }
}

#[test]
fn porter_and_porter2_remain_distinct() {
    assert_eq!(Stemmer::create(Algorithm::Porter).stem("skies"), "ski");
    assert_eq!(Stemmer::create(Algorithm::English).stem("skies"), "sky");
    assert_eq!(Stemmer::create(Algorithm::English).stem("dying"), "die");
}

#[test]
fn english_compatibility_for_existing_indexes() {
    // CurrySoftware/rust-stemmers#21: preserve the inherited Snowball behavior
    // in this maintenance release. A future algorithm upgrade must review it.
    let stemmer = Stemmer::create(Algorithm::English);
    for input in &["internal", "international", "interning"] {
        assert_eq!(stemmer.stem(input), "intern");
    }
}

#[test]
fn unicode_boundaries() {
    let pieces = &[
        "a", "é", "e\u{301}", "я", "ա", "क", "க", "א", "中", "🦀", "\0",
    ];
    for &algorithm in ALGORITHMS {
        let stemmer = Stemmer::create(algorithm);
        for prefix in pieces {
            for suffix in pieces {
                let input = format!("{}running{}", prefix, suffix);
                // This exercises byte-boundary navigation in every algorithm.
                let output = std::panic::catch_unwind(|| stemmer.stem(&input));
                assert!(output.is_ok(), "{:?}: {:?}", algorithm, input);
                // Reusing a stemmer must not retain state from the prior token.
                assert_eq!(stemmer.stem(""), "");
            }
        }
    }
}

#[test]
fn output_ownership_and_lifetime() {
    let input = String::from("cat");
    let output = {
        let stemmer = Stemmer::create(Algorithm::English);
        stemmer.stem(&input)
    };
    assert!(matches!(output, Cow::Borrowed("cat")));

    let input = String::from("running");
    let output = Stemmer::create(Algorithm::English).stem(&input);
    assert!(matches!(output, Cow::Owned(_)));
    assert_eq!(output, "run");
    assert_eq!(input, "running");
}

#[test]
fn cloned_stemmer_can_be_used_on_another_thread() {
    let stemmer = Stemmer::create(Algorithm::English);
    let cloned = stemmer.clone();
    let worker = std::thread::spawn(move || cloned.stem("running").into_owned());
    assert_eq!(stemmer.stem("fruitlessly"), "fruitless");
    assert_eq!(worker.join().unwrap(), "run");
}
