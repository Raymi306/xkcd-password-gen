#![cfg(test)]

// provides:
// static WORDLIST: &[&str] = &[...]
include!(concat!(env!("OUT_DIR"), "/wordlist.rs"));

use rand::SeedableRng;
use rand::TryRngCore;
use rand::rngs::SmallRng;

use crate::config::ConfigBuilder;
use crate::password_maker::PasswordMaker;

pub fn make_seeded_maker(seed: u64) -> PasswordMaker<SmallRng> {
    let rng = SmallRng::seed_from_u64(seed).unwrap_err();
    PasswordMaker {
        rng,
        config: ConfigBuilder::new().build().unwrap(),
        wordlist: WORDLIST.iter().map(|s| String::from(*s)).collect(),
    }
}

pub fn make_wordlist() -> Vec<String> {
    [
        "modern", "labor", "hello", "world", "water", "fire", "deep", "ice", "pie",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

pub fn string_is_uppercase(word: &str) -> bool {
    word.chars().all(char::is_uppercase)
}

pub fn string_is_lowercase(word: &str) -> bool {
    word.chars().all(char::is_lowercase)
}
