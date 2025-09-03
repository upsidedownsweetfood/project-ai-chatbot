use crate::{components::output_box::OutputBox, utils::ollama_stuff::OllamaClient};
use dioxus::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub ollama_client: OllamaClient,
}

#[component]
pub fn Hero() -> Element {
    let mut name = use_signal(|| String::new());
    let mut received_output = use_signal(|| String::new());

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
                    received_output.set(name.to_string());
                }},
                "Enter"
            }
            OutputBox { output: received_output}
        }
    }
}