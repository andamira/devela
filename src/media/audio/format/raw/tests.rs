// devela::media::audio::format::raw::tests

use crate::{AudioChannels, PcmRaw, PcmRawBuf, PcmRawError, PcmSample, PcmSpec};

/* fixtures */

mod fixture {
    use super::*;

    pub(super) const I16_STEREO_BYTES: &[u8] = b"\0\0\0\0\xFF\x7F\0\x80";
    pub(super) const U8_MONO_BYTES: &[u8] = &[0x00, 0x80, 0xFF];
    pub(super) const I16_STEREO_SPEC: PcmSpec =
        PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 44_100);
    pub(super) const U8_MONO_SPEC: PcmSpec =
        PcmSpec::new(PcmSample::U8, AudioChannels::Mono, 8_000);
    pub(super) const INVALID_SPEC: PcmSpec = PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 0);
}

/* expectations */

mod expect {
    use super::*;

    pub(super) fn i16_stereo_44k<B: AsRef<[u8]>>(raw: &PcmRawBuf<B>) {
        assert_eq!(raw.spec(), fixture::I16_STEREO_SPEC);
        assert_eq!(raw.bytes(), fixture::I16_STEREO_BYTES);
        assert_eq!(raw.len(), 8);
        assert_eq!(raw.frame_bytes(), 4);
        assert_eq!(raw.frames().unwrap(), 2);
        assert!(raw.has_complete_frames());
    }
}

/* parsing */

mod parse {
    use super::{expect, fixture, *};

    #[test]
    fn parses_i16_stereo_borrowed() {
        let raw = PcmRaw::from_bytes(fixture::I16_STEREO_BYTES, fixture::I16_STEREO_SPEC).unwrap();
        expect::i16_stereo_44k(&raw);
    }
    #[test]
    fn parses_u8_mono() {
        let raw = PcmRaw::from_bytes(fixture::U8_MONO_BYTES, fixture::U8_MONO_SPEC).unwrap();
        assert_eq!(raw.spec(), fixture::U8_MONO_SPEC);
        assert_eq!(raw.bytes(), fixture::U8_MONO_BYTES);
        assert_eq!(raw.frames().unwrap(), 3);
        assert_eq!(raw.frame_bytes(), 1);
    }
    #[test]
    fn parses_const() {
        const RAW: Result<PcmRawBuf<&'static [u8]>, PcmRawError> =
            PcmRaw::from_bytes(fixture::I16_STEREO_BYTES, fixture::I16_STEREO_SPEC);
        const FRAMES: usize = match RAW {
            Ok(raw) => match raw.frames_const() {
                Ok(frames) => frames,
                Err(_) => 0,
            },
            Err(_) => 0,
        };
        assert_eq!(FRAMES, 2);
    }
    #[test]
    fn rejects_incomplete_frame() {
        let err = PcmRaw::from_bytes(&fixture::I16_STEREO_BYTES[..7], fixture::I16_STEREO_SPEC)
            .unwrap_err();
        assert_eq!(err, PcmRawError::InvalidDataLength);
    }
    #[test]
    fn rejects_invalid_spec() {
        let err = PcmRaw::from_bytes(fixture::I16_STEREO_BYTES, fixture::INVALID_SPEC).unwrap_err();
        assert_eq!(err, PcmRawError::InvalidSpec);
    }
}

/* encoding */

mod encode {
    use super::{fixture, *};

    #[test]
    fn encodes_into() {
        let mut out = [0u8; 8];
        let written =
            PcmRaw::write_into(&mut out, fixture::I16_STEREO_SPEC, fixture::I16_STEREO_BYTES)
                .unwrap();
        assert_eq!(written, fixture::I16_STEREO_BYTES.len());
        assert_eq!(&out, fixture::I16_STEREO_BYTES);
    }
    #[test]
    fn rejects_small_destination() {
        let mut out = [0u8; 4];
        let err = PcmRaw::write_into(&mut out, fixture::I16_STEREO_SPEC, fixture::I16_STEREO_BYTES)
            .unwrap_err();
        assert_eq!(err, PcmRawError::NotEnoughSpace);
    }
    #[test]
    fn rejects_incomplete_frame() {
        let mut out = [0u8; 8];
        let err =
            PcmRaw::write_into(&mut out, fixture::I16_STEREO_SPEC, &fixture::I16_STEREO_BYTES[..7])
                .unwrap_err();
        assert_eq!(err, PcmRawError::InvalidDataLength);
    }
}

/* allocation */

#[cfg(feature = "alloc")]
mod owned {
    use super::{fixture, *};

    #[test]
    fn parses_vec_and_borrows() {
        let raw =
            PcmRaw::from_vec(fixture::I16_STEREO_BYTES.to_vec(), fixture::I16_STEREO_SPEC).unwrap();
        let borrowed = raw.as_borrowed();
        assert_eq!(borrowed.bytes(), fixture::I16_STEREO_BYTES);
        assert_eq!(borrowed.spec(), fixture::I16_STEREO_SPEC);
    }
    #[test]
    fn encodes_to_vec() {
        let bytes = PcmRaw::to_vec(fixture::I16_STEREO_SPEC, fixture::I16_STEREO_BYTES).unwrap();
        assert_eq!(bytes, fixture::I16_STEREO_BYTES);
    }
}

/* file I/O */

#[cfg(feature = "std")]
mod file_io {
    use super::{fixture, *};
    use crate::Fs;

    #[test]
    fn writes_file_and_reparses() {
        let path = std::env::temp_dir().join("devela_test_pcm_raw_write.raw");
        PcmRaw::to_file(&path, fixture::I16_STEREO_SPEC, fixture::I16_STEREO_BYTES).unwrap();
        let raw = PcmRaw::from_file(&path, fixture::I16_STEREO_SPEC).unwrap();
        assert_eq!(raw.bytes(), fixture::I16_STEREO_BYTES);
        assert_eq!(raw.frames().unwrap(), 2);
        let _ = Fs::remove_file(path);
    }
}
