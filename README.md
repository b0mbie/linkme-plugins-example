This Cargo workspace demonstrates an example implementation of a plugin system in Rust.
It includes:
- a type-safe *core* (`crates/core`) crate,
- a *macro-based runtime* (`crates/macro-runtime`) using [`linkme`](https://crates.io/crates/linkme) for plugins,
- an example *plugin runner* (`examples/runner`), and
- an *example plugin* (`examples/macro-runtime`) utilizing the macro-based runtime.

Run `cargo docs` to generate crate documentation (or `cargo docs --open` to also open it in the default browser),
and `cargo run-plugin` to invoke the example plugin runner with the path to a plugin binary.
