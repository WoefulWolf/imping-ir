//! The type table: type definitions, their stable ids, and their kinds.

use serde::{Deserialize, Serialize};

/// A stable identifier for a type within a module, referenced by functions,
/// fields, globals and other types.
///
/// Ids are assigned by the exporter and are stable across re-exports of the
/// same build, so references and diffs stay meaningful.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(
    feature = "schema",
    derive(schemars::JsonSchema),
    schemars(transparent)
)]
pub struct TypeId(pub u32);

/// A named type definition in a module's type table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TypeDef {
    /// The type's stable id.
    pub id: TypeId,
    /// The type's source name (e.g. `"Vector3"`). Not required to be unique.
    pub name: String,
    /// What kind of type this is, and its kind-specific data.
    #[serde(flatten)]
    pub kind: TypeKind,
}

/// A type's kind and the data specific to it.
///
/// Serialized with a `"kind"` discriminator alongside the enclosing
/// [`TypeDef`]'s `id` and `name`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TypeKind {
    /// A built-in primitive type.
    Primitive {
        /// Which primitive this is.
        primitive: Primitive,
    },
    /// A pointer to another type.
    Pointer {
        /// The pointed-to type.
        pointee: TypeId,
        /// Whether the pointee is `const`.
        #[serde(default, rename = "const")]
        is_const: bool,
    },
    /// A fixed-length array.
    Array {
        /// The element type.
        element: TypeId,
        /// The number of elements.
        count: u64,
    },
    /// A C `struct`.
    Struct {
        /// The total size in bytes.
        size: u64,
        /// The alignment in bytes.
        align: u64,
        /// The fields, in declaration order.
        #[serde(default)]
        fields: Vec<Field>,
    },
    /// A C `union`.
    Union {
        /// The total size in bytes.
        size: u64,
        /// The alignment in bytes.
        align: u64,
        /// The members.
        #[serde(default)]
        fields: Vec<Field>,
    },
    /// A C enumeration: a set of named integer constants over an underlying
    /// integer type.
    Enum {
        /// The underlying integer type.
        underlying: TypeId,
        /// The named constants.
        #[serde(default)]
        variants: Vec<EnumVariant>,
    },
    /// A function (pointer) type.
    Function {
        /// The calling convention.
        calling_convention: crate::CallingConvention,
        /// The parameter types, in order.
        #[serde(default)]
        parameters: Vec<TypeId>,
        /// The return type (use the `void` primitive for no return value).
        return_type: TypeId,
        /// Whether the function is variadic.
        #[serde(default)]
        variadic: bool,
    },
    /// An alias for another type (a `typedef`).
    Typedef {
        /// The aliased type.
        target: TypeId,
    },
    /// A type whose layout is unknown, with an optional known size.
    Opaque {
        /// The size in bytes, if known.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        size: Option<u64>,
    },
}

/// A member of a [`TypeKind::Struct`] or [`TypeKind::Union`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Field {
    /// The field's name.
    pub name: String,
    /// The field's type.
    #[serde(rename = "type")]
    pub type_id: TypeId,
    /// The field's byte offset from the start of the containing type.
    pub offset: u64,
    /// Bitfield placement, if this field is a bitfield.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bits: Option<BitField>,
}

/// The bit placement of a bitfield [`Field`] within its backing storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct BitField {
    /// The least-significant bit occupied, counted from the field's `offset`.
    pub bit_offset: u32,
    /// The number of bits occupied.
    pub bit_width: u32,
}

/// A named constant of a [`TypeKind::Enum`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct EnumVariant {
    /// The constant's name.
    pub name: String,
    /// The constant's value (signed to allow negative enumerators).
    pub value: i64,
}

/// A built-in primitive type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Primitive {
    /// No value; used for `void` returns and `void*` pointees.
    Void,
    /// A boolean.
    Bool,
    /// An 8-bit unsigned integer.
    U8,
    /// A 16-bit unsigned integer.
    U16,
    /// A 32-bit unsigned integer.
    U32,
    /// A 64-bit unsigned integer.
    U64,
    /// An 8-bit signed integer.
    I8,
    /// A 16-bit signed integer.
    I16,
    /// A 32-bit signed integer.
    I32,
    /// A 64-bit signed integer.
    I64,
    /// A 32-bit IEEE 754 float.
    F32,
    /// A 64-bit IEEE 754 float.
    F64,
}
