use crate::models::clicked::ClickListeners;
use dioxus::prelude::*;
use once_cell::unsync::OnceCell;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, EventTarget, MouseEvent};

// Глобальная переменная для хранения состояния инициализации
thread_local! {
 static LISTENER_INITIALIZED: OnceCell<ClickListeners> = OnceCell::new();
}

fn initialize_document_click_listener(clicked: ClickListeners) {
    let closure = Closure::wrap(Box::new(move |evt: MouseEvent| {
        let target = evt.target().and_then(|t| t.dyn_into::<EventTarget>().ok());
        let id = target.and_then(|t| t.dyn_into::<web_sys::Element>().ok()?.get_attribute("id"));
        for callback in clicked.id().borrow_mut().iter_mut() {
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

pub fn use_document_click_listener() -> ClickListeners {
    let mut clicked = LISTENER_INITIALIZED.with(|l| {
        l.get_or_init(|| {
            let clicked = ClickListeners::new();
            initialize_document_click_listener(clicked.clone());
            clicked
        }).clone()
    });
    use_drop(move || {});
    clicked
}