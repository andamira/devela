// devela_macros::bodies
//
//! The bodies of the proc_macro functions defined in `lib.rs`.
//

#[cfg(feature = "alloc")]
crate::items! {
    #[cfg(test)]
    mod tests;

    mod shared;

    mod compile;
    mod ident;
    mod niche;
    pub(crate) use {compile::*, ident::*, niche::*};
}
