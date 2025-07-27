// devela::lang::ffi::js::types
//
//! Defines Javascript related types.
//

mod instant; // JsInstant
mod primitives; // js_number, js_int32, js_unit32, js_bool…
mod text; // JsTextMetrics, JsTextMetricsFull

pub use {instant::*, primitives::*, text::*};

// WIPZONE
// #[cfg(feature = "alloc")]
// crate::items! { mod bson; pub use bson::*; }
// #[cfg(feature = "std")]
// crate::items! { mod json; pub use json::*; }
