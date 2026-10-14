//! Functions, their calling conventions, parameters and parameter storage.

use crate::TypeId;
use serde::{Deserialize, Serialize};

/// A function in a module.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Function {
    /// The function's name (without namespace). Overloads may share a name.
    pub name: String,
    /// The namespace path leading to the function, outermost first (e.g.
    /// `["Game", "Entity"]`). Empty for a free function.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespace: Vec<String>,
    /// The function's address, relative to the module's image base.
    #[serde(with = "crate::hex::int")]
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    pub rva: u32,
    /// The calling convention.
    pub calling_convention: CallingConvention,
    /// The return type (the `void` primitive for no return value).
    pub return_type: TypeId,
    /// The parameters, in order.
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    /// Whether the function is variadic.
    #[serde(default)]
    pub variadic: bool,
    /// Whether the function uses non-standard argument storage described by the
    /// `parameters`' [`Storage`], rather than its calling convention's defaults.
    #[serde(default)]
    pub custom_storage: bool,
    /// Free-form tags attached to the function (e.g. in Ghidra).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// The bytes expected at the function's prologue, used to verify the build
    /// before hooking. Empty if no verification bytes were recorded.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        with = "crate::hex::bytes"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    pub verification: Vec<u8>,
}

/// A function's calling convention.
///
/// On x64 there is a single convention, [`Win64`](Self::Win64); the others
/// apply to x86.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum CallingConvention {
    /// x86 `__cdecl`.
    Cdecl,
    /// x86 `__stdcall`.
    Stdcall,
    /// x86 `__fastcall`.
    Fastcall,
    /// x86 `__thiscall`.
    Thiscall,
    /// The single x64 convention.
    Win64,
}

/// A function parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Parameter {
    /// The parameter's name, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The parameter's type.
    #[serde(rename = "type")]
    pub type_id: TypeId,
    /// Where the parameter is passed.
    pub storage: Storage,
}

/// Where a [`Parameter`] is passed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Storage {
    /// Passed in a register, named by the architecture's convention (e.g.
    /// `"rcx"`, `"xmm0"`).
    Register {
        /// The register name, lowercase.
        value: String,
    },
    /// Passed on the stack, at the given byte offset.
    Stack {
        /// The byte offset of the parameter.
        offset: i64,
    },
}
