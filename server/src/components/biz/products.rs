use dioxus::prelude::*;

#[component]
pub fn ProductManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "📦 商品管理" }
            p { "此处显示商品管理相关内容..." }
        }
    }
}