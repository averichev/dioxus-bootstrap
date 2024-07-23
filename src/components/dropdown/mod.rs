pub mod dropdown_menu;

use dioxus::prelude::*;
use crate::components::button::Button;
use crate::components::{Clicked, DarkMode};
use uuid::Uuid;
use crate::components::dropdown::dropdown_menu::DropdownMenu;

#[derive(PartialEq, Clone, Props)]
pub struct DropdownProps {
    children: Element,
    menu: Option<Element>
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let uid = use_memo(Uuid::new_v4);
    let mut show = use_signal(|| false);

    let _dark_mode = use_context::<Signal<DarkMode>>();
    let clicked = use_context::<Signal<Clicked>>();
    use_effect(move || {
        if uid.to_string() == clicked().id {} else {
            show.set(false);
        }
    });
    let on_click = move |_| {
        show.toggle();
    };
    let _id_clicked = clicked().id;
    rsx! {
        div{
            class: "dropdown",
            Button{
                class: "dropdown-toggle",
                on_click,
                id: uid.to_string(),
                {props.children}
            }
            DropdownMenu{
                show: Some(*show.read()),
                children: props.menu
            }
        },
    }
}

