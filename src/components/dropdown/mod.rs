use dioxus::prelude::*;
use wasm_bindgen::JsValue;
use crate::components::button::Button;
use crate::components::DocumentClick;
use web_sys::{console, EventTarget};
use uuid::Uuid;

#[derive(PartialEq, Clone, Props)]
pub struct DropdownProps {
    children: Element
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let uid = Uuid::new_v4().to_string();
    let click = use_context::<DocumentClick>();
    let mut show = use_signal(|| false);
    let mut class = String::from("dropdown-menu");
    if show() {
        class.push_str(" show");
    }

    let on_click = move |_| {
        console::log_1(&JsValue::from_str("button clicked!".to_string().as_str()));
        show.toggle();
        let r = click.target.read();
        match r.as_ref() {
            None => {}
            Some(s) => {
                console::log_1(&JsValue::from_str(s.as_str()));
            }
        }
    };
    rsx! {
        div{
            class: "dropdown",
            Button{
                {props.children},
                class: "dropdown-toggle",
                on_click,
                id: uid
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