use dioxus::prelude::*;

#[component]
pub fn PermissionManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "👥 权限管理" }
            p { "此处显示权限管理相关内容..." }
        }
    }
}