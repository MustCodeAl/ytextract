//! Error produced by YouTube and this library

/// Errors produced by this Library
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A Error that can occur when requesting web content
    #[error("An Error occurred while requesting web content: {0}")]
    Request(#[from] reqwest::Error),

    /// A Error reported by YouTube
    #[error(transparent)]
    Youtube(#[from] Youtube),
}

/// A Error reported by YouTube.
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub struct Youtube(pub(crate) String);

impl std::fmt::Display for Youtube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// The Error produced when a invalid Id is found
#[derive(thiserror::Error, Debug, Clone)]
pub enum Id<const N: usize> {
    /// A invalid Id was found.
    ///
    /// A Id is only valid when all characters are on of:
    ///
    /// - `0..=9`
    /// - `a..=z`
    /// - `A..=Z`
    /// - `_`
    /// - `-`
    #[error("Found invalid id: '{0}'")]
    InvalidId(String),

    /// A Id was not the expected length
    #[error("Expected a id of length {N} but found a id of length {0}")]
    InvalidLength(usize),
}
