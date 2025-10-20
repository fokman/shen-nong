use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        h2 { "数据中心" }
        div { 
            id: "dashboard-view",
            "查看农场数据统计和分析报告。"
        }
    }
}