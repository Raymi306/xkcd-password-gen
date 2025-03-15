pub trait StrIsEnumMember: Sized {
    fn to_static_str(&'static self) -> &'static str;
    fn into_iter() -> impl Iterator<Item = (&'static str, Self)>;
    fn to_member(name: &str) -> Result<Self, ()>;
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
    fn to_static_str(&'static self) -> &'static str {
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
    fn to_member(name: &str) -> Result<Self, ()> {
        Self::into_iter()
            .find(|(s, _)| *s == name)
            .map(|inner| inner.1)
            .ok_or(())
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
    fn to_static_str(&'static self) -> &'static str {
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
    fn to_member(name: &str) -> Result<Self, ()> {
        Self::into_iter()
            .find(|(s, _)| *s == name)
            .map(|inner| inner.1)
            .ok_or(())
    }
}
