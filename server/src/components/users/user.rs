use dioxus::prelude::*;

#[component]
pub fn UserList() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "👥 用户列表" }
            p { "此处显示用户列表相关内容..." }
        }
    }
}