// devela_macros/src/bodies/_.rs
//
//! The bodies of the proc_macro functions defined in `index.rs`.
//

crate::items! {
    #[cfg(test)]
    mod _test;

    #[path = "shared/_.rs"]
    mod shared;

    mod derive; // macro_apply!, macro_derive!, macro_derive-with!
    mod compile; // cif!, compile!, compile_attr!, (compile_doc!)
    mod enumint; // enumint!
    mod ident; // coalesce!, field_of!, ident_total!, ident_total_unique!, ident_unique!
    mod mods_in; // mods_in!
    #[path = "paste/_.rs"]
    mod paste; // paste!
    mod repeat; // repeat!

    pub(crate) use {
        derive::*,
        compile::*,
        enumint::*,
        ident::*,
        mods_in::*,
        paste::*,
        repeat::*,
    };
}
