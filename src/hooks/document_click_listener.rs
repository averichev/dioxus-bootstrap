use dioxus::dioxus_core::{use_hook};
use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use once_cell::unsync::OnceCell;
use tracing::debug;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, MouseEvent, window};
use crate::models::clicked::Clicked;

// Глобальная переменная для хранения состояния инициализации
thread_local! {
 static LISTENER_INITIALIZED: OnceCell<Clicked> = OnceCell::new();
}

fn initialize_document_click_listener(clicked: Clicked) {
    debug!("initialize_document_click_listener");
    let closure = Closure::wrap(Box::new(move |evt: MouseEvent| {
        let target = evt.target().and_then(|t| t.dyn_into::<EventTarget>().ok());
        let id = target.and_then(|t| t.dyn_into::<web_sys::Element>().ok()?.get_attribute("id"));

        for callback in clicked.id().borrow_mut().iter_mut() {
            debug!("callback {:?}", id);
            callback(id.clone());
        }
    }) as Box<dyn FnMut(_)>);

    window()
        .unwrap()
        .document()
        .unwrap()
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

pub fn use_document_click_listener(handler: impl FnMut(Option<String>) + 'static) {
    let mut clicked = LISTENER_INITIALIZED.with(|l| {
        l.get_or_init(|| {
            let clicked = Clicked::new();
            initialize_document_click_listener(clicked.clone());
            clicked
        }).clone()
    });

    use_hook(|| {
        debug!("use_hook");
        clicked.push(Box::new(handler));
    });

    use_drop(move || {});
}
