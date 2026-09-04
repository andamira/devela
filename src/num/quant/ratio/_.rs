// devela/src/num/quant/ratio/_.rs
//
//!
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // Ratio[<I|U><8|16|32|64|128>]
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
