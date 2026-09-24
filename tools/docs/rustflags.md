# Rust compiler flags

Cargo rustflags come from alternative sources, not cumulative layers.

For rustc, Cargo selects them in this order:

1. `CARGO_ENCODED_RUSTFLAGS`
2. `RUSTFLAGS`
3. matching `[target.<triple>].rustflags` and `[target.'cfg(...)'].rustflags`
4. `[build].rustflags` / `CARGO_BUILD_RUSTFLAGS`

The first applicable source wins, except that matching target triple and target
cfg entries are joined.

Consequences for `x`:

- Avoid setting `RUSTFLAGS` for generic project configuration: it masks
  target-specific flags.
- Put target requirements in `.cargo/config.toml` under `[target.*]`.
- Use `CARGO_BUILD_RUSTFLAGS` for optional fallback/local preferences.
- Use `CARGO_ENCODED_RUSTFLAGS` from build scripts to inspect the effective
  flags selected by Cargo.
- Flags that must compose with target-specific flags should eventually be
  expressed as matching target cfg configuration, rather than `RUSTFLAGS`.

Example:

```toml
[target.avr-none]
rustflags = ["-C", "target-cpu=atmega328p"]
