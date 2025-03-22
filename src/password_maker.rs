//! Provides the [`PasswordMaker`] struct.
//!
//! The password generation algorithm is implemented here.
// provides:
// static WORDLIST: &[&str] = &[...]
include!(concat!(env!("OUT_DIR"), "/wordlist.rs"));

use std::iter;

use rand::TryRngCore;
use rand::prelude::*;
use rand_core::UnwrapErr;

use crate::config::Config;
use crate::config::ConfigBuilder;
use crate::consts::DIGIT_ALPHABET;
use crate::types::PaddingType;
use crate::types::WordTransformationType;
use crate::word_transformer;

/// Turn a [`Config`] into passwords.
#[derive(Debug)]
pub struct PasswordMaker<T>
where
    T: TryRngCore,
{
    pub rng: UnwrapErr<T>,
    pub config: Config,
    pub wordlist: Vec<String>,
}

impl<T> Default for PasswordMaker<T>
where
    T: TryRngCore + Default,
{
    fn default() -> Self {
        #[expect(
            clippy::unwrap_used,
            reason = "we control this default and it must not fail"
        )]
        let config = ConfigBuilder::new().build().unwrap();
        Self {
            rng: T::default().unwrap_err(),
            wordlist: WORDLIST.iter().map(|s| String::from(*s)).collect(),
            config,
        }
    }
}

/// Note that [`rand_core::SeedableRng`] does not impl [`Default`].
/// This is a small struct, creating an instance without a `new` method
/// is not too bad, and `SeedableRng` is only useful for testing.
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
where
    T: TryRngCore,
{
    /// Filter out words that do not fit between the configured minimum and maximum length.
    ///
    /// Return indexes indicating which words we wish to keep.
    /// Working with indexes avoids pointer hell and reduces memory allocation and storage requirements.
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
    /// Choose with replacement a configured number of words.
    ///
    /// Convert each chosen word from an index into a [`String`].
    fn choose_words(&mut self, indices: &[u32]) -> Vec<String> {
        if indices.is_empty() {
            return Vec::new();
        }
        let n = self.config.word_count as usize;
        let mut buf = Vec::with_capacity(n);
        for _ in 0..n {
            buf.push(indices.choose(&mut self.rng).expect(concat!(
                "invariant 1: `indices` must not be empty and should have been guarded above.\n",
                "invariant 2: size_hint on a slice iterator with no intermediary ",
                "iterator adapters should always be accurate.",
            )));
        }
        buf.into_iter()
            .map(|n| self.wordlist[*n as usize].clone())
            .collect()
    }
    /// Use the configured [`WordTransformationType`] to transform a [`Vec<String>`] of words.
    fn transform_words(&mut self, words: Vec<String>) -> Vec<String> {
        if words.is_empty() {
            return words;
        }
        match self.config.word_transformation {
            WordTransformationType::None => words,
            WordTransformationType::Lower => word_transformer::lower(words),
            WordTransformationType::Upper => word_transformer::upper(words),
            WordTransformationType::CapitalizeFirst => word_transformer::capitalize_first(words),
            WordTransformationType::CapitalizeLast => word_transformer::capitalize_last(words),
            WordTransformationType::CapitalizeNotFirst => {
                word_transformer::capitalize_not_first(words)
            }
            WordTransformationType::AlternatingLowerUpper => {
                word_transformer::alternating_lower_upper(words)
            }
            WordTransformationType::AlternatingUpperLower => {
                word_transformer::alternating_upper_lower(words)
            }
            WordTransformationType::RandomUpperLower => {
                word_transformer::random_upper_lower(&mut self.rng, words)
            }
        }
    }
    /// Choose with replacement `n` digits to form and return an [`Option<String>`].
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
    /// Create the before and after pseudo-words.
    /// A pseudo-word is a string of 0 or more digits.
    fn create_pseudo_words(&mut self) -> (Option<String>, Option<String>) {
        let before = self.choose_n_digits(self.config.digits_before as usize);
        let after = self.choose_n_digits(self.config.digits_after as usize);
        (before, after)
    }
    /// Choose a separator character from the configured choices.
    fn choose_separator(&mut self) -> Option<char> {
        self.config
            .separator_character
            .choose(&mut self.rng)
            .copied()
    }
    /// Given the password we have created thus far, create the before and after padding.
    /// [`PaddingType::Fixed`] prepends and appends an equal number of padding characters.
    /// [`PaddingType::Adaptive`] will append padding characters to meet the desired length.
    /// Note that if the desired length is shorter than the unpadded password, adaptive
    /// padding is a no-op.
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
    /// Create a password.
    ///
    /// The password generation algorithm is very similar to the one found in Crypt::HSXKPasswd,
    /// see [https://metacpan.org/pod/Crypt::HSXKPasswd](https://metacpan.org/pod/Crypt::HSXKPasswd) or below for a local copy:
    ///
    /// 1. Pick random words from the dictionary.
    /// 2. Apply transformations to the words.
    /// 3. Create pseudo-words made up for randomly chosen digits and add them as the first and last words.
    /// 4. Insert a copy of the same symbol between each of the words and pseudo-words. This symbol is referred to as the separator character.
    /// 5. Pad the password with multiple instances of the same symbol front and/or back. This symbol is referred to as the padding character.
    fn create_password(&mut self) -> String {
        let filtered_word_indices = self.filter_wordlist();
        let chosen_words = self.choose_words(&filtered_word_indices);
        let mut transformed_words = self.transform_words(chosen_words);
        let (front_digits, back_digits) = self.create_pseudo_words();
        let separator = self.choose_separator();

        // begin constructing the password sans padding
        let mut parts = vec![front_digits.unwrap_or(String::new())];
        parts.append(&mut transformed_words);
        parts.push(back_digits.unwrap_or(String::new()));
        let unpadded_password = parts.join(&separator.map(String::from).unwrap_or_default());

        let (front_padding, rear_padding) = self.create_padding(&unpadded_password);

        [
            front_padding.unwrap_or(String::new()),
            unpadded_password,
            rear_padding.unwrap_or(String::new()),
        ]
        .join("")
    }
    /// Create passwords.
    /// This is the public interface for the [`PasswordMaker`] struct.
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
        // some test parametrization wouldn't go amiss here.
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

    /// `choose_words` should be choosing with replacement,
    /// if `config.word_count > config.wordlist.len()`,
    /// do not panic and ensure the final length is == the configured word count.
    #[test]
    fn test_choose_words() {
        let params = [2, 100];

        for param in params {
            let mut maker = make_seeded_maker(1);
            maker.config.word_count = param;
            let indices: [u32; 2] = [1, 2];
            let result = maker.choose_words(&indices);
            assert_eq!(result.len(), param as usize);
        }
    }

    /// It is possible to incorrectly use rand methods such that
    /// you choose random items but place them in a non-random order.
    #[ignore = "not written yet"]
    #[test]
    const fn test_choose_words_result_is_shuffled() {}
}
