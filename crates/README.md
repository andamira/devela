# Crates

This directory contains the Rust crates maintained as part of the devela repository.

```text
crates/
├── devela/          foundational devela library
│   └── macros/      procedural macros required by devela
├── bridge/          foreign-language bindings and ABI bridges
├── extend/          ecosystem extensions and adapters
├── micros/          microcontrollers, boards, and embedded hardware
├── sentry/          downstream validation of public behavior
```

## Crate roles

### `devela`

The foundational library.

It owns the common vocabulary, representations, traits, algorithms,
and portable mechanisms that the rest of the crate family can build upon.


### `devela/macros`

Package: `devela_macros`.

Procedural macros supporting `devela`.

It is kept alongside `devela` because it is primarily an implementation
companion made separate by Rust's procedural-macro crate boundary.


### `bridge`

Package: `devela_bridge`.

Foreign-language bindings and ABI bridges for devela.

This includes explicit ABI surfaces and the machinery needed to
expose selected devela functionality to other programming languages.


### `extend`

Package: `devela_extend`.

Ecosystem extensions and adapters for devela.

This crate connects devela abstractions with independently developed Rust libraries
and frameworks without making those integrations part of the foundational crate.

### `micros`

Package: `devela_micros`.

Microcontroller, board, and embedded hardware support for devela.

It contains concrete hardware realizations built upon the
portable abstractions and low-level mechanisms provided by devela.

### `sentry`

Package: `devela_sentry`.

Downstream validation of devela's public API and behavior.

It exercises devela as an external consumer in order to catch regressions and
validate assumptions that cannot be tested faithfully from inside the main crate.
