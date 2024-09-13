pub mod nav_item;

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct NavProps {
    children: Option<Element>,
    alignment: Option<NavAlignment>,
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
    rsx! {
        ul{
            class,
            {props.children}
        }
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