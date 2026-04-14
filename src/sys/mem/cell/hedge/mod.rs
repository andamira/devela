// devela::sys::mem::cell::hedge
//
//!
//

#[cfg(test)]
mod tests;

mod ctrl; // MemHedgeCtrl
mod error; // MemHedgeError
mod read; // MemHedgeRead
mod state; // MemHedgeState

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ctrl::*,
            error::*,
            read::*,
            state::*,
        };
    }
}
