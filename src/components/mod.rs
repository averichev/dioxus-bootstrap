pub mod button;
pub mod dropdown;


use dioxus::core_macro::component;
use dioxus::hooks::use_effect;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, EventTarget};
use dioxus::html::*;


pub(crate) struct DocumentClick {
    target: Option<EventTarget>
}

#[component]
pub fn DocumentListener() -> Element {
    let mut click = use_context_provider(|| Signal::new(DocumentClick{ target: None }));
    let document_click = use_context::<Signal<DocumentClick>>();
    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let closure = Closure::wrap(Box::new(move |evt: web_sys::MouseEvent| {
            console::log_1(&"Clicked  !".into());
            let target = evt.target().unwrap();
            console::log_1(target.as_ref());
            click.write().target = Some(target);
        }) as Box<dyn FnMut(_)>);

        document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    });
    rsx!{}
}
