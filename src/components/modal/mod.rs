use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ModalProps {
    #[props(default = false)]
    opened: bool,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    let mut class = "modal".to_string();
    if props.opened {
        class.push_str(" opened");
    }
    rsx! {
        div { tabindex: "-1", class,
            div { class: "modal-dialog",
                div { class: "modal-content",
                    div { class: "modal-header",
                        h5 { class: "modal-title", "Modal title" }
                        button {
                            r#type: "button",
                            "data-bs-dismiss": "modal",
                            "aria-label": "Close",
                            class: "btn-close",
                        }
                    }
                    div { class: "modal-body",
                        p { "Modal body text goes here." }
                    }
                    div { class: "modal-footer",
                        button {
                            r#type: "button",
                            "data-bs-dismiss": "modal",
                            class: "btn btn-secondary",
                            "Close"
                        }
                        button { r#type: "button", class: "btn btn-primary", "Save changes" }
                    }
                }
            }
        }
    }
}