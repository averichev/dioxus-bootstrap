use dioxus::html::*;
use dioxus::prelude::*;
use crate::components::button::Button;
use crate::components::DocumentClick;
use web_sys::{console, EventTarget};

#[derive(PartialEq, Clone, Props)]
pub struct DropdownProps {
    children: Element
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let click = use_context::<Signal<DocumentClick>>();
    let mut show = use_signal(|| false);
    let mut class = String::from("dropdown-menu");
    if show() {
        class.push_str(" show");
    }

    let on_click = move |_| {
        show.toggle();
        console::log_1(click.read().target.as_ref().unwrap());
    };
    rsx! {
        div{
            class: "dropdown",
            Button{
                {props.children},
                class: "dropdown-toggle",
                on_click
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