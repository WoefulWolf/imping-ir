//! Semantic validation of an [`Ir`](crate::Ir): referential integrity and
//! basic invariants of the type table.
//!
//! This only *reports* problems; it does not decide which are fatal. Consumers
//! (such as codegen) call [`Ir::validate`](crate::Ir::validate) and apply their
//! own policy — for example downgrading some issues to warnings.

use crate::{Ir, Module, Primitive, TypeDef, TypeId, TypeKind};
use core::fmt;
use std::collections::HashMap;

/// A single problem found by [`Ir::validate`](crate::Ir::validate).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValidationError {
    /// A referenced type id has no definition in its module.
    UnknownType {
        /// The module the reference is in.
        module: String,
        /// The undefined type id.
        type_id: TypeId,
        /// A human-readable description of where the reference occurs.
        location: String,
    },
    /// Two type definitions in a module share the same id.
    DuplicateTypeId {
        /// The module the duplicate is in.
        module: String,
        /// The duplicated type id.
        type_id: TypeId,
    },
    /// An enum's `underlying` type does not resolve to an integer primitive.
    EnumUnderlyingNotInteger {
        /// The module the enum is in.
        module: String,
        /// The id of the enum type whose underlying type is not an integer.
        type_id: TypeId,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::UnknownType {
                module,
                type_id,
                location,
            } => write!(
                f,
                "module \"{module}\": {location} references undefined type id {}",
                type_id.0
            ),
            ValidationError::DuplicateTypeId { module, type_id } => {
                write!(f, "module \"{module}\": duplicate type id {}", type_id.0)
            }
            ValidationError::EnumUnderlyingNotInteger { module, type_id } => write!(
                f,
                "module \"{module}\": enum type id {} has a non-integer underlying type",
                type_id.0
            ),
        }
    }
}

impl core::error::Error for ValidationError {}

/// The collection of problems found by [`Ir::validate`](crate::Ir::validate).
///
/// Non-empty whenever validation fails; its [`Display`](fmt::Display) lists
/// every problem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationErrors(Vec<ValidationError>);

impl ValidationErrors {
    /// Returns the individual errors.
    pub fn errors(&self) -> &[ValidationError] {
        &self.0
    }

    /// Consumes the collection, returning the individual errors.
    pub fn into_errors(self) -> Vec<ValidationError> {
        self.0
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} IR validation error(s)", self.0.len())?;
        for error in &self.0 {
            write!(f, "\n  - {error}")?;
        }
        Ok(())
    }
}

impl core::error::Error for ValidationErrors {}

impl Ir {
    /// Checks every module's cross-references and type-table invariants,
    /// returning all problems found.
    ///
    /// This is not run by the parser; call it explicitly. It reports issues but
    /// does not decide which are fatal — that is the caller's policy.
    ///
    /// # Errors
    ///
    /// Returns [`ValidationErrors`] listing every problem if any are found.
    pub fn validate(&self) -> core::result::Result<(), ValidationErrors> {
        let mut errors = Vec::new();
        for module in &self.modules {
            validate_module(module, &mut errors);
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors(errors))
        }
    }
}

fn validate_module(module: &Module, errors: &mut Vec<ValidationError>) {
    // Index the type table, reporting duplicate ids.
    let mut by_id: HashMap<u32, &TypeDef> = HashMap::with_capacity(module.types.len());
    for def in &module.types {
        if by_id.insert(def.id.0, def).is_some() {
            errors.push(ValidationError::DuplicateTypeId {
                module: module.name.clone(),
                type_id: def.id,
            });
        }
    }

    let check = |id: TypeId, location: String, errors: &mut Vec<ValidationError>| {
        if !by_id.contains_key(&id.0) {
            errors.push(ValidationError::UnknownType {
                module: module.name.clone(),
                type_id: id,
                location,
            });
        }
    };

    // References within the type table.
    for def in &module.types {
        match &def.kind {
            TypeKind::Primitive { .. } | TypeKind::Opaque { .. } => {}
            TypeKind::Pointer { pointee, .. } => {
                check(*pointee, format!("type `{}` pointee", def.name), errors);
            }
            TypeKind::Array { element, .. } => {
                check(*element, format!("type `{}` element", def.name), errors);
            }
            TypeKind::Struct { fields, .. } | TypeKind::Union { fields, .. } => {
                for field in fields {
                    check(
                        field.type_id,
                        format!("type `{}` field `{}`", def.name, field.name),
                        errors,
                    );
                }
            }
            TypeKind::Enum { underlying, .. } => {
                check(
                    *underlying,
                    format!("type `{}` underlying", def.name),
                    errors,
                );
                // Only check integer-ness if the underlying type exists.
                if by_id.contains_key(&underlying.0)
                    && !matches!(resolve_primitive(&by_id, *underlying), Some(p) if is_integer(p))
                {
                    errors.push(ValidationError::EnumUnderlyingNotInteger {
                        module: module.name.clone(),
                        type_id: def.id,
                    });
                }
            }
            TypeKind::Function {
                parameters,
                return_type,
                ..
            } => {
                for (index, param) in parameters.iter().enumerate() {
                    check(
                        *param,
                        format!("type `{}` parameter {index}", def.name),
                        errors,
                    );
                }
                check(
                    *return_type,
                    format!("type `{}` return type", def.name),
                    errors,
                );
            }
            TypeKind::Typedef { target } => {
                check(*target, format!("type `{}` target", def.name), errors);
            }
        }
    }

    // References from functions.
    for function in &module.functions {
        let name = qualify(&function.namespace, &function.name);
        check(
            function.return_type,
            format!("function `{name}` return type"),
            errors,
        );
        for (index, param) in function.parameters.iter().enumerate() {
            check(
                param.type_id,
                format!("function `{name}` parameter {index}"),
                errors,
            );
        }
    }

    // References from globals.
    for global in &module.globals {
        let name = qualify(&global.namespace, &global.name);
        check(global.type_id, format!("global `{name}`"), errors);
    }

    // Vtable entry signatures.
    for vtable in &module.vtables {
        for entry in &vtable.entries {
            if let Some(signature) = entry.signature {
                check(
                    signature,
                    format!("vtable `{}` entry {}", vtable.name, entry.index),
                    errors,
                );
            }
        }
    }
}

/// Joins a namespace path and name for display, e.g. `Game::Entity::Update`.
fn qualify(namespace: &[String], name: &str) -> String {
    if namespace.is_empty() {
        name.to_string()
    } else {
        format!("{}::{name}", namespace.join("::"))
    }
}

/// Follows typedefs (guarding against cycles) to the primitive a type resolves
/// to, if any.
fn resolve_primitive(by_id: &HashMap<u32, &TypeDef>, start: TypeId) -> Option<Primitive> {
    let mut id = start;
    for _ in 0..64 {
        match &by_id.get(&id.0)?.kind {
            TypeKind::Primitive { primitive } => return Some(*primitive),
            TypeKind::Typedef { target } => id = *target,
            _ => return None,
        }
    }
    None
}

/// Returns whether a primitive is an integer type.
fn is_integer(primitive: Primitive) -> bool {
    matches!(
        primitive,
        Primitive::U8
            | Primitive::U16
            | Primitive::U32
            | Primitive::U64
            | Primitive::I8
            | Primitive::I16
            | Primitive::I32
            | Primitive::I64
    )
}
