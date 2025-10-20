use dioxus::prelude::*;

#[component]
pub fn OrderManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "📦 订单管理" }
            p { "此处显示订单管理相关内容..." }
        }
    }
}