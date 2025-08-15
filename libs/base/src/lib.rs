// devela_base::lib
//
//!
//

// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]

/* imports */

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate self as devela_base;

// mod code;
