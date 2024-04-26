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
    //let click = consume_context::<DocumentClick>();
    let uid = Uuid::new_v4().to_string();
    let click = use_context::<Signal<DocumentClick>>();
    let mut show = use_signal(|| false);
    let mut class = String::from("dropdown-menu");
    if show() {
        class.push_str(" show");
    }

    let on_click = move |_| {
        console::log_1(&JsValue::from_str("button clicked!".to_string().as_str()));
        show.toggle();
        let r = click.read().target.read().clone();
        match r {
            None => {}
            Some(s) => {
                console::log_1(&s);
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