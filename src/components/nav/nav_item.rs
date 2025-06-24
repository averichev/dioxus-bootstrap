use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct NavItemProps {
    children: Option<Element>,
    #[props(default = false)]
    disabled: bool,
    to: NavigationTarget,
}

#[derive(PartialEq, Clone)]
pub struct NavItem {
    pub text: String,
    pub to: NavigationTarget,
}

#[component]
pub(crate) fn NavItem(props: NavItemProps) -> Element {
    let mut link_class = String::from("nav-link");
    match props.disabled {
        true => link_class.push_str(" disabled"),
        _ => {}
    };
    let route = props.to.clone();
    rsx! {
        li{
            class: link_class,
            Link{
                to: route,
                {props.children}
            }
        }
    }
}