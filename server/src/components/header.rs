use dioxus::prelude::*;

use crate::Route;


#[derive(Props, PartialEq, Clone)]
pub struct HeaderProps {
    pub on_menu_click: EventHandler<String>,
}

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "header-container",
            div {
                class: "logo-area",
                div {
                    class: "logo",
                    "神农开心农场"
                }
            }

           Outlet::<Route> {}

            div {
                class: "user-area",
                div {
                    class: "user-info",
                    div {
                        class: "user-avatar",
                        "管"
                    }
                    div {
                        class: "user-details",
                        div {
                            class: "user-name",
                            "管理员"
                        }
                        div {
                            class: "user-role",
                            "系统管理员"
                        }
                    }
                }
            }
        }
    }
}