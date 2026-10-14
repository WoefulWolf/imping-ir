//! Intermediate representation types and JSON Schema for the imping modding
//! suite.
//!
//! One IR file ([`Ir`]) describes one build of a target program: a
//! [`FormatVersion`] and a list of [`Module`]s, each carrying its identity, its
//! type table, and its functions, globals and vtables. The types here
//! `serde`-serialize to and from that file as JSON, and — with the `schema`
//! feature — emit its [JSON Schema](json_schema).
//!
//! Addresses are relative to each module's image base and are written as
//! `0x`-prefixed hex strings; type references use numeric [`TypeId`]s into the
//! module's type table.
//!
//! # Reading and writing
//!
//! ```
//! use imping_ir::{Architecture, Ir};
//!
//! let json = r#"{
//!   "format_version": { "major": 1, "minor": 0 },
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
    /// Parses IR from JSON bytes, rejecting an unsupported major version.
    ///
    /// The version is checked before the body, so a file with an incompatible
    /// major version fails with [`Error::UnsupportedVersion`] rather than a
    /// confusing structural error.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if the bytes are not valid IR JSON, or
    /// [`Error::UnsupportedVersion`] if the major version is not
    /// [`SUPPORTED_MAJOR`].
    pub fn from_json_slice(bytes: &[u8]) -> Result<Self> {
        /// Reads just the version, tolerating the rest of the document.
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

    /// Parses IR from a JSON string. See [`from_json_slice`](Self::from_json_slice).
    ///
    /// # Errors
    ///
    /// As for [`from_json_slice`](Self::from_json_slice).
    pub fn from_json_str(text: &str) -> Result<Self> {
        Self::from_json_slice(text.as_bytes())
    }

    /// Serializes the IR to compact JSON.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if serialization fails.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    /// Serializes the IR to pretty-printed JSON.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if serialization fails.
    pub fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

/// Returns the JSON Schema describing the IR file format.
///
/// Serialize the returned schema with `serde_json` to obtain the schema
/// document. Requires the `schema` feature.
#[cfg(feature = "schema")]
pub fn json_schema() -> schemars::Schema {
    schemars::schema_for!(Ir)
}
