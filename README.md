# devela

[![Repository](https://flat.badgen.net/badge/github/v0.22.0/blue?icon=git)](https://github.com/andamira/devela)
[![Documentation ahead](https://flat.badgen.net/badge/docs/ahead*/blue)](https://andamira.github.io/libera/doc/devela/)
[![MSRV ahead 1.83.0](https://flat.badgen.net/badge/MSRV/1.83.0/blue)](https://releases.rs/docs/1.83.0/)
[![ccode](https://tokei.rs/b1/github/andamira/devela?category=code&style=flat-square)](https://github.com/andamira/devela)
[![ccomment](https://tokei.rs/b1/github/andamira/devela?category=comments&style=flat-square)](https://docs.rs/devela/)
[![cfile](https://tokei.rs/b1/github/andamira/devela?category=files&style=flat-square)](https://github.com/andamira/devela/tree/main/)
<br/>
[![Crate](https://img.shields.io/crates/v/devela.svg)](https://crates.io/crates/devela)
[![Documentation](https://docs.rs/devela/badge.svg)](https://docs.rs/devela/)
[![MSRV 1.80.1](https://flat.badgen.net/badge/MSRV/1.80.1/purple)](https://releases.rs/docs/1.80.1/)

---

*A cohesive development layer*

> A lightweight, enduring foundation for multi-disciplinary development.


## Features

Features are grouped in several categories mostly independent from each other,
so that they can be enabled orthogonally, and they're not fully additive,
e.g. it's not possible to enable at the same time `std` and `no_std`.

- [Development]: (`__*`).
- [Environment]: (`alloc`, `std`, `no_std`).
- [Module] + submodules.
- [Safety]: (`safe*`, `unsafe*`).
- [Nightly]: (`nightly_*`).
- [Capability]: (`_*`).
- [Dependency]: (`dep_*`).

<!-- -->
[Development]: https://andamira.github.io/libera/doc/devela/_doc/features/index.html#development-features
[Environment]: https://andamira.github.io/libera/doc/devela/_doc/features/index.html#environment-features
[Module]: https://andamira.github.io/libera/doc/devela/_doc/features/index.html#module-features
[Safety]: https://andamira.github.io/libera/doc/devela/_doc/features/index.html#safety-features
[Nightly]: https://andamira.github.io/libera/doc/devela/_doc/features/index.html#nightly-features
[Capability]: https://andamira.github.io/libera/doc/devela/_doc/features/index.html#capability-features
[Dependency]: https://andamira.github.io/libera/doc/devela/_doc/features/index.html#dependency-features
<!-- -->
[Environment]: https://docs.rs/devela/latest/devela/_doc/features/index.html#environment-features
[Module]: https://docs.rs/devela/latest/devela/_doc/features/index.html#module-features
[Safety]: https://docs.rs/devela/latest/devela/_doc/features/index.html#safety-features
[Nightly]: https://docs.rs/devela/latest/devela/_doc/features/index.html#nightly-features
[Capability]: https://docs.rs/devela/latest/devela/_doc/features/index.html#capability-features
[Dependency]: https://docs.rs/devela/latest/devela/_doc/features/index.html#dependency-features

# Dependencies

It includes a curated palette of lightweight optional crates.


## Status
This project is currently in an experimental stage of development.

## License
This project is dual licensed under either [MIT](LICENSE-MIT)
or [Apache-2.0](LICENSE-APACHE), and includes includes several
[vendored works](DOCS/VENDORED.md).

## Contributing
Contributions are welcome, see [CONTRIBUTING](DOCS/CONTRIBUTING.md)
