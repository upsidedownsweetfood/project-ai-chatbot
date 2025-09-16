use crate::{
    components::output_box::OutputBox,
    utils::ollama_stuff::{ChatResponseBody, ChatRoleMessage, OllamaClient},
};
use dioxus::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub ollama_client: OllamaClient,
    pub model: String,
    pub messages: Vec<ChatRoleMessage>,
}

#[component]
pub fn Hero() -> Element {
    let mut name = use_signal(|| String::new());
    let received_output = use_signal(|| String::new());
    let app_state = use_context::<AppState>();

    rsx! {
        div {
            input {
                class: "input",
                placeholder: "Enter your name",
                oninput: move |event| name.set(event.value())
            }
            button {
                onclick: move |_| {
                    let name = name.to_string();
                    let model = app_state.model.clone();
                    let ollama_client = app_state.ollama_client.clone();
                    let mut messages = app_state.messages.clone();
                    let mut received_output = received_output.clone();

                    spawn(async move {
                        let res = ollama_client.chat(name, &model, &mut messages).await;
                        if let Ok(response) = res {
                            if let Ok(body) = response.json::<ChatResponseBody>().await {
                                received_output.set(body.message.content);
                            }
                        }
                    });
                },
                "Enter"
            }
            OutputBox { output: received_output }
        }
    }
}
