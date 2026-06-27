#!/usr/bin/env -S rust-script -c
//! ```cargo
//! [package]
//! name = "alsa"
//! [dependencies.devela]
//! path = "../../../.."
//! # git = "https://github.com/andamira/devela"
//! features = ["alsa", "std"]
//! ```
// devela/examples/sys/device/audio/alsa.rs

use devela::{
    Alsa, AlsaError, AudioChannels, Error, FloatConst, PcmBuf, PcmDrain, PcmRaw, PcmSample,
    PcmSampleType, PcmSink, PcmSource, PcmSpec, PcmWav, PcmWavFmt, sleep4,
};

const RATE: u32 = 48_000;
const SECONDS: usize = 3;
const CHUNK_FRAMES: usize = 512;
const CAPTURE_WAV: &str = "capture.wav";

fn main() {
    if let Err(err) = try_main() {
        eprintln!("alsa_lab: {err}");
        std::process::exit(1);
    }
}
fn try_main() -> Result<(), Box<dyn Error>> {
    Alsa::require_available()?;

    let action = std::env::args().nth(1).unwrap_or_else(|| "roundtrip".into());
    match action.as_str() {
        "devices" => list_devices()?,
        "beep" => {
            play_beep(880.0, 160)?;
            sleep4![0.5];
            play_beep(440.0, 160)?;
        }
        "roundtrip" => run_interleaved_roundtrip(true, true)?,
        "planar" => run_planar_roundtrip(true)?,
        "all" => {
            list_devices()?;
            run_interleaved_roundtrip(true, true)?;
            run_planar_roundtrip(true)?;
        }
        _ => {
            eprintln!("usage: alsa_lab [devices|beep|roundtrip|planar|all]");
            eprintln!("unknown action: {action}");
        }
    }
    Ok(())
}

/* actions */

fn list_devices() -> Result<(), AlsaError> {
    Alsa::for_each_pcm_device(|dev| {
        println!("{dev:?}");
        Ok(())
    })
}
fn run_interleaved_roundtrip(save: bool, cue: bool) -> Result<(), Box<dyn Error>> {
    let requested = PcmSpec::new(PcmSample::I16, AudioChannels::Mono, RATE);
    cue_recording_start(cue)?;
    let mut ain = Alsa::open_default_capture()?;
    let spec = ain.configure_interleaved(requested)?;
    report_recording("interleaved", spec);
    let samples = record_interleaved::<_, i16>(&mut ain, spec, SECONDS)?;
    drop(ain);
    cue_recording_stop(cue)?;
    report_captured_interleaved(spec, &samples);
    let mut aout = Alsa::open_default_playback()?;
    aout.configure_interleaved(spec)?;
    report_playback("interleaved");
    play_interleaved::<_, i16>(&mut aout, spec, &samples)?;
    if save {
        save_wav_i16(CAPTURE_WAV, spec, &samples)?;
        report_saved(CAPTURE_WAV);
    }
    Ok(())
}
fn run_planar_roundtrip(cue: bool) -> Result<(), Box<dyn Error>> {
    let requested = PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, RATE);
    cue_recording_start(cue)?;
    let mut ain = Alsa::open_default_capture()?;
    let spec = ain.configure_planar(requested)?;
    let frames = frames_for(spec);
    let (mut left, mut right) = (vec![0i16; frames], vec![0i16; frames]);
    let mut capture_planes: [&mut [i16]; 2] = [&mut left, &mut right];
    report_recording("planar", spec);
    ain.read_all_planar(PcmBuf::from_planar_mut(&mut capture_planes[..], spec))?;
    drop(ain);
    cue_recording_stop(cue)?;
    report_captured_planar(spec, &[&left, &right]);
    let mut aout = Alsa::open_default_playback()?;
    aout.configure_planar(spec)?;
    let playback_planes: &[&[i16]] = &[&left, &right];
    report_playback("planar");
    aout.write_all_planar(PcmBuf::from_planar(playback_planes, spec))?;
    aout.drain()?;
    Ok(())
}

/* generic interleaved path */

fn record_interleaved<I, T>(
    input: &mut I,
    spec: PcmSpec,
    seconds: usize,
) -> Result<Vec<T>, Box<dyn Error>>
where
    T: PcmSampleType,
    I: PcmSource<T>,
    I::Error: Error + 'static,
{
    let frames = spec.sample_rate as usize * seconds;
    let mut samples = vec![T::SILENCE; frames * spec.channel_count()];
    input.read_all(PcmBuf::from_interleaved_mut(&mut samples[..], spec))?;
    Ok(samples)
}

fn play_interleaved<O, T>(
    output: &mut O,
    spec: PcmSpec,
    samples: &[T],
) -> Result<(), Box<dyn Error>>
where
    T: PcmSampleType,
    O: PcmSink<T> + PcmDrain,
    O::Error: Error + 'static,
{
    output.write_all_drain(PcmBuf::from_interleaved(samples, spec))?;
    Ok(())
}

/* cues / reports / files */

fn cue_recording_start(enabled: bool) -> Result<(), AlsaError> {
    if enabled {
        eprintln!("cue: recording starts after beep");
        sleep4![0.9];
        play_beep(880.0, 500)?;
        sleep4![0.1];
    }
    Ok(())
}
fn cue_recording_stop(enabled: bool) -> Result<(), AlsaError> {
    if enabled {
        eprintln!("cue: recording stopped");
        play_beep(440.0, 160)?;
        sleep4![0.5];
    }
    Ok(())
}
fn frames_for(spec: PcmSpec) -> usize {
    spec.sample_rate as usize * SECONDS
}
fn report_recording(layout: &str, spec: PcmSpec) {
    eprintln!("Recording {SECONDS}s of {spec} as {layout}...");
}
fn report_playback(layout: &str) {
    println!("Playing back {layout}...");
}
fn report_saved(path: &str) {
    println!("Saved {path}");
    sleep4![1];
}
fn report_captured_interleaved(spec: PcmSpec, samples: &[i16]) {
    let frames = frames_for(spec);
    let peak = peak_i16(samples);
    println!("Captured {frames} frames, {} samples, peak amplitude: {peak}", samples.len());
}
fn report_captured_planar(spec: PcmSpec, planes: &[&[i16]]) {
    let frames = frames_for(spec);
    print!("Captured {frames} frames, peaks:");
    for (i, plane) in planes.iter().enumerate() {
        print!(" ch{i}={}", peak_i16(plane));
    }
    println!();
}

/* small local synth cue */

fn play_beep(freq: f32, millis: usize) -> Result<(), AlsaError> {
    let requested = PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, RATE);
    let mut pcm = Alsa::open_default_playback()?;
    let spec = pcm.configure_interleaved(requested)?;
    let channels = spec.channel_count();
    let total_frames = spec.sample_rate as usize * millis / 1000;
    let mut phase = 0.0f32;
    let step = f32::TAU * freq / spec.sample_rate as f32;
    let mut data = vec![0i16; CHUNK_FRAMES * channels];
    let mut remaining = total_frames;
    while remaining > 0 {
        let frames = remaining.min(CHUNK_FRAMES);
        let samples = frames * channels;
        for frame in 0..frames {
            let s = (phase.sin() * i16::MAX as f32 * 0.18) as i16;
            phase += step;
            if phase >= f32::TAU {
                phase -= f32::TAU;
            }
            let base = frame * channels;
            for ch in 0..channels {
                data[base + ch] = s;
            }
        }
        pcm.write_all(PcmBuf::from_interleaved(&data[..samples], spec))?;
        remaining -= frames;
    }
    pcm.drain()?;
    Ok(())
}

/* file / reports */

fn save_wav_i16(path: &str, spec: PcmSpec, samples: &[i16]) -> Result<(), Box<dyn Error>> {
    let mut bytes = vec![0u8; samples.len() * 2];
    let written = PcmRaw::encode_i16_le_into(&mut bytes, spec, samples)?;
    PcmWav::to_file(path, PcmWavFmt::from_spec(spec)?, &bytes[..written])?;
    Ok(())
}
fn peak_i16(samples: &[i16]) -> i32 {
    samples.iter().map(|&s| i32::from(s).abs()).max().unwrap_or(0)
}
