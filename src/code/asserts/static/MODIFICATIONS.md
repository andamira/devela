This is derived work from the
[`static_assertions`](https://crates.io/crates/static_assertions/1.1.0) crate,
including the following modifications:

- merge the multiple `const_assert*` macros into a single one with multiple arms.
- rename the `to_bool` macro to `const_bool`, and simplify.
- rename the `ToBool` trait to `ConstBool`, as well as its inner items.
- update syntax to 2012 edition.