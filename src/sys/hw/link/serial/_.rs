// devela/sys/hw/link/serial/_.rs
//
//! Serial communication interfaces.
//
// Unified access to UART, RS-232, USB-serial adapters,
// and platform serial backends for bidirectional byte streams.
//

crate::mods_in! {
    // mod_ posix_tty; //
    // mod_ usb_tty; // cdc_acm
    // mod_ rs232; // uart
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::{
        //     posix_tty::_all::*,
        //     usb_tty::_all::*,
        //     rs232::_all::*,
        // };
    }
}
