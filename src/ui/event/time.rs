// devela::ui::event::time
//
//! Defines [`EventTimestampMode`], [`EventTimestamp`].
//

use crate::{
    ConstInit, DebugExt, Duration, FmtResult, Formatter, TimeSplitNorm, f32bits, f32bits_niche,
    impl_trait,
};

#[doc = crate::_tags!(event time)]
/// Selects how an [`EventTimestamp`] should be formatted.
#[doc = crate::_doc_location!("ui/event")]
///
/// This controls whether the timestamp is shown as integer milliseconds,
/// floating-point milliseconds, both representations, or chosen automatically
/// using bit-pattern heuristics.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventTimestampMode {
    /// Use heuristic selection (integer, float, or dual).
    #[default]
    Auto,
    /// Force floating-point interpretation.
    Float,
    /// Force integer interpretation.
    Int,
    /// Show both integer and floating-point interpretations.
    Dual,
}
impl DebugExt for EventTimestamp {
    type Ctx = EventTimestampMode;
    fn fmt_with(&self, f: &mut Formatter, ctx: &Self::Ctx) -> FmtResult<()> {
        match ctx {
            Self::Ctx::Auto => self.fmt_auto_ms(f),
            Self::Ctx::Float => self.fmt_float_ms(f),
            Self::Ctx::Int => self.fmt_int_ms(f),
            Self::Ctx::Dual => self.fmt_dual_ms(f),
        }
    }
}

#[doc = crate::_tags!(event time)]
/// The time at which the event occurs, stored as single-precision milliseconds.
#[doc = crate::_doc_location!("ui/event")]
///
/// Backend dependent and relative to an arbitrary origin.
///
/// The underlying representation can be treated as either `f32` milliseconds
/// or `u32` milliseconds, depending on the event source.
#[must_use]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
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

/// General methods.
#[rustfmt::skip]
impl EventTimestamp {
    /// Converts the timestamp to a `Duration`, truncating fractional milliseconds.
    pub fn as_duration(&self) -> Duration {
        // Always round-down to integer ms before converting.
        // Ensures stable splits for both float and int paths.
        let ms = self.ms.as_float() as u64;
        Duration::from_millis(ms)
    }
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
    /// intended to be paired with [`as_millis_u32`][Self::as_millis_u32].
    pub const fn from_millis_u32(ms: u32) -> Self {
        Self::new(f32bits_niche::from_bits(ms))
    }

    /// Interprets the stored bits as `u32` milliseconds and returns them.
    ///
    /// Only meaningful if the timestamp was created through
    /// [`from_millis_u32`][Self::from_millis_u32].
    #[must_use]
    pub const fn as_millis_u32(&self) -> u32 { self.ms.as_bits() }

    /// Converts integer milliseconds to `f32` and stores the resulting bit pattern.
    ///
    /// This is the integer → floating-point path and is intended to be paired
    /// with [`as_millis_f32_to_u32`][Self::as_millis_f32_to_u32].
    pub const fn from_millis_u32_as_f32(ms: u32) -> Self {
        Self::new(f32bits_niche::new(ms as f32))
    }

    /// Returns the stored `f32` milliseconds truncated to `u32`.
    ///
    /// Only meaningful if the timestamp was created from a floating-point-based
    /// constructor (such as [`from_secs_f32`][Self::from_secs_f32] or
    /// [`from_millis_f32`][Self::from_millis_f32]).
    #[must_use]
    pub const fn as_millis_f32_to_u32(&self) -> u32 { self.ms.as_float() as u32 }
}

/// Methods related to formatting.
#[rustfmt::skip]
impl EventTimestamp {
    /// Formats the timestamp using heuristic detection.
    ///
    /// Chooses between integer, float, or dual representations based on the
    /// stored bit pattern. Intended for default `Debug` output.
    pub fn fmt_auto_ms(&self, f: &mut Formatter) -> FmtResult<()> {
        // valid finite float-ms live between 0.001 ms and ~11 days:
        let sure_float = |f:f32| f.is_finite() && (1e-3..=1e9).contains(&f);
        // NaNs, infinities, subnormal/tiny values, absurdly large magnitudes:
        let sure_int   = |f: f32| !f.is_finite() || !(1e-8..=1e12).contains(&f);
        let float = self.ms.as_float();
        if sure_float(float) { self.fmt_float_ms(f) }
        else if sure_int(float) { self.fmt_int_ms(f) }
        else { self.fmt_dual_ms(f) } // narrow ambiguous zone; show both representations
    }
    /// Formats the timestamp by interpreting the stored bits as integer milliseconds.
    pub fn fmt_int_ms(&self, f: &mut Formatter) -> FmtResult<()> {
        write![f, "{} ms", self.ms.as_bits()]
    }
    /// Formats the timestamp by interpreting the stored bits as floating-point milliseconds.
    pub fn fmt_float_ms(&self, f: &mut Formatter) -> FmtResult<()> {
        write![f, "{} ms", self.ms.as_float()]
    }
    /// Formats the timestamp by showing both integer and floating-point interpretations.
    pub fn fmt_dual_ms(&self, f: &mut Formatter) -> FmtResult<()> {
        write![f, "{}|{:.6} ms", self.ms.as_bits(), self.ms.as_float()]
    }

    /// Formats the timestamp using a full `TimeSplit` from years down to nanoseconds.
    pub fn fmt_split_full(&self, f: &mut Formatter) -> FmtResult<()> {
        let split = TimeSplitNorm::from_duration(self.as_duration());
        write!(f, "{}mo {}d {:02}h:{:02}m:{:02}s {:03}ms {:03}us {:03}ns",
            split.mo, split.d, split.h, split.m, split.s, split.ms, split.us, split.ns
        )
    }
}

impl_trait! { fmt::Debug for EventTimestamp |self, f| self.fmt_auto_ms(f) }
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
