use dioxus::html::*;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
    variant: Option<ButtonVariant>,
    children: Element,
    on_click: Option<EventHandler<MouseEvent>>,
    class: Option<String>,
    id: Option<String>,
    #[props(default = false)]
    disabled: bool,
    #[props(default = false)]
    processing: Option<bool>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let mut class = String::from("btn");
    match props.class {
        None => {}
        Some(nc) => {
            class.push_str(" ");
            class.push_str(nc.as_str());
        }
    }
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
    // let on_click = move |evt: MouseEvent| {
    //     match props.on_click {
    //         None => {
    //             evt.stop_propagation()
    //         }
    //         Some(handler) => {
    //             handler.call(evt)
    //         }
    //     }
    // };

    let spinner = match props.processing {
        Some(true) => {
            rsx! {
                span{
                    class:"spinner-border spinner-border-sm me-1",
                    "aria-hidden":"true"
                }
                span{
                    "role": "status",
                    "Loading..."
                }
            }
        }
        _ => {
            rsx! {
                {props.children}
            }
        }
    };

    rsx! {
        button{
            id: props.id,
            class,
            //onclick: on_click,
            r#type: "submit",
            disabled: props.disabled,
            {spinner}
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Link,
}