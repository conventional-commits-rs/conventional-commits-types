//! Defines structures that can be used to work with conventional commits.
//! The implementation resembles the v1.0.0 specification defined over at [conventionalcommits.org](https://www.conventionalcommits.org/en/v1.0.0/#specification).

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// The `:<space>` separator.
pub const SEPARATOR_COLON: &str = ": ";

/// The `<space>#` separator for footer notes.
///
/// This type of separator is mostly used when using issues or PR numbers as
/// value.
pub const SEPARATOR_HASHTAG: &str = " #";

/// A commit message.
///
/// As per the specification, a commit message is made out of a mandatory
/// description, an optional body and `0..n` optional footers. The different
/// sections are separated by an empty newline. Footers can be each separated
/// with a newline, however, this is not needed.
///
/// # Example
///
/// ```text
/// feat(some scope): a short and concise description
///
/// This is a longer body message. It can wrapped around
/// and be put onto multiple lines.
///
/// This is still part of the body.
///
/// Fixes #123
/// PR-close #124
/// Signed-off-by: SirWindfield
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Commit<'a> {
    /// The optional body.
    pub body: Option<&'a str>,
    /// The mandatory description.
    pub desc: &'a str,
    /// A list of footers. Empty when none are part of the commit message.
    pub footer: Vec<Footer<'a>>,
    /// Set if the commit is a breaking change.
    pub is_breaking_change: bool,
    /// The optional scope.
    pub scope: Option<&'a str>,
    /// The mandatory type.
    ///
    /// Types other than `feat` and `fix` are optional. For more information, please take a look at the [specification](https://www.conventionalcommits.org/en/v1.0.0/#specification), paragraphs 1-3.
    pub ty: &'a str,
}

impl<'a> Commit<'a> {
    /// Creates a default commit.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a commit with the given values.
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

/// A commit message footer.
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
    /// Creates a default footer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a footer with the given values.
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
    /// The `:<space>` separator is mostly used for values that do not involve
    /// issue or PR references.
    ColonSpace,
    /// The `#<space>` separator is, more often than not, used for issue and PR
    /// references.
    SpaceHashTag,
}

impl Default for FooterSeparator {
    /// Returns the default FooterSeparator, the ColonSpace.
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
