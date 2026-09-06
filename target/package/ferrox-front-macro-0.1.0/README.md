# Ferrox Front Macro (`ferrox-front-macro`)

`ferrox-front-macro` provides the procedural macro `rsx!` for compiling declarative, HTML-like component markup
into type-safe WebAssembly DOM node builders at Rust compile time.

## Key Features
- 🚀 **`rsx!` Macro**: Write clean HTML structures directly inside Rust component functions.
- 🛡️ **Compile-Time Validation**: Catch unclosed tags, invalid attributes, and type mismatches during `cargo check`.
- ⚡ **Zero Runtime Parsing**: HTML structure is transformed into direct Wasm DOM creation calls during compilation.
