# html-auy-macros

Procedural macro crate that implements `html!` for [`html-auy`](https://crates.io/crates/html-auy).

Use `html-auy` instead of depending on this crate directly:

```toml
[dependencies]
html-auy = "0.1"
```

The macro parser, validator, and code generator live in
[`html-auy-macros-core`](https://crates.io/crates/html-auy-macros-core) so they can be tested and
fuzzed without a compiler plugin.

## License

MIT. See `LICENSE`.
