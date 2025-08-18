// devela_macros::bodies
//
//! The bodies of the proc_macro functions defined in `lib.rs`.
//

crate::items! {
    #[cfg(test)]
    mod tests;

    mod shared;

    mod compile; // cif, compile[_attr|_doc]
    mod enumint; // enumint
    mod ident;
    pub(crate) use {compile::*, enumint::*, ident::*};
}
