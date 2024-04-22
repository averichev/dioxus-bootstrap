pub mod row;
pub mod col;

use dioxus::html::*;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ContainerProps {
    children: Element
}
#[component]
pub fn Container(props: ContainerProps) -> Element {
    rsx!{
        div{
            class: "container",
            {props.children}
        }
    }
}