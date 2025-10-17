use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct MenuItem {
    pub name: String,
    pub icon: String,
    pub children: Vec<MenuItem>,
    pub page: String,
}

#[derive(Props, PartialEq, Clone)]
pub struct AdminMenuProps {
    pub on_menu_click: EventHandler<String>,
}

pub fn admin_menu(props: AdminMenuProps) -> Element {
    let menu_items = vec![
        MenuItem {
            name: "数据监控".to_string(),
            icon: "📊".to_string(),
            children: vec![
                MenuItem {
                    name: "仪表盘".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "dashboard.html".to_string(),
                },
                MenuItem {
                    name: "实时监控".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "real-time-monitor.html".to_string(),
                },
                MenuItem {
                    name: "数据分析".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "data-analysis.html".to_string(),
                },
            ],
            page: "".to_string(),
        },
        MenuItem {
            name: "农场管理".to_string(),
            icon: "🏠".to_string(),
            children: vec![
                MenuItem {
                    name: "农场列表".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "farm-management.html".to_string(),
                },
                MenuItem {
                    name: "设备管理".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "farm-management.html".to_string(),
                },
                MenuItem {
                    name: "作物管理".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "farm-management.html".to_string(),
                },
            ],
            page: "".to_string(),
        },
        MenuItem {
            name: "业务管理".to_string(),
            icon: "📦".to_string(),
            children: vec![
                MenuItem {
                    name: "订单管理".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "order-management.html".to_string(),
                },
                MenuItem {
                    name: "商品管理".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "order-management.html".to_string(),
                },
                MenuItem {
                    name: "库存管理".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "order-management.html".to_string(),
                },
            ],
            page: "".to_string(),
        },
        MenuItem {
            name: "用户管理".to_string(),
            icon: "👥".to_string(),
            children: vec![
                MenuItem {
                    name: "用户列表".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "user-management.html".to_string(),
                },
                MenuItem {
                    name: "权限管理".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "user-management.html".to_string(),
                },
                MenuItem {
                    name: "角色管理".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "user-management.html".to_string(),
                },
            ],
            page: "".to_string(),
        },
        MenuItem {
            name: "系统设置".to_string(),
            icon: "⚙️".to_string(),
            children: vec![
                MenuItem {
                    name: "基本设置".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "system-settings.html".to_string(),
                },
                MenuItem {
                    name: "安全管理".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "system-settings.html".to_string(),
                },
                MenuItem {
                    name: "日志管理".to_string(),
                    icon: "".to_string(),
                    children: vec![],
                    page: "system-settings.html".to_string(),
                },
            ],
            page: "".to_string(),
        },
    ];

    rsx! {
        div {
            class: "nav-area",
            ul {
                class: "layui-nav layui-bg-gray",
                for item in menu_items {
                    li {
                        class: "layui-nav-item",
                        a {
                            href: "javascript:;",
                            "{item.icon} {item.name}"
                        }
                        if !item.children.is_empty() {
                            dl {
                                class: "layui-nav-child",
                                for child in item.children {
                                    dd {
                                        a {
                                            href: "javascript:;",
                                            "data-page": "{child.page}",
                                            "data-name": "{child.name}",
                                            onclick: move |_| {
                                                props.on_menu_click.call(child.page.clone());
                                            },
                                            "{child.name}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}