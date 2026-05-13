// devela_macros::bodies
//
//! The bodies of the proc_macro functions defined in `index.rs`.
//

crate::items! {
    #[cfg(test)]
    mod tests;

    mod shared;

    mod derive; // macro_apply!, macro_derive!, macro_derive-with!
    mod compile; // cif!, compile!, compile_attr!, (compile_doc!)
    mod enumint; // enumint!
    mod ident; // coalesce!, field_of!, ident_total!, ident_total_unique!, ident_unique!
    mod paste; // paste!
    mod repeat; // repeat!

    pub(crate) use {
        derive::*,
        compile::*,
        enumint::*,
        ident::*,
        paste::*,
        repeat::*,
    };
}
