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
    let model = env::var("OLLAMA_MODEL").unwrap();
    let ollama_client = OllamaClient::new(reqwest::Client::new(), value);

    let app_state = use_context_provider(|| AppState {
        ollama_client,
        model: model.clone(),
        messages: Vec::<ChatRoleMessage>::new(),
    });

    spawn(async move {
        use_context::<AppState>()
            .ollama_client
            .pull_model(model.as_str())
            .await
            .unwrap_or_else(|err| {
                panic!("Failed to pull model {}: {}", model, err);
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
