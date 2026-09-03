// devela/src/media/font/format/bdf/_parse/_.rs
//
//! Private BDF grammar and header parsing.
//

crate::mods_in! {
    mod glyph; // Bdf<Bitmap, Encoding|Glyph>
    mod header; // Bdf<Version|Metrics|Section|Header>
    mod number; // Bdf<Number>
    mod parser; // BdfParser
    mod syntax; // Bdf<Reader|Line|Fields>
}
crate::mods_out! { // crate_internals
    // _reexports {
    //     use crate::BdfError as E;
    // }
    _crate_internals {
        pub(crate) use super::{
            glyph::{BdfBitmap, BdfEncoding, BdfGlyph},
            header::{BdfHeader, BdfMetrics, BdfVersion, read_i32_pair, read_number_pair},
            number::BdfNumber,
            syntax::{BdfFields, BdfLine, BdfReader},
            parser::BdfParser,
        };
    }
}

// IMPROVE
pub(super) type BdfResult<T> = Result<T, super::BdfError>;

macro_rules! bdf_try {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => return Err(error),
        }
    };
}
pub(super) use bdf_try;
