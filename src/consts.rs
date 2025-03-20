use crate::types::PaddingType;
use crate::types::WordTransformation;

pub const DIGIT_ALPHABET: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub const DEFAULT_SYMBOL_ALPHABET: [char; 18] = [
    '!', '@', '$', '%', '^', '&', '*', '-', '_', '+', '=', ':', '|', '~', '?', '/', '.', ';',
];

pub const DEFAULT_COUNT: u8 = 1;
pub const DEFAULT_WORD_COUNT: u8 = 4;
pub const DEFAULT_WORD_MIN_LENGTH: u8 = 3;
pub const DEFAULT_WORD_MAX_LENGTH: u8 = 11;
pub const DEFAULT_WORD_TRANSFORMATION: WordTransformation =
    WordTransformation::AlternatingLowerUpper;
pub const DEFAULT_DIGITS_BEFORE: u8 = 2;
pub const DEFAULT_DIGITS_AFTER: u8 = 2;
pub const DEFAULT_PADDING_TYPE: PaddingType = PaddingType::Fixed;
pub const DEFAULT_PADDING_LENGTH_FIXED: u8 = 2;
pub const DEFAULT_PADDING_LENGTH_ADAPTIVE: u8 = 42;
