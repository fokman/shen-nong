use dioxus::prelude::*;

#[component]
pub fn RoleManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "👥 角色管理" }
            p { "此处显示角色管理相关内容..." }
        }
    }
}