// devela/src/num/fin/ord/order/_.rs
//
//! Algorithms and structures that depend on relative position rather than magnitude alone.
//

crate::mods_in! {
    mod namespace; // Order
    // impls:
    // mod gosper;
    // mod hilbert;
    // mod morton;
    // mod peano;
    mod rowcol;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::namespace::Order;
    }
}
