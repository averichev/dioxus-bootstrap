pub mod nav_item;

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct NavProps {
    alignment: Option<NavAlignment>,
    current_route: NavigationTarget,
    items: Vec<crate::components::nav::nav_item::NavItem>,
}

#[component]
pub fn Nav(props: NavProps) -> Element {
    let mut class = String::from("nav");
    match props.alignment {
        Some(a) => {
            match a {
                NavAlignment::Horizontal(h) => {
                    match h {
                        NavHorizontalAlignment::Center => {
                            class.push_str(" justify-content-center");
                        }
                        NavHorizontalAlignment::End => {
                            class.push_str(" justify-content-end");
                        }
                        _ => {}
                    }
                }
                NavAlignment::Vertical => {
                    class.push_str(" flex-column")
                }
            }
        }
        _ => {}
    }
    let items = props.items.iter().map(|item| {
        let item_clone = item.clone();
        rsx! {
            crate::components::nav::nav_item::NavItem { to: item_clone.to, {item_clone.text} }
        }
    });
    rsx! {
        ul { class, {items} }
    }
}

#[derive(PartialEq, Clone)]
pub enum NavAlignment {
    Horizontal(NavHorizontalAlignment),
    Vertical,
}

#[derive(PartialEq, Clone)]
pub enum NavHorizontalAlignment {
    Start,
    Center,
    End,
}