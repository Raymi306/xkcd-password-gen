use std::fmt;

use strenum::StrEnum;

type MinimalSupportedInteger = u8;

pub trait Integer: std::str::FromStr + Into<MinimalSupportedInteger> + PartialOrd + Copy {}

// to support more builtin integer types, just add below with specific types
// and change MinimalSupportedInteger
// eg.
//     impl Integer for u8 {}
//     type MinimalSupportedInteger = u16
impl Integer for MinimalSupportedInteger {}

#[derive(Clone, Debug)]
pub enum ValidationError {
    // to support numbers larger or smaller than T<u8>, change u8
    InvalidNumber(String, MinimalSupportedInteger, MinimalSupportedInteger),
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

pub trait StrEnum: Sized + Default + Clone + Copy
where
    Self: 'static,
{
    const NAME: &'static str;
    const NAME_MEMBER_ARR: &[(&str, Self)];
    fn to_static_str(&self) -> &'static str;
    fn into_iter() -> impl Iterator<Item = &'static (&'static str, Self)>;
    fn to_member(member: &str) -> Result<&Self, ValidationError> {
        Self::into_iter()
            .find(|(s, _)| *s == member)
            .map(|inner| &inner.1)
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

#[derive(StrEnum, Copy, Clone, Debug)]
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

#[derive(StrEnum, Copy, Clone, Debug)]
pub enum PaddingType {
    None,
    #[default]
    Fixed,
    Adaptive,
}

// TODO, fixes main.rs help brittleness in a const manner
// do more stupid stuff with macros
//
// #[derive(StrEnum, Copy, Clone, Debug)]
// pub enum RngType {
//     #[default, example = "(the system's native secure RNG)"]
//     OsRng,
//     #[example = "(a reasonably secure userspace RNG)"]
//     Csprng,
// }
#[derive(StrEnum, Copy, Clone, Debug)]
pub enum RngType {
    #[default]
    OsRng,
    Csprng,
}
