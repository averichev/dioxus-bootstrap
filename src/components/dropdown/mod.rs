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
    let uid = use_memo(Uuid::new_v4);

    let mut show = use_signal(|| false);
    let mut class = String::from("dropdown-menu");
    if show() {
        class.push_str(" show");
    }

    let store = DocumentClick::new();


    //let click = use_context::<DocumentClick>().target;
    // use_effect(move || {
    //     let r = click.read();
    //     console::log_1(&JsValue::from_str(&format!("Detected change, new clicked ID: {:?}", r.target.read())));
    //     match r.target.as_ref() {
    //         None => {
    //             console::log_1(&JsValue::from_str("нет '"));
    //             //show.set(false);
    //         }
    //         Some(s) => {
    //             console::log_1(&JsValue::from_str("есть id"));
    //             console::log_1(&JsValue::from_str(s.as_str()));
    //             /*if uid.to_string().eq(s.rea) {
    //                 console::log_1(&JsValue::from_str("равны"));
    //             }*/
    //         }
    //     };
    // });



    let on_click = move |_| {
        console::log_1(&JsValue::from_str("button clicked!".to_string().as_str()));
        show.toggle();
    };
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
        div{
            "value: {store.value()}"
        }
    }
}