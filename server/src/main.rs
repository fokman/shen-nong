mod components;

use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

// src/routes.rs 或直接在 main.rs 中
use serde::{Deserialize, Serialize};

#[derive(Routable, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        Dashboard,

        #[route("/monitor/real-time")]
        RealTimeMonitor,

        #[route("/monitor/analysis")]
        DataAnalysis,
        
        #[route("/farm/list")]
        FarmList,

        #[route("/farm/devices")]
        DeviceManagement,

        #[route("/farm/crops")]
        CropManagement,

        // #[route("/business/orders")]
        // OrderManagement,

        // #[route("/business/products")]
        // ProductManagement,

        // #[route("/business/inventory")]
        // InventoryManagement,

        // #[route("/users/list")]
        // UserList,

        // #[route("/users/roles")]
        // RoleManagement,

        // #[route("/users/permissions")]
        // PermissionManagement,

        // #[route("/settings/basic")]
        // BasicSettings,

        // #[route("/settings/security")]
        // SecuritySettings,

        // #[route("/settings/logs")]
        // LogManagement,
}

#[component]
pub fn MainLayout() -> Element {
    let current_route = use_route::<Route>();
    let mut sidebar_open = use_signal(|| "data-monitor");

    rsx! {
        div { class: "admin-layout",
            // 顶部栏
            header { class: "admin-header",
                div { class: "logo", "神农开心农场" }
                div { class: "user-info",
                    span { "管理员" }
                }
            }

            div { class: "admin-body",
                // 侧边栏
                aside { class: "admin-sidebar",
                    nav {
                        div { class: "menu-group",
                            div {
                                class: "menu-title",
                                onclick: move |_| sidebar_open.set("data-monitor"),
                                "📊 数据监控"
                            }
                            if sidebar_open() == "data-monitor" {
                                div { class: "menu-items",
                                    Link { to: Route::Dashboard {}, "仪表盘" }
                                    Link { to: Route::RealTimeMonitor {}, "实时监控" }
                                    Link { to: Route::DataAnalysis {}, "数据分析" }
                                }
                            }
                        }

                        div { class: "menu-group",
                            div {
                                class: "menu-title",
                                onclick: move |_| sidebar_open.set("farm"),
                                "🏠 农场管理"
                            }
                            if sidebar_open() == "farm" {
                                div { class: "menu-items",
                                    Link { to: Route::FarmList {},"农场列表"}
                                    Link { to: Route::DeviceManagement {}, "设备管理" }
                                    Link { to: Route::CropManagement {}, "作物管理" }
                                }
                            }
                        }
                    }
                }

                // 主内容
                main { class: "admin-main",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div{
            Router::<Route> {}
            }
    }
}

#[component]
fn Dashboard() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "📊 仪表盘" }
            p { "欢迎来到神农开心农场数据监控中心！" }
        }
    }
}

#[component]
fn RealTimeMonitor() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⏱️ 实时监控" }
            p { "此处显示农场传感器实时数据..." }
        }
    }
}

#[component]
fn DataAnalysis() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⏱️ 数据分析" }
            p { "此处显示农场传感器实时数据..." }
        }
    }
}

#[component]
fn FarmList() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⏱️ 农场列表" }
            p { "此处显示农场传感器实时数据..." }
        }
    }
}

#[component]
fn DeviceManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⏱️ 设备管理" }
            p { "此处显示农场传感器实时数据..." }
        }
    }
}

#[component]
fn CropManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⏱️ 作物管理" }
            p { "此处显示农场传感器实时数据..." }
        }
    }
}
