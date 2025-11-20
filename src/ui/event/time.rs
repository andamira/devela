// devela::ui::event::time
//
//! Defines [`EventTimestamp`].
//

use crate::{ConstInit, f32bits, f32bits_niche, impl_trait};

/// The time at which the event occurs, stored as single-precision milliseconds.
///
/// Backend dependent and relative to an arbitrary origin.
///
/// The underlying representation can be treated as either `f32` milliseconds
/// or `u32` milliseconds, depending on the event source.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EventTimestamp {
    /// The millisecond representation as `f32bits_niche` for compact storage,
    /// with a dual possible interpretation as either `f32` or `u32` milliseconds.
    ms: f32bits_niche,
}

// private helpers
#[rustfmt::skip]
#[allow(dead_code)]
impl EventTimestamp {
    pub(crate) const fn new(ms: f32bits_niche) -> Self { Self { ms } }
    pub(crate) const fn from_non_niche(ms: f32bits) -> Self { Self::new(ms.to_niche()) }
    pub(crate) const fn get_niche(self) -> f32bits_niche { self.ms }
    pub(crate) const fn get_non_niche(self) -> f32bits { self.ms.to_non_niche() }
}

/// Methods related to floating-point representations.
#[rustfmt::skip]
impl EventTimestamp {
    /// Creates a timestamp from milliseconds, ensuring a valid value.
    pub const fn from_secs_f32(seconds: f32) -> Self {
        EventTimestamp::new(f32bits_niche::new(seconds * 1000.0))
    }

    /// Creates a timestamp from milliseconds, ensuring a valid value.
    pub const fn from_millis_f32(ms: f32) -> Self {
        EventTimestamp::new(f32bits_niche::new(ms))
    }
    /// Converts to seconds as `f32` for calculations.
    pub const fn as_secs_f32(self) -> f32 { self.ms.as_float() * 0.001 }

    /// Converts to seconds as `f32` for calculations.
    pub const fn as_millis_f32(self) -> f32 { self.ms.as_float() }
}

/// Methods related to integer representations.
#[rustfmt::skip]
impl EventTimestamp {
    /// Creates a timestamp from integer milliseconds.
    ///
    /// The `u32` value is stored directly as the underlying bit pattern and is
    /// intended to be paired with [`as_millis_u32`].
    pub const fn from_millis_u32(ms: u32) -> Self {
        Self::new(f32bits_niche::from_bits(ms))
    }

    /// Interprets the stored bits as `u32` milliseconds and returns them.
    ///
    /// Only meaningful if the timestamp was created through [`from_millis_u32`].
    #[must_use]
    pub const fn as_millis_u32(&self) -> u32 { self.ms.as_bits() }

    /// Converts integer milliseconds to `f32` and stores the resulting bit pattern.
    ///
    /// This is the integer → floating-point path and is intended to be paired
    /// with [`as_millis_f32_to_u32`].
    pub const fn from_millis_u32_as_f32(ms: u32) -> Self {
        Self::new(f32bits_niche::new(ms as f32))
    }

    /// Returns the stored `f32` milliseconds truncated to `u32`.
    ///
    /// Only meaningful if the timestamp was created from a floating-point-based
    /// constructor (such as [`from_secs_f32`] or [`from_millis_f32`]).
    #[must_use]
    pub const fn as_millis_f32_to_u32(&self) -> u32 { self.ms.as_float() as u32 }
}

impl_trait! { fmt::Display for EventTimestamp |self, f| self.as_millis_f32().fmt(f) }
impl ConstInit for EventTimestamp {
    const INIT: Self = Self::new(f32bits_niche::INIT);
}

#[rustfmt::skip]
#[cfg(all(feature = "js", not(windows)))]
mod impl_js {
    pub use super::*;
    pub use crate::JsInstant;

    impl EventTimestamp {
        /// Converts a `JsInstant` to an `EventTimestamp`, ensuring a valid value.
        pub const fn from_js(from: JsInstant) -> EventTimestamp {
            EventTimestamp::from_millis_f32(from.as_millis_f64() as f32)
        }

        /// Converts an `EventTimestamp` to a `JsInstant`.
        pub const fn to_js(self) -> JsInstant {
            JsInstant::from_millis_f64(self.as_millis_f32() as f64)
        }
    }
    impl From<JsInstant> for EventTimestamp {
        fn from(from: JsInstant) -> Self { EventTimestamp::from_js(from) }
    }
    impl From<EventTimestamp> for JsInstant {
        fn from(from: EventTimestamp) -> Self { from.to_js() }
    }
}

/* tests */

#[cfg(test)]
const _SIZE: () = {
    assert![size_of::<f32bits_niche>() == 4];
    assert![size_of::<Option<f32bits_niche>>() == 4];
};
