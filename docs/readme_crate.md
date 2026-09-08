# devela

devela is a general-purpose Rust library built from small, composable parts.
Its modules cover data, mathematics, text, media, and system interfaces.

The library supports stable Rust and `no_std`, with features for allocation,
`std`, and platform support. Many APIs work with fixed or caller-provided storage.

The library is pre-1.0 and still taking shape. APIs may change between releases.

## Getting started

Add the library to your `Cargo.toml`:

```toml
[dependencies]
devela = "0.29"
```

No Cargo features are enabled by default. The baseline supports `no_std`
and does not require allocation.

Enable `alloc` or `std` as needed; `std` includes `alloc`.
Additional features select the functionality you want to use.
See [Features and configuration][features] for the available choices.

## Documentation

* [API documentation][api]: browse the library by module.
* [All items][items]: explore a flat view of the public API.
* [Design notes][design]: understand the principles behind the library.

[features]: https://docs.rs/devela/latest/devela/_doc/reference/features/index.html
[api]: https://docs.rs/devela/latest/devela/
[items]: https://docs.rs/devela/latest/devela/all/
[design]: https://docs.rs/devela/latest/devela/_doc/design/
