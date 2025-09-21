use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::OutputCallbackInfo;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use tokio::time::sleep;
use rodio::{OutputStreamBuilder, Sink};
use cpal::{SampleRate, StreamConfig};
use minimp3::{Decoder, Frame, Error};
use std::sync::{Arc, Mutex};
use std::thread;

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



pub fn play_mp3_file() {
    std::env::set_var("PULSE_SERVER", "unix:/mnt/wslg/PulseServer");
    
    let file_path = "../SoundHelix-Song-1.mp3";
    
    println!("Tentativo con OutputStreamBuilder...");
    
    let file = File::open(file_path).expect("Impossibile aprire il file MP3");
    let source = rodio::Decoder::new(BufReader::new(file))
        .expect("Impossibile decodificare il file MP3");
    
    match OutputStreamBuilder::open_default_stream() {
        Ok(stream) => {
            println!("Stream aperto con successo!");
            
            let sink = Sink::connect_new(&stream.mixer());
            
            println!("Avvio riproduzione...");
            sink.append(source);
            sink.sleep_until_end();
            println!("Riproduzione completata");
            return;
        }
        Err(e) => {
            println!("OutputStreamBuilder fallito: {}", e);
            println!("Tentativo con CPAL diretto...");
        }
    }
    
    let host = cpal::default_host();
    println!("Host audio: {:?}", host.id());
    
    match host.output_devices() {
        Ok(devices) => {
            println!("Dispositivi output disponibili:");
            for (i, device) in devices.enumerate() {
                println!("  {}: {}", i, device.name().unwrap_or("Sconosciuto".to_string()));
            }
        }
        Err(e) => println!("Errore nel listare dispositivi: {}", e),
    }
    
    let device = match host.default_output_device() {
        Some(device) => {
            println!("Dispositivo di default: {}", device.name().unwrap_or("Sconosciuto".to_string()));
            device
        }
        None => {
            eprintln!("Nessun dispositivo audio trovato!");
            return;
        }
    };
    
    let file = File::open(file_path).expect("Impossibile aprire il file MP3");
    let mut decoder = Decoder::new(BufReader::new(file));
    let mut audio_data: Vec<f32> = Vec::new();
    let mut sample_rate = 44100;
    let mut channels = 2;
    
    loop {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate: sr, channels: ch, .. }) => {
                sample_rate = sr as u32;
                channels = ch as u16;
                
                for sample in data.iter() {
                    audio_data.push(*sample as f32 / i16::MAX as f32);
                }
            }
            Err(Error::Eof) => break,
            Err(e) => panic!("Errore nella decodifica: {:?}", e),
        }
    }
    
    println!("Decodificato: {} samples, {}Hz, {} canali", audio_data.len(), sample_rate, channels);
    
    let config = StreamConfig {
        channels,
        sample_rate: SampleRate(sample_rate),
        buffer_size: cpal::BufferSize::Default,
    };
    
    let audio_data = Arc::new(audio_data);
    let position = Arc::new(Mutex::new(0usize));
    let finished = Arc::new(Mutex::new(false));
    
    let audio_data_clone = Arc::clone(&audio_data);
    let position_clone = Arc::clone(&position);
    let finished_clone = Arc::clone(&finished);
    
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut pos = position_clone.lock().unwrap();
            let mut is_finished = finished_clone.lock().unwrap();
            
            if *is_finished {
                for sample in data.iter_mut() {
                    *sample = 0.0;
                }
                return;
            }
            
            for sample in data.iter_mut() {
                if *pos < audio_data_clone.len() {
                    *sample = audio_data_clone[*pos];
                    *pos += 1;
                } else {
                    *sample = 0.0;
                    *is_finished = true;
                }
            }
        },
        |err| {
            eprintln!("Errore nello stream audio: {}", err);
        },
        None,
    );
    
    match stream {
        Ok(stream) => {
            println!("Stream CPAL creato con successo");
            if let Err(e) = stream.play() {
                eprintln!("Errore nell'avvio dello stream: {}", e);
                return;
            }
            
            loop {
                thread::sleep(Duration::from_millis(100));
                let is_finished = *finished.lock().unwrap();
                if is_finished {
                    break;
                }
            }
            println!("Riproduzione completata");
        }
        Err(e) => {
            eprintln!("Impossibile creare lo stream CPAL: {}", e);
        }
    }
}