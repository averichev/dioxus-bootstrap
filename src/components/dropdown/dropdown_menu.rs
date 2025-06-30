use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct DropdownMenuProps {
    show: Option<bool>,
    children: Option<Element>,
}

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let mut class = String::from("dropdown-menu");
    match props.show {
        None => {
            class.push_str(" show");
        }
        Some(s) => {
            if s {
                class.push_str(" show");
            }
        }
    }
    rsx! {
        ul { class, {props.children} }
    }
}