// devela::media::audio::pcm::sample
//
//! Defines [`PcmSample`], [`PcmSampleType`].
//

#[cfg(feature = "alsa")]
use crate::_alsa_raw;
use crate::{_impl_init, impl_trait};

#[doc = crate::_tags!(audio)]
/// Numeric encoding of a single PCM sample.
#[doc = crate::_doc_location!("media/audio")]
#[allow(missing_docs)]
#[must_use]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PcmSample {
    I8,
    U8,
    #[default]
    I16,
    I24,
    I32,
    F32,
    F64,
}
_impl_init![Self::I16 => PcmSample];
impl_trait![fmt::Display for PcmSample |self, f| f.write_str(self.as_code())];

impl PcmSample {
    /// Returns the number of meaningful sample bits.
    #[must_use]
    pub const fn bits(self) -> u8 {
        match self {
            Self::I8 => 8,
            Self::U8 => 8,
            Self::I16 => 16,
            Self::I24 => 24,
            Self::I32 | Self::F32 => 32,
            Self::F64 => 64,
        }
    }
    /// Returns the canonical byte width for packed raw PCM.
    #[must_use]
    pub const fn bytes(self) -> usize {
        match self {
            Self::I8 => 1,
            Self::U8 => 1,
            Self::I16 => 2,
            Self::I24 => 3,
            Self::I32 | Self::F32 => 4,
            Self::F64 => 8,
        }
    }
    /// Checks the equality of two PCM samples.
    #[must_use]
    pub const fn eq(self, other: Self) -> bool {
        self as u8 == other as u8
    }
    /// Returns whether this is an integer sample encoding.
    #[must_use]
    pub const fn is_int(self) -> bool {
        matches!(self, Self::I8 | Self::U8 | Self::I16 | Self::I24 | Self::I32)
    }
    /// Returns whether this is a signed integer sample encoding.
    #[must_use]
    pub const fn is_signed_int(self) -> bool {
        matches!(self, Self::I8 | Self::I16 | Self::I24 | Self::I32)
    }

    /// Returns whether this is an unsigned integer sample encoding.
    #[must_use]
    pub const fn is_unsigned_int(self) -> bool {
        matches!(self, Self::U8)
    }
    /// Returns whether this is a floating-point sample encoding.
    #[must_use]
    pub const fn is_float(self) -> bool {
        matches!(self, Self::F32 | Self::F64)
    }
    /// Returns whether this sample encoding can represent negative values.
    #[must_use]
    pub const fn has_negative(self) -> bool {
        matches!(self, Self::I8 | Self::I16 | Self::I24 | Self::I32 | Self::F32 | Self::F64)
    }
    /// Returns a compact encoding label.
    #[must_use]
    pub const fn as_code(self) -> &'static str {
        match self {
            Self::I8 => "i8",
            Self::U8 => "u8",
            Self::I16 => "i16",
            Self::I24 => "i24",
            Self::I32 => "i32",
            Self::F32 => "f32",
            Self::F64 => "f64",
        }
    }

    // Converts the sample to ALSA format.
    #[cfg(feature = "alsa")]
    #[cfg_attr(not(ffi_alsa··), allow(dead_code))]
    pub(crate) const fn to_alsa(self) -> _alsa_raw::snd_pcm_format_t {
        match self {
            Self::I8 => _alsa_raw::SND_PCM_FORMAT_S8,
            Self::U8 => _alsa_raw::SND_PCM_FORMAT_U8,
            Self::I16 => _alsa_raw::SND_PCM_FORMAT_S16_LE,
            Self::I24 => _alsa_raw::SND_PCM_FORMAT_S24_3LE,
            Self::I32 => _alsa_raw::SND_PCM_FORMAT_S32_LE,
            Self::F32 => _alsa_raw::SND_PCM_FORMAT_FLOAT_LE,
            Self::F64 => _alsa_raw::SND_PCM_FORMAT_FLOAT64_LE,
        }
    }
}

#[doc = crate::_tags!(audio)]
/// Rust sample type with a fixed PCM sample encoding.
#[doc = crate::_doc_location!("media/audio")]
///
/// This trait connects typed sample buffers such as `&[i16]` or `&mut [f32]`
/// to their corresponding [`PcmSample`] encoding.
///
/// It is sealed because backend I/O depends on the Rust type layout matching
/// the declared PCM sample representation.
#[expect(private_bounds, reason = "Sealed")]
pub trait PcmSampleType: Copy + Sealed {
    /// The PCM sample encoding represented by this Rust type.
    const SAMPLE: PcmSample;

    /// The silent value for this PCM sample type.
    ///
    /// This is the value that represents zero amplitude in the corresponding
    /// PCM encoding.
    const SILENCE: Self;
}

/// Marker trait to prevent downstream implementations of the [`PcmSampleType`] trait.
trait Sealed {}
macro_rules! _pcm_sample_type {
    () => {
        _pcm_sample_type!(u8,U8,128; i8,I8,0; i16,I16,0; i32,I32,0; f32,F32,0.0; f64,F64,0.0);
    };
    ($($T:ty, $sample:ident, $SI:literal);+ $(;)?) => {
        $( _pcm_sample_type!(% $T, $sample, $SI);)+
    };
    (% $T:ty, $sample:ident, $SI:literal) => {
        impl Sealed for $T {}
        impl PcmSampleType for $T {
            const SAMPLE: PcmSample = PcmSample::$sample;
            const SILENCE: Self = $SI;
        }
    };
}
_pcm_sample_type!();
