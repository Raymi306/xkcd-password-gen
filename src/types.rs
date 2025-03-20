use std::fmt;

#[derive(Clone, Debug)]
pub enum ValidationError {
    InvalidNumber(String, u8, u8),
    InvalidEnum(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidNumber(value, min, max) => {
                format!("`{value}` must be a positive integer between {min} and {max}")
            }
            Self::InvalidEnum(msg) => msg.clone(),
        };
        write!(f, "{msg}")
    }
}

impl std::error::Error for ValidationError {}

// TODO make a derive macro for this.
// Handle Display impl
pub trait StrIsEnumMember: Sized {
    const NAME: &'static str;
    fn to_static_str(&self) -> &'static str;
    fn into_iter() -> impl Iterator<Item = (&'static str, Self)>;
    fn to_member(member: &str) -> Result<Self, ValidationError> {
        Self::into_iter()
            .find(|(s, _)| *s == member)
            .map(|inner| inner.1)
            .ok_or_else(|| {
                let valid_choices = Self::into_iter()
                    .map(|inner| inner.0)
                    .collect::<Vec<&str>>()
                    .join(", ");
                let parent = Self::NAME;
                let msg = format!(
                    "`{member}` is not a valid {parent}. Possible choices: {valid_choices}"
                );
                ValidationError::InvalidEnum(msg)
            })
    }
}

#[derive(Debug, Default)]
pub enum WordTransformationType {
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

impl StrIsEnumMember for WordTransformationType {
    const NAME: &'static str = "WordTransformationType";
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

impl fmt::Display for WordTransformationType {
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
    const NAME: &'static str = "PaddingType";
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

impl fmt::Display for PaddingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = self.to_static_str();
        write!(f, "{msg}")
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum RngType {
    #[default]
    OsRng,
    Csprng,
}

impl StrIsEnumMember for RngType {
    const NAME: &'static str = "RngType";
    fn to_static_str(&self) -> &'static str {
        match self {
            Self::OsRng => "osrng",
            Self::Csprng => "csprng",
        }
    }
    fn into_iter() -> impl Iterator<Item = (&'static str, Self)> {
        [
            (Self::OsRng.to_static_str(), Self::OsRng),
            (Self::Csprng.to_static_str(), Self::Csprng),
        ]
        .into_iter()
    }
}

impl fmt::Display for RngType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = self.to_static_str();
        write!(f, "{msg}")
    }
}
