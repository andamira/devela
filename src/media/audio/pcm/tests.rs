// devela::media::audio::pcm::tests

use crate::{AudioChannels, PcmBuf, PcmSample, PcmSpec};

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

/* PcmBuf */

const PCM_INTERLEAVED: PcmBuf<i16, &[i16]> =
    PcmBuf::new(&[0, 1, 2, 3], PcmSample::I16, AudioChannels::Stereo, 44_100);
const PCM_INTERLEAVED_FRAMES: Option<usize> = PCM_INTERLEAVED.frames();
const PCM_INTERLEAVED_COMPLETE: bool = PCM_INTERLEAVED.has_complete_frames();
const _: () = {
    assert!(matches!(PCM_INTERLEAVED_FRAMES, Some(2)));
    assert!(PCM_INTERLEAVED_COMPLETE);
};
const LEFT: &[i16] = &[0, 2];
const RIGHT: &[i16] = &[1, 3];
const PLANES: &[&[i16]] = &[LEFT, RIGHT];
const PCM_PLANAR: PcmBuf<i16, &[&[i16]]> =
    PcmBuf::new(PLANES, PcmSample::I16, AudioChannels::Stereo, 44_100);
const PCM_PLANAR_FRAMES: Option<usize> = PCM_PLANAR.frames();
const _: () = {
    assert!(matches!(PCM_PLANAR_FRAMES, Some(2)));
};

#[test]
fn pcm_buf_interleaved_frames() {
    let samples = [0i16; 480 * 2];
    let pcm = PcmBuf::new(&samples[..], PcmSample::I16, AudioChannels::Stereo, 48_000);
    assert_eq![pcm.samples_len(), 960];
    assert_eq![pcm.frames(), Some(480)];
    assert!(pcm.has_complete_frames());
    assert_eq![pcm.format(), PcmSample::I16];
    assert_eq![pcm.channels(), AudioChannels::Stereo];
    assert_eq![pcm.channel_count(), 2];
    assert_eq![pcm.sample_rate(), 48_000];
    assert_eq![pcm.spec(), PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 48_000)];
}
#[test]
fn pcm_buf_interleaved_rejects_incomplete_frames() {
    let samples = [0i16; 5];
    let pcm = PcmBuf::new(&samples[..], PcmSample::I16, AudioChannels::Stereo, 48_000);
    assert_eq![pcm.samples_len(), 5];
    assert_eq![pcm.frames(), None];
    assert!(!pcm.has_complete_frames());
}
#[test]
fn pcm_buf_interleaved_mut_slice() {
    let mut data = [0i16; 4];
    let mut pcm = PcmBuf::new(&mut data[..], PcmSample::I16, AudioChannels::Stereo, 44_100);
    assert_eq![pcm.samples_len(), 4];
    assert_eq![pcm.frames(), Some(2)];
    pcm.data_mut()[0] = 7;
    assert_eq![pcm.data()[0], 7];
}
#[test]
fn pcm_buf_interleaved_visit_each_frame_mut() {
    let mut data = [0i16, 1, 2, 3];
    let mut pcm = PcmBuf::new(&mut data[..], PcmSample::I16, AudioChannels::Stereo, 44_100);
    pcm.visit_each_frame_mut(|frame, samples| {
        samples[0] += frame as i16;
        samples[1] += frame as i16;
    });
    assert_eq![pcm.data(), &[0, 1, 3, 4]];
}
#[test]
fn pcm_buf_planar_metadata_and_frames() {
    let left = [1i16, 2, 3, 4];
    let right = [5i16, 6, 7, 8];
    let planes: &[&[i16]] = &[&left, &right];
    let pcm = PcmBuf::new(planes, PcmSample::I16, AudioChannels::Stereo, 48_000);
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
fn pcm_buf_planar_rejects_wrong_plane_count() {
    let left = [1i16, 2, 3, 4];
    let planes: &[&[i16]] = &[&left];
    let pcm = PcmBuf::new(planes, PcmSample::I16, AudioChannels::Stereo, 48_000);
    assert_eq![pcm.planes_len(), 1];
    assert_eq![pcm.channel_count(), 2];
    assert_eq![pcm.frames(), None];
}
#[test]
fn pcm_buf_planar_rejects_uneven_planes() {
    let left = [1i16, 2, 3, 4];
    let right = [5i16, 6, 7];
    let planes: &[&[i16]] = &[&left, &right];
    let pcm: PcmBuf<i16, &[&[i16]]> =
        PcmBuf::new(planes, PcmSample::I16, AudioChannels::Stereo, 48_000);
    assert_eq![pcm.frames(), None];
    assert!(!pcm.has_complete_frames());
}
#[test]
fn pcm_buf_planar_mut_slice() {
    let mut left = [1i16, 2];
    let mut right = [3i16, 4];
    let mut planes: [&mut [i16]; 2] = [&mut left, &mut right];
    let mut pcm = PcmBuf::new(&mut planes[..], PcmSample::I16, AudioChannels::Stereo, 44_100);
    assert_eq![pcm.planes_len(), 2];
    assert_eq![pcm.frames(), Some(2)];
    pcm.plane_mut(1).unwrap()[0] = 9;
    assert_eq![pcm.plane(1).unwrap()[0], 9];
}
#[test]
fn pcm_buf_planar_visit_each_mut() {
    let mut left = [1i16, 2];
    let mut right = [3i16, 4];
    let mut planes: [&mut [i16]; 2] = [&mut left, &mut right];
    let mut pcm: PcmBuf<i16, &mut [&mut [i16]]> =
        PcmBuf::new(&mut planes[..], PcmSample::I16, AudioChannels::Stereo, 44_100);
    pcm.visit_each_mut(|channel, frame, sample| {
        *sample += (channel + frame) as i16;
    });
    assert_eq![pcm.plane(0).unwrap(), &[1, 3]];
    assert_eq![pcm.plane(1).unwrap(), &[4, 6]];
}
