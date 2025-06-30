use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct RowProps {
    children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    rsx! {
        div { class: "row", {props.children} }
    }
}