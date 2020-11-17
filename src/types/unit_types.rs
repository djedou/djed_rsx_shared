
#[cfg(feature = "impl-external-yoga")]
use djed_yoga;

use djed_self_tokenize_macro::SelfTokenize;
use djed_self_tokenize_trait::ToCustomTokens;

#[derive(Debug, PartialEq, Copy, Clone, SelfTokenize)]
pub enum SharedUnit {
    None,
    Point(u32),
    Percent(u32),
    Auto
}

impl SharedUnit {
    pub fn point(&self) -> u32 {
        if let &SharedUnit::Point(value) = self {
            value
        } else {
            panic!("Not a point");
        }
    }

    pub fn percent(&self) -> u32 {
        if let &SharedUnit::Percent(value) = self {
            value
        } else {
            panic!("Not a percentage");
        }
    }
}

impl AsRef<str> for SharedUnit {
    fn as_ref(&self) -> &str {
        match self {
            &SharedUnit::None => "",
            &SharedUnit::Point(_) => "pt",
            &SharedUnit::Percent(_) => "%",
            &SharedUnit::Auto => "auto"
        }
    }
}

#[cfg(feature = "impl-external-yoga")]
impl From<djed_yoga::StyleUnit> for SharedUnit {
    fn from(value: djed_yoga::StyleUnit) -> Self {
        match value {
            djed_yoga::StyleUnit::UndefinedValue => SharedUnit::None,
            djed_yoga::StyleUnit::Point(v) => SharedUnit::Point(v.into_inner() as u32),
            djed_yoga::StyleUnit::Percent(v) => SharedUnit::Percent(v.into_inner() as u32),
            djed_yoga::StyleUnit::Auto => SharedUnit::Auto
        }
    }
}
