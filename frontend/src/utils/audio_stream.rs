use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{OutputCallbackInfo, SampleRate};
use std::time::Duration;
use tokio::time::sleep;

pub async fn stream_audio_data(input_data: &[f32], sample_rate: u32) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("Error while querying configs");
    let config = supported_configs_range
        .next()
        .expect("No supported config?!")
        .with_sample_rate(SampleRate(sample_rate));

    let audio_len: f32 = input_data.len() as f32;
    let audio_data = input_data.to_vec();
    let mut pos = 0;

    let output_data = move |data: &mut [f32], _: &OutputCallbackInfo| {
        for sample in data.iter_mut() {
            if pos < audio_data.len() {
                *sample = audio_data[pos];
                pos += 1;
            } else {
                *sample = 0.0; // Silence if we've run out of data
            }
        }
    };

    let output_stream = device
        .build_output_stream(
            &config.into(),
            output_data,
            |err| eprintln!("an error occurred on stream: {}", err),
            None,
        )
        .unwrap();

    output_stream.play().unwrap();

    let duration_secs = audio_len / sample_rate as f32;
    sleep(Duration::from_secs_f32(duration_secs)).await;

    output_stream.pause().unwrap();
}

pub fn generate_sine_wave(sample_rate: u32, freq: f32, duration_secs: f32) -> Vec<f32> {
    let samples = (sample_rate as f32 * duration_secs) as usize;
    (0..samples)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            (2.0 * std::f32::consts::PI * freq * t).sin() * 0.5 // volume 50%
        })
        .collect()
}
