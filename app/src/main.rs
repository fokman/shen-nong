use dioxus::prelude::*;
use crate::components::dashboard::{Dashboard, RealTimeMonitor, DataAnalysis};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/farm")]
    Farm {},
    #[route("/market")]
    Market {},
    #[route("/profile")]
    Profile {},
    #[route("/dashboard")]
    Dashboard {},
    
    // 管理后台路由
    #[layout(AdminLayout)]
    #[route("/admin")]
    Admin {},
    
    // 数据监控
    #[route("/admin/dashboard")]
    AdminDashboard {},
    #[route("/admin/real-time-monitor")]
    RealTimeMonitor {},
    #[route("/admin/data-analysis")]
    DataAnalysis {},
    
    // 农场管理
    #[route("/admin/farm-list")]
    FarmList {},
    #[route("/admin/device-management")]
    DeviceManagement {},
    #[route("/admin/crop-management")]
    CropManagement {},
    
    // 业务管理
    #[route("/admin/order-management")]
    OrderManagement {},
    #[route("/admin/product-management")]
    ProductManagement {},
    #[route("/admin/inventory-management")]
    InventoryManagement {},
    
    // 用户管理
    #[route("/admin/user-list")]
    UserList {},
    #[route("/admin/permission-management")]
    PermissionManagement {},
    #[route("/admin/role-management")]
    RoleManagement {},
    
    // 系统设置
    #[route("/admin/basic-settings")]
    BasicSettings {},
    #[route("/admin/security-management")]
    SecurityManagement {},
    #[route("/admin/log-management")]
    LogManagement {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            h1 { "神农开心农场 - Shennong Happy Farm" }
            p { "基于物联网技术的远程种养体验平台" }
            div { id: "links",
                Link {
                    to: Route::Farm {},
                    class: "nav-link",
                    "🌱 我的农场"
                }
                Link {
                    to: Route::Market {},
                    class: "nav-link", 
                    "🛒 农贸市场"
                }
                Link {
                    to: Route::Profile {},
                    class: "nav-link",
                    "👤 个人中心"
                }
                Link {
                    to: Route::Dashboard {},
                    class: "nav-link",
                    "📊 数据中心"
                }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

/// Farm page
#[component]
fn Farm() -> Element {
    rsx! {
        h2 { "我的农场" }
        div { 
            id: "farm-view",
            "欢迎来到您的虚拟农场！在这里您可以远程种植和养殖。"
        }
    }
}

/// Market page
#[component]
fn Market() -> Element {
    rsx! {
        h2 { "农贸市场" }
        div { 
            id: "market-view",
            "购买或销售农产品、禽蛋等商品。"
        }
    }
}

/// Profile page
#[component]
fn Profile() -> Element {
    rsx! {
        h2 { "个人中心" }
        div { 
            id: "profile-view",
            "管理您的个人信息和农场设置。"
        }
    }
}

/// Dashboard page

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                class: "nav-item",
                "🏠 首页"
            }
            Link {
                to: Route::Farm {},
                class: "nav-item",
                "🌱 农场"
            }
            Link {
                to: Route::Market {},
                class: "nav-item",
                "🛒 市场"
            }
            Link {
                to: Route::Profile {},
                class: "nav-item",
                "👤 我"
            }
        }

        Outlet::<Route> {}
    }
}

/// Admin layout with sidebar
#[component]
fn AdminLayout() -> Element {
    rsx! {
        div {
            class: "admin-container",
            
            // Header
            div {
                class: "admin-header",
                div {
                    class: "logo-area",
                    div { class: "logo", "神农开心农场" }
                }
                div {
                    class: "user-area",
                    div {
                        class: "user-info",
                        div { class: "user-avatar", "管" }
                        div {
                            class: "user-details",
                            div { class: "user-name", "管理员" }
                            div { class: "user-role", "系统管理员" }
                        }
                    }
                }
            }
            
            // Sidebar and content
            div {
                class: "admin-body",
                Sidebar {}
                div {
                    class: "admin-content",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

/// Sidebar component with menu items
#[component]
fn Sidebar() -> Element {
    rsx! {
        div {
            class: "sidebar",
            
            // 数据监控
            div {
                class: "menu-section",
                div { class: "menu-title", "📊 数据监控" }
                ul {
                    class: "menu-items",
                    li {
                        Link {
                            to: Route::AdminDashboard {},
                            class: "menu-item",
                            "仪表盘"
                        }
                    }
                    li {
                        Link {
                            to: Route::RealTimeMonitor {},
                            class: "menu-item",
                            "实时监控"
                        }
                    }
                    li {
                        Link {
                            to: Route::DataAnalysis {},
                            class: "menu-item",
                            "数据分析"
                        }
                    }
                }
            }
            
            // 农场管理
            div {
                class: "menu-section",
                div { class: "menu-title", "🏠 农场管理" }
                ul {
                    class: "menu-items",
                    li {
                        Link {
                            to: Route::FarmList {},
                            class: "menu-item",
                            "农场列表"
                        }
                    }
                    li {
                        Link {
                            to: Route::DeviceManagement {},
                            class: "menu-item",
                            "设备管理"
                        }
                    }
                    li {
                        Link {
                            to: Route::CropManagement {},
                            class: "menu-item",
                            "作物管理"
                        }
                    }
                }
            }
            
            // 业务管理
            div {
                class: "menu-section",
                div { class: "menu-title", "📦 业务管理" }
                ul {
                    class: "menu-items",
                    li {
                        Link {
                            to: Route::OrderManagement {},
                            class: "menu-item",
                            "订单管理"
                        }
                    }
                    li {
                        Link {
                            to: Route::ProductManagement {},
                            class: "menu-item",
                            "商品管理"
                        }
                    }
                    li {
                        Link {
                            to: Route::InventoryManagement {},
                            class: "menu-item",
                            "库存管理"
                        }
                    }
                }
            }
            
            // 用户管理
            div {
                class: "menu-section",
                div { class: "menu-title", "👥 用户管理" }
                ul {
                    class: "menu-items",
                    li {
                        Link {
                            to: Route::UserList {},
                            class: "menu-item",
                            "用户列表"
                        }
                    }
                    li {
                        Link {
                            to: Route::PermissionManagement {},
                            class: "menu-item",
                            "权限管理"
                        }
                    }
                    li {
                        Link {
                            to: Route::RoleManagement {},
                            class: "menu-item",
                            "角色管理"
                        }
                    }
                }
            }
            
            // 系统设置
            div {
                class: "menu-section",
                div { class: "menu-title", "⚙️ 系统设置" }
                ul {
                    class: "menu-items",
                    li {
                        Link {
                            to: Route::BasicSettings {},
                            class: "menu-item",
                            "基本设置"
                        }
                    }
                    li {
                        Link {
                            to: Route::SecurityManagement {},
                            class: "menu-item",
                            "安全管理"
                        }
                    }
                    li {
                        Link {
                            to: Route::LogManagement {},
                            class: "menu-item",
                            "日志管理"
                        }
                    }
                }
            }
        }
    }
}

// ==================== 管理后台页面组件 ====================

/// Admin home page
#[component]
fn Admin() -> Element {
    rsx! {
        div {
            class: "admin-home",
            h1 { "神农开心农场管理系统" }
            p { "欢迎使用管理后台" }
        }
    }
}

// 数据监控页面
#[component]
fn AdminDashboard() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "仪表盘" }
            div { class: "page-content", "数据监控仪表盘页面" }
        }
    }
}



// 农场管理页面
#[component]
fn FarmList() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "农场列表" }
            div { class: "page-content", "农场列表管理页面" }
        }
    }
}

#[component]
fn DeviceManagement() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "设备管理" }
            div { class: "page-content", "设备管理页面" }
        }
    }
}

#[component]
fn CropManagement() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "作物管理" }
            div { class: "page-content", "作物管理页面" }
        }
    }
}

// 业务管理页面
#[component]
fn OrderManagement() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "订单管理" }
            div { class: "page-content", "订单管理页面" }
        }
    }
}

#[component]
fn ProductManagement() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "商品管理" }
            div { class: "page-content", "商品管理页面" }
        }
    }
}

#[component]
fn InventoryManagement() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "库存管理" }
            div { class: "page-content", "库存管理页面" }
        }
    }
}

// 用户管理页面
#[component]
fn UserList() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "用户列表" }
            div { class: "page-content", "用户列表管理页面" }
        }
    }
}

#[component]
fn PermissionManagement() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "权限管理" }
            div { class: "page-content", "权限管理页面" }
        }
    }
}

#[component]
fn RoleManagement() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "角色管理" }
            div { class: "page-content", "角色管理页面" }
        }
    }
}

// 系统设置页面
#[component]
fn BasicSettings() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "基本设置" }
            div { class: "page-content", "基本设置页面" }
        }
    }
}

#[component]
fn SecurityManagement() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "安全管理" }
            div { class: "page-content", "安全管理页面" }
        }
    }
}

#[component]
fn LogManagement() -> Element {
    rsx! {
        div {
            class: "page-container",
            h2 { "日志管理" }
            div { class: "page-content", "日志管理页面" }
        }
    }
}
