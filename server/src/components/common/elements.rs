use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InputProps {
    id: String,
    placeholder: String,
}

pub fn sn_input(props: InputProps) -> Element {
    rsx! {
        div {
            class: "column is-2",
            input {
                id: "{props.id}",
                class: "input is-small",
                r#type: "text",
                placeholder: "{props.placeholder}"
            }
        }
    }
}

//-------------------------Button-------------------------
#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    class: String,
    icon: String,
    text: String,
}

pub fn sn_button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "{props.class}",
            span {
                class: "icon is-small",
                "{props.icon}"
            }
             span {
                class: "is-hidden-mobile",
                "{props.text}"
            }
        }
    }
}
