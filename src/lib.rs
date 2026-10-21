//! Intermediate representation (IR) types and JSON Schema for the imping
//! modding suite.
//!
//! One IR file describes one build of a target. Addresses are relative to each
//! module's image base and written as `0x` hex strings; types are referenced
//! by [`TypeId`] into the module's type table.
//!
//! - [`Ir`]: the whole file, read with [`Ir::from_json_str`], written with
//!   [`Ir::to_json_pretty`] and checked with [`Ir::validate`].
//! - [`FormatVersion`]: readers reject other major versions.
//! - [`Module`]: one image's identity, [`TypeDef`]s, [`Function`]s,
//!   [`Global`]s and [`Vtable`]s.
//! - `json_schema()`: the file's JSON Schema, with the `schema` feature.
//!
//! # Examples
//!
//! ```
//! use imping_ir::{Architecture, Ir};
//!
//! let json = r#"{
//!   "format_version": { "major": 1, "minor": 1 },
//!   "modules": [ { "name": "game.exe", "architecture": "x64",
//!     "image_base": "0x140000000", "timestamp": "0x64b8f2a1",
//!     "image_size": "0xa2b000" } ]
//! }"#;
//!
//! let ir = Ir::from_json_str(json)?;
//! assert_eq!(ir.modules[0].architecture, Architecture::X64);
//! assert_eq!(ir.modules[0].image_base, 0x1_4000_0000);
//!
//! let written = ir.to_json_pretty()?;
//! assert_eq!(Ir::from_json_str(&written)?, ir);
//! # Ok::<(), imping_ir::Error>(())
//! ```

#![warn(missing_docs)]

mod error;
mod function;
mod hex;
mod items;
mod module;
mod types;
mod validate;
mod version;

pub use crate::error::{Error, Result};
pub use crate::function::{CallingConvention, Function, Parameter, Storage};
pub use crate::items::{Global, Vtable, VtableEntry};
pub use crate::module::{Architecture, HashAlgorithm, Module, ModuleHash};
pub use crate::types::{BitField, EnumVariant, Field, Primitive, TypeDef, TypeId, TypeKind};
pub use crate::validate::{ValidationError, ValidationErrors};
pub use crate::version::{CURRENT_MINOR, FormatVersion, SUPPORTED_MAJOR};

use serde::{Deserialize, Serialize};

/// A complete IR file: a format version and the modules of one target build.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Ir {
    /// The IR format version.
    pub format_version: FormatVersion,
    /// The modules of the build.
    #[serde(default)]
    pub modules: Vec<Module>,
}

impl Ir {
    /// Parses IR from JSON bytes.
    ///
    /// The version is read first, so a file of another major version fails
    /// with [`Error::UnsupportedVersion`] instead of a structural error.
    ///
    /// # Errors
    ///
    /// - [`Error::UnsupportedVersion`] if the major version isn't [`SUPPORTED_MAJOR`].
    /// - [`Error::Json`] if the bytes aren't valid IR JSON.
    pub fn from_json_slice(bytes: &[u8]) -> Result<Self> {
        /// Just the version; the rest of the document is ignored.
        #[derive(Deserialize)]
        struct Header {
            format_version: FormatVersion,
        }

        let header: Header = serde_json::from_slice(bytes)?;
        if !header.format_version.is_supported() {
            return Err(Error::UnsupportedVersion {
                found: header.format_version,
                supported_major: SUPPORTED_MAJOR,
            });
        }
        Ok(serde_json::from_slice(bytes)?)
    }

    /// Parses IR from a JSON string, like [`from_json_slice`](Self::from_json_slice).
    ///
    /// # Errors
    ///
    /// Same as [`from_json_slice`](Self::from_json_slice).
    pub fn from_json_str(text: &str) -> Result<Self> {
        Self::from_json_slice(text.as_bytes())
    }

    /// Serializes the IR to compact JSON.
    ///
    /// # Errors
    ///
    /// [`Error::Json`] if serialization fails.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    /// Serializes the IR to pretty-printed JSON.
    ///
    /// # Errors
    ///
    /// [`Error::Json`] if serialization fails.
    pub fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

/// Returns the JSON Schema of the IR file format.
///
/// Requires the `schema` feature. Serialize the result with `serde_json` to get
/// the schema document.
///
/// # Examples
///
/// ```
/// let schema = imping_ir::json_schema();
/// let text = serde_json::to_string_pretty(&schema)?;
/// assert!(text.contains("format_version"));
/// # Ok::<(), serde_json::Error>(())
/// ```
#[cfg(feature = "schema")]
pub fn json_schema() -> schemars::Schema {
    schemars::schema_for!(Ir)
}
