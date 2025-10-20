mod components;


use crate::components::{Dashboard, DataAnalysis, RealTimeMonitor,UserList,RoleManagement,PermissionManagement};

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

const MAIN_CSS: Asset = asset!("/assets/main.css");

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

        #[route("/business/orders")]
        OrderManagement,

        #[route("/business/products")]
        ProductManagement,

        #[route("/business/inventory")]
        InventoryManagement,

        #[route("/users/list")]
        UserList,

        #[route("/users/roles")]
        RoleManagement,

        #[route("/users/permissions")]
        PermissionManagement,

        #[route("/settings/basic")]
        BasicSettings,

        #[route("/settings/security")]
        SecuritySettings,

        #[route("/settings/logs")]
        LogManagement,
}

#[component]
fn Sidebar() -> Element {
    let mut sidebar_open = use_signal(|| "data-monitor");

    rsx! {
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
                //农场管理
                div { class: "menu-group",
                    div {
                        class: "menu-title",
                        onclick: move |_| sidebar_open.set("farm"),
                        "🏠 农场管理"
                    }
                    if sidebar_open() == "farm" {
                        div { class: "menu-items",
                            Link { to: Route::FarmList {}, "农场列表" }
                            Link { to: Route::DeviceManagement {}, "设备管理" }
                            Link { to: Route::CropManagement {}, "作物管理" }
                        }
                    }
                }
                //业务管理
                div { class: "menu-group",
                    div {
                        class: "menu-title",
                        onclick: move |_| sidebar_open.set("business"),
                        "📦 业务管理"
                    }
                    if sidebar_open() == "business" {
                        div { class: "menu-items",
                            Link { to: Route::OrderManagement {}, "订单管理" }
                            Link { to: Route::ProductManagement {}, "商品管理" }
                            Link { to: Route::InventoryManagement {}, "库存管理" }
                        }
                    }
                }
                // 用户管理
                div { class: "menu-group",
                    div {
                        class: "menu-title",
                        onclick: move |_| sidebar_open.set("users"),
                        "👥 用户管理"
                    }
                    if sidebar_open() == "users" {
                        div { class: "menu-items",
                            Link { to: Route::UserList {}, "用户列表" }
                            Link { to: Route::PermissionManagement {}, "权限管理" }
                            Link { to: Route::RoleManagement {}, "角色管理" }
                        }
                    }
                }

                //系统设置
                div { class: "menu-group",
                    div {
                        class: "menu-title",
                        onclick: move |_| sidebar_open.set("settings"),
                        "⚙️ 系统设置"
                    }
                    if sidebar_open() == "settings" {
                        div { class: "menu-items",
                            Link { to: Route::BasicSettings {}, "基本设置" }
                            Link { to: Route::SecuritySettings {}, "安全设置" }
                            Link { to: Route::LogManagement {}, "日志管理" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn MainLayout() -> Element {
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
                Sidebar {}
                // 主内容
                main { class: "admin-main", Outlet::<Route> {} }
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
        div { Router::<Route> {} }
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

#[component]
fn OrderManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "📦 订单管理" }
            p { "此处显示订单管理相关内容..." }
        }
    }
}

#[component]
fn ProductManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "📦 商品管理" }
            p { "此处显示商品管理相关内容..." }
        }
    }
}

#[component]
fn InventoryManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "📦 库存管理" }
            p { "此处显示库存管理相关内容..." }
        }
    }
}



#[component]
fn BasicSettings() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⚙️ 基本设置" }
            p { "此处显示基本设置相关内容..." }
        }
    }
}

#[component]
fn SecuritySettings() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⚙️ 安全设置" }
            p { "此处显示安全设置相关内容..." }
        }
    }
}

#[component]
fn LogManagement() -> Element {
    rsx! {
        div { class: "page-content",
            h2 { "⚙️ 日志管理" }
            p { "此处显示日志管理相关内容..." }
        }
    }
}
