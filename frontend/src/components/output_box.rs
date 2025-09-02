use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct OutputBoxProps {
    output: String,
}

#[component]
pub fn OutputBox(props: OutputBoxProps) -> Element {
    rsx! {
        div {
            class: "OutputBox",
            p {{props.output}}
        }
    }
}