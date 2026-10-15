//! Builds a small IR document in code and prints it as JSON.
//!
//! Run with: `cargo run --example describe`

use imping_ir::{
    Architecture, CallingConvention, Function, Ir, Module, Parameter, Primitive, Storage, TypeDef,
    TypeId, TypeKind,
};

fn main() -> imping_ir::Result<()> {
    let module = Module {
        name: "game.exe".to_string(),
        architecture: Architecture::X64,
        image_base: 0x1_4000_0000,
        timestamp: 0x64b8_f2a1,
        image_size: 0x00a2_b000,
        hash: None,
        types: vec![
            TypeDef {
                id: TypeId(0),
                name: "void".to_string(),
                namespace: vec![],
                kind: TypeKind::Primitive {
                    primitive: Primitive::Void,
                },
            },
            TypeDef {
                id: TypeId(1),
                name: "int".to_string(),
                namespace: vec![],
                kind: TypeKind::Primitive {
                    primitive: Primitive::I32,
                },
            },
        ],
        functions: vec![Function {
            name: "SetHealth".to_string(),
            namespace: vec!["Game".to_string(), "Player".to_string()],
            rva: 0x1_4230,
            calling_convention: CallingConvention::Win64,
            return_type: TypeId(0),
            parameters: vec![Parameter {
                name: Some("amount".to_string()),
                type_id: TypeId(1),
                storage: Storage::Register {
                    value: "ecx".to_string(),
                },
            }],
            variadic: false,
            custom_storage: false,
            tags: vec![],
            verification: vec![],
        }],
        globals: vec![],
        vtables: vec![],
    };

    let ir = Ir {
        format_version: imping_ir::FormatVersion::CURRENT,
        modules: vec![module],
    };

    println!("{}", ir.to_json_pretty()?);
    Ok(())
}
