# devela

[![Crate](https://img.shields.io/crates/v/devela.svg)](https://crates.io/crates/devela)
[![API](https://docs.rs/devela/badge.svg)](https://docs.rs/devela/)
[![MSRV: 1.82.0](https://flat.badgen.net/badge/MSRV/1.82.0/purple)](https://releases.rs/docs/1.82.0/)
<br/>
[![Code](https://tokei.rs/b1/github/andamira/devela?category=code)](https://github.com/andamira/devela)
[![Documentation](https://tokei.rs/b1/github/andamira/devela?category=comments)](https://docs.rs/devela/)
[![Files](https://tokei.rs/b1/github/andamira/devela?category=files)](https://github.com/andamira/devela/tree/main/)
---

A meta-cohesive development layer.

Devela is a project shaped by the search for clarity and flow,
aiming to create an enduring foundation.

It is an ongoing experiment to explore modularity and interdisciplinarity,
capturing the essential building blocks from complementary domains
and blending them into practical, integral abstractions.

The core components work seamlessly together without external dependencies.
For broader needs, it includes a curated palette of optional crates with
lightweight designs and shallow dependency trees.

## Design and structure

### Modules

The root modules:
[`code`], [`data`], [`error`], [`mem`], [`num`], [`rend`], [`sys`], [`text`] and [`work`]
are organized to cover distinct aspects of development, with a focus on
dividing responsibilities in a clear and comprehensive way.

Additional modules serve supportive roles,
such as [`_dep`] for optional dependencies,
`_core`, [`_doc`] and [`all`].

For detailed information about each module, see the
[Modules Section](https://docs.rs/devela/latest/devela/#modules).

[`code`]: https://docs.rs/devela/latest/devela/code/index.html
[`data`]: https://docs.rs/devela/latest/devela/data/index.html
[`error`]: https://docs.rs/devela/latest/devela/error/index.html
[`mem`]: https://docs.rs/devela/latest/devela/mem/index.html
[`num`]: https://docs.rs/devela/latest/devela/num/index.html
[`rend`]: https://docs.rs/devela/latest/devela/rend/index.html
[`sys`]: https://docs.rs/devela/latest/devela/sys/index.html
[`text`]: https://docs.rs/devela/latest/devela/text/index.html
[`work`]: https://docs.rs/devela/latest/devela/work/index.html

[`_dep`]: https://docs.rs/devela/latest/devela/_dep/index.html
[`_doc`]: https://docs.rs/devela/latest/devela/_doc/index.html
[`all`]: https://docs.rs/devela/latest/devela/all/index.html

### Dependencies
The features that enable a dependency are prefixed with `dep_` and dashes are
replaced with underscores. For example, to enable the `portable-atomic` crate
you have to enable the `dep_portable_atomic` feature and it will be available
in the `crate::_dep::portable_atomic` module.

## Status
This project is currently in an experimental stage of development.

The api docs for the unpublished WIP version is in: https://andamira.github.io/libera/doc/devela/

## License
This project is dual licensed under either [MIT](LICENSE-MIT)
or [Apache-2.0](LICENSE-APACHE), and includes includes several
[derived works](DOCS/DERIVED.md).

## Contributing
Contributions are welcome, see [CONTRIBUTING](DOCS/CONTRIBUTING.md)
