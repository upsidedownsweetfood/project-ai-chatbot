use crate::{
    components::output_box::OutputBox,
    utils::ollama_stuff::{ChatResponseBody, ChatRoleMessage, OllamaClient},
};
use dioxus::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub ollama_client: OllamaClient,
    pub model: String,
}

#[component]
pub fn Hero() -> Element {
    let mut chat_input = use_signal(|| String::new());
    let mut received_output = use_signal(|| String::new());
    let mut messages = use_signal(|| Vec::<ChatRoleMessage>::new());

    messages.push(ChatRoleMessage {
        role: "system".into(),
        content: "Tu sei un assistente di bancasella aiuta il cliente mostrando grandissima deferenza".into(),
    });

    let app_state = use_context::<AppState>();

    rsx! {
        div {
            input {
                class: "input",
                placeholder: "...",
                oninput: move |event| chat_input.set(event.value())
            }
            button {
                onclick: move |_| {
                    let model = app_state.model.clone();
                    let ollama_client = app_state.ollama_client.clone();

                    spawn(async move {
                        let mut msgs = messages.read().clone();
                        let res = ollama_client.chat(chat_input.to_string(), &model, &mut msgs).await.unwrap();
                        received_output.set(format!("{:?}", res.json::<ChatResponseBody>().await.unwrap().message.content));

                        messages.set(msgs);
                    });
                },
                "Enter"
            }
            OutputBox { output: received_output }
        }
    }
}
