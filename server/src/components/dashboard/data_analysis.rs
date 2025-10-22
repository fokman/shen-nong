use dioxus::prelude::*;


#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div {
            class: "dashboard-container",
            // 头部区域
            div {
                class: "dashboard-header",
                div {
                    class: "columns is-reversed",
                    div {
                        class: "column is-narrow",
                        div {
                            class: "buttons",
                            button {
                                class: "button is-primary is-small",
                                "今日"
                            }
                            button {
                                class: "button is-primary is-small",
                                "本周"
                            }
                            button {
                                class: "button is-primary is-small",
                                "本月"
                            }
                            button {
                                class: "button is-primary is-small",
                                "本季度"
                            }
                            button {
                                class: "button is-link is-small",
                                "刷新数据"
                            }
                        }
                    }
                }
            },
            // 统计卡片区域
            div {
                class: "stats-row",
                div {
                    class: "columns is-multiline",
                    div {
                        class: "column is-one-quarter",
                        div {
                            class: "card",
                            div {
                                class: "card-content",
                                div {
                                    class: "content",
                                    p {
                                        class: "title is-4",
                                        "1,248"
                                    }
                                    p {
                                        class: "subtitle",
                                        "活跃农场数量"
                                    }
                                    p {
                                        class: "has-text-success",
                                        "↑ 12.5%"
                                    }
                                }
                                footer {
                                    class: "card-footer",
                                    span {
                                        class: "card-footer-item",
                                        i {
                                            class: "fas fa-farm",
                                        }
                                    }
                                }
                            }
                        }
                    },
                    div {
                        class: "column is-one-quarter",
                        div {
                            class: "card",
                            div {
                                class: "card-content",
                                div {
                                    class: "content",
                                    p {
                                        class: "title is-4",
                                        "856,420"
                                    }
                                    p {
                                        class: "subtitle",
                                        "本月总收入(元)"
                                    }
                                    p {
                                        class: "has-text-success",
                                        "↑ 8.3%"
                                    }
                                }
                                footer {
                                    class: "card-footer",
                                    span {
                                        class: "card-footer-item",
                                        i {
                                            class: "fas fa-money-bill-wave",
                                        }
                                    }
                                }
                            }
                        }
                    },
                    div {
                        class: "column is-one-quarter",
                        div {
                            class: "card",
                            div {
                                class: "card-content",
                                div {
                                    class: "content",
                                    p {
                                        class: "title is-4",
                                        "15,892"
                                    }
                                    p {
                                        class: "subtitle",
                                        "注册用户数"
                                    }
                                    p {
                                        class: "has-text-success",
                                        "↑ 5.7%"
                                    }
                                }
                                footer {
                                    class: "card-footer",
                                    span {
                                        class: "card-footer-item",
                                        i {
                                            class: "fas fa-users",
                                        }
                                    }
                                }
                            }
                        }
                    },
                    div {
                        class: "column is-one-quarter",
                        div {
                            class: "card",
                            div {
                                class: "card-content",
                                div {
                                    class: "content",
                                    p {
                                        class: "title is-4",
                                        "3,456"
                                    }
                                    p {
                                        class: "subtitle",
                                        "今日订单数"
                                    }
                                    p {
                                        class: "has-text-danger",
                                        "↓ 2.1%"
                                    }
                                }
                                footer {
                                    class: "card-footer",
                                    span {
                                        class: "card-footer-item",
                                        i {
                                            class: "fas fa-shopping-cart",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            // 图表和活动区域
            div {
                class: "stats-row",
                div {
                    class: "columns",
                    div {
                        class: "column is-three-quarters",
                        // 图表区域
                        div {
                            class: "card",
                            header {
                                class: "card-header",
                                p {
                                    class: "card-header-title",
                                    "收入趋势分析"
                                }
                                div {
                                    class: "card-header-icons",
                                    button {
                                        class: "button is-small is-primary",
                                        "本月"
                                    }
                                    button {
                                        class: "button is-small",
                                        "本季度"
                                    }
                                    button {
                                        class: "button is-small",
                                        "本年度"
                                    }
                                }
                            }
                            div {
                                class: "card-content",
                                p {
                                    "收入趋势图表区域"
                                }
                            }
                        }
                    }
                    div {
                        class: "column",
                        // 活动区域
                        div {
                            class: "card",
                            header {
                                class: "card-header",
                                p {
                                    class: "card-header-title",
                                    "最近活动"
                                }
                                span {
                                    class: "tag is-warning",
                                    "5条新消息"
                                }
                            }
                            div {
                                class: "card-content",
                                div {
                                    class: "content",
                                    article {
                                        class: "media",
                                        figure {
                                            class: "media-left",
                                            span {
                                                class: "icon has-text-success",
                                                i {
                                                    class: "fas fa-check-circle",
                                                }
                                            }
                                        }
                                        div {
                                            class: "media-content",
                                            div {
                                                class: "content",
                                                p {
                                                    strong {
                                                        "新农场 \"阳光农场\" 注册成功并完成首次种植"
                                                    }
                                                    small {
                                                        "2分钟前"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    article {
                                        class: "media",
                                        figure {
                                            class: "media-left",
                                            span {
                                                class: "icon has-text-warning",
                                                i {
                                                    class: "fas fa-exclamation-triangle",
                                                }
                                            }
                                        }
                                        div {
                                            class: "media-content",
                                            div {
                                                class: "content",
                                                p {
                                                    strong {
                                                        "设备 #A123 温度异常，请及时检查"
                                                    }
                                                    small {
                                                        "15分钟前"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    article {
                                        class: "media",
                                        figure {
                                            class: "media-left",
                                            span {
                                                class: "icon has-text-info",
                                                i {
                                                    class: "fas fa-info-circle",
                                                }
                                            }
                                        }
                                        div {
                                            class: "media-content",
                                            div {
                                                class: "content",
                                                p {
                                                    strong {
                                                        "用户 \"张三\" 完成大额订单，金额 ¥12,800"
                                                    }
                                                    small {
                                                        "1小时前"
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
            },
            // 系统状态和快捷操作
            div {
                class: "stats-row",
                div {
                    class: "columns",
                    div {
                        class: "column",
                        // 系统状态
                        div {
                            class: "card",
                            header {
                                class: "card-header",
                                p {
                                    class: "card-header-title",
                                    "系统状态监控"
                                }
                            }
                            div {
                                class: "card-content",
                                div {
                                    class: "columns",
                                    div {
                                        class: "column",
                                        div {
                                            class: "content",
                                            p {
                                                class: "title is-6",
                                                "API服务"
                                            }
                                            p {
                                                class: "subtitle is-6",
                                                "运行正常"
                                            }
                                        }
                                    }
                                    div {
                                        class: "column",
                                        div {
                                            class: "content",
                                            p {
                                                class: "title is-6",
                                                "数据库"
                                            }
                                            p {
                                                class: "subtitle is-6",
                                                "连接稳定"
                                            }
                                        }
                                    }
                                    div {
                                        class: "column",
                                        div {
                                            class: "content",
                                            p {
                                                class: "title is-6",
                                                "缓存服务"
                                            }
                                            p {
                                                class: "subtitle is-6 has-text-warning",
                                                "负载较高"
                                            }
                                        }
                                    }
                                    div {
                                        class: "column",
                                        div {
                                            class: "content",
                                            p {
                                                class: "title is-6",
                                                "文件存储"
                                            }
                                            p {
                                                class: "subtitle is-6",
                                                "空间充足"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "column",
                        // 快捷操作
                        div {
                            class: "card",
                            header {
                                class: "card-header",
                                p {
                                    class: "card-header-title",
                                    "快捷操作"
                                }
                            }
                            div {
                                class: "card-content",
                                div {
                                    class: "columns",
                                    div {
                                        class: "column",
                                        div {
                                            class: "content",
                                            p {
                                                class: "title is-6",
                                                "数据报表"
                                            }
                                            p {
                                                class: "subtitle is-6",
                                                "查看详细数据分析报告"
                                            }
                                        }
                                    }
                                    div {
                                        class: "column",
                                        div {
                                            class: "content",
                                            p {
                                                class: "title is-6",
                                                "消息中心"
                                            }
                                            p {
                                                class: "subtitle is-6",
                                                "处理系统通知信息"
                                            }
                                        }
                                    }
                                    div {
                                        class: "column",
                                        div {
                                            class: "content",
                                            p {
                                                class: "title is-6",
                                                "系统设置"
                                            }
                                            p {
                                                class: "subtitle is-6",
                                                "配置系统参数选项"
                                            }
                                        }
                                    }
                                    div {
                                        class: "column",
                                        div {
                                            class: "content",
                                            p {
                                                class: "title is-6",
                                                "用户管理"
                                            }
                                            p {
                                                class: "subtitle is-6",
                                                "管理用户权限分配"
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
}
