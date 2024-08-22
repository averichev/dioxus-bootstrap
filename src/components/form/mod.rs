use dioxus::prelude::*;
use tracing::debug;
use uuid::Uuid;
use crate::hooks::uid_generator::use_uid_generator;

#[derive(PartialEq, Clone, Props)]
pub struct FormProps {
    children: Option<Element>,
    onsubmit: EventHandler<FormEvent>,
}

#[component]
pub fn Form(props: FormProps) -> Element {
    rsx! {
        form{
            onsubmit: move |event| {
                debug!("Submitted! {event:?}");
                debug!("{:?}", event.data());
                props.onsubmit.call(event)
            },
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct FormFieldProps {
    children: Option<Element>,
    label: Option<String>,
    name: Option<String>,
    oninput: EventHandler<FormEvent>,
    #[props(default = false)]
    is_invalid: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub struct FormFieldError {
    pub code: String,
    pub message: String,
}

#[derive(PartialEq, Clone, Props)]
pub struct InvalidFeedbackProps {
    errors: Vec<FormFieldError>,
}

#[component]
pub fn InvalidFeedback(props: InvalidFeedbackProps) -> Element {
    rsx! {
        div{
            class: "invalid-feedback",
            for error in props.errors {
                div {
                    "data-code": error.code,
                    "{error.message}"
                }
            }
        }
    }
}

#[component]
pub fn FormField(props: FormFieldProps) -> Element {
    let uid = use_uid_generator();
    let label = match props.label {
        None => {
            None
        }
        Some(l) => {
            rsx! {
            label{
                class: "form-label",
                r#for: uid().to_string(),
                {l}
            }
        }
        }
    };
    rsx! {
        div{
            class: "mb-3",
            {label}
            FormControl{
                r#type: FormControlType::Text,
                id: uid().to_string(),
                name: props.name,
                oninput: props.oninput,
                is_invalid: props.is_invalid
            }
        }
    }
}


#[derive(Clone, PartialEq, Props)]
pub struct FormControlProps {
    r#type: Option<FormControlType>,
    size: Option<FormControlSize>,
    placeholder: Option<String>,
    id: Option<String>,
    name: Option<String>,
    oninput: EventHandler<FormEvent>,
    #[props(default = false)]
    is_invalid: bool,
}


#[component]
pub fn FormControl(props: FormControlProps) -> Element {
    let mut data = use_signal(|| "".to_string());
    rsx! {
        input {
            r#type: get_type(props.r#type),
            class: get_class(props.size, props.is_invalid),
            placeholder: props.placeholder,
            id: props.id,
            name: props.name,
            oninput: move |event| props.oninput.call(event)
        }
        div{
            {data}
        }
    }
}

fn get_class(control_size: Option<FormControlSize>, is_invalid: bool) -> String {
    let mut class = String::from("form-control");
    match control_size {
        None => {}
        Some(n) => {
            match n {
                FormControlSize::Sm => {
                    class.push_str(" form-control-sm")
                }
                FormControlSize::Default => {}
                FormControlSize::Lg => {
                    class.push_str(" form-control-lg")
                }
            }
        }
    };

    if is_invalid == true {
        class.push_str(" is-invalid")
    }

    class
}

fn get_type(control_type: Option<FormControlType>) -> String {
    let t = match control_type {
        None => {
            "text"
        }
        Some(n) => {
            match n {
                FormControlType::Text => {
                    "text"
                }
                FormControlType::Checkbox => {
                    "checkbox"
                }
            }
        }
    };
    t.to_string()
}

#[derive(Clone, PartialEq)]
pub enum FormControlType {
    Text,
    Checkbox,
}

#[derive(Clone, PartialEq)]
pub enum FormControlSize {
    Sm,
    Default,
    Lg,
}