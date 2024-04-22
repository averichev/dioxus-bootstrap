use dioxus::html::*;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ColProps {
    children: Element,
}

#[component]
pub fn Col(props: ColProps) -> Element {
    rsx! {
        div{
            class: "col",
            {props.children}
        }
    }
}