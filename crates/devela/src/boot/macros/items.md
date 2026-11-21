It can be useful to apply an attribute to a group of items.

It can also preserve the formatting of the code provided as arguments,
but the [`sf!`][crate::sf] macro is better for that, since it works with any arbitrary
code sequences like statements, expressions… instead of with just Rust items.

# Examples
```
use devela::items;

#[cfg(feature = "something")]
items! {
    mod something {
        pub struct SomeThing;
    }
    pub use something::SomeThing;
}
```
