// devela::ui::event::time
//
//! Defines [`EventTimestamp`].
//

use crate::{NonZeroU32, impl_trait};

/// The time at which the event actually occurs, stored as single-precision milliseconds.
///
/// Backend dependant, relative to an arbitrary moment.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EventTimestamp {
    /// The millisecond `f32` bit representation as `NonZeroU32` for compact storage.
    ms: NonZeroU32,
}

// private helpers
#[rustfmt::skip]
impl EventTimestamp {
    pub(crate) const fn new(ms: NonZeroU32) -> Self { Self { ms } }
}

/// floating-point methods
#[rustfmt::skip]
impl EventTimestamp {
    /// Creates a timestamp from milliseconds, ensuring a valid value.
    ///
    /// If the input is `0.0`, it defaults to `f32::EPSILON`.
    pub const fn from_secs_f32(seconds: f32) -> Self {
        let ms = (seconds * 1000.0).to_bits();
        let valid_ms = if ms == 0 { f32::EPSILON.to_bits() } else { ms };
        #[cfg(any(feature = "safe_ui", not(feature = "unsafe_niche")))]
        { EventTimestamp::new(NonZeroU32::new(valid_ms).unwrap()) }
        #[cfg(all(not(feature = "safe_ui"), feature = "unsafe_niche"))]
        // SAFETY: we make sure to never pass 0
        EventTimestamp::new(unsafe { NonZeroU32::new_unchecked(valid_ms) })

    }
    /// Tries to create a timestamp from milliseconds.
    /// Returns `None` if the input is `0.0`.
    pub const fn try_from_secs_f32(seconds: f32) -> Option<Self> {
        let ms = (seconds * 1000.0).to_bits();
        if ms == 0 {
            None
        } else {
            #[cfg(any(feature = "safe_ui", not(feature = "unsafe_niche")))]
            { Some(EventTimestamp::new(NonZeroU32::new(ms).unwrap())) }
            #[cfg(all(not(feature = "safe_ui"), feature = "unsafe_niche"))]
            // SAFETY: we make sure to never pass 0
            { Some(EventTimestamp::new(unsafe { NonZeroU32::new_unchecked(ms) })) }
        }
    }

    /// Creates a timestamp from milliseconds, ensuring a valid value.
    ///
    /// If the input is `0.0`, it defaults to `f32::EPSILON`.
    pub const fn from_millis_f32(ms: f32) -> Self {
        let ms = ms.to_bits();
        let valid_ms = if ms == 0 { f32::EPSILON.to_bits() } else { ms };
        #[cfg(any(feature = "safe_ui", not(feature = "unsafe_niche")))]
        { EventTimestamp::new(NonZeroU32::new(valid_ms).unwrap()) }
        #[cfg(all(not(feature = "safe_ui"), feature = "unsafe_niche"))]
        // SAFETY: we make sure to never pass 0
        EventTimestamp::new(unsafe { NonZeroU32::new_unchecked(valid_ms) })
    }
    /// Tries to create a timestamp from milliseconds.
    /// Returns `None` if the input is `0.0`.
    pub const fn try_from_millis_f32(ms: f32) -> Option<Self> {
        Self::try_from_millis_u32(ms.to_bits())
    }
    /// Converts to seconds as `f32` for calculations.
    pub const fn as_secs_f32(self) -> f32 { f32::from_bits(self.ms.get()) * 0.001 }
    /// Converts to seconds as `f32` for calculations.
    pub const fn as_millis_f32(self) -> f32 { f32::from_bits(self.ms.get()) }
}

/// Integer methods
#[rustfmt::skip]
impl EventTimestamp {
    /// Creates some timestamp from integer milliseconds, or returns `None` if it's 0.
    #[must_use]
    pub const fn try_from_millis_u32(ms: u32) -> Option<Self> {
        if ms == 0 {
            None
        } else {
            #[cfg(any(feature = "safe_ui", not(feature = "unsafe_niche")))]
            { Some(EventTimestamp::new(NonZeroU32::new(ms).unwrap())) }
            #[cfg(all(not(feature = "safe_ui"), feature = "unsafe_niche"))]
            // SAFETY: we make sure to never pass 0
            { Some(EventTimestamp::new(unsafe { NonZeroU32::new_unchecked(ms) })) }
        }
    }

    /// Interprets the stored bits as `u32` milliseconds and returns them as so.
    ///
    /// Useful when the original timestamp was created from integer data.
    #[must_use]
    pub const fn as_millis_u32(&self) -> u32 { self.ms.get() }

    /// Creates a timestamp from integer milliseconds by converting them to `f32`
    /// and storing the resulting bit pattern.
    ///
    /// Ensures the stored payload is nonzero by replacing zero with `f32::EPSILON`.
    #[must_use]
    pub const fn from_millis_u32_as_f32(ms: u32) -> Self {
        let f = if ms == 0 { f32::EPSILON } else { ms as f32 };
        let bits = f.to_bits();
        #[cfg(any(feature = "safe_ui", not(feature = "unsafe_niche")))]
        { EventTimestamp::new(NonZeroU32::new(bits).unwrap()) }
        #[cfg(all(not(feature = "safe_ui"), feature = "unsafe_niche"))]
        // SAFETY: bit pattern is guaranteed nonzero.
        EventTimestamp::new(unsafe { NonZeroU32::new_unchecked(bits) })
    }

    /// Interprets the stored bits as `f32` milliseconds and returns them as `u32`.
    ///
    /// Useful when the original timestamp was created from floating-point data
    /// and an integer millisecond view is desired.
    #[must_use]
    pub const fn as_millis_f32_to_u32(&self) -> u32 { f32::from_bits(self.ms.get()) as u32 }
}

impl Default for EventTimestamp {
    fn default() -> Self {
        Self::new(NonZeroU32::new(f32::EPSILON.to_bits()).unwrap())
    }
}
impl_trait! { fmt::Display for EventTimestamp |self, f| self.as_millis_f32().fmt(f) }

#[rustfmt::skip]
#[cfg(all(feature = "js", not(windows)))]
mod impl_js {
    pub use super::*;
    pub use crate::JsInstant;

    impl EventTimestamp {
        /// Converts a `JsInstant` to an `EventTimestamp`, ensuring a valid value.
        ///
        /// If the input is `0.0`, it defaults to `f32::EPSILON`.
        pub const fn from_js(from: JsInstant) -> EventTimestamp {
            EventTimestamp::from_millis_f32(from.as_millis_f64() as f32)
        }
        /// Tries to convert a `JsInstant` to an `EventTimestamp`.
        /// Returns `None` if the input is `0.0`.
        pub const fn try_from_js(from: JsInstant) -> Option<EventTimestamp> {
            EventTimestamp::try_from_millis_f32(from.as_millis_f64() as f32)
        }

        /// Converts an `EventTimestamp` to a `JsInstant`.
        pub const fn to_js(self) -> JsInstant {
            JsInstant::from_millis_f64(self.as_millis_f32() as f64)
        }
        // /// Tries to convert an `EventTimestamp` to a `JsInstant`.
        // ///
        // /// If the current value is `f32::EPSILON`, returns `None`.
        // pub const fn try_to_js(self) -> Option<JsInstant> {
        //     todo![]
        // }
    }
    impl From<JsInstant> for EventTimestamp {
        fn from(from: JsInstant) -> Self { EventTimestamp::from_js(from) }
    }
    impl From<EventTimestamp> for JsInstant {
        fn from(from: EventTimestamp) -> Self { from.to_js() }
    }
}
