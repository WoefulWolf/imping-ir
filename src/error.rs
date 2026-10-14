//! The error type returned when reading or writing IR.

use crate::FormatVersion;
use core::fmt;

/// The result of an IR operation.
pub type Result<T> = core::result::Result<T, Error>;

/// Everything that can go wrong reading or writing an IR file.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The IR could not be parsed from, or written to, JSON.
    Json(serde_json::Error),
    /// The IR's major format version is not supported by this reader.
    ///
    /// Readers accept any minor version of the supported major, but reject a
    /// different major, since it may be structurally incompatible.
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
