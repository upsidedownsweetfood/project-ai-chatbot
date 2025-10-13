use dioxus::prelude::*;
use gloo_timers;

#[derive(Clone, PartialEq)]
struct ChatMessage {
    id: usize,
    content: String,
    is_user: bool,
}

#[derive(Clone, PartialEq)]
enum SynthState {
    Idle,
    Speaking,
}

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    static CSS: Asset = asset!("/assets/main.css");
    let mut messages = use_signal(|| {
        vec![ChatMessage {
            id: 0,
            content: "Ciao! Sono il tuo assistente AI integrato.".to_string(),
            is_user: false,
        }]
    });
    let mut input_value = use_signal(|| "".to_string());
    let mut synth_state = use_signal(|| SynthState::Idle);
    let mut next_id = use_signal(|| 1);

    let mut handle_send = move || {
        let text = input_value();
        if text.trim().is_empty() {
            return;
        }
        messages.write().push(ChatMessage {
            id: next_id(),
            content: text.clone(),
            is_user: true,
        });
        next_id += 1;
        input_value.set("".to_string());
        synth_state.set(SynthState::Speaking);
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(10_000).await;
            messages.write().push(ChatMessage {
                id: next_id(),
                content: format!("Ho ricevuto: \"{}\". Sto analizzando la richiesta...", text,),
                is_user: false,
            });
            next_id += 1;
            synth_state.set(SynthState::Idle);
        });
    };

    rsx! {
        document::Stylesheet { href: CSS }
        main { class: "main-container",
            section { class: "split-panel chat-panel",
                div { class: "chat-history",
                    {
                        messages()
                            .iter()
                            .map(|msg| {
                                let wrapper_class = if msg.is_user {
                                    "message-wrapper user"
                                } else {
                                    "message-wrapper bot"
                                };
                                rsx! {
                                    div { key: "{msg.id}", class: "{wrapper_class}",
                                        div { class: "message-bubble", "{msg.content}" }
                                    }
                                }
                            })
                    }
                }
                div { class: "input-area",
                    input {
                        r#type: "text",
                        placeholder: "Scrivi qui...",
                        value: "{input_value}",
                        oninput: move |evt| input_value.set(evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter {
                                handle_send();
                            }
                        },
                    }

                    button {
                        class: "send-btn",
                        onclick: move |_| {
                            handle_send();
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "20",
                            height: "20",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line {
                                x1: "22",
                                y1: "2",
                                x2: "11",
                                y2: "13",
                            }
                            polygon { points: "22 2 15 22 11 13 2 9 22 2" }
                        }
                    }
                }
            }

            section { class: "split-panel synth-panel",
                SynthVisualizer { state: synth_state() }
            }
        }
    }
}

#[component]
fn SynthVisualizer(state: SynthState) -> Element {
    let state_class = match state {
        SynthState::Idle => "idle",
        SynthState::Speaking => "speaking",
    };

    rsx! {
        div { class: "orb-container",
            div { class: "orb {state_class}" }
        }
    }
}
