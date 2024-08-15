pub mod dropdown_menu;

use dioxus::prelude::*;
use tracing::debug;
use crate::components::button::Button;
use uuid::Uuid;
use crate::components::dropdown::dropdown_menu::DropdownMenu;
use crate::hooks::document_click_listener::use_document_click_listener;
use crate::hooks::uid_generator::use_uid_generator;

#[derive(PartialEq, Clone, Props)]
pub struct DropdownProps {
    children: Element,
    menu: Option<Element>,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let uid = use_uid_generator();
    let mut show = use_signal(|| false);
    let uid_string = uid.to_string();
    let uid_string_func = uid_string.clone();
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
                        if uid_string_func != s {
                            *show.write() = false;
                        }
                    }
                }
            })
        }));
    });
    let on_click = move |_| {
        show.toggle();
    };

    rsx! {
        div{
            class: "dropdown",
            Button{
                class: "dropdown-toggle",
                on_click,
                id: uid_string,
                {props.children}
            }
            DropdownMenu{
                children: props.menu,
                show: Some(*show.read())
            }
        }
    }
}