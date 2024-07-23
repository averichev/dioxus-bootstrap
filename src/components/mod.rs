pub mod button;
pub mod dropdown;
pub mod form;


use dioxus::core_macro::component;
use dioxus::hooks::use_effect;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(PartialEq, Clone, Props)]
pub struct DocumentListenerProps {
    children: Element,
}

#[derive(Clone, Copy)]
pub struct DarkMode(pub bool);

#[derive(Clone)]
pub struct Clicked {
    id: String,
}

#[component]
pub fn DocumentListener(props: DocumentListenerProps) -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));
    let mut clicked = use_context_provider(|| Signal::new(Clicked { id: "".to_string() }));
    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let closure = Closure::wrap(Box::new(move |evt: web_sys::MouseEvent| {
            let target = evt.target().unwrap();
            let element = target.dyn_into::<web_sys::Element>().unwrap();
            if let Some(data_id) = element.get_attribute("id") {
                clicked.write().id = data_id;
            } else {
                clicked.write().id = String::new();
            }
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    });
    rsx! {
        {props.children},
    }
}
