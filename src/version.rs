//! The IR file's format version.

use serde::{Deserialize, Serialize};

/// The major format version this crate reads and writes.
///
/// Other majors are rejected; see [`FormatVersion`]. Independent of the
/// crate's own version.
pub const SUPPORTED_MAJOR: u16 = 1;

/// The minor format version this crate writes.
pub const CURRENT_MINOR: u16 = 1;

/// The format version of an IR file.
///
/// Breaking changes bump the major, backward-compatible additions the minor.
/// Readers accept any minor of [`SUPPORTED_MAJOR`] and ignore fields they
/// don't know.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FormatVersion {
    /// The major version. Incompatible changes increment it.
    pub major: u16,
    /// The minor version. Backward-compatible additions increment it.
    pub minor: u16,
}

impl FormatVersion {
    /// The version this crate writes: [`SUPPORTED_MAJOR`].[`CURRENT_MINOR`].
    pub const CURRENT: FormatVersion = FormatVersion {
        major: SUPPORTED_MAJOR,
        minor: CURRENT_MINOR,
    };

    /// Returns whether this crate can read a file of this version, i.e. whether
    /// its major is [`SUPPORTED_MAJOR`].
    pub fn is_supported(&self) -> bool {
        self.major == SUPPORTED_MAJOR
    }
}
