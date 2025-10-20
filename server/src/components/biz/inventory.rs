use dioxus::prelude::*;

#[component]
pub fn InventoryManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "📦 库存管理" }
            p { "此处显示库存管理相关内容..." }
        }
    }
}