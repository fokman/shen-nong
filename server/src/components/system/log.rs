use dioxus::prelude::*;

#[component]
pub fn LogManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⚙️ 日志管理" }
            p { "此处显示日志管理相关内容..." }
        }
    }
}