// devela_base_macros::bodies
//
//! The bodies of the proc_macro functions defined in `lib.rs`.
//

crate::items! {
    #[cfg(test)]
    mod tests;

    mod shared;

    mod compile; // cif, compile[_attr|_doc]
    mod ident; // coalesce, field_of, ident_total, ident_total_unique, ident_unique
    pub(crate) use {compile::*, ident::*};
}
