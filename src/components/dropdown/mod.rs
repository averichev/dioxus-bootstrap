use dioxus::prelude::*;
use wasm_bindgen::JsValue;
use crate::components::button::Button;
use crate::components::{Clicked, DarkMode, DocumentClick};
use web_sys::{console, EventTarget};
use uuid::Uuid;
use web_sys::js_sys::Atomics::store;

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

    let mut store = DocumentClick::new();
    let dark_mode = use_context::<Signal<DarkMode>>();
    let clicked = use_context::<Signal<Clicked>>();


    use_effect(move || {
        console::log_1(&JsValue::from_str(&format!("Detected change, new clicked ID: {:?}", store.value())));
        if uid.to_string() == clicked().id {
            console::log_1(&JsValue::from_str("равны"));
        }
        else {
            show.set(false);
        }
    });


    let on_click = move |_| {
        console::log_1(&JsValue::from_str("button clicked!".to_string().as_str()));
        ////store.set("fake id".to_string());
        show.toggle();
    };
    let style = if dark_mode().0 { "дарк" } else { "лайт" };
    let style2 = store.target();
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
        div{
            "store value: {style2}"
        },
        div{
            "мод: {style}"
        },
        div{
            "clicked: {id_clicked}"
        }
    }
}