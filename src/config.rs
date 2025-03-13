use crate::DEFAULT_SYMBOL_ALPHABET;
use crate::WordTransformation;
use crate::PaddingType;
use crate::StrIsEnumMember;

#[derive(Clone, Debug)]
pub enum ValidationError {
    InvalidNumber(u8, u8),
    InvalidEnum(String),
    DuplicateChoices,
}

// TODO impl Display and uncomment
//impl std::error::Error for ValidationError {}

fn validate_option_string_u8(val: Option<String>, default: u8, min: u8, max: u8) -> Result<u8, ValidationError> {
    let result = val
        .map(|inner| inner.parse::<u8>())
        .transpose()
        .map_err(|_| ValidationError::InvalidNumber(min, max))
        .map(|inner| inner.unwrap_or(default))?;
    if result < min || result > max {
        Err(ValidationError::InvalidNumber(min, max))
    } else {
        Ok(result)
    }
}

fn string_to_unique_chars(val: String) -> Vec<char> {
    let mut result = val.chars().collect::<Vec<char>>();
    result.sort();
    result.dedup();
    result
}

fn validate_option_string_enum<T>(val: Option<String>) -> Result<T, ValidationError>
where T: StrIsEnumMember + Default {
    val
        .map(|inner| T::to_member(&inner.to_ascii_lowercase()))
        .transpose()
        .map_err(|_| ValidationError::InvalidEnum("TODO".to_owned()))
        .map(|inner| inner.unwrap_or_default())
}

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

#[derive(Debug, Default)]
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

pub type BuildResult = std::result::Result<Config, ValidationError>;

impl ConfigBuilder {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn build(self) -> BuildResult {
        let count = validate_option_string_u8(self.count, 1, 1, 255)?;
        let word_count = validate_option_string_u8(self.word_count, 4, 0, 32)?;
        let word_min_length = validate_option_string_u8(self.word_min_length, 4, 1, 255)?;
        let word_max_length = validate_option_string_u8(self.word_max_length, 11, word_min_length, 255)?;
        let word_transformation: WordTransformation = validate_option_string_enum(self.word_transformation)?;
        let digits_before = validate_option_string_u8(self.digits_before, 2, 0, 255)?;
        let digits_after = validate_option_string_u8(self.digits_after, 2, 0, 255)?;
        let padding_type: PaddingType = validate_option_string_enum(self.padding_type)?;
        let padding_length = validate_option_string_u8(self.padding_length, 2, 0, 255)?;
        // TODO this will surely fail with empty string?
        let padding_character = if let Some(inner) = self.padding_character {
            string_to_unique_chars(inner)
        } else {
            DEFAULT_SYMBOL_ALPHABET.to_vec()
        };
        let separator_character = if let Some(inner) = self.separator_character {
            string_to_unique_chars(inner)
        } else {
            DEFAULT_SYMBOL_ALPHABET.to_vec()
        };
        let result = Config {
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
        };
        Ok(result)
    }
    // TODO macro it up in here
    pub fn count(mut self, value: Option<String>) -> Self {
        self.count = value;
        self
    }
    pub fn word_count(mut self, value: Option<String>) -> Self {
        self.word_count = value;
        self
    }
    pub fn word_min_length(mut self, value: Option<String>) -> Self {
        self.word_min_length = value;
        self
    }
    pub fn word_max_length(mut self, value: Option<String>) -> Self {
        self.word_max_length = value;
        self
    }
    pub fn word_transformation(mut self, value: Option<String>) -> Self {
        self.word_transformation = value;
        self
    }
    pub fn digits_before(mut self, value: Option<String>) -> Self {
        self.digits_before = value;
        self
    }
    pub fn digits_after(mut self, value: Option<String>) -> Self {
        self.digits_after = value;
        self
    }
    pub fn padding_type(mut self, value: Option<String>) -> Self {
        self.padding_type = value;
        self
    }
    pub fn padding_length(mut self, value: Option<String>) -> Self {
        self.padding_length = value;
        self
    }
    pub fn padding_character(mut self, value: Option<String>) -> Self {
        self.padding_character = value;
        self
    }
    pub fn separator_character(mut self, value: Option<String>) -> Self {
        self.separator_character = value;
        self
    }
}
