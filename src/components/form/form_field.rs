use dioxus::prelude::*;
use crate::prelude::*;
use crate::hooks::uid_generator::use_uid_generator;

#[derive(PartialEq, Clone, Props)]
pub struct FormFieldProps {
    children: Option<Element>,
    label: Option<String>,
    name: Option<String>,
    oninput: EventHandler<FormEvent>,
    errors: Option<Vec<FormControlError>>,
    #[props(default = false)]
    readonly: bool,
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
                errors: props.errors,
                readonly: props.readonly,
            }
        }
    }
}