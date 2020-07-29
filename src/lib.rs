//! Defines structures that can be used to work with conventional-commits commit
//! messages.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// The `:<space>` separator.
pub const SEPARATOR_COLON: &str = ": ";

/// The `<space>#` separator for footer notes.
pub const SEPARATOR_HASHTAG: &str = " #";

/// A commit message.
///
/// The different sections are separated by an empty newline.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Commit<'a> {
    /// A body.
    pub body: Option<&'a str>,
    /// A short summary.
    pub desc: &'a str,
    /// A list of footers. Empty when none.
    pub footer: Vec<Footer<'a>>,
    /// Set if the commit is a breaking change.
    pub is_breaking_change: bool,
    /// The optional scope of the commit.
    pub scope: Option<&'a str>,
    /// The type of the commit.
    pub ty: &'a str,
}

impl<'a> Commit<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(
        ty: &'a str,
        scope: Option<&'a str>,
        desc: &'a str,
        body: Option<&'a str>,
        is_breaking_change: bool,
        footer: Vec<Footer<'a>>,
    ) -> Self {
        Self {
            ty,
            scope,
            desc,
            body,
            is_breaking_change,
            footer,
        }
    }
}

/// A commit footer.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Footer<'a> {
    /// The footer word token.
    pub token: &'a str,
    /// The separator.
    pub separator: FooterSeparator,
    /// The footer's value.
    pub value: &'a str,
}

impl<'a> Footer<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(token: &'a str, separator: FooterSeparator, value: &'a str) -> Self {
        Self {
            token,
            separator,
            value,
        }
    }
}

/// The separator used to separate the token and value of a footer.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum FooterSeparator {
    ColonSpace,
    SpaceHashTag,
}

impl Default for FooterSeparator {
    fn default() -> Self {
        FooterSeparator::ColonSpace
    }
}

impl fmt::Display for FooterSeparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FooterSeparator::ColonSpace => write!(f, "{}", SEPARATOR_COLON),
            FooterSeparator::SpaceHashTag => write!(f, "{}", SEPARATOR_HASHTAG),
        }
    }
}

impl FromStr for FooterSeparator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            SEPARATOR_COLON => Ok(FooterSeparator::ColonSpace),
            SEPARATOR_HASHTAG => Ok(FooterSeparator::SpaceHashTag),
            _ => Err("footer separator not recognized".to_string()),
        }
    }
}
