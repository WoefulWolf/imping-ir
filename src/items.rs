//! Globals and vtables.

use crate::TypeId;
use serde::{Deserialize, Serialize};

/// A global variable in a module.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Global {
    /// The global's name.
    pub name: String,
    /// The namespace path, outermost first. Empty for a free global.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespace: Vec<String>,
    /// The global's address, relative to the module's image base.
    #[serde(with = "crate::hex::int")]
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    pub rva: u32,
    /// The global's type.
    #[serde(rename = "type")]
    pub type_id: TypeId,
}

/// A virtual function table in a module.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Vtable {
    /// The vtable's name (typically its class name).
    pub name: String,
    /// The namespace path, outermost first. Empty for a free vtable.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespace: Vec<String>,
    /// The vtable's address, relative to the module's image base.
    #[serde(with = "crate::hex::int")]
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    pub rva: u32,
    /// The entries, in slot order.
    #[serde(default)]
    pub entries: Vec<VtableEntry>,
}

/// One slot of a [`Vtable`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VtableEntry {
    /// The slot's index in the vtable.
    pub index: u32,
    /// The slot's method name, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The target function's address, relative to the module's image base.
    #[serde(with = "crate::hex::int")]
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    pub rva: u32,
    /// The slot's function (pointer) type, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<TypeId>,
}
