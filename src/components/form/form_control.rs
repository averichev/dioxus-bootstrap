use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct FormControlProps {
    r#type: Option<FormControlType>,
    size: Option<FormControlSize>,
    placeholder: Option<String>,
    id: Option<String>,
    name: Option<String>,
    oninput: Option<EventHandler<FormEvent>>,
    errors: Option<Vec<FormControlError>>,
    #[props(default = false)]
    readonly: bool,
}

#[component]
pub fn FormControl(props: FormControlProps) -> Element {
    let data = use_signal(|| "".to_string());
    let mut touched = use_signal(|| false);
    let is_invalid = match props.errors {
        None => false,
        Some(ref v) => {
            if *touched.read() == false {
                false
            } else {
                v.len() > 0
            }
        }
    };

    let props_type = props.r#type.clone();

    let control = match props.r#type {
        None => VNode::empty(),
        Some(control_type) => match control_type {
            FormControlType::Textarea => {
                rsx!{
                    textarea {
                        class: get_class(props.size, is_invalid),
                        placeholder: props.placeholder,
                        id: props.id,
                        name: props.name,
                        readonly: props.readonly,
                        oninput: move |event| {
                            touched.set(true);
                            match props.oninput {
                                None => {}
                                Some(s) => {
                                    s.call(event);
                                }
                            }
                        },
                    }
                }
            }
            _ => {
                rsx! {
                    input {
                        r#type: get_type(props_type),
                        class: get_class(props.size, is_invalid),
                        placeholder: props.placeholder,
                        id: props.id,
                        name: props.name,
                        readonly: props.readonly,
                        oninput: move |event| {
                            touched.set(true);
                            match props.oninput {
                                None => {}
                                Some(s) => {
                                    s.call(event);
                                }
                            }
                        },
                    }
                }
            }
        },
    };

    rsx! {
        {control}
        InvalidFeedback { errors: props.errors }
        div { {data} }
    }
}

#[derive(Clone, PartialEq)]
pub enum FormControlType {
    Text,
    Checkbox,
    Email,
    Textarea,
}

fn get_class(control_size: Option<FormControlSize>, is_invalid: bool) -> String {
    let mut class = String::from("form-control");
    match control_size {
        None => {}
        Some(n) => match n {
            FormControlSize::Sm => class.push_str(" form-control-sm"),
            FormControlSize::Default => {}
            FormControlSize::Lg => class.push_str(" form-control-lg"),
        },
    };

    if is_invalid == true {
        class.push_str(" is-invalid")
    } else {
        class.push_str(" ")
    }

    class
}

fn get_type(control_type: Option<FormControlType>) -> String {
    let t = match control_type {
        None => "text",
        Some(n) => match n {
            FormControlType::Text => "text",
            FormControlType::Checkbox => "checkbox",
            FormControlType::Email => "email",
            FormControlType::Textarea => "textarea",
        },
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
