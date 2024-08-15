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
    let mut listener = use_document_click_listener();
    use_hook(move || {
        let cur_scope = current_scope_id().unwrap();
        let rt = Runtime::current().unwrap();
        listener.add_listener(Box::new(move |id: Option<String>| {
            rt.on_scope(cur_scope, || {
                debug!("Updating resources!!");
                debug!("{:?}", id);
                if id == None {
                    debug!("id is None {:?}", id);
                    if *show.read() == true {
                        debug!("is Opened");
                        show.toggle();
                    }
                }
            })
        }));
    });

    let cb = use_callback(move || {
        debug!("use_callback");
    });


    // let cb = {
    //     let mut show = show.clone();
    //     let callback_uid = uid_string.clone();
    // };

    // Оборачиваем вызов колбэка в замыкание, которое передается в `use_document_click_listener`
    // use_document_click_listener({
    //     move |id| {
    //         debug!("handler {:?}", id);
    //         // use_callback(move || {
    //         //     debug!("use_callback {:?}", id);
    //         // }).call(); // этот код работает
    //         cb.call();
    //         //state.handler(id).call(); // вот этот код вызывает панику
    //     }
    // });




    let on_click = move |_| {
        debug!("on click");
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