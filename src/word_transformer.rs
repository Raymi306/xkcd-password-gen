use std::iter;

use rand::prelude::*;

pub fn lower(words: Vec<String>) -> Vec<String> {
    words.into_iter().map(|word| word.to_ascii_lowercase()).collect()
}
pub fn upper(words: Vec<String>) -> Vec<String> {
    words.into_iter().map(|word| word.to_ascii_uppercase()).collect()
}
pub fn capitalize_first(words: Vec<String>) -> Vec<String> {
    words.into_iter().map(capitalize_first_char).collect()
}
pub fn capitalize_last(words: Vec<String>) -> Vec<String> {
    words.into_iter().map(capitalize_last_char).collect()
}
pub fn capitalize_not_first(words: Vec<String>) -> Vec<String> {
    words.into_iter().map(capitalize_not_first_char).collect()
}
pub fn alternating_lower_upper(words: Vec<String>) -> Vec<String> {
    words.into_iter().enumerate().map(|(i, word)| if i % 2 == 0 { word.to_ascii_lowercase() } else { word.to_ascii_uppercase() }).collect()
}
pub fn alternating_upper_lower(words: Vec<String>) -> Vec<String> {
    words.into_iter().enumerate().map(|(i, word)| if i % 2 == 0 { word.to_ascii_uppercase() } else { word.to_ascii_lowercase() }).collect()
}
pub fn random_upper_lower(rng: &mut (impl Rng + ?Sized), words: Vec<String>) -> Vec<String> {
    words.into_iter().map(|word| if rng.random::<bool>() { word.to_ascii_uppercase() } else { word.to_ascii_lowercase() }).collect()
}
fn capitalize_first_char(word: String) -> String {
    let first = word.chars().take(1).map(|c| c.to_ascii_uppercase());
    first.chain(word.chars().skip(1)).collect()
}
fn capitalize_last_char(word: String) -> String {
    // UTF character length weirdness reminder
    let num_chars = word.chars().count();
    if num_chars == 0 { return word };
    // unwrap guard ^
    word.chars().take(num_chars - 1).chain(iter::once(word.chars().last().unwrap().to_ascii_uppercase())).collect()
}
fn capitalize_not_first_char(word: String) -> String {
    if word.len() <= 1 {
        return word;
    }
    // unwrap guard ^
    iter::once(word.chars().next().unwrap()).chain(word.chars().skip(1).map(|c| c.to_ascii_uppercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn test_capitalize_first() {
        assert_eq!("Foo".to_owned(), capitalize_first_char("foo".to_owned()));
    }

    #[test]
    fn test_capitalize_first_empty() {
        assert_eq!("".to_owned(), capitalize_first_char("".to_owned()));
    }

    #[test]
    fn test_capitalize_last() {
        assert_eq!("foO".to_owned(), capitalize_last_char("foo".to_owned()));
    }

    #[test]
    fn test_capitalize_last_empty() {
        assert_eq!("".to_owned(), capitalize_last_char("".to_owned()));
    }

    #[test]
    fn test_capitalize_not_first() {
        assert_eq!("fOO".to_owned(), capitalize_not_first_char("foo".to_owned()));
    }

    #[test]
    fn test_capitalize_not_first_empty() {
        assert_eq!("".to_owned(), capitalize_not_first_char("".to_owned()));
    }

    #[test]
    fn test_capitalize_not_first_len_1() {
        assert_eq!("a".to_owned(), capitalize_not_first_char("a".to_owned()));
    }

    #[test]
    fn test_word_transformer_lower() {
        let result = lower(make_wordlist());
        for word in result {
            assert!(string_is_lowercase(word));
        }
    }

    #[test]
    fn test_word_transformer_upper() {
        let result = upper(make_wordlist());
        for word in result {
            assert!(string_is_uppercase(word));
        }
    }

    #[test]
    fn test_word_transformer_capitalize_first() {
        let result = capitalize_first(vec!("foo".to_owned(), "bar".to_owned()));
        assert!(result[0] == "Foo" && result[1] == "Bar")
    }

    #[test]
    fn test_word_transformer_capitalize_last() {
        let result = capitalize_last(vec!("foo".to_owned(), "bar".to_owned()));
        assert!(result[0] == "foO" && result[1] == "baR")
    }

    #[test]
    fn test_word_transformer_capitalize_not_first() {
        let result = capitalize_not_first(vec!("foo".to_owned(), "bar".to_owned()));
        assert!(result[0] == "fOO" && result[1] == "bAR")
    }

    #[test]
    fn test_word_transformer_alternating_lower_upper() {
        let sample = vec!(
            "foo".to_owned(),
            "bar".to_owned(),
            "baz".to_owned(),
            "bee".to_owned(),
        );
        let result = alternating_lower_upper(sample);
        println!("{:?}", result);
        assert_eq!(result[0], "foo");
        assert_eq!(result[1], "BAR");
        assert_eq!(result[2], "baz");
        assert_eq!(result[3], "BEE");
    }

    #[test]
    fn test_word_transformer_alternating_upper_lower() {
        let sample = vec!(
            "foo".to_owned(),
            "bar".to_owned(),
            "baz".to_owned(),
            "bee".to_owned(),
        );
        let result = alternating_upper_lower(sample);
        println!("{:?}", result);
        assert_eq!(result[0], "FOO");
        assert_eq!(result[1], "bar");
        assert_eq!(result[2], "BAZ");
        assert_eq!(result[3], "bee");
    }

    #[ignore = "needs random seeding"]
    #[test]
    fn test_word_transformer_random() {
        let mut rng = ThreadRng::default();
        random_upper_lower(&mut rng, vec!("hello".to_owned(), "world".to_owned()));
    }
}
