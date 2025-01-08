// devela::data::collections::tuple
//
//! Re-exports thec code-generated [`Tuple`] trait, and defines related items.
//
// See also
// - https://dev-doc.rust-lang.org/stable/unstable-book/library-features/tuple-trait.html

#[cfg(test)]
mod tests;

#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_tuple")))]
mod codegen {
    use crate::{Debug, Display, FmtResult, Formatter};

    // - trait `Tuple` and its multi-arity impls.
    // - enums `TupleEnumRef` and `TupleEnumMut`.
    include!(concat!(env!("OUT_DIR"), "/build/tuple.rs"));

    /// Marker trait to prevent downstream implementations of the [`Tuple`] trait.
    trait Sealed {}

    /// A formatting wrapper for [`Tuple`]s, implementing `Display` and `Debug`.
    #[repr(transparent)]
    pub struct TupleFmt<'a, T: Tuple>(&'a T);

    /// Private trait for [`Tuple`]s with elements that implement `Debug`.
    trait TupleDebug: Tuple {
        fn fmt_debug(&self, f: &mut Formatter) -> FmtResult<()>;
    }
    impl<T: TupleDebug> Debug for TupleFmt<'_, T> {
        fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
            self.0.fmt_debug(f)
        }
    }

    /// Private trait for [`Tuple`]s with elements that implement `Display`.
    trait TupleDisplay: Tuple {
        fn fmt_display(&self, f: &mut Formatter) -> FmtResult<()>;
    }
    impl<T: TupleDisplay> Display for TupleFmt<'_, T> {
        fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
            self.0.fmt_display(f)
        }
    }
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::codegen::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
