use dioxus::html::*;
use dioxus::prelude::*;
use crate::components::button::Button;

#[derive(PartialEq, Clone, Props)]
pub struct DropdownProps {
    children: Element
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let mut show = use_signal(|| false);
    let mut class = String::from("dropdown-menu");
    if show() {
        class.push_str(" show");
    }
    rsx! {
        div{
            class: "dropdown",
            Button{
                {props.children},
                class: "dropdown-toggle",
                on_click: move |_| show.toggle()
            },
            ul{
                class,
                li{
                    a{
                        class: "dropdown-item",
                        "item"
                    }
                }
            }
        }
    }
}