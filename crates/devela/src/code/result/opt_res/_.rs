//
//! Optional values.
//

crate::mods_in! {
    mod ext_option; // OptionExt
    mod ext_result; // ResultExt
    mod fmt; // OptionFmt, OptionFmtOr, OptionFmtOrElse
    mod opt_res; // serr, sok, OptRes, OptResExt
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            ext_option::OptionExt,
            ext_result::ResultExt,
            fmt::*,
            opt_res::*,
        };
        #[doc(inline)]
        pub use crate::boot::base::code::unwrap;
    }
}
