use dioxus::prelude::*;

/// Dashboard page
#[component]
pub fn Dashboard() -> Element {
    rsx! {
        h2 { "数据中心" }
        div { 
            id: "dashboard-view",
            "实时监控环境数据、作物生长状态等信息。"
        }
    }
}