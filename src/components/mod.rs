pub mod button;
pub mod dropdown;
pub mod form;

use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicUsize;
use dioxus::core_macro::component;
use dioxus::hooks::use_effect;
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, Level};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, window};

#[derive(PartialEq, Clone, Props)]
pub struct DocumentListenerProps {
    children: Element,
}

#[derive(Clone, Copy)]
pub struct DarkMode(pub bool);

#[derive(Clone)]
pub struct Clicked {
    id: Option<String>,
}

impl Clicked {
    pub(crate) fn set_id(&mut self, id: Option<String>) {
        debug!("set_id, {:?}", id);
        self.id = id;
    }
}

static SUBSCRIPTIONS: Lazy<Arc<RwLock<Clicked>>> = Lazy::new(|| {
    let closure = Closure::wrap(Box::new(move |evt: web_sys::MouseEvent| {
        let target = evt.target().unwrap();
        let element = target.dyn_into::<web_sys::Element>().unwrap();
        let id = element.get_attribute("id");
        let b = SUBSCRIPTIONS.write().unwrap().set_id(id);
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);
    window()
        .unwrap()
        .document()
        .unwrap()
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
    Arc::new(RwLock::new(Clicked { id: None }))
});

#[component]
pub fn DocumentListener(props: DocumentListenerProps) -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));
    let mut clicked = use_context_provider(|| Signal::new(Clicked { id: None }));
    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let closure = Closure::wrap(Box::new(move |evt: web_sys::MouseEvent| {
            let target = evt.target().unwrap();
            let element = target.dyn_into::<web_sys::Element>().unwrap();
            if let Some(data_id) = element.get_attribute("id") {
                clicked.write().id = Some(data_id);
            } else {
                clicked.write().id = None;
            }
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    });
    rsx! {
        {props.children},
    }
}

pub fn use_shortcut() {
    {
        use_hook(move || {
            SUBSCRIPTIONS.write().unwrap().set_id(None)
        });
        use_drop(move || {
            SUBSCRIPTIONS.write().unwrap().set_id(None)
        })
    }
}