use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "header-container",
            Logo{},
            HeaderMenu{},
        }
    }
}

//顶部菜单左边的logo区域
#[component]
fn Logo() -> Element {
    rsx! {
        div{
            class:"logo-area",
            div{
                class:"logo",
                "神农开心农场"
            }
        }
    }
}


/// Main admin layout component
#[component]
fn AdminMenu() -> Element {
    let mut active_menu = use_signal(|| "dashboard");
    let mut open_accordions = use_signal(|| vec!["data-monitor"]);
    
    rsx! {
        div {
            class: "layout",
            // Sidebar
            div {
                class: "sidebar",
                div {
                    class: "logo",
                    h1 { "神农开心农场" }
                }
                ul {
                    class: "accordion",
                    // Data Monitor Accordion
                    li {
                        class: "accordion-item",
                        div {
                            class: "accordion-header",
                            onclick: move |_| toggle_accordion(&mut open_accordions, "data-monitor"),
                            span { "📊 数据监控" }
                            span {
                                class: "accordion-arrow {if open_accordions().contains(&\"data-monitor\") { \"open\" } else { \"\" }}",
                                "▼"
                            }
                        }
                        div {
                            class: "accordion-content {if open_accordions().contains(&\"data-monitor\") { \"open\" } else { \"\" }}",
                            div {
                                class: "submenu-item {if active_menu() == \"dashboard\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("dashboard"),
                                "仪表盘"
                            }
                            div {
                                class: "submenu-item {if active_menu() == \"real-time-monitor\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("real-time-monitor"),
                                "实时监控"
                            }
                        }
                    }
                    
                    // Farm Management Accordion
                    li {
                        class: "accordion-item",
                        div {
                            class: "accordion-header",
                            onclick: move |_| toggle_accordion(&mut open_accordions, "farm-management"),
                            span { "🏠 农场管理" }
                            span {
                                class: "accordion-arrow {if open_accordions().contains(&\"farm-management\") { \"open\" } else { \"\" }}",
                                "▼"
                            }
                        }
                        div {
                            class: "accordion-content {if open_accordions().contains(&\"farm-management\") { \"open\" } else { \"\" }}",
                            div {
                                class: "submenu-item {if active_menu() == \"farm-list\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("farm-list"),
                                "农场列表"
                            }
                            div {
                                class: "submenu-item {if active_menu() == \"device-management\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("device-management"),
                                "设备管理"
                            }
                        }
                    }
                    
                    // Business Management Accordion
                    li {
                        class: "accordion-item",
                        div {
                            class: "accordion-header",
                            onclick: move |_| toggle_accordion(&mut open_accordions, "business-management"),
                            span { "📦 业务管理" }
                            span {
                                class: "accordion-arrow {if open_accordions().contains(&\"business-management\") { \"open\" } else { \"\" }}",
                                "▼"
                            }
                        }
                        div {
                            class: "accordion-content {if open_accordions().contains(&\"business-management\") { \"open\" } else { \"\" }}",
                            div {
                                class: "submenu-item {if active_menu() == \"order-management\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("order-management"),
                                "订单管理"
                            }
                            div {
                                class: "submenu-item {if active_menu() == \"product-management\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("product-management"),
                                "商品管理"
                            }
                        }
                    }
                    
                    // User Management Accordion
                    li {
                        class: "accordion-item",
                        div {
                            class: "accordion-header",
                            onclick: move |_| toggle_accordion(&mut open_accordions, "user-management"),
                            span { "👥 用户管理" }
                            span {
                                class: "accordion-arrow {if open_accordions().contains(&\"user-management\") { \"open\" } else { \"\" }}",
                                "▼"
                            }
                        }
                        div {
                            class: "accordion-content {if open_accordions().contains(&\"user-management\") { \"open\" } else { \"\" }}",
                            div {
                                class: "submenu-item {if active_menu() == \"user-list\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("user-list"),
                                "用户列表"
                            }
                            div {
                                class: "submenu-item {if active_menu() == \"permission-management\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("permission-management"),
                                "权限管理"
                            }
                        }
                    }
                    
                    // Data Analysis Accordion
                    li {
                        class: "accordion-item",
                        div {
                            class: "accordion-header",
                            onclick: move |_| toggle_accordion(&mut open_accordions, "data-analysis"),
                            span { "📊 数据分析" }
                            span {
                                class: "accordion-arrow {if open_accordions().contains(&\"data-analysis\") { \"open\" } else { \"\" }}",
                                "▼"
                            }
                        }
                        div {
                            class: "accordion-content {if open_accordions().contains(&\"data-analysis\") { \"open\" } else { \"\" }}",
                            div {
                                class: "submenu-item {if active_menu() == \"data-reports\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("data-reports"),
                                "数据报表"
                            }
                            div {
                                class: "submenu-item {if active_menu() == \"trend-analysis\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("trend-analysis"),
                                "趋势分析"
                            }
                        }
                    }
                    
                    // System Settings Accordion
                    li {
                        class: "accordion-item",
                        div {
                            class: "accordion-header",
                            onclick: move |_| toggle_accordion(&mut open_accordions, "system-settings"),
                            span { "⚙️ 系统设置" }
                            span {
                                class: "accordion-arrow {if open_accordions().contains(&\"system-settings\") { \"open\" } else { \"\" }}",
                                "▼"
                            }
                        }
                        div {
                            class: "accordion-content {if open_accordions().contains(&\"system-settings\") { \"open\" } else { \"\" }}",
                            div {
                                class: "submenu-item {if active_menu() == \"basic-settings\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("basic-settings"),
                                "基本设置"
                            }
                            div {
                                class: "submenu-item {if active_menu() == \"security-management\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("security-management"),
                                "安全管理"
                            }
                        }
                    }
                    
                    // Message Center Accordion
                    li {
                        class: "accordion-item",
                        div {
                            class: "accordion-header",
                            onclick: move |_| toggle_accordion(&mut open_accordions, "message-center"),
                            span { "🔔 消息中心" }
                            span {
                                class: "accordion-arrow {if open_accordions().contains(&\"message-center\") { \"open\" } else { \"\" }}",
                                "▼"
                            }
                        }
                        div {
                            class: "accordion-content {if open_accordions().contains(&\"message-center\") { \"open\" } else { \"\" }}",
                            div {
                                class: "submenu-item {if active_menu() == \"system-notifications\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("system-notifications"),
                                "系统通知"
                            }
                            div {
                                class: "submenu-item {if active_menu() == \"user-messages\" { \"active\" } else { \"\" }}",
                                onclick: move |_| active_menu.set("user-messages"),
                                "用户消息"
                            }
                        }
                    }
                }
            }
            
            // Main Content Area
            div {
                class: "main-content",
                // Header
                div {
                    class: "header",
                    h1 {
                        class: "page-title",
                        match active_menu() {
                            "dashboard" => "仪表盘",
                            "real-time-monitor" => "实时监控",
                            "farm-list" => "农场列表",
                            "device-management" => "设备管理",
                            "order-management" => "订单管理",
                            "product-management" => "商品管理",
                            "user-list" => "用户列表",
                            "permission-management" => "权限管理",
                            "data-reports" => "数据报表",
                            "trend-analysis" => "趋势分析",
                            "basic-settings" => "基本设置",
                            "security-management" => "安全管理",
                            "system-notifications" => "系统通知",
                            "user-messages" => "用户消息",
                            _ => "仪表盘"
                        }
                    }
                    div {
                        class: "user-info",
                        span { "管理员" }
                        div { class: "avatar", "👤" }
                    }
                }
                
                // Content Area
                div {
                    class: "content-area",
                    match active_menu() {
                        "dashboard" => rsx! { DashboardContent {} },
                        "real-time-monitor" => rsx! { RealTimeMonitorContent {} },
                        "farm-list" => rsx! { FarmListContent {} },
                        "device-management" => rsx! { DeviceManagementContent {} },
                        "order-management" => rsx! { OrderManagementContent {} },
                        "product-management" => rsx! { ProductManagementContent {} },
                        "user-list" => rsx! { UserListContent {} },
                        "permission-management" => rsx! { PermissionManagementContent {} },
                        "data-reports" => rsx! { DataReportsContent {} },
                        "trend-analysis" => rsx! { TrendAnalysisContent {} },
                        "basic-settings" => rsx! { BasicSettingsContent {} },
                        "security-management" => rsx! { SecurityManagementContent {} },
                        "system-notifications" => rsx! { SystemNotificationsContent {} },
                        "user-messages" => rsx! { UserMessagesContent {} },
                        _ => rsx! { DashboardContent {} }
                    }
                }
            }
        }
        
        style { {include_str!("./admin_menu.css")} }
    }
}

/// Toggle accordion state
fn toggle_accordion(open_accordions: &mut Signal<Vec<&'static str>>, accordion_id: &'static str) {
    let mut current = open_accordions();
    if current.contains(&accordion_id) {
        current.retain(|&id| id != accordion_id);
    } else {
        current.push(accordion_id);
    }
    open_accordions.set(current);
}

/// Dashboard content component
#[component]
fn DashboardContent() -> Element {
    rsx! {
        div {
            class: "dashboard-content",
            h2 { "欢迎使用神农开心农场管理系统" }
            p { "实时监控环境数据、作物生长状态等信息。" }
            
            // Statistics cards
            div {
                class: "stats-cards",
                div {
                    class: "stat-card",
                    div { class: "stat-label", "今日访问量" }
                    div { class: "stat-value", "1,256" }
                    div { class: "stat-change trend-up", "+12.5% 环比增长" }
                }
                div {
                    class: "stat-card",
                    div { class: "stat-label", "活跃用户" }
                    div { class: "stat-value", "892" }
                    div { class: "stat-change trend-up", "+8.3% 环比增长" }
                }
                div {
                    class: "stat-card",
                    div { class: "stat-label", "订单数量" }
                    div { class: "stat-value", "156" }
                    div { class: "stat-change trend-up", "+23.6% 环比增长" }
                }
                div {
                    class: "stat-card",
                    div { class: "stat-label", "总收入" }
                    div { class: "stat-value", "¥28,560" }
                    div { class: "stat-change trend-up", "+15.8% 同比增长" }
                }
            }
        }
    }
}

/// Real-time monitor content component
#[component]
fn RealTimeMonitorContent() -> Element {
    rsx! {
        div {
            class: "real-time-monitor",
            h2 { "实时监控" }
            p { "监控农场环境和作物生长状态。" }
            // Add real-time monitoring components here
        }
    }
}

// Placeholder components for other menu items
#[component]
fn FarmListContent() -> Element { rsx! { h2 { "农场列表" } p { "管理农场信息。" } } }
#[component]
fn DeviceManagementContent() -> Element { rsx! { h2 { "设备管理" } p { "管理农场设备。" } } }
#[component]
fn OrderManagementContent() -> Element { rsx! { h2 { "订单管理" } p { "处理用户订单。" } } }
#[component]
fn ProductManagementContent() -> Element { rsx! { h2 { "商品管理" } p { "管理商品信息。" } } }
#[component]
fn UserListContent() -> Element { rsx! { h2 { "用户列表" } p { "管理系统用户。" } } }
#[component]
fn PermissionManagementContent() -> Element { rsx! { h2 { "权限管理" } p { "管理用户权限。" } } }
#[component]
fn DataReportsContent() -> Element { rsx! { h2 { "数据报表" } p { "查看数据分析报告。" } } }
#[component]
fn TrendAnalysisContent() -> Element { rsx! { h2 { "趋势分析" } p { "分析业务趋势。" } } }
#[component]
fn BasicSettingsContent() -> Element { rsx! { h2 { "基本设置" } p { "配置系统参数。" } } }
#[component]
fn SecurityManagementContent() -> Element { rsx! { h2 { "安全管理" } p { "管理安全设置。" } } }
#[component]
fn SystemNotificationsContent() -> Element { rsx! { h2 { "系统通知" } p { "查看系统通知。" } } }
#[component]
fn UserMessagesContent() -> Element { rsx! { h2 { "用户消息" } p { "处理用户消息。" } } }