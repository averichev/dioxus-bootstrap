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

    // Создаем мемоизированное замыкание, которое не принимает параметры
    let cb = {
        let mut show = show.clone();
        let callback_uid = uid_string.clone();

        use_callback(move || {
            // Этот код будет выполнен при вызове cb()
            // Вы можете передать параметры через capture
            if let Some(id) = Some(callback_uid.clone()) {
                debug!("Clicked element id: {}", id);
                if id == callback_uid {
                    debug!("id is equal to uid");
                    debug!("id {}", id);
                    debug!("uid {}", callback_uid);
                    debug!("is opened: {}", *show.read());
                    if *show.read() == true {
                        *show.write() = false;
                    }
                } else {
                    debug!("id is NOT equal to uid");
                    debug!("id {}", id);
                    debug!("uid {}", callback_uid);
                    *show.write() = false;
                }
            } else {
                debug!("Clicked on an element without id");
            }
        })
    };

    // Оборачиваем вызов колбэка в замыкание, которое передается в `use_document_click_listener`
    use_document_click_listener(move |id| {
        cb();  // Просто вызываем cb, параметры можно передать через closure capture
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
