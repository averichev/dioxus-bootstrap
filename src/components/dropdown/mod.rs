pub mod dropdown_menu;

use dioxus::prelude::*;
use serde_json::Value::String;
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
    let uid = use_re(|| Uuid::new_v4().to_string());
    rsx! {
        div{
            class: "dropdown",
            Button{
                class: "dropdown-toggle",
                id: uid.to_string(),
                {props.children}
            }
            DropdownMenu{
                children: props.menu
            }
        }
    }
}

