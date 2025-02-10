This is derived work from the
[stated-scope-guard](https://crates.io/crates/stated-scope-guard/0.1.0) crate
including the following modifications:

- remove the `DismissibleScopeGuard` type.
- rename the previous `new` method to `with`.
- refactor the `new_dismissible` function as the `new` method.
- feature-gate all uses of `unsafe`.
- update docs and examples.
