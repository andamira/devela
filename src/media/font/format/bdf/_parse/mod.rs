// devela/src/media/font/format/bdf/_parse/mod.rs
//
//! Private BDF grammar and header parsing.
//

// mod bitmap; //
// mod glyph; // Bdf<Encoding|Glyph>
mod header; // Bdf<Version|Metrics|Section|Header>
mod number; // Bdf<Number>
mod syntax; // Bdf<Reader|Line|Fields>

crate::structural_mods! { // _mods
    _mods {
        pub(crate) use super::{
            // bitmap::*,
            // glyph::*,
            header::*,
            number::*,
            syntax::*,
        };
    }
}

/* local result propagation */

macro_rules! bdf_try {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => return Err(error),
        }
    };
}
use bdf_try;

type BdfResult<T> = Result<T, super::BdfError>;
