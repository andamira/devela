// devela/src/num/alg/matrix/_.rs
//
//! Static and borrowed matrix representations.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // Matrix
    mod methods; // construction, shape, access, structural operations
    mod ops; // overloadable operators
    mod primitive; // const primitive arithmetic

    // mod layout; // TODO MatrixLayout: rows, columns, offset and strides
    // mod view; // TODO MatrixView<D>: external backing interpretation
    //
    // #[cfg(feature = "alloc")]
    // mod buf; // TODO MatrixBuf<T>: dynamic owning dense matrix
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::Matrix,
            // layout::MatrixLayout,
            // view::MatrixView,
        };
        // #[cfg(feature = "alloc")]
        // pub use super::buf::MatrixBuf;
    }
}
