use dioxus::prelude::*;
use dioxus::document;
use crate::models::clicked::ClickListeners;
use once_cell::unsync::OnceCell;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, EventTarget, MouseEvent};


thread_local! {
 static LISTENER_INITIALIZED: OnceCell<ClickListeners> = OnceCell::new();
}

#[cfg(target_arch = "wasm32")]
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

#[cfg(not(target_arch = "wasm32"))]
fn initialize_document_click_listener(clicked: ClickListeners) {
    let rt = Runtime::current().unwrap();
    let cur_scope = current_scope_id().unwrap();
    rt.spawn(cur_scope, async move {
        let mut eval = document::eval(
            r#"
                document.addEventListener('click', (evt) => {
                    const target = evt.target;
                    const id = target ? target.id : null;
                    dioxus.send(id);
                });
            "#,
        );
        while let Ok(id) = eval.recv::<Option<String>>().await {
            for callback in clicked.id().borrow_mut().iter_mut() {
                callback(id.clone());
            }
        }
    });
}

pub(crate) fn use_document_click_listener() -> ClickListeners {
    let clicked = LISTENER_INITIALIZED.with(|l| {
        l.get_or_init(|| {
            let clicked = ClickListeners::new();
            initialize_document_click_listener(clicked.clone());
            clicked
        }).clone()
    });
    use_drop(move || {});
    clicked
}