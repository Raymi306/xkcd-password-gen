use config_builder_derive::AutoConfigBuilder;

use crate::consts::DEFAULT_COUNT;
use crate::consts::DEFAULT_DIGITS_AFTER;
use crate::consts::DEFAULT_DIGITS_BEFORE;
use crate::consts::DEFAULT_PADDING_LENGTH_ADAPTIVE;
use crate::consts::DEFAULT_PADDING_LENGTH_FIXED;
use crate::consts::DEFAULT_SYMBOL_ALPHABET;
use crate::consts::DEFAULT_WORD_COUNT;
use crate::consts::DEFAULT_WORD_MAX_LENGTH;
use crate::consts::DEFAULT_WORD_MIN_LENGTH;
use crate::types::Integer;
use crate::types::PaddingType;
use crate::types::RngType;
use crate::types::StrEnum;
use crate::types::ValidationError;
use crate::types::WordTransformationType;

#[derive(Debug)]
pub struct Config {
    pub count: u8,
    pub word_count: u8,
    pub word_min_length: u8,
    pub word_max_length: u8,
    pub word_transformation: WordTransformationType,
    pub digits_before: u8,
    pub digits_after: u8,
    pub padding_type: PaddingType,
    pub padding_length: u8,
    pub padding_character: Vec<char>,
    pub separator_character: Vec<char>,
    pub rng_type: RngType,
}

#[derive(AutoConfigBuilder, Debug, Default)]
pub struct ConfigBuilder {
    count: Option<String>,
    word_count: Option<String>,
    word_min_length: Option<String>,
    word_max_length: Option<String>,
    word_transformation: Option<String>,
    digits_before: Option<String>,
    digits_after: Option<String>,
    padding_type: Option<String>,
    padding_length: Option<String>,
    padding_character: Option<String>,
    separator_character: Option<String>,
    rng_type: Option<String>,
}

fn validate_int<T: Integer>(
    value: Option<String>,
    min: T,
    max: T,
    default: T,
) -> Result<T, ValidationError> {
    value.map_or(Ok(default), |inner| {
        /*
         * the below would be preferable and would avoid the unsafe block,
         * but it won't compile without cloning `inner`.
         * if the compiler can't figure that out, not super confident about
         * it optimizing unwrap() to unwrap_unchecked() behind the scenes.
         * yes, this is a microoptimization that doesn't matter, but
         * it also avoids some ugly generic bounds that `expect/unwrap` require.
         *
         * ```
         * let result = inner.parse::<T>().map_err(|_| {
         *     ValidationError::InvalidNumber(inner, min.into(), max.into())
         * })?;
         * ```
         */
        let parse_result = inner.parse::<T>();

        if parse_result.is_err() {
            return Err(ValidationError::InvalidNumber(
                inner,
                min.into(),
                max.into(),
            ));
        }

        #[expect(unsafe_code, reason = "error case explicitly handled above")]
        let result = unsafe { parse_result.unwrap_unchecked() };

        if (min..=max).contains(&result) {
            Ok(result)
        } else {
            Err(ValidationError::InvalidNumber(
                inner,
                min.into(),
                max.into(),
            ))
        }
    })
}

fn validate_enum<T: StrEnum>(value: Option<String>) -> Result<T, ValidationError> {
    value.map_or(Ok(T::default()), |inner| {
        T::to_member(&inner.to_ascii_lowercase())
    })
}

fn uniquify_chars(value: Option<String>, default: &[char]) -> Vec<char> {
    value.map_or_else(
        || default.to_vec(),
        |inner| {
            let mut result = inner.chars().collect::<Vec<char>>();
            result.sort_unstable();
            result.dedup();
            result
        },
    )
}

impl ConfigBuilder {
    pub fn build(self) -> Result<Config, ValidationError> {
        let count = validate_int::<u8>(self.count, 1, 255, DEFAULT_COUNT)?;
        let word_count = validate_int::<u8>(self.word_count, 0, 32, DEFAULT_WORD_COUNT)?;
        let word_min_length =
            validate_int::<u8>(self.word_min_length, 1, 255, DEFAULT_WORD_MIN_LENGTH)?;
        let word_max_length = validate_int::<u8>(
            self.word_max_length,
            word_min_length,
            255,
            DEFAULT_WORD_MAX_LENGTH,
        )?;
        let word_transformation =
            validate_enum::<WordTransformationType>(self.word_transformation)?;
        let digits_before = validate_int::<u8>(self.digits_before, 0, 255, DEFAULT_DIGITS_BEFORE)?;
        let digits_after = validate_int::<u8>(self.digits_after, 0, 255, DEFAULT_DIGITS_AFTER)?;
        let padding_character = uniquify_chars(self.padding_character, &DEFAULT_SYMBOL_ALPHABET);
        let padding_type = if padding_character.is_empty() {
            PaddingType::None
        } else {
            validate_enum::<PaddingType>(self.padding_type)?
        };
        let padding_length = validate_int::<u8>(self.padding_length, 0, 255, {
            match padding_type {
                PaddingType::Fixed => DEFAULT_PADDING_LENGTH_FIXED,
                PaddingType::Adaptive => DEFAULT_PADDING_LENGTH_ADAPTIVE,
                PaddingType::None => 0,
            }
        })?;
        let separator_character =
            uniquify_chars(self.separator_character, &DEFAULT_SYMBOL_ALPHABET);
        let rng_type = validate_enum::<RngType>(self.rng_type)?;

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
            rng_type,
        })
    }
}
