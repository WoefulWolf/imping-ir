# imping-ir

Intermediate representation types and JSON Schema for the imping modding suite. One IR file describes one build of a target program; these types read and write that file, and can emit its JSON Schema. The crate depends on nothing else in the suite.

Status: early development, the schema is being designed. Not published.

## Development

Run all checks (formatting, clippy, tests, docs) from the repo root:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/check.ps1
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
