use dioxus::prelude::*;

use super::admin_menu;

#[component]
pub fn Header() -> Element {
    let mut current_page = use_signal(|| "dashboard.html".to_string());

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

            admin_menu {
                on_menu_click: move |page| {
                    current_page.write().clone_from(&page);
                }
            }

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