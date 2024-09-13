use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct NavItemProps {
    children: Option<Element>,
    #[props(default = false)]
    disabled: bool,
    to: IntoRoutable
}

#[component]
pub fn NavItem(props: NavItemProps) -> Element {
    let mut link_class = String::from("nav-link");
    match props.disabled {
        true => link_class.push_str(" disabled"),
        _ => {}
    };
    rsx! {
        li{
            class: "nav-item",
            Link{
                class: link_class,
                to: props.to,
                {props.children}
            }
        }
    }
}