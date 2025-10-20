use dioxus::prelude::*;

#[component]
pub fn FarmList() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⏱️ 农场列表" }
            p { "此处显示农场传感器实时数据..." }
        }
    }
}