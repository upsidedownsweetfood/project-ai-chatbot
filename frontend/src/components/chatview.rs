use crate::{
    components::output_box::OutputBox,
    utils::ollama_stuff::{ChatResponseBody, ChatRoleMessage},
    utils::tts::say_message,
    AppState,
};
use dioxus::prelude::*;

#[component]
pub fn ChatView() -> Element {
    let mut chat_input = use_signal(|| String::new());
    let mut received_output = use_signal(|| String::new());
    let mut messages = use_signal(|| Vec::<ChatRoleMessage>::new());

    messages.push(ChatRoleMessage {
        role: "system".into(),
        content:
            "Tu sei un assistente di bancasella aiuta il cliente mostrando grandissima deferenza"
                .into(),
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
                        let response = res.json::<ChatResponseBody>().await.unwrap().message.content;
                        received_output.set(format!("{:?}", response.clone()));
                        say_message(response.to_string()).expect("Failed to say hello");


                        messages.set(msgs);
                    });
                },
                "Enter"
            }
            OutputBox { output: received_output }
        }
    }
}
