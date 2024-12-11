// devela::data::collections::tuple
//
//!
//

#[cfg(test)]
mod tests;

mod codegen {
    use crate::{Debug, Display, FmtResult, Formatter};

    // - trait `Tuple` and its multi-arity impls.
    // - enums `TupleEnumRef` and `TupleEnumMut`.
    include!(concat!(env!("OUT_DIR"), "/build/tuple.rs"));

    #[doc = crate::doc_private!()]
    /// Marker trait to prevent downstream implementations of the [`Tuple`] trait.
    trait Sealed {}

    /// A formatting wrapper for [`Tuple`]s, implementing `Display` and `Debug`.
    #[repr(transparent)]
    pub struct TupleFmt<'a, T: Tuple>(&'a T);

    #[doc = crate::doc_private!()]
    /// Private trait for [`Tuple`]s with elements that implement `Debug`.
    trait TupleDebug: Tuple {
        fn fmt_debug(&self, f: &mut Formatter) -> FmtResult<()>;
    }
    impl<T: TupleDebug> Debug for TupleFmt<'_, T> {
        fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
            self.0.fmt_debug(f)
        }
    }

    #[doc = crate::doc_private!()]
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

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::codegen::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
