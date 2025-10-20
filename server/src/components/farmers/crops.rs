use dioxus::prelude::*;

#[component]
pub fn CropManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⏱️ 作物管理" }
            p { "此处显示农场传感器实时数据..." }
        }
    }
}