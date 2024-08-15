use dioxus::prelude::*;
use crate::hooks::document_click_listener::use_document_click_listener;

pub mod document_click_listener;
pub mod uid_generator;

pub fn use_autoclose(mut show: Signal<bool>, uid: String) {
    let mut listener = use_document_click_listener();
    use_hook(move || {
        let cur_scope = current_scope_id().unwrap();
        let rt = Runtime::current().unwrap();
        listener.add_listener(Box::new(move |id: Option<String>| {
            rt.on_scope(cur_scope, || {
                match id {
                    None => {
                        if *show.read() == true {
                            *show.write() = false;
                        }
                    }
                    Some(s) => {
                        if uid != s {
                            *show.write() = false;
                        }
                    }
                }
            })
        }));
    })
}