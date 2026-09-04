// devela/src/code/result/opt_res/_.rs
//
//! Optional values.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test_unwrap;

    mod ext_option; // OptionExt
    mod ext_result; // ResultExt
    mod fmt; // OptionFmt, OptionFmtOr, OptionFmtOrElse
    mod opt_res; // serr, sok, OptRes, OptResExt
    mod unwrap; // unwrap!
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            ext_option::OptionExt,
            ext_result::ResultExt,
            fmt::*,
            opt_res::*,
            unwrap::unwrap,
        };
    }
}
