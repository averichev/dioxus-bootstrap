pub mod button;
pub mod dropdown;


use std::cell::Ref;
use dioxus::core_macro::component;
use dioxus::hooks::use_effect;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, EventTarget};
use dioxus::html::*;
use uuid::Uuid;

#[modx::store]
pub struct DocumentClick {
    pub(crate) target: Option<String>,
}

impl DocumentClick {
    pub(crate) fn set(&mut self, target: String){
        self.target.set(Some(target));
    }
    pub(crate) fn clear(&mut self){
        self.target.set(None);
    }
    pub(crate) fn value(&self) -> String {
        match self.target.as_ref() {
            None => {
                String::new()
            }
            Some(n) => {
                n.to_string()
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct DocumentListenerProps {
    children: Element,
}

#[component]
pub fn DocumentListener(props: DocumentListenerProps) -> Element {
    let mut store = DocumentClick::new();
    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let closure = Closure::wrap(Box::new(move |evt: web_sys::MouseEvent| {
            console::log_1(&"document clicked!".into());
            console::log_1(evt.as_ref());
            let target = evt.target().unwrap();
            let element = target.dyn_into::<web_sys::Element>().unwrap();
            if let Some(data_id) = element.get_attribute("id") {
                store.set(data_id);
            } else {
                store.clear();
            }
        }) as Box<dyn FnMut(_)>);
    });
    rsx! {
        {props.children}
    }
}
