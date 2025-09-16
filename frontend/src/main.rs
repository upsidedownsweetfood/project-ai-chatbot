mod components;
mod utils;

use core::panic;
use std::env;

use dioxus::prelude::*;

use crate::{
    components::{
        error,
        hero::{AppState, Hero},
    },
    utils::ollama_stuff::{ChatRoleMessage, OllamaClient},
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
fn AppInit(value: String) -> Element {
    let mut ollama_client = OllamaClient::new(reqwest::Client::new(), value);
    let mut model = env::var("OLLAMA_MODEL").unwrap();

    use_context_provider(|| AppState {
        ollama_client: ollama_client,
        model,
    });

    spawn(async move {
        let app_state = use_context::<AppState>();

        app_state.ollama_client
            .pull_model(&app_state.model)
            .await
            .unwrap_or_else(|err| {
                panic!("Failed to pull model {}: {}", app_state.model, err);
            });
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
    }
}

#[component]
fn App() -> Element {
    match env::var("OLLAMA_API") {
        Ok(value) => AppInit(AppInitProps { value }),
        Err(e) => error::Error(error::ErrorProps { err: e.to_string() }),
    }
}

fn main() {
    dioxus::launch(App);
}
