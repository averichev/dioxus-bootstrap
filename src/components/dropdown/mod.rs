use dioxus::prelude::*;
use wasm_bindgen::JsValue;
use crate::components::button::Button;
use crate::components::{Clicked, DarkMode};
use web_sys::{console, EventTarget};
use uuid::Uuid;

#[derive(PartialEq, Clone, Props)]
pub struct DropdownProps {
    children: Element,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let uid = use_memo(Uuid::new_v4);
    let mut show = use_signal(|| false);
    let mut class = String::from("dropdown-menu");
    if show() {
        class.push_str(" show");
    }
    let dark_mode = use_context::<Signal<DarkMode>>();
    let clicked = use_context::<Signal<Clicked>>();
    use_effect(move || {
        if uid.to_string() == clicked().id {} else {
            show.set(false);
        }
    });
    let on_click = move |_| {
        show.toggle();
    };
    let id_clicked = clicked().id;
    rsx! {
        div{
            class: "dropdown",
            Button{
                {props.children},
                class: "dropdown-toggle",
                on_click,
                id: uid.to_string()
            },
            ul{
                class,
                li{
                    a{
                        class: "dropdown-item",
                        "item"
                    }
                }
            },
        },
    }
}