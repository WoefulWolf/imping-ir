//! The error type returned when reading or writing IR.

use crate::FormatVersion;
use core::fmt;

/// A `Result` with this crate's [`Error`].
pub type Result<T> = core::result::Result<T, Error>;

/// An error reading or writing an IR file.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The IR couldn't be parsed from, or written to, JSON.
    Json(serde_json::Error),
    /// The file's major format version isn't [`SUPPORTED_MAJOR`](crate::SUPPORTED_MAJOR).
    ///
    /// A different major may be structurally incompatible; see [`FormatVersion`].
    UnsupportedVersion {
        /// The version found in the file.
        found: FormatVersion,
        /// The major version this crate supports.
        supported_major: u16,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Json(_) => f.write_str("the IR could not be read or written as JSON"),
            Error::UnsupportedVersion {
                found,
                supported_major,
            } => write!(
                f,
                "unsupported IR format version {}.{} (this reader supports major version {})",
                found.major, found.minor, supported_major
            ),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::Json(error) => Some(error),
            Error::UnsupportedVersion { .. } => None,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}
