//! Reading, writing, round-tripping and version handling of IR files.

use imping_ir::{
    Architecture, CallingConvention, Error, FormatVersion, Ir, Primitive, Storage, TypeId, TypeKind,
};

const SAMPLE: &str = include_str!("fixtures/sample.json");

#[test]
fn parses_sample_and_decodes_hex_addresses() {
    let ir = Ir::from_json_str(SAMPLE).expect("parse sample");

    assert_eq!(ir.format_version, FormatVersion::CURRENT);
    assert_eq!(ir.modules.len(), 1);

    let module = &ir.modules[0];
    assert_eq!(module.name, "game.exe");
    assert_eq!(module.architecture, Architecture::X64);
    assert_eq!(module.image_base, 0x1_4000_0000);
    assert_eq!(module.timestamp, 0x64b8_f2a1);
    assert_eq!(module.image_size, 0x00a2_b000);

    // A struct with its fields and offsets.
    let vector3 = &module.types[3];
    assert_eq!(vector3.name, "Vector3");
    let TypeKind::Struct { size, fields, .. } = &vector3.kind else {
        panic!("expected a struct");
    };
    assert_eq!(*size, 12);
    assert_eq!(fields.len(), 3);
    assert_eq!(fields[2].offset, 8);

    // A bitfield field carries its bit placement.
    let flags = &module.types[5];
    let TypeKind::Struct { fields, .. } = &flags.kind else {
        panic!("expected a struct");
    };
    let team = fields[1].bits.expect("team is a bitfield");
    assert_eq!((team.bit_offset, team.bit_width), (1, 3));

    // An enum over its underlying type.
    let TypeKind::Enum {
        underlying,
        variants,
    } = &module.types[6].kind
    else {
        panic!("expected an enum");
    };
    assert_eq!(*underlying, TypeId(2));
    assert_eq!(variants[1].value, 1);

    // Functions: hex RVA, verification bytes, storage, variadic.
    let normalize = &module.functions[0];
    assert_eq!(normalize.rva, 0x1_4230);
    assert_eq!(normalize.calling_convention, CallingConvention::Win64);
    assert_eq!(normalize.verification, vec![0x48, 0x89, 0x5c, 0x24, 0x08]);
    assert_eq!(normalize.tags, ["hot"]);
    match &normalize.parameters[0].storage {
        Storage::Register { value } => assert_eq!(value, "rcx"),
        other => panic!("expected a register, got {other:?}"),
    }

    let log = &module.functions[1];
    assert!(log.variadic);
    assert_eq!(log.return_type, TypeId(0));
    assert!(matches!(
        log.parameters[0].storage,
        Storage::Stack { offset: 0 }
    ));

    assert_eq!(module.globals[0].rva, 0x3a_1200);
    assert_eq!(module.vtables[0].entries.len(), 2);
    assert_eq!(module.vtables[0].entries[0].rva, 0x1_5000);
}

#[test]
fn round_trips_through_json() {
    let ir = Ir::from_json_str(SAMPLE).expect("parse sample");
    let written = ir.to_json_pretty().expect("serialize");
    let reparsed = Ir::from_json_str(&written).expect("reparse");
    assert_eq!(ir, reparsed);

    // Addresses are written back as hex strings, not numbers.
    assert!(written.contains("\"rva\": \"0x14230\""));
    assert!(written.contains("\"image_base\": \"0x140000000\""));
}

#[test]
fn rejects_unknown_major_version() {
    let json = r#"{ "format_version": { "major": 99, "minor": 0 }, "modules": [] }"#;
    let error = Ir::from_json_str(json).expect_err("should reject major 99");
    assert!(matches!(
        error,
        Error::UnsupportedVersion {
            found: FormatVersion { major: 99, .. },
            supported_major: 1,
        }
    ));
}

#[test]
fn accepts_newer_minor_and_ignores_unknown_fields() {
    // A future minor adds a field an old reader does not know; it must still
    // read the parts it understands.
    let json = r#"{
        "format_version": { "major": 1, "minor": 7 },
        "modules": [],
        "future_top_level_field": 123
    }"#;
    let ir = Ir::from_json_str(json).expect("accept newer minor");
    assert_eq!(ir.format_version.minor, 7);
    assert!(ir.modules.is_empty());
}

#[test]
fn odd_length_verification_is_rejected() {
    let json = r#"{
        "format_version": { "major": 1, "minor": 0 },
        "modules": [ { "name": "m", "architecture": "x86",
            "image_base": "0x400000", "timestamp": "0x0", "image_size": "0x1000",
            "functions": [ { "name": "f", "rva": "0x10", "calling_convention": "cdecl",
                "return_type": 0, "verification": "abc" } ] } ]
    }"#;
    assert!(matches!(Ir::from_json_str(json), Err(Error::Json(_))));
}

#[test]
fn void_is_a_primitive_type() {
    let ir = Ir::from_json_str(SAMPLE).expect("parse sample");
    let TypeKind::Primitive { primitive } = &ir.modules[0].types[0].kind else {
        panic!("expected a primitive");
    };
    assert_eq!(*primitive, Primitive::Void);
}

#[cfg(feature = "schema")]
#[test]
fn json_schema_generates() {
    let schema = imping_ir::json_schema();
    let value = serde_json::to_value(&schema).expect("schema to JSON value");
    assert!(value.is_object(), "schema should be a JSON object");
    // The schema must at least describe the top-level `modules` property.
    let text = serde_json::to_string(&value).unwrap();
    assert!(text.contains("modules"));
}
