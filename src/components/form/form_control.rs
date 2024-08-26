use dioxus::prelude::*;
use crate::prelude::*;
use tracing::debug;

#[derive(Clone, PartialEq, Props)]
pub struct FormControlProps {
    r#type: Option<FormControlType>,
    size: Option<FormControlSize>,
    placeholder: Option<String>,
    id: Option<String>,
    name: Option<String>,
    oninput: EventHandler<FormEvent>,
    errors: Option<Vec<FormControlError>>,
    #[props(default = false)]
    readonly: bool,
}


#[component]
pub fn FormControl(props: FormControlProps) -> Element {
    let data = use_signal(|| "".to_string());
    let mut touched = use_signal(|| false);
    let is_invalid = match props.errors {
        None => {
            false
        }
        Some(ref v) => {
            if *touched.read() == false {
                false
            }
            else {
                v.len() > 0
            }
        }
    };
    rsx! {
        input {
            r#type: get_type(props.r#type),
            class: get_class(props.size, is_invalid),
            placeholder: props.placeholder,
            id: props.id,
            name: props.name,
            readonly: props.readonly,
            oninput: move |event| {
                touched.set(true);
                props.oninput.call(event);
            }
        }
        InvalidFeedback{
            errors: props.errors
        }
        div{
            {data}
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum FormControlType {
    Text,
    Checkbox,
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
    else {
        class.push_str(" ")
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
pub enum FormControlSize {
    Sm,
    Default,
    Lg,
}

#[derive(PartialEq, Clone, Debug)]
pub struct FormControlError {
    pub code: String,
    pub message: String,
}