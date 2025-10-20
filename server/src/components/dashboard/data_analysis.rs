use dioxus::prelude::*;

#[component]
pub fn DataAnalysis() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "数据分析" }
            div { class: "page-content", "数据分析页面" }
        }
    }
}