//! Defines structures that can be used to work with conventional-commits commit
//! messages.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A commit message.
///
/// The different sections are separated by an empty newline.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Commit<'a> {
    /// A body.
    pub body: &'a str,
    /// A short summary.
    pub desc: &'a str,
    /// A list of footers.
    pub footer: Vec<Footer<'a>>,
    /// Set if the commit is a breaking change.
    pub is_breaking_change: bool,
    /// The scope of the commit.
    pub scope: &'a str,
    /// The type of the commit.
    pub ty: &'a str,
}

/// A commit footer.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Footer<'a> {
    /// The footer word token.
    pub token: &'a str,
    /// The footer's value.
    pub value: &'a str,
}
