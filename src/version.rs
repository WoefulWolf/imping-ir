//! The IR file's format version.

use serde::{Deserialize, Serialize};

/// The major format version this crate reads and writes.
///
/// A reader rejects any other major version (see
/// [`Ir::from_json_slice`](crate::Ir::from_json_slice)). This is independent of
/// the crate's own package version.
pub const SUPPORTED_MAJOR: u16 = 1;

/// The minor format version this crate writes.
pub const CURRENT_MINOR: u16 = 1;

/// The format version of an IR file: a major and a minor number.
///
/// A change that old readers cannot understand bumps the major; a
/// backward-compatible addition bumps the minor. Readers accept any minor of
/// [`SUPPORTED_MAJOR`] and ignore fields they do not recognise.
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

    /// Returns whether a reader of this crate can read a file of this version,
    /// i.e. whether its major matches [`SUPPORTED_MAJOR`].
    pub fn is_supported(&self) -> bool {
        self.major == SUPPORTED_MAJOR
    }
}
