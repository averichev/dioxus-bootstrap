use dioxus::html::*;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
    variant: Option<ButtonVariant>,
    children: Element,
    on_click: EventHandler<MouseEvent>
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let mut class = String::from("btn");
    match props.variant {
        None => {}
        Some(variant) => {
            match variant {
                ButtonVariant::Primary => {
                    class.push_str(" btn-primary")
                }
                ButtonVariant::Success => {
                    class.push_str(" btn-success")
                }
                ButtonVariant::Secondary => {
                    class.push_str(" btn-secondary")
                }
                ButtonVariant::Danger => {
                    class.push_str(" btn-danger")
                }
                ButtonVariant::Warning => {
                    class.push_str(" btn-warning")
                }
                ButtonVariant::Info => {
                    class.push_str(" btn-info")
                }
                ButtonVariant::Light => {
                    class.push_str(" btn-light")
                }
                ButtonVariant::Dark => {
                    class.push_str(" btn-dark")
                }
                ButtonVariant::Link => {
                    class.push_str(" btn-link")
                }
            }
        }
    }
    rsx! {
        button{
            class,
            onclick: move |evt| props.on_click.call(evt),
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum ButtonVariant{
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Link
}