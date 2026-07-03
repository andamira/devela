// devela/src/code/util/derive/mod.rs

#[cfg(test)]
mod _test;

mod alias; // macro_apply_alias!, derive_alias!

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            alias::*,
        };
    }
    _reexports {
        /// Applies a declarative macro to the decorated item.
        #[doc = crate::_doc_meta!{location(proc "code/util")}]
        ///
        /// Expands `#[macro_apply(m)] item` as `m! { item }`.
        ///
        /// The macro receives ownership of the item and must re-emit it if it should remain.
        ///
        /// # Examples
        /// ```
        #[doc = include_str!("./macro_apply_examples.rs")]
        /// ```
        #[doc = crate::_doc_vendor!("macro_rules_attribute")]
        #[allow(rustdoc::invalid_html_tags)] #[doc = "<!--"]
        pub use devela_macros::macro_apply;

        /// Runs classic derives and declarative derives from one list.
        #[doc = crate::_doc_meta!{location(proc "code/util")}]
        ///
        /// Entries ending in `!` are called as declarative macros.
        /// Other entries are forwarded to Rust's built-in `derive`.
        ///
        /// The optional helper attribute `#[macro_derive_args(...)]`
        /// may be used to pass item-local arguments to declarative derives.
        ///
        /// # Examples
        /// ```
        #[doc = include_str!("./macro_derive_examples.rs")]
        /// ```
        #[doc = crate::_doc_vendor!("macro_rules_attribute")]
        #[allow(rustdoc::invalid_html_tags)] #[doc = "<!--"]
        pub use devela_macros::macro_derive;

        /// Runs declarative derive-like macros over the decorated item.
        #[doc = crate::_doc_meta!{location(proc "code/util")}]
        ///
        /// Each macro receives a copy of the item and may emit impls or side-items.
        /// The original item is preserved.
        ///
        /// The optional helper attribute `#[macro_derive_args(...)]`
        /// may be used to pass item-local arguments to those declarative derives.
        ///
        /// # Examples
        /// ```
        #[doc = include_str!("./macro_derive_with_examples.rs")]
        /// ```
        #[doc = crate::_doc_vendor!("macro_rules_attribute")]
        #[allow(rustdoc::invalid_html_tags)] #[doc = "<!--"]
        pub use devela_macros::macro_derive_with;
    }
}
