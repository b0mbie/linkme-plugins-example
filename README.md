This Cargo workspace demonstrates an example implementation of a plugin system in Rust.
It includes:
- a type-safe *core* crate,
- a *macro-based runtime* for plugins,
- an example *plugin runner*, and
- an *example plugin* utilizing the macro-based runtime.

Run `cargo docs` to generate crate documentation (or `cargo docs --open` to also open it in the default browser),
and `cargo run-plugin` to invoke the example plugin runner with the path to a plugin binary.
