This is derived work from the
[`grapheme_machine`](https://crates.io/crates/grapheme_machine/0.2.0) crate,
including the following modifications:

- made all methods *const*.
- make all unsafe optional.
- remove all dependencies.
- replace `u8char` with own type `char_utf8`.
- add higher-level type `GraphemeScanner`.
- rename types:
  - `ClusterAction` to `GraphemeBoundary`.
  - `CharProperties` to `GraphemeProps`.
  - `GCBProperty` to `GraphemePropCb`.
  - `InCBProperty` to `GraphemePropInCb`.
- rename machine state variants:
  - `Base` to Start
  - `AwaitEmojiFlag` to `AwaitRegionalPair`.
  - `GB11BeforeZwj` to `BeforeZwj`.
  - `GB11AfterZwj` to `AfterZwj`.
  - `GB9cConsonant` to `IndicConsonant`.
  - `GB9cLinker` to `IndicLinker`.
- rename acronyms in identifiers to CamelCase.
- add const equality comparison for all items.
- derive additional utility traits.
- update docs & comments.
- misc. refactors.
