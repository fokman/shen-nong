use dioxus::prelude::*;

#[component]
pub fn RealTimeMonitor() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "实时监控" }
            div { class: "page-content", "实时监控页面" }
        }
    }
}