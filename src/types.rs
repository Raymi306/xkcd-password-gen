use std::fmt;

#[derive(Clone, Debug)]
pub enum ValidationError {
    InvalidNumber(u8, u8),
    InvalidEnum(String),
    EmptyString,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidNumber(min, max) => {
                format!("Value must be a positive integer between {min} and {max}")
            }
            Self::InvalidEnum(msg) => msg.clone(),
            Self::EmptyString => "Value must not be empty".to_owned(),
        };
        write!(f, "{msg}")
    }
}

impl std::error::Error for ValidationError {}

// TODO make a derive macro for this
pub trait StrIsEnumMember: Sized {
    fn to_static_str(&self) -> &'static str;
    fn into_iter() -> impl Iterator<Item = (&'static str, Self)>;
    fn to_member(name: &str) -> Result<Self, ValidationError> {
        Self::into_iter()
            .find(|(s, _)| *s == name)
            .map(|inner| inner.1)
            .ok_or_else(|| {
                let valid_choices = Self::into_iter()
                    .map(|inner| inner.0)
                    .collect::<Vec<&str>>()
                    .join(", ");
                let msg = format!("`{name}` is not a valid enum member. Possible members: {valid_choices}");
                ValidationError::InvalidEnum(msg)
            })
    }
}

#[derive(Debug, Default)]
pub enum WordTransformation {
    None,
    Lower,
    Upper,
    CapitalizeFirst,
    CapitalizeLast,
    CapitalizeNotFirst,
    #[default]
    AlternatingLowerUpper,
    AlternatingUpperLower,
    RandomUpperLower,
}

impl StrIsEnumMember for WordTransformation {
    fn to_static_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Lower => "lower",
            Self::Upper => "upper",
            Self::CapitalizeFirst => "capitalize-first",
            Self::CapitalizeLast => "capitalize-last",
            Self::CapitalizeNotFirst => "capitalize-not-first",
            Self::AlternatingLowerUpper => "alternating-lower-upper",
            Self::AlternatingUpperLower => "alternating-upper-lower",
            Self::RandomUpperLower => "random-upper-lower",
        }
    }
    fn into_iter() -> impl Iterator<Item = (&'static str, Self)> {
        [
            (Self::None.to_static_str(), Self::None),
            (Self::Lower.to_static_str(), Self::Lower),
            (Self::Upper.to_static_str(), Self::Upper),
            (Self::CapitalizeFirst.to_static_str(), Self::CapitalizeFirst),
            (Self::CapitalizeLast.to_static_str(), Self::CapitalizeLast),
            (
                Self::CapitalizeNotFirst.to_static_str(),
                Self::CapitalizeNotFirst,
            ),
            (
                Self::AlternatingLowerUpper.to_static_str(),
                Self::AlternatingLowerUpper,
            ),
            (
                Self::AlternatingUpperLower.to_static_str(),
                Self::AlternatingUpperLower,
            ),
            (
                Self::RandomUpperLower.to_static_str(),
                Self::RandomUpperLower,
            ),
        ]
        .into_iter()
    }
}

impl fmt::Display for &'static WordTransformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = self.to_static_str();
        write!(f, "{msg}")
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum PaddingType {
    None,
    #[default]
    Fixed,
    Adaptive,
}

impl StrIsEnumMember for PaddingType {
    fn to_static_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Fixed => "fixed",
            Self::Adaptive => "adaptive",
        }
    }
    fn into_iter() -> impl Iterator<Item = (&'static str, Self)> {
        [
            (Self::None.to_static_str(), Self::None),
            (Self::Fixed.to_static_str(), Self::Fixed),
            (Self::Adaptive.to_static_str(), Self::Adaptive),
        ]
        .into_iter()
    }
}

impl fmt::Display for &'static PaddingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = self.to_static_str();
        write!(f, "{msg}")
    }
}
