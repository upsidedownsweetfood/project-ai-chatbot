mod components;
mod utils;

use dioxus::prelude::*;
use components::output_box::OutputBox;
use utils::ollama_stuff;

use crate::utils::ollama_stuff::OllamaClient;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone)]
struct AppState {
    ollama_client: OllamaClient
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let app_state = use_context_provider(|| AppState {
        ollama_client: OllamaClient::new(reqwest::Client::new(), "localhost:1234".into())
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    let mut name = use_signal(|| String::new());

    rsx! {
        div {
            input {
                class: "input",
                placeholder: "Enter your name",
                oninput: move | event | name.set(event.value())
            }
            button {
                onclick: { move | event | {
                    use_context::<AppState>().ollama_client;
                }},
                "Enter"
            }
            OutputBox { output: name}
        }
    }
}
