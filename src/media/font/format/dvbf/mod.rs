// devela/src/media/font/format/dvbf/mod.rs
//
//!
//

#[cfg(test)]
mod _test;

mod define; // Dvbf
mod error; // DvbfError

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::*,
            error::*,
        };
    }
}
