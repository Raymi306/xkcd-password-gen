// provides:
// static WORDLIST: &[&str] = &[...]
include!(concat!(env!("OUT_DIR"), "/wordlist.rs"));

use std::iter;

use rand::prelude::*;
use rand::TryRngCore;
use rand_core::UnwrapErr;

use crate::config::Config;
use crate::config::ConfigBuilder;
use crate::consts::DIGIT_ALPHABET;
use crate::types::PaddingType;
use crate::types::WordTransformation;
use crate::word_transformer;

#[derive(Debug)]
pub struct PasswordMaker<T>
where T: TryRngCore,
{
    pub rng: UnwrapErr<T>,
    pub config: Config,
    pub wordlist: Vec<String>,
}

impl<T> Default for PasswordMaker<T>
where T: TryRngCore + Default,
{
    fn default() -> Self {
        Self {
            rng: T::default().unwrap_err(),
            #[expect(
                clippy::unwrap_used,
                reason = "we control this default and it must not fail"
            )]
            config: ConfigBuilder::new().build().unwrap(),
            wordlist: WORDLIST.iter().map(|s| String::from(*s)).collect(),
        }
    }
}

pub struct PasswordMakerSeedable<T: TryRngCore>(PasswordMaker<T>);

impl<T> Default for PasswordMakerSeedable<T>
where T: TryRngCore + SeedableRng {
    fn default() -> Self {
        Self ( PasswordMaker {
            rng: T::from_os_rng().unwrap_err(),
            #[expect(
                clippy::unwrap_used,
                reason = "we control this default and it must not fail"
            )]
            config: ConfigBuilder::new().build().unwrap(),
            wordlist: WORDLIST.iter().map(|s| String::from(*s)).collect(),
        })
    }
}

impl<T> PasswordMaker<T>
where
    T: TryRngCore + Default,
{
    pub fn new(config: Config) -> Self {
        Self {
            rng: T::default().unwrap_err(),
            config,
            wordlist: WORDLIST.iter().map(|s| String::from(*s)).collect(),
        }
    }
}
impl<T> PasswordMaker<T>
where T: TryRngCore,
{
    #[expect(
        clippy::cast_possible_truncation,
        reason = "u32 MAX is more than enough for any reasonable word list length"
    )]
    fn filter_wordlist(&self) -> Vec<u32> {
        let min_len = self.config.word_min_length as usize;
        let max_len = self.config.word_max_length as usize;
        self.wordlist
            .iter()
            .enumerate()
            .filter(|(_, word)| (min_len..=max_len).contains(&word.chars().count()))
            .map(|(i, _)| i as u32)
            .collect()
    }
    fn choose_words(&mut self, indices: &[u32]) -> Vec<String> {
        if indices.is_empty() {
            return Vec::new();
        }
        let n = self.config.word_count as usize;
        let mut buf = Vec::with_capacity(n);
        for _ in 0..n {
            buf.push(indices.choose(&mut self.rng).expect(
                concat!(
                    "invariant 1: `indices` must not be empty and should have been guarded above.",
                    "invariant 2: size_hint on a slice iterator with no intermediary ",
                    "iterator adapters should always be accurate.\n",
                )
            ));
        }
        buf.into_iter()
            .map(|n| self.wordlist[*n as usize].clone())
            .collect()
    }
    fn transform_words(&mut self, words: Vec<String>) -> Vec<String> {
        if words.is_empty() {
            return words;
        }
        match self.config.word_transformation {
            WordTransformation::None => words,
            WordTransformation::Lower => word_transformer::lower(words),
            WordTransformation::Upper => word_transformer::upper(words),
            WordTransformation::CapitalizeFirst => word_transformer::capitalize_first(words),
            WordTransformation::CapitalizeLast => word_transformer::capitalize_last(words),
            WordTransformation::CapitalizeNotFirst => word_transformer::capitalize_not_first(words),
            WordTransformation::AlternatingLowerUpper => {
                word_transformer::alternating_lower_upper(words)
            }
            WordTransformation::AlternatingUpperLower => {
                word_transformer::alternating_upper_lower(words)
            }
            WordTransformation::RandomUpperLower => {
                word_transformer::random_upper_lower(&mut self.rng, words)
            }
        }
    }
    fn choose_n_digits(&mut self, n: usize) -> Option<String> {
        if n == 0 {
            return None;
        }
        let mut buf = Vec::with_capacity(n);
        for _ in 0..n {
            #[expect(
                clippy::unwrap_used,
                reason = "as DIGIT_ALPHABET is const, it is not empty and should not provide bad size hints"
            )]
            buf.push(DIGIT_ALPHABET.choose(&mut self.rng).unwrap());
        }
        Some(buf.into_iter().collect())
    }
    fn create_pseudo_words(&mut self) -> (Option<String>, Option<String>) {
        let before = self.choose_n_digits(self.config.digits_before as usize);
        let after = self.choose_n_digits(self.config.digits_after as usize);
        (before, after)
    }
    fn choose_separator(&mut self) -> Option<char> {
        self.config
            .separator_character
            .choose(&mut self.rng)
            .copied()
    }
    fn create_padding(&mut self, password: &str) -> (Option<String>, Option<String>) {
        let len = self.config.padding_length as usize;
        let (before_len, after_len) = match self.config.padding_type {
            PaddingType::None => (0, 0),
            PaddingType::Fixed => (len, len),
            PaddingType::Adaptive => (0, len.saturating_sub(password.chars().count())),
        };
        let padding_character = self.config.padding_character.choose(&mut self.rng);
        let before = iter::repeat(padding_character).take(before_len).collect();
        let after = iter::repeat(padding_character).take(after_len).collect();
        (before, after)
    }
    fn create_password(&mut self) -> String {
        let filtered_word_indices = self.filter_wordlist();
        let chosen_words = self.choose_words(&filtered_word_indices);
        let mut transformed_words = self.transform_words(chosen_words);
        let (front_digits, back_digits) = self.create_pseudo_words();
        let separator = self.choose_separator();
        let mut parts = vec![front_digits.unwrap_or(String::new())];
        parts.append(&mut transformed_words);
        parts.push(back_digits.unwrap_or(String::new()));
        let unpadded_password = parts.join(&separator.map(String::from).unwrap_or_default());
        let (front_padding, rear_padding) = self.create_padding(&unpadded_password);
        let final_password = format!(
            "{}{}{}",
            front_padding.unwrap_or(String::new()),
            unpadded_password,
            rear_padding.unwrap_or(String::new())
        );
        final_password
    }
    pub fn create_passwords(&mut self) -> Vec<String> {
        let count = self.config.count as usize;
        let mut buf = Vec::with_capacity(count);
        for _ in 0..count {
            buf.push(self.create_password());
        }
        buf
    }
}

#[cfg(test)]
mod password_maker_tests {
    use crate::test_helpers::*;

    #[test]
    fn test_filter_wordlist() {
        let mut maker = make_seeded_maker(1);
        maker.wordlist = make_wordlist();
        maker.config.word_min_length = 4;
        maker.config.word_max_length = 4;
        let result = maker.filter_wordlist();
        let expected = [5, 6];
        let matches = result
            .iter()
            .zip(expected.iter())
            .filter(|&(a, b)| a == b)
            .count();
        assert_eq!(result.len(), matches, "result.len() == matches");
        assert_eq!(expected.len(), matches, "expected.len() == matches");
    }

    #[test]
    fn test_choose_words_ok() {
        let params = [2, 100];

        for param in params {
            let mut maker = make_seeded_maker(1);
            maker.config.word_count = param;
            let indices: [u32; 2] = [1, 2];
            let result = maker.choose_words(&indices);
            assert_eq!(result.len(), param as usize);
        }
    }

    #[ignore = "not written yet"]
    #[test]
    fn test_choose_words_result_is_shuffled() {}
}
