# devela

[![Crate](https://img.shields.io/crates/v/devela.svg)](https://crates.io/crates/devela)
[![MSRV](https://img.shields.io/crates/msrv/devela?label=MSRV)](https://crates.io/crates/devela)
[![docs.rs](https://img.shields.io/docsrs/devela?label=docs.rs)](https://docs.rs/devela/latest/devela/)
[![WIP docs](https://img.shields.io/badge/docs-WIP-blue)](https://andamira.github.io/)

---

> A development substrate of coherence.

This is a place where I am slowly building the foundations for all I want to make.

devela matures from what I learn along the way and is gradually finding its shape.

It is still becoming, and growing from small parts. You're welcome to look around.


## Crates

 | crate                 | role                              | source       | docs   | [maturity] | [coverage] |
 | --------------------- | --------------------------------- | ------------ | -------------------- | :-: | :-: |
 | [`devela`][c1]        | foundation                        | [source][s1] | [latest][d1] · [wip][w1] | 5 | 3 |
 | [`devela_macros`][c2] | procedural macros                 | [source][s2] | [latest][d2] · [wip][w2] | 4 | 5 |
 | [`devela_bridge`][c3] | foreign-language and ABI bridges  | [source][s3] | [latest][d3] · [wip][w3] | 2 | 1 |
 | [`devela_extend`][c4] | ecosystem extensions and adapters | [source][s4] | [latest][d4] · [wip][w4] | 1 | 1 |
 | [`devela_micros`][c5] | embedded hardware support         | [source][s5] | [latest][d5] · [wip][w5] | 3 | 2 |
 |  `devela_sentry`      | downstream validation and probes  | [source][s6] | —                        | 3 | 2 |

Before 1.0, APIs remain free to evolve.

[maturity]: docs/assessment.md
[coverage]: docs/assessment.md

## Examples

- [`devela` examples](crates/devela/examples/) explore the library,
  platforms, and user interfaces.
- [`devela_micros` examples](crates/micros/examples/) run directly on
  microcontrollers, boards, and microcomputers.
- [`devela_bridge` examples](crates/bridge/examples/) show interfaces
  with C and Odin.

[c1]: https://crates.io/crates/devela
[s1]: crates/devela
[d1]: https://docs.rs/devela
[w1]: https://andamira.github.io/devela/wip/devela/

[c2]: https://crates.io/crates/devela_macros
[s2]: crates/devela/macros
[d2]: https://docs.rs/devela_macros
[w2]: https://andamira.github.io/devela_macros/wip/devela_macros/

[c3]: https://crates.io/crates/devela_bridge
[s3]: crates/bridge
[d3]: https://docs.rs/devela_bridge
[w3]: https://andamira.github.io/devela_bridge/wip/devela_bridge/

[c4]: https://crates.io/crates/devela_extend
[s4]: crates/extend
[d4]: https://docs.rs/devela_extend
[w4]: https://andamira.github.io/devela_extend/wip/devela_extend/

[c5]: https://crates.io/crates/devela_micros
[s5]: crates/micros
[d5]: https://docs.rs/devela_micros
[w5]: https://andamira.github.io/devela_micros/wip/devela_micros/

[s6]: crates/sentry
