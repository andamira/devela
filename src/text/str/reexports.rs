// devela::text::str:reexports
//
//! String related re-exports.
//

use crate::{ConstDefault, impl_cdef, paste, unwrap};

crate::mod_path!(+pub _c "../../../libs/base/src/text/str/reexports.rs");
crate::mod_path!(alloc +pub _a "../../../libs/base_alloc/src/text/str/reexports.rs");
crate::mod_path!(std +pub _s "../../../libs/base_std/src/text/str/reexports.rs");

pub use devela_base::text::str::{Str, StringU8, StringU16, StringU32, StringUsize};

macro_rules! impl_cdef_for_string_u {
    () => { impl_cdef_for_string_u![u8, u16, u32, usize]; };
    (// $t: the length type. E.g.: u8.
    $($t:ty),+ $(,)?) => {
        $(
            paste! { impl_cdef_for_string_u![@[<String $t:camel>], $t]; }
        )+
    };
    (// $name: the name of the type. E.g.: StringU8.
    @$name:ty, $t:ty) => { paste! {
        impl<const CAP: usize> ConstDefault for $name<CAP> {
            #[doc = "Returns an empty string.\n\n#Panics\n\nPanics if `CAP > `[`" $t "::MAX`]."]
            const DEFAULT: Self = unwrap![ok Self::new()];
        }
    }};
}
impl_cdef_for_string_u!();

/* from other modules */

pub use crate::CStr;
#[cfg(feature = "alloc")]
pub use crate::CString;

/* impl ConstDefault */

impl_cdef!["" => &str];
#[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
impl crate::ConstDefault for &mut str {
    // SAFETY: The empty string is valid UTF-8.
    const DEFAULT: Self = unsafe { ::core::str::from_utf8_unchecked_mut(&mut []) };
}
#[cfg(feature = "alloc")]
impl_cdef![Self::new() => String];
