//! Regression test for https://github.com/shssoichiro/ffmpeg-the-third/issues/95
//!
//! `examples/transcode-audio.rs` is expected to let the caller pick the output
//! sample rate independently of the input file's sample rate. Before the fix,
//! the encoder's rate was hardcoded to the decoder's rate, so the output was
//! always re-resampled back to the input's rate no matter what filter (e.g.
//! `aresample=44100`) or explicit rate argument was requested.

use std::path::PathBuf;
use std::process::Command;

use ffmpeg::{codec, format, media};
use ffmpeg_the_third as ffmpeg;

const INPUT_RATE: u32 = 48_000;
const TARGET_RATE: u32 = 44_100;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn cargo_bin() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

/// Writes a short mono 16-bit PCM sine-wave WAV fixture at `INPUT_RATE`,
/// hand-built as raw bytes. This crate only links the FFmpeg *libraries*
/// (no dev setup step installs the `ffmpeg` CLI), so the fixture must not
/// depend on an external binary -- and a plain WAV header is simple enough
/// that it doesn't need one.
fn generate_input_wav(path: &std::path::Path) {
    const FREQUENCY_HZ: f64 = 440.0;
    const DURATION_SECS: u32 = 1;
    const BITS_PER_SAMPLE: u16 = 16;
    const CHANNELS: u16 = 1;

    let num_samples = INPUT_RATE * DURATION_SECS;
    let block_align = CHANNELS * (BITS_PER_SAMPLE / 8);
    let byte_rate = INPUT_RATE * u32::from(block_align);
    let data_size = num_samples * u32::from(block_align);

    let mut wav = Vec::with_capacity(44 + data_size as usize);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_size).to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // fmt chunk size (PCM)
    wav.extend_from_slice(&1u16.to_le_bytes()); // audio format: PCM
    wav.extend_from_slice(&CHANNELS.to_le_bytes());
    wav.extend_from_slice(&INPUT_RATE.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&block_align.to_le_bytes());
    wav.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes());

    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());

    for n in 0..num_samples {
        let t = f64::from(n) / f64::from(INPUT_RATE);
        let amplitude = (t * FREQUENCY_HZ * 2.0 * std::f64::consts::PI).sin();
        let sample = (amplitude * f64::from(i16::MAX) * 0.8) as i16;
        wav.extend_from_slice(&sample.to_le_bytes());
    }

    std::fs::write(path, wav).expect("failed to write test fixture WAV");
}

fn output_sample_rate(path: &std::path::Path) -> u32 {
    ffmpeg::init().unwrap();

    let ictx = format::input(path).unwrap();
    let stream = ictx
        .streams()
        .best(media::Type::Audio)
        .expect("no audio stream in transcoder output");
    let context = codec::context::Context::from_parameters(stream.parameters()).unwrap();
    let decoder = context.decoder().audio().unwrap();

    decoder.rate()
}

#[test]
fn transcode_audio_honors_requested_output_sample_rate() {
    let work_dir = std::env::temp_dir().join("ffmpeg_the_third_test_issue_95");
    let _ = std::fs::remove_dir_all(&work_dir);
    std::fs::create_dir_all(&work_dir).unwrap();

    let input_path = work_dir.join("input.wav");
    let output_path = work_dir.join("output.wav");

    generate_input_wav(&input_path);

    // No seek requested (4th arg left empty), target sample rate 44100 (5th arg).
    let status = Command::new(cargo_bin())
        .current_dir(manifest_dir())
        .args([
            "run",
            "--quiet",
            "--example",
            "transcode-audio",
            "--",
            input_path.to_str().unwrap(),
            output_path.to_str().unwrap(),
            "aresample=44100",
            "",
            &TARGET_RATE.to_string(),
        ])
        .status()
        .expect("failed to run transcode-audio example");

    assert!(
        status.success(),
        "transcode-audio example exited with an error"
    );

    let actual_rate = output_sample_rate(&output_path);
    assert_eq!(
        actual_rate, TARGET_RATE,
        "expected transcode-audio to produce {TARGET_RATE}Hz output, got {actual_rate}Hz \
         (input was {INPUT_RATE}Hz) -- see issue #95"
    );

    let _ = std::fs::remove_dir_all(&work_dir);
}
