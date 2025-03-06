// devela::lang:js::types
//
//! Defines the [`Js`] namespace and other items.
//

mod event; // JsEvent
mod instant; // JsInstant
mod permission; // JsPermission, JsPermissionState
mod text; // JsTextMetrics, JsTextMetricsFull
mod worker; // JsWorker, JsWorkerError, JsWorkerJob

pub use {event::*, instant::*, permission::*, text::*, worker::*};

// WIPZONE
// #[cfg(feature = "alloc")]
// crate::items! { mod bson; pub use bson::*; }
// #[cfg(feature = "std")]
// crate::items! { mod json; pub use json::*; }

/// A Javascript namespace.
///
/// # Methods
/// - core APis
///   - [console](#web-api-console)
///   - [events](#web-api-events)
///   - [history](#web-api-history--navigation)
///   - [permissions](#web-api-permissions)
/// - extended APis
///   - media & graphics
///     - [canvas](#web-api-canvas)
//   - system & hardware
///   - performance & optimization
//     - time
//   - advanced & experimental
pub struct Js;
