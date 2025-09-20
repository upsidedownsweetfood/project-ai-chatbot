mod components;
mod utils;

use core::panic;
use std::env;

use dioxus::prelude::*;

use crate::{
    components::{chatview::ChatView, error},
    utils::{
        audio_stream::{generate_sine_wave, stream_audio_data},
        ollama_stuff::OllamaClient,
    },
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone)]
pub struct AppState {
    pub ollama_client: OllamaClient,
    pub model: String,
}

#[component]
fn AppInit(value: String) -> Element {
    let ollama_client = OllamaClient::new(reqwest::Client::new(), value);
    let model = env::var("OLLAMA_MODEL").unwrap();

    use_context_provider(|| AppState {
        ollama_client: ollama_client,
        model,
    });

    spawn(async move {
        let app_state = use_context::<AppState>();

        app_state
            .ollama_client
            .pull_model(&app_state.model)
            .await
            .unwrap_or_else(|err| {
                panic!("Failed to pull model {}: {}", app_state.model, err);
            });
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        ChatView {}
    }
}

#[component]
fn App() -> Element {
    match env::var("OLLAMA_API") {
        Ok(value) => AppInit(AppInitProps { value }),
        Err(e) => error::Error(error::ErrorProps { err: e.to_string() }),
    }
}

// fn main() {
//     dioxus::launch(App);
// }

#[tokio::main]
async fn main() {
    let sample_rate = 44100;
    let freq = 440.0; // La
    let duration_secs = 2.0;
    let audio_data = generate_sine_wave(sample_rate, freq, duration_secs);

    println!(
        "Riproduco sinusoide {} Hz per {} secondi...",
        freq, duration_secs
    );
    stream_audio_data(&audio_data, sample_rate).await;
    println!("Fine riproduzione.");
}
