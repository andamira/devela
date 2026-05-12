This is derived work from the
[`macro_rules_attribute`](https://crates.io/crates/macro_rules_attribute/0.2.2) crate,
including the following modifications:

- rename `apply` to `macro_apply`.
- rename `derive` to `macro_derive` to avoid ambiguity with Rust's built-in `derive`.
- rename `macro_rules_derive` to `macro_derive_with`.
- rename `attribute_alias!` to `macro_apply_alias!`.
- rename `derive_alias!` to `macro_derive_alias!`.
- replace the upstream-shaped alias syntax with direct alias-builder syntax.
- add argument support to `macro_apply`, `macro_derive`, and `macro_derive_with`.
- add argument support to `macro_apply_alias!` and `macro_derive_alias!`.
- remove the generic `custom` helper attribute.
- remove the legacy `macro_rules_attribute` public alias.
- replace `Custom` with the hidden helper derive `__macro_derive_helpers`.
- rename `derive_args` to `macro_derive_args`.
- keep `macro_derive_args` as the documented helper attribute for declarative derives.
- simplify the helper derive so it only registers derive helper attributes and emits no code.
- simplify path parsing and macro invocation logic.
- preserve optional trailing `!` support for `macro_apply` and `macro_derive_with`.
- preserve mixed derive support where entries ending in `!` are treated as declarative derives.
- remove dependency on `paste`.
- misc. refactors.
