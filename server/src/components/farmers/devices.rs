use dioxus::prelude::*;

#[component]
pub fn DeviceManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⏱️ 设备管理" }
            p { "此处显示农场传感器实时数据..." }
        }
    }
}