// devela/src/sys/mem/cell/hedge/_.rs
//
//!
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod ctrl; // MemHedgeCtrl
    mod error; // MemHedgeError
    mod read; // MemHedgeRead
    mod state; // MemHedgeState
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            ctrl::*,
            error::*,
            read::*,
            state::*,
        };
    }
}
