// devela::media::audio::pcm

use crate::{AudioChannels, PcmBuffer, PcmPlanar, PcmSample, PcmSpec};

#[test]
fn pcm_sample_sizes() {
    assert_eq![PcmSample::I16.bits(), 16];
    assert_eq![PcmSample::I16.bytes(), 2];
    assert!(PcmSample::I16.is_int());
    assert!(PcmSample::F32.is_float());
}

#[test]
fn pcm_spec_frame_bytes() {
    let spec = PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 48_000);
    assert_eq![spec.channel_count(), 2];
    assert_eq![spec.frame_bytes(), 4];
}

#[test]
#[cfg(feature = "alloc")]
fn pcm_spec_display() {
    use crate::ToString;
    let spec = PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 48_000);
    assert_eq![spec.to_string(), "i16/2.0@48000Hz"];
}

#[test]
fn pcm_buffer_frames() {
    let samples = [0i16; 480 * 2];
    let pcm = PcmBuffer::new(&samples, PcmSample::I16, AudioChannels::Stereo, 48_000);
    assert_eq![pcm.samples_len(), 960];
    assert_eq![pcm.frames(), Some(480)];
    assert!(pcm.has_complete_frames());
    let spec = pcm.spec();
    assert_eq![spec.sample, PcmSample::I16];
    assert_eq![spec.channels, AudioChannels::Stereo];
    assert_eq![spec.sample_rate, 48_000];
}
#[test]
fn pcm_buffer_rejects_incomplete_frames() {
    let samples = [0i16; 5];
    let pcm = PcmBuffer::new(&samples, PcmSample::I16, AudioChannels::Stereo, 48_000);
    assert_eq![pcm.samples_len(), 5];
    assert_eq![pcm.frames(), None];
    assert!(!pcm.has_complete_frames());
}

#[test]
fn pcm_planar_metadata_and_frames() {
    let left = [1i16, 2, 3, 4];
    let right = [5i16, 6, 7, 8];
    let planes: &[&[i16]] = &[&left, &right];
    let pcm = PcmPlanar::new(planes, PcmSample::I16, AudioChannels::Stereo, 48_000);
    assert_eq![pcm.format(), PcmSample::I16];
    assert_eq![pcm.channels(), AudioChannels::Stereo];
    assert_eq![pcm.channel_count(), 2];
    assert_eq![pcm.sample_rate(), 48_000];
    assert_eq![pcm.spec(), PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 48_000)];
    assert_eq![pcm.planes_len(), 2];
    assert_eq![pcm.frames(), Some(4)];
    assert!(!pcm.is_empty());
}

#[test]
fn pcm_planar_rejects_wrong_plane_count() {
    let left = [1i16, 2, 3, 4];
    let planes: &[&[i16]] = &[&left];
    let pcm = PcmPlanar::new(planes, PcmSample::I16, AudioChannels::Stereo, 48_000);
    assert_eq![pcm.planes_len(), 1];
    assert_eq![pcm.channel_count(), 2];
    assert_eq![pcm.frames(), None];
}

#[test]
fn pcm_planar_rejects_uneven_planes() {
    let left = [1i16, 2, 3, 4];
    let right = [5i16, 6, 7];
    let planes: &[&[i16]] = &[&left, &right];
    let pcm = PcmPlanar::new(planes, PcmSample::I16, AudioChannels::Stereo, 48_000);
    assert_eq![pcm.frames(), None];
}
