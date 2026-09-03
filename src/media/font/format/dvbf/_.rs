// devela/src/media/font/format/dvbf/_.rs
//
//!
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // Dvbf
    mod error; // DvbfError
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::Dvbf,
            error::DvbfError,
        };
    }
}
