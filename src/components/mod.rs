pub mod button;
pub mod dropdown;


use dioxus::core_macro::component;
use dioxus::hooks::use_effect;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, EventTarget};
use dioxus::html::*;

#[derive(Clone, Copy, Default)]
pub struct DocumentClick {
    target: Signal<Option<String>>,
}

#[derive(PartialEq, Clone, Props)]
pub struct DocumentListenerProps{
    children: Element
}

#[component]
pub fn DocumentListener(props: DocumentListenerProps) -> Element {
    use_context_provider(|| DocumentClick::default());
    let mut click = use_context::<DocumentClick>();
    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let closure = Closure::wrap(Box::new(move |evt: web_sys::MouseEvent| {
            console::log_1(&"document clicked!".into());
            console::log_1(evt.as_ref());
            let target = evt.target().unwrap();
            let element = target.dyn_into::<web_sys::Element>().unwrap();
            if let Some(data_id) = element.get_attribute("id") {
                *click.target.write() = Some(data_id);
            }
            else {
                *click.target.write() = None;
            }
        }) as Box<dyn FnMut(_)>);

        document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    });
    rsx!{
        {props.children}
    }
}
