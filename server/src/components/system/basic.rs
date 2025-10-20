use dioxus::prelude::*;

#[component]
pub fn BasicSettings() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⚙️ 基本设置" }
            p { "此处显示基本设置相关内容..." }
        }
    }
}