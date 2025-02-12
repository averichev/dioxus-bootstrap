#![allow(non_snake_case)]

use dioxus::document;
use dioxus::hooks::use_resource;

pub mod components;
pub mod hooks;
pub mod layout;
pub mod models;

pub mod prelude {
    pub use crate::components::button::*;
    pub use crate::components::dropdown::*;
    pub use crate::components::form::form::*;
    pub use crate::components::form::form_control::*;
    pub use crate::components::form::form_field::*;
    pub use crate::components::form::invalid_feedback::*;
    pub use crate::components::form::*;
    pub use crate::components::nav::nav_item::*;
    pub use crate::components::nav::*;
    pub use crate::hooks::uid_generator::*;
    pub use crate::layout::col::*;
    pub use crate::layout::row::*;
    pub use crate::layout::Container;
}

pub fn update_body_class(class_name: &str) {
    let class_name = class_name.to_string();
    let _ = use_resource(move || {
        let value = class_name.clone();
        async move {
            let eval = document::eval(
                r#"
            let msg = await dioxus.recv();
            document.body.className = msg;
            "#,
            );
            eval.send(&value).unwrap();
        }
    });
}


pub fn update_main_class(class_name: &str) {
    let class_name = class_name.to_string();
    let _ = use_resource(move || {
        let value = class_name.clone();
        async move {
            let eval = document::eval(
                r#"
                let msg = await dioxus.recv();
                const element = document.getElementById("main");
                if (element) {
                    element.className = msg;
                }
            "#,
            );
            eval.send(&value).unwrap();
        }
    });
}

pub fn update_html_class(class_name: &str) {
    let class_name = class_name.to_string();
    let _ = use_resource(move || {
        let value = class_name.clone();
        async move {
            let eval = document::eval(
                r#"
                let msg = await dioxus.recv();
                const htmlElement = document.documentElement; // Получаем тег <html>
                if (htmlElement) {
                    htmlElement.className = msg;
                }
            "#,
            );
            eval.send(&value).unwrap();
        }
    });
}
