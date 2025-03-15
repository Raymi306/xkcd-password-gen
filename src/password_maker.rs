// provides:
// static WORDLIST: &[&str] = &[...]
include!(concat!(env!("OUT_DIR"), "/wordlist.rs"));

use std::iter;

use rand::prelude::*;

use crate::config::Config;
use crate::config::ConfigBuilder;
use crate::consts::DIGIT_ALPHABET;
use crate::types::PaddingType;
use crate::types::WordTransformation;
use crate::word_transformer;

#[derive(Debug)]
pub struct PasswordMaker<T>
where
    T: Rng + Default,
{
    rng: Box<T>,
    config: Config,
    wordlist: Vec<String>,
}

impl Default for PasswordMaker<ThreadRng> {
    fn default() -> PasswordMaker<ThreadRng> {
        Self {
            rng: Box::new(ThreadRng::default()),
            config: ConfigBuilder::new().build().unwrap(),
            wordlist: WORDLIST.iter().map(|s| String::from(*s)).collect(),
        }
    }
}

impl<T> PasswordMaker<T>
where
    T: Rng + Default,
{
    pub fn new(config: Config) -> PasswordMaker<T> {
        Self {
            rng: Box::new(T::default()),
            config,
            wordlist: WORDLIST.iter().map(|s| String::from(*s)).collect(),
        }
    }
    fn filter_wordlist(&self) -> Vec<u32> {
        let min_len = self.config.word_min_length as usize;
        let max_len = self.config.word_max_length as usize;
        let filtered_indices: Vec<u32> = self
            .wordlist
            .iter()
            .enumerate()
            .filter(|(_, word)| word.chars().count() >= min_len && word.chars().count() <= max_len)
            .map(|(i, _)| i as u32)
            .collect();
        filtered_indices
    }
    fn choose_words(&mut self, indices: &[u32]) -> Result<Vec<String>, String> {
        let quantity = self.config.word_count as usize;
        if quantity > self.wordlist.len() {
            return Err("TODO: Insufficient items in wordlist".to_owned());
        }
        let mut selected = indices.iter().choose_multiple(&mut self.rng, quantity);
        selected.shuffle(&mut self.rng);
        let result = selected
            .into_iter()
            .map(|n| self.wordlist[*n as usize].clone())
            .collect();
        Ok(result)
    }
    fn transform_words(&mut self, words: Vec<String>) -> Result<Vec<String>, String> {
        if words.is_empty() {
            return Ok(words);
        }
        Ok(match self.config.word_transformation {
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
        })
    }
    fn choose_n(&mut self, n: usize, collection: &[char]) -> Option<String> {
        if n == 0 {
            return None;
        }
        let mut buf = Vec::with_capacity(n);
        for _ in 0..n {
            buf.push(collection.choose(&mut self.rng).unwrap());
        }
        Some(buf.into_iter().collect())
    }
    fn create_pseudo_words(&mut self) -> (Option<String>, Option<String>) {
        let before = self.choose_n(self.config.digits_before as usize, &DIGIT_ALPHABET);
        let after = self.choose_n(self.config.digits_after as usize, &DIGIT_ALPHABET);
        (before, after)
    }
    fn choose_separator(&mut self) -> Option<char> {
        Some(
            *self
                .config
                .separator_character
                .choose(&mut self.rng)
                .unwrap(),
        )
    }
    fn create_padding(&mut self, password: &str) -> (Option<String>, Option<String>) {
        let len = self.config.padding_length as usize;
        let (before_len, after_len) = match self.config.padding_type {
            PaddingType::None => (0, 0),
            PaddingType::Fixed => (len, len),
            PaddingType::Adaptive => (0, len.saturating_sub(password.chars().count())),
        };
        let padding_character = Some(*self.config.padding_character.choose(&mut self.rng).unwrap());
        let before = iter::repeat(padding_character).take(before_len).collect();
        let after = iter::repeat(padding_character).take(after_len).collect();
        (before, after)
    }
    fn create_password(&mut self) -> Result<String, String> {
        let filtered_word_indices = self.filter_wordlist();
        let chosen_words = self.choose_words(&filtered_word_indices)?;
        let mut transformed_words = self.transform_words(chosen_words)?;
        let (front_digits, back_digits) = self.create_pseudo_words();
        let separator = self.choose_separator();
        let mut parts = vec![front_digits.unwrap_or("".to_owned())];
        parts.append(&mut transformed_words);
        parts.push(back_digits.unwrap_or("".to_owned()));
        let unpadded_password = parts.join(&separator.map(String::from).unwrap_or("".to_owned()));
        let (front_padding, rear_padding) = self.create_padding(&unpadded_password);
        let final_password = format!(
            "{}{}{}",
            front_padding.unwrap_or("".to_owned()),
            unpadded_password,
            rear_padding.unwrap_or("".to_owned())
        );
        Ok(final_password)
    }
    pub fn create_passwords(&mut self) -> Result<Vec<String>, String> {
        let count = self.config.count as usize;
        let mut buf = Vec::with_capacity(count);
        for _ in 0..count {
            buf.push(self.create_password()?);
        }
        Ok(buf)
    }
}

#[cfg(test)]
mod password_maker_tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn test_filter_wordlist() {
        let mut maker = PasswordMaker::default();
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
        assert_eq!(result.len(), matches);
        assert_eq!(expected.len(), matches);
    }

    #[test]
    fn test_choose_words_ok() {
        const WORD_COUNT: u8 = 2;

        let mut maker = PasswordMaker::default();
        maker.config.word_count = WORD_COUNT;

        let indices: [u32; 2] = [1, 2];

        let result = maker.choose_words(&indices);
        assert_eq!(result.unwrap().len(), WORD_COUNT as usize);
    }

    #[ignore = "not written yet"]
    #[test]
    fn test_choose_words_result_is_shuffled() {
        assert!(false);
    }

    #[test]
    fn test_choose_words_err() {
        const WORD_COUNT: u8 = 100;

        let mut maker = PasswordMaker::default();
        maker.wordlist = make_wordlist();
        maker.config.word_count = WORD_COUNT;

        let indices: [u32; 2] = [1, 2];

        let result = maker.choose_words(&indices);
        assert!(result.is_err());
    }
}
