mod components;

use crate::components::{
    Dashboard,
    DataAnalysis,
    RealTimeMonitor,
    UserList,
    RoleManagement,
    PermissionManagement,
    FarmList,
    DeviceManagement,
    CropManagement,
    OrderManagement,
    ProductManagement,
    InventoryManagement,
    BasicSettings,
    SecuritySettings,
    LogManagement,
};

use dioxus::prelude::*;
use serde::{ Deserialize, Serialize };

const MAIN_CSS: Asset = asset!("/assets/bulma.css");

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
pub fn MainLayout() -> Element {
    rsx! {
        div { class: "container is-fluid",
            nav {
                class: "navbar",
                role: "navigation",
                "aria-label": "main navigation",
                div { class: "navbar-brand",
                    a { class: "navbar-item", "开心农场" }
                }
                div { class: "navbar-menu",
                    div { class: "navbar-start",
                        // 仪表盘
                        div { class: "navbar-item has-dropdown is-hoverable",
                            a { class: "navbar-link", "仪表盘" }
                            div { class: "navbar-dropdown",
                                Link {
                                    to: Route::RealTimeMonitor {},
                                    class: "navbar-item",
                                    "实时监控"
                                }
                                Link {
                                    to: Route::DataAnalysis {},
                                    class: "navbar-item",
                                    "数据分析"
                                }
                                Link {
                                    to: Route::Dashboard {},
                                    class: "navbar-item",
                                    "统计"
                                }
                            }
                        }
                        // 农场管理
                        div { class: "navbar-item has-dropdown is-hoverable",
                            a { class: "navbar-link", "农场管理" }
                            div { class: "navbar-dropdown",
                                Link {
                                    to: Route::FarmList {},
                                    class: "navbar-item",
                                    "农场列表"
                                }
                                Link {
                                    to: Route::DeviceManagement {},
                                    class: "navbar-item",
                                    "设备管理"
                                }
                                Link {
                                    to: Route::CropManagement {},
                                    class: "navbar-item",
                                    "作物管理"
                                }
                            }
                        }
                        // 业务管理
                        div { class: "navbar-item has-dropdown is-hoverable",
                            a { class: "navbar-link", "业务管理" }
                            div { class: "navbar-dropdown",
                                Link {
                                    to: Route::ProductManagement {},
                                    class: "navbar-item",
                                    "产品管理"
                                }
                                Link {
                                    to: Route::OrderManagement {},
                                    class: "navbar-item",
                                    "订单管理"
                                }
                                Link {
                                    to: Route::InventoryManagement {},
                                    class: "navbar-item",
                                    "库存管理"
                                }
                            }
                        }
                        // 用户管理
                        div { class: "navbar-item has-dropdown is-hoverable",
                            a { class: "navbar-link", "用户管理" }
                            div { class: "navbar-dropdown",
                                Link {
                                    to: Route::UserList {},
                                    class: "navbar-item",
                                    "用户管理"
                                }
                                Link {
                                    to: Route::PermissionManagement {},
                                    class: "navbar-item",
                                    "权限配置"
                                }
                                Link {
                                    to: Route::RoleManagement {},
                                    class: "navbar-item",
                                    "角色管理"
                                }
                            }
                        }
                        // 系统维护
                        div { class: "navbar-item has-dropdown is-hoverable",
                            a { class: "navbar-link", "系统维护" }
                            div { class: "navbar-dropdown",
                                Link {
                                    to: Route::BasicSettings {},
                                    class: "navbar-item",
                                    "基础信息"
                                }
                                Link {
                                    to: Route::LogManagement {},
                                    class: "navbar-item",
                                    "日志管理"
                                }
                                Link {
                                    to: Route::SecuritySettings {},
                                    class: "navbar-item",
                                    "安全管理"
                                }
                            }
                        }
                    }
                }
            }
            hr { class: "navbar-divider" }
            section { Outlet::<Route> {} }
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
