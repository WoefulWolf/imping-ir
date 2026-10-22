# imping-ir

The intermediate representation (IR) of the imping toolkit: one JSON file describing one build of a program (its identity, types, functions and globals), with serde types, validation and a JSON Schema. It depends on nothing else in imping.

```toml
[dependencies]
imping-ir = { git = "https://github.com/WoefulWolf/imping-ir" }
```

```rust
let text = std::fs::read_to_string("target.ir.json")?;
let ir = imping_ir::Ir::from_json_str(&text)?;
ir.validate()?;
for module in &ir.modules {
    println!("{}: {} functions", module.name, module.functions.len());
}
```

The JSON Schema comes from `imping_ir::json_schema()`, behind the `schema` feature.

[API docs](https://woefulwolf.github.io/imping-ir/) · Early development; the format may still change.

If imping helped your project, a mention or link back is appreciated. I'd also love to see what you make with it: share it in [Show and tell](https://github.com/WoefulWolf/imping/discussions/categories/show-and-tell), and it might get featured here!

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.

AI assistance was used for parts of this project.
