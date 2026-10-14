//! Modules and their identity.

use crate::{Function, Global, TypeDef, Vtable};
use serde::{Deserialize, Serialize};

/// One module (image) of a target build: an executable or a shared library.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Module {
    /// The module's file name (e.g. `"game.exe"`, `"d3d11.dll"`).
    pub name: String,
    /// The module's architecture.
    pub architecture: Architecture,
    /// The module's preferred load address, from its image header.
    #[serde(with = "crate::hex::int")]
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    pub image_base: u64,
    /// The module's build timestamp, from its image header.
    #[serde(with = "crate::hex::int")]
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    pub timestamp: u32,
    /// The module's loaded image size in bytes, from its image header.
    #[serde(with = "crate::hex::int")]
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    pub image_size: u32,
    /// An optional content hash of the module file, for stronger identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<ModuleHash>,
    /// The module's type table.
    #[serde(default)]
    pub types: Vec<TypeDef>,
    /// The module's functions.
    #[serde(default)]
    pub functions: Vec<Function>,
    /// The module's global variables.
    #[serde(default)]
    pub globals: Vec<Global>,
    /// The module's vtables.
    #[serde(default)]
    pub vtables: Vec<Vtable>,
}

/// A module's instruction-set architecture.
///
/// This also fixes the ABI: x64 uses the single Win64 convention, whilst x86
/// selects a per-function [`CallingConvention`](crate::CallingConvention).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    /// 32-bit x86.
    X86,
    /// 64-bit x86-64.
    X64,
}

/// A content hash of a module file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ModuleHash {
    /// The hash algorithm.
    pub algorithm: HashAlgorithm,
    /// The hash digest as a lowercase hex string.
    pub value: String,
}

/// A hash algorithm used by [`ModuleHash`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum HashAlgorithm {
    /// SHA-256.
    Sha256,
}
