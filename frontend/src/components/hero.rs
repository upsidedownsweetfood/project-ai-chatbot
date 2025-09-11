use crate::{components::output_box::OutputBox, utils::ollama_stuff::{ChatResponseBody, ChatRoleMessage, OllamaClient}};
use dioxus::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub ollama_client: OllamaClient,
    pub model: String,
    pub messages: Signal<Vec<ChatRoleMessage>>
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
                    spawn(async move {
                        let model = use_context::<AppState>().model;
                        let mut current_messages = use_context::<AppState>().messages.read().clone();
                        let res = use_context::<AppState>().ollama_client.chat(name.to_string(), model.as_str(), &mut current_messages).await;

                        received_output.set(format!("{:?}", current_messages));
                        use_context::<AppState>().messages.set(current_messages.to_vec());
                        //received_output.set(format!("{:?}", res.unwrap().json::<ChatResponseBody>().await.unwrap().message.content));
                    });
                }},
                "Enter"
            }
            OutputBox { output: received_output}
        }
    }
}