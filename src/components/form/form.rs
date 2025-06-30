use dioxus::prelude::*;
use tracing::debug;

#[derive(PartialEq, Clone, Props)]
pub struct FormProps {
    children: Option<Element>,
    onsubmit: EventHandler<FormEvent>,
}

#[component]
pub fn Form(props: FormProps) -> Element {
    rsx! {
        form {
            onsubmit: move |event| {
                debug!("Submitted! {event:?}");
                debug!("{:?}", event.data());
                props.onsubmit.call(event)
            },
            {props.children}
        }
    }
}