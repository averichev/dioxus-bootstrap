pub mod col;
pub mod row;

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ContainerProps {
    children: Element,
    breakpoint: Option<ContainerBreakpoint>,
    class: Option<String>,
}
#[component]
pub fn Container(props: ContainerProps) -> Element {
    let mut class = "container".to_string();
    match props.breakpoint {
        Some(n) => {
            class.push_str("-");
            match n {
                ContainerBreakpoint::Sm => {
                    class.push_str("sm");
                }
                ContainerBreakpoint::Md => {
                    class.push_str("md");
                }
                ContainerBreakpoint::Lg => {
                    class.push_str("lg");
                }
                ContainerBreakpoint::Xl => {
                    class.push_str("xl");
                }
                ContainerBreakpoint::Xxl => {
                    class.push_str("xxl");
                }
                ContainerBreakpoint::Fluid => {
                    class.push_str("fluid");
                }
            }
        }
        _ => {}
    };
    match props.class {
        None => {}
        Some(s) => {
            class.push_str(" ");
            class.push_str(&s);
        }
    }
    rsx! {
        div{
            class,
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum ContainerBreakpoint {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Fluid,
}
