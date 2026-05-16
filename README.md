This Cargo workspace demonstrates an example implementation of a module system for plugins in Rust.
It includes:
- a type-safe *core* (`crates/core`) crate;
- a *module-based runtime* (`crates/module-runtime`) for plugins,
  which uses [`linkme`](https://crates.io/crates/linkme) to allow initialization separate modules;
- an example *plugin runner* (`examples/runner`);
- an *example plugin* (`examples/module-runtime`) utilizing the module-based runtime.

Run `cargo docs` to generate crate documentation (or `cargo docs --open` to also open it in the default browser),
and `cargo run-plugin` to invoke the example plugin runner with the path to a plugin binary.

To test out an example, you can run the following:
```sh
cargo build --release --package example-*

# Run `examples/tick`:
cargo run-plugin ./target/release/libexample_tick.so
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
