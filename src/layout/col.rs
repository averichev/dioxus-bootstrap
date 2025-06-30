use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ColProps {
    children: Element,
    class: Option<String>,
}

#[component]
pub fn Col(props: ColProps) -> Element {
    let mut class = String::from("col");
    match props.class {
        Some(cls) => {
            class.push_str(" ");
            class.push_str(cls.as_str());
        }
        _ => {}
    };
    rsx! {
        div { class, {props.children} }
    }
}
