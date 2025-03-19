use config_builder_derive::StringConfBuilder;

use crate::consts::DEFAULT_SYMBOL_ALPHABET;
use crate::types::PaddingType;
use crate::types::StrIsEnumMember;
use crate::types::WordTransformation;
use crate::types::ValidationError;

#[derive(Debug)]
pub struct Config {
    pub count: u8,
    pub word_count: u8,
    pub word_min_length: u8,
    pub word_max_length: u8,
    pub word_transformation: WordTransformation,
    pub separator_character: Vec<char>,
    pub digits_before: u8,
    pub digits_after: u8,
    pub padding_type: PaddingType,
    pub padding_length: u8,
    pub padding_character: Vec<char>,
}

const DEFAULT_COUNT: u8 = 2;
const DEFAULT_WORD_COUNT: u8 = 4;
const DEFAULT_WORD_MIN_LENGTH: u8 = 3;
const DEFAULT_WORD_MAX_LENGTH: u8 = 11;
const DEFAULT_WORD_TRANSFORMATION: WordTransformation = WordTransformation::AlternatingLowerUpper;
const DEFAULT_DIGITS_BEFORE: u8 = 2;
const DEFAULT_DIGITS_AFTER: u8 = 2;
const DEFAULT_PADDING_TYPE: PaddingType = PaddingType::Fixed;
const DEFAULT_PADDING_LENGTH_FIXED: u8 = 2;
const DEFAULT_PADDING_LENGTH_ADAPTIVE: u8 = 32;

#[derive(StringConfBuilder, Debug, Default)]
pub struct ConfigBuilder {
    count: Option<String>,
    word_count: Option<String>,
    word_min_length: Option<String>,
    word_max_length: Option<String>,
    word_transformation: Option<String>,
    separator_character: Option<String>,
    digits_before: Option<String>,
    digits_after: Option<String>,
    padding_type: Option<String>,
    padding_length: Option<String>,
    padding_character: Option<String>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> Result<Config, ValidationError> {
        macro_rules! validate_u8 {
            ($value:ident, $min:expr, $max:expr, $default:expr) => {
                if let Some(inner) = self.$value {
                    let result = inner
                        .parse::<u8>()
                        .map_err(|_| ValidationError::InvalidNumber($min, $max))?;

                    if !($min..=$max).contains(&result) {
                        Err(ValidationError::InvalidNumber($min, $max))
                    } else {
                        Ok(result)
                    }?
                } else {
                    $default
                }
            };
        }

        macro_rules! validate_enum {
            ($value:ident, $type:ty, $default:expr) => {
                if let Some(inner) = self.$value {
                    <$type>::to_member(&inner.to_ascii_lowercase())
                        .map_err(|_| ValidationError::InvalidEnum("TODO".to_owned()))?
                } else {
                    $default
                }
            };
        }

        macro_rules! unique_chars {
            ($value:ident, $default:ident) => {
                if let Some(inner) = self.$value {
                    if inner.is_empty() {
                        return Err(ValidationError::EmptyString);
                    }
                    let mut result = inner.chars().collect::<Vec<char>>();
                    result.sort();
                    result.dedup();
                    Ok(result)
                } else {
                    Ok($default.to_vec())
                }?
            };
        }

        let count = validate_u8!(count, 1, 255, DEFAULT_COUNT);
        let word_count = validate_u8!(word_count, 0, 32, DEFAULT_WORD_COUNT);
        let word_min_length = validate_u8!(word_min_length, 1, 255, DEFAULT_WORD_MIN_LENGTH);
        let word_max_length = validate_u8!(
            word_max_length,
            word_min_length,
            255,
            DEFAULT_WORD_MAX_LENGTH
        );
        let word_transformation = validate_enum!(
            word_transformation,
            WordTransformation,
            DEFAULT_WORD_TRANSFORMATION
        );
        let digits_before = validate_u8!(digits_before, 0, 255, DEFAULT_DIGITS_BEFORE);
        let digits_after = validate_u8!(digits_after, 0, 255, DEFAULT_DIGITS_AFTER);
        let padding_type = validate_enum!(padding_type, PaddingType, DEFAULT_PADDING_TYPE);
        let padding_length = validate_u8!(padding_length, 0, 255, {
            match padding_type {
                PaddingType::Fixed => DEFAULT_PADDING_LENGTH_FIXED,
                PaddingType::Adaptive => DEFAULT_PADDING_LENGTH_ADAPTIVE,
                PaddingType::None => 0,
            }
        });
        let padding_character = unique_chars!(padding_character, DEFAULT_SYMBOL_ALPHABET);
        let separator_character = unique_chars!(separator_character, DEFAULT_SYMBOL_ALPHABET);

        Ok(Config {
            count,
            word_count,
            word_min_length,
            word_max_length,
            word_transformation,
            digits_before,
            digits_after,
            padding_type,
            padding_length,
            padding_character,
            separator_character,
        })
    }
}
