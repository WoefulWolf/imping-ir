//! Semantic validation of IR.

use imping_ir::{
    Architecture, CallingConvention, Function, Global, Ir, Module, Primitive, TypeDef, TypeId,
    TypeKind, ValidationError,
};

/// Builds a module named `m` with a zeroed identity.
fn module(types: Vec<TypeDef>, functions: Vec<Function>, globals: Vec<Global>) -> Module {
    Module {
        name: "m".to_string(),
        architecture: Architecture::X64,
        image_base: 0,
        timestamp: 0,
        image_size: 0,
        hash: None,
        types,
        functions,
        globals,
        vtables: vec![],
    }
}

fn ir(module: Module) -> Ir {
    Ir {
        format_version: imping_ir::FormatVersion::CURRENT,
        modules: vec![module],
    }
}

fn primitive(id: u32, name: &str, primitive: Primitive) -> TypeDef {
    TypeDef {
        id: TypeId(id),
        name: name.to_string(),
        namespace: vec![],
        kind: TypeKind::Primitive { primitive },
    }
}

fn function(name: &str, return_type: TypeId) -> Function {
    Function {
        name: name.to_string(),
        namespace: vec![],
        rva: 0x10,
        calling_convention: CallingConvention::Win64,
        return_type,
        parameters: vec![],
        variadic: false,
        custom_storage: false,
        tags: vec![],
        verification: vec![],
    }
}

#[test]
fn valid_sample_passes() {
    let ir = Ir::from_json_str(include_str!("fixtures/sample.json")).expect("parse");
    assert!(ir.validate().is_ok(), "{:?}", ir.validate());
}

#[test]
fn dangling_type_reference_is_reported() {
    let ir = ir(module(
        vec![primitive(0, "void", Primitive::Void)],
        vec![function("f", TypeId(99))],
        vec![],
    ));

    let errors = ir.validate().expect_err("should be invalid");
    assert!(errors.errors().iter().any(|error| matches!(
        error,
        ValidationError::UnknownType {
            type_id: TypeId(99),
            ..
        }
    )));
}

#[test]
fn duplicate_type_id_is_reported() {
    let ir = ir(module(
        vec![
            primitive(0, "void", Primitive::Void),
            primitive(0, "also void", Primitive::Void),
        ],
        vec![],
        vec![],
    ));

    let errors = ir.validate().expect_err("should be invalid");
    assert!(errors.errors().iter().any(|error| matches!(
        error,
        ValidationError::DuplicateTypeId {
            type_id: TypeId(0),
            ..
        }
    )));
}

#[test]
fn enum_over_non_integer_is_reported() {
    let types = vec![
        primitive(0, "float", Primitive::F32),
        TypeDef {
            id: TypeId(1),
            name: "Bad".to_string(),
            namespace: vec![],
            kind: TypeKind::Enum {
                underlying: TypeId(0),
                variants: vec![],
            },
        },
    ];
    let errors = ir(module(types, vec![], vec![]))
        .validate()
        .expect_err("invalid");
    assert!(errors.errors().iter().any(|error| matches!(
        error,
        ValidationError::EnumUnderlyingNotInteger {
            type_id: TypeId(1),
            ..
        }
    )));
}

#[test]
fn enum_over_typedef_to_integer_is_valid() {
    let types = vec![
        primitive(0, "u32", Primitive::U32),
        TypeDef {
            id: TypeId(1),
            name: "DWORD".to_string(),
            namespace: vec![],
            kind: TypeKind::Typedef { target: TypeId(0) },
        },
        TypeDef {
            id: TypeId(2),
            name: "Kind".to_string(),
            namespace: vec![],
            kind: TypeKind::Enum {
                underlying: TypeId(1),
                variants: vec![],
            },
        },
    ];
    assert!(ir(module(types, vec![], vec![])).validate().is_ok());
}

#[test]
fn all_problems_are_collected() {
    let ir = ir(module(
        vec![
            primitive(0, "void", Primitive::Void),
            primitive(0, "dup", Primitive::Void),
        ],
        vec![function("f", TypeId(42))],
        vec![Global {
            name: "g".to_string(),
            namespace: vec![],
            rva: 0x20,
            type_id: TypeId(7),
        }],
    ));

    let errors = ir.validate().expect_err("should be invalid");
    // Duplicate id, unknown return type, unknown global type.
    assert_eq!(errors.errors().len(), 3);
}
