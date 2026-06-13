// devela/src/sys/net/http/mod.rs
//
#![doc = crate::_DOC_SYS_NET!()] // public
#![doc = crate::_doc!(modules: crate::sys::net; http)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//
#![allow(unused)] // TEMP WIP

#[cfg(test)]
mod tests;

mod error; // HttpError
mod namespace; // Http, HttpMethod
// mod parse;
mod request; // HttpRequestLine
mod response; // HttpResponseHead
mod status; // HttpStatusClass, HttpStatus
mod version; // HttpVersion

// #[cfg(feature = "alloc")]
// mod owned; // owned request/response helpers

// #[cfg(feature = "std")]
// mod server;
// #[cfg(feature = "std")]
// mod static_;

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            error::*,
            namespace::*,
            request::*,
            response::*,
            status::*,
            version::*,
        };
        // #[cfg(feature = "alloc")]
        // pub use super::{
        //     owned::*,
        // };
        // #[cfg(feature = "std")]
        // pub use super::{
        //     server::*,
        //     static_::*,
        // }
    }
    _crate_internals {
        // pub use super::parse::*;
    }
}
