//
#![doc = crate::_DOC_SYS_LOG!()] // public
#![doc = crate::_doc!(modules: crate::sys; log)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod bench; //
    mod diag; // DiagLevel, DiagOut
    mod_ r#static; // LoggerStatic, slog!
    // mod trace; //
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            diag::*,
            r#static::_all::{LoggerStatic, slog},
        };
    }
    _hidden {
        pub use super::r#static::_hidden::*;
    }
}
