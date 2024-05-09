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
    pub target: String,
}

impl DocumentClick {
    pub fn set(&mut self, target: String){
        console::log_1(&"set id in store".into());
        self.target.set(target);
    }
    pub fn clear(&mut self){
        console::log_1(&"clear".into());
        self.target.set(String::new());
    }
    pub fn value(&mut self) -> Signal<String> {
        console::log_1(&"read value".into());
        self.target
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct DocumentListenerProps {
    children: Element,
}


#[derive(Clone, Copy)]
pub struct DarkMode(pub bool);

#[derive(Clone)]
pub struct Clicked{
    id: String
}

#[component]
pub fn DocumentListener(props: DocumentListenerProps) -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));
    let mut clicked = use_context_provider(|| Signal::new(Clicked{ id: "".to_string() }));
    let mut store = DocumentClick::new();
    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let closure = Closure::wrap(Box::new(move |evt: web_sys::MouseEvent| {
            console::log_1(&"document clicked!".into());
            console::log_1(evt.as_ref());
            let target = evt.target().unwrap();
            let element = target.dyn_into::<web_sys::Element>().unwrap();
            if let Some(data_id) = element.get_attribute("id") {
                store.set(data_id.clone());
                clicked.write().id = data_id;
            } else {
                store.clear();
                clicked.write().id = String::new();
            }
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    });
    use_effect(move || {
        console::log_1(&JsValue::from_str(&format!("Detected change in DocumentListener, new clicked ID: {:?}", store.value())));
    });
    rsx! {
        {props.children},
        div{
            "store.value: {store.value()}"
        }
    }
}
