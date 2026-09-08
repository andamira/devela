// devela/src/sys/net/transport/_.rs
//
#![doc = crate::_DOC_SYS_NET_TRANSPORT!()] // public
#![doc = crate::_doc!(modules: crate::sys::net; transport)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: net)]
//

crate::mods_in! {
    #[cfg(feature = "std")]
    mod _reexport_std;

    // mod_ quic; // Secure multiplexed transport over UDP
    // mod_ sctp; // Message-oriented transport with multistreaming and multihoming
    mod_ tcp; // Reliable ordered byte-stream transport
    mod_ udp; // Connectionless datagram transport

}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            // quic::_all::*,
            // sctp::_all::*,
            tcp::_all::*,
            udp::_all::*,
        };
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
