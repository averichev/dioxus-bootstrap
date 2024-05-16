use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct DropdownMenuProps {
    show: Option<bool>,
}

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let mut class = String::from("dropdown-menu");
    match props.show {
        None => {}
        Some(s) => {
            if s {
                class.push_str(" show");
            }
        }
    }
    rsx! {
        ul{
            class,
            li{
                a{
                    class: "dropdown-item",
                    "item"
                }
            }
        },
    }
}