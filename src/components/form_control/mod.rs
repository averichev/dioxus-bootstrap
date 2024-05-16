use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct FormControlProps {
    r#type: Option<FormControlType>,
    size: Option<FormControlSize>,
    placeholder: Option<String>,
}


#[component]
pub fn FormControl(props: FormControlProps) -> Element {
    rsx! {
        input {
            r#type: get_type(props.r#type),
            class: get_class(props.size),
            placeholder: props.placeholder
        }
    }
}

fn get_class(control_size: Option<FormControlSize>) -> String {
    let mut class = String::from("form-control");
    let t = match control_size {
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