// devela_base_core::num::int::alias

#![allow(non_camel_case_types)]

use crate::paste;

/// Generates [up|down]casted `[i|u]size` aliases.
macro_rules! iusize_alias {
    ($casted:literal, $sign:literal, $ty:ident, $($width:literal : $cast:ty),+) => { paste! {
        $(
            #[doc = crate::_tags!(primitive num)]
            #[doc = $casted " pointer-sized " $sign " integer primitive."]
            #[doc = crate::_doc_location!("num")]
            #[cfg(target_pointer_width = $width)]
            pub type $ty = $cast;
        )+
    }}
}
iusize_alias!["A downcasted", "signed", isize_down, "16":i8, "32":i16, "64":i32, "128":i64];
iusize_alias!["A downcasted", "unsigned", usize_down, "16":u8, "32":u16, "64":u32, "128":u64];

iusize_alias!["An upcasted", "signed", isize_up, "8":i16, "16":i32, "32":i64, "64":i128];
iusize_alias!["An upcasted", "unsigned", usize_up, "8":u16, "16":u32, "32":u64, "64":u128];
