# devela

[![Crate](https://img.shields.io/crates/v/devela.svg)](https://crates.io/crates/devela)
[![API](https://docs.rs/devela/badge.svg)](https://docs.rs/devela/)
[![MSRV: 1.82.0](https://flat.badgen.net/badge/MSRV/1.82.0/purple)](https://releases.rs/docs/1.82.0/)
<br/>
[![Code](https://tokei.rs/b1/github/andamira/devela?category=code)](https://github.com/andamira/devela)
[![Documentation](https://tokei.rs/b1/github/andamira/devela?category=comments)](https://docs.rs/devela/)
[![Files](https://tokei.rs/b1/github/andamira/devela?category=files)](https://github.com/andamira/devela/tree/main/)
---

*A cohesive development layer.*

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
Features that enable an optional dependency are prefixed with `dep_`, replacing
dashes with underscores. For example, for the `x-y` crate you'll have to enable
the `dep_x_y` feature, and it will become available under `crate::_dep::x_y`.

There are currently 18 optional dependencies:
[`atomic`](https://crates.io/crates/atomic),
[`bytemuck`](https://crates.io/crates/bytemuck),
[`const-str`](https://crates.io/crates/const-str),
[`hashbrown`](https://crates.io/crates/hashbrown),
[`jiff`](https://crates.io/crates/jiff),
[`libm`](https://crates.io/crates/libm),
[`log`](https://crates.io/crates/log),
[`macroquad`](https://crates.io/crates/macroquad),
[`memchr`](https://crates.io/crates/memchr),
[`miniquad`](https://crates.io/crates/miniquad),
[`portable-atomic`](https://crates.io/crates/portable-atomic),
[`rand_core`](https://crates.io/crates/rand_core),
[`rayon`](https://crates.io/crates/rayon),
[`rodio`](https://crates.io/crates/rodio),
[`tinyaudio`](https://crates.io/crates/tinyaudio),
[`unicode-segmentation`](https://crates.io/crates/unicode-segmentation),
[`unicode-width`](https://crates.io/crates/unicode-width),
[`wide`](https://crates.io/crates/wide).


## Status
This project is currently in an experimental stage of development.

The api docs for the unpublished WIP version is in: https://andamira.github.io/libera/doc/devela/

## License
This project is dual licensed under either [MIT](LICENSE-MIT)
or [Apache-2.0](LICENSE-APACHE), and includes includes several
[derived works](DOCS/DERIVED.md).

## Contributing
Contributions are welcome, see [CONTRIBUTING](DOCS/CONTRIBUTING.md)
