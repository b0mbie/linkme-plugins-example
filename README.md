This Cargo workspace demonstrates an example implementation of a plugin system in Rust.
It includes:
- a type-safe *core* (`crates/core`) crate,
- a *macro-based runtime* (`crates/macro-runtime`) using [`linkme`](https://crates.io/crates/linkme) for plugins,
- an example *plugin runner* (`examples/runner`), and
- an *example plugin* (`examples/macro-runtime`) utilizing the macro-based runtime.

Run `cargo docs` to generate crate documentation (or `cargo docs --open` to also open it in the default browser),
and `cargo run-plugin` to invoke the example plugin runner with the path to a plugin binary.

To test out an example, you can run the following:
```sh
cargo build --release --package example-*

# Run `examples/macro-runtime`:
cargo run-plugin ./target/release/libexample_macro_runtime.so
```
In this case, the output could be something like:
```
Loading plugin
`on_load!`: invocation #2
`on_load!`: invocation #1

Notifying plugin that everything has loaded
`on_all_loaded!`: invoked

Unloading plugin
`on_unload!`: invoked
```
