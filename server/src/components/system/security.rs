use dioxus::prelude::*;

#[component]
pub fn SecuritySettings() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⚙️ 安全设置" }
            p { "此处显示安全设置相关内容..." }
        }
    }
}