use dioxus::prelude::*;

use super::Header;

/// Dashboard page
#[component]
pub fn Dashboard() -> Element {
    rsx! {
        Header {}
        DashboardContainer {}
    }
}

/// 仪表盘容器
#[component]
fn DashboardContainer() -> Element {
    rsx! {
        div {
            class: "dashboard-container",
            DashboardHeader {}
            StatsRow {}
            ChartAndActivitySection {}
            SystemAndQuickSection {}
        }
    }
}

/// 头部区域
#[component]
fn DashboardHeader() -> Element {
    rsx! {
        div {
            class: "dashboard-header",
            div {
                div {
                    class: "header-title",
                    "仪表盘"
                }
                div {
                    class: "header-subtitle",
                    "实时监控农场运营数据，掌握业务动态"
                }
            }
            div {
                class: "header-controls",
                div {
                    class: "layui-btn-group",
                    button {
                        class: "layui-btn layui-btn-primary layui-btn-xs",
                        "今日"
                    }
                    button {
                        class: "layui-btn layui-btn-primary layui-btn-xs",
                        "本周"
                    }
                    button {
                        class: "layui-btn layui-btn-primary layui-btn-xs",
                        "本月"
                    }
                    button {
                        class: "layui-btn layui-btn-primary layui-btn-xs",
                        "本季度"
                    }
                }
                button {
                    class: "layui-btn layui-btn-normal layui-btn-sm",
                    i {
                        class: "layui-icon",
                        "&#xe669;"
                    }
                    " 刷新数据"
                }
            }
        }
    }
}

/// 统计卡片区域
#[component]
fn StatsRow() -> Element {
    rsx! {
        div {
            class: "stats-row",
            div {
                class: "layui-row layui-col-space15",
                div {
                    class: "layui-col-md3",
                    div {  class: "stat-card farms",

                    }
                }
                StatCard {
                }
                StatCard {
                }
                StatCard {

                }
                StatCard {
                }
            }
        }
    }
}

/// 统计卡片组件
#[component]
fn StatCard() -> Element {
    rsx! {
        div {
            class: "layui-col-md3",
            div {
                class: "stat-card ",
                div {
                    class: "stat-content",
                    div {
                        class: "stat-info",
                        div {
                            class: "stat-value",
                            "value"
                        }
                        div {
                            class: "stat-label",
                            "label"
                        }
                        div {
                            class: "stat-trend",
                            i {
                                class: "layui-icon",
                                "&#xe619;"
                            }
                            "trend"
                        }
                    }
                    div {
                        class: "stat-icon",
                        i {
                            class: "layui-icon",
                            "icon"
                        }
                    }
                }
            }
        }
    }
}

/// 图表和活动区域
#[component]
fn ChartAndActivitySection() -> Element {
    rsx! {
        div {
            class: "stats-row",
            div {
                class: "layui-row layui-col-space15",
                div {
                    class: "layui-col-md8",
                    ChartSection {}
                }
                div {
                    class: "layui-col-md4",
                    ActivitySection {}
                }
            }
        }
    }
}

/// 图表区域
#[component]
fn ChartSection() -> Element {
    rsx! {
        div {
            class: "chart-section",
            div {
                class: "chart-card",
                div {
                    class: "chart-header",
                    div {
                        class: "chart-title",
                        "收入趋势分析"
                    }
                    div {
                        class: "chart-actions",
                        button {
                            class: "layui-btn layui-btn-xs layui-btn-primary",
                            "本月"
                        }
                        button {
                            class: "layui-btn layui-btn-xs",
                            "本季度"
                        }
                        button {
                            class: "layui-btn layui-btn-xs",
                            "本年度"
                        }
                    }
                }
                div {
                    class: "chart-content",
                    i {
                        class: "layui-icon",
                        "&#xe62a;"
                    }
                    " 收入趋势图表区域"
                }
            }
        }
    }
}

/// 活动区域
#[component]
fn ActivitySection() -> Element {
    rsx! {
        div {
            class: "activity-section",
            div {
                class: "activity-card",
                div {
                    class: "activity-header",
                    div {
                        class: "activity-title",
                        "最近活动"
                    }
                    div {
                        class: "activity-badge",
                        "5条新消息"
                    }
                }
                div {
                    class: "activity-list",
                    ActivityItem {
                        icon_type: "success",
                        icon: "&#xe605;",
                        message: "新农场 \"阳光农场\" 注册成功并完成首次种植",
                        time: "2分钟前"
                    }
                    ActivityItem {
                        icon_type: "warning",
                        icon: "&#xe756;",
                        message: "设备 #A123 温度异常，请及时检查",
                        time: "15分钟前"
                    }
                    ActivityItem {
                        icon_type: "info",
                        icon: "&#xe60b;",
                        message: "用户 \"张三\" 完成大额订单，金额 ¥12,800",
                        time: "1小时前"
                    }
                }
            }
        }
    }
}

/// 活动项组件
#[component]
fn ActivityItem(
    icon_type: &'static str,
    icon: &'static str,
    message: &'static str,
    time: &'static str,
) -> Element {
    rsx! {
        div {
            class: "activity-item",
            div {
                class: "activity-icon {icon_type}",
                i {
                    class: "layui-icon",
                    "{icon}"
                }
            }
            div {
                class: "activity-content",
                div {
                    class: "activity-message",
                    "{message}"
                }
                div {
                    class: "activity-time",
                    "{time}"
                }
            }
        }
    }
}

/// 系统状态和快捷操作区域
#[component]
fn SystemAndQuickSection() -> Element {
    rsx! {
        div {
            class: "stats-row",
            div {
                class: "layui-row layui-col-space15",
                div {
                    class: "layui-col-md6",
                    SystemSection {}
                }
                div {
                    class: "layui-col-md6",
                    QuickSection {}
                }
            }
        }
    }
}

/// 系统状态区域
#[component]
fn SystemSection() -> Element {
    rsx! {
        div {
            class: "system-section",
            div {
                class: "system-card",
                div {
                    class: "system-header",
                    div {
                        class: "system-title",
                        "系统状态监控"
                    }
                }
                div {
                    class: "system-grid",
                    SystemItem {

                    }
                    SystemItem {

                    }
                    SystemItem {

                    }
                    SystemItem {

                    }
                }
            }
        }
    }
}

/// 系统状态项组件
#[component]
fn SystemItem() -> Element {
    rsx! {
        div {
            class: "system-item",
            div {
                class: "system-icon",
                i {
                    class: "layui-icon",
                    "icon"
                }
            }
            div {
                class: "system-info",
                div {
                    class: "system-label",
                    "label"
                }
                div {
                    class: "system-status",
                    "status"
                }
            }
        }
    }
}

/// 快捷操作区域
#[component]
fn QuickSection() -> Element {
    rsx! {
        div {
            class: "quick-section",
            div {
                class: "quick-card",
                div {
                    class: "quick-header",
                    div {
                        class: "quick-title",
                        "快捷操作"
                    }
                }
                div {
                    class: "quick-grid",
                    ActionItem {
                        icon: "&#xe62a;",
                        title: "数据报表",
                        desc: "查看详细数据分析报告"
                    }
                    ActionItem {
                        icon: "&#xe645;",
                        title: "消息中心",
                        desc: "处理系统通知信息"
                    }
                    ActionItem {
                        icon: "&#xe614;",
                        title: "系统设置",
                        desc: "配置系统参数选项"
                    }
                    ActionItem {
                        icon: "&#xe770;",
                        title: "用户管理",
                        desc: "管理用户权限分配"
                    }
                }
            }
        }
    }
}

/// 快捷操作项组件
#[component]
fn ActionItem(icon: &'static str, title: &'static str, desc: &'static str) -> Element {
    rsx! {
        div {
            class: "action-item",
            div {
                class: "action-icon",
                i {
                    class: "layui-icon",
                    "{icon}"
                }
            }
            div {
                class: "action-title",
                "{title}"
            }
            div {
                class: "action-desc",
                "{desc}"
            }
        }
    }
}
