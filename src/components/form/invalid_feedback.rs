use dioxus::prelude::*;
use crate::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct InvalidFeedbackProps {
    errors: Option<Vec<FormControlError>>,
}

#[component]
pub fn InvalidFeedback(props: InvalidFeedbackProps) -> Element {
    match props.errors {
        None => {
            None
        }
        Some(errors) => {
            rsx! {
                div{
                    class: "invalid-feedback",
                    for error in errors {
                        div {
                            "data-code": error.code,
                            "{error.message}"
                        }
                    }
                }
            }
        }
    }
}