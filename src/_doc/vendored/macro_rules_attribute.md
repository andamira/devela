This is derived work from the
[`macro_rules_attribute`](https://crates.io/crates/macro_rules_attribute/0.2.2) crate,
including the following modifications:

- integrate the attribute adapters directly into `devela_macros`.
- expose the public surface through `devela::code::util`.
- keep the core adapters: `macro_apply`, `macro_derive_with`, `macro_derive`.
- rename `apply` to `macro_apply`.
- rename `macro_rules_derive` to `macro_derive_with`.
- rename `derive` to `macro_derive` to avoid ambiguity with Rust's built-in `derive`.
- keep `derive_alias!` as the derive-bundle aliasing macro.
- rename `attribute_alias!` to `attr_alias!`.
- remove the generic `custom` helper attribute.
- remove the legacy `macro_rules_attribute` public alias.
- replace `Custom` with the hidden helper derive `__derive_helpers`.
- keep `derive_args` as the documented helper attribute for declarative derives.
- simplify the helper derive so it only registers derive helper attributes and emits no code.
- simplify path parsing and macro invocation logic.
- preserve optional trailing `!` support for `macro_apply` and `macro_derive_with`.
- preserve mixed derive support where entries ending in `!` are treated as declarative derives.
- remove dependency on `paste`.
- misc. refactors.
