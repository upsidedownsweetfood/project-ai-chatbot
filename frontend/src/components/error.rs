use dioxus::prelude::*;

#[component]
pub fn Error(err: String) -> Element {
    rsx! {
        p {{err}}
    }
}