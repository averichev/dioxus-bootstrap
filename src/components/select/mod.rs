use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct SelectProps {
    options: Vec<SelectOption>,
}

#[derive(PartialEq, Clone)]
pub struct SelectOption {
    pub value: String,
    pub text: String,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let items = props.options.iter().map(|item| {
        let item_clone = item.clone();
        rsx! {
            option { value: item_clone.value, {item_clone.text} }
        }
    });
    rsx! {
        select { class: "form-select", {items} }
    }
}