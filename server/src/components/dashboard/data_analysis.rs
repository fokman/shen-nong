use dioxus::prelude::*;

#[component]
pub fn DataAnalysis() -> Element {
    rsx! {
        section { class: "section",
            div { class: "columns",
                div { class: "column",
                    div { class: "field is-horizontal",
                        div { class: "field-body",
                            div { class: "field",
                                div { class: "control",
                                    label { class: "label", "时间范围" }
                                    div { class: "select is-fullwidth",
                                        select {
                                            option { value: "today", "今日" }
                                            option { value: "week", selected: "selected", "本周" }
                                            option { value: "month", "本月" }
                                            option { value: "quarter", "本季度" }
                                            option { value: "year", "本年" }
                                            option { value: "custom", "自定义" }
                                        }
                                    }
                                }
                            }
                            div { class: "field",
                                div { class: "control",
                                    label { class: "label", "开始日期" }
                                    input { class: "input", r#type: "date", value: "2024-10-08" }
                                }
                            }
                            div { class: "field",
                                div { class: "control",
                                    label { class: "label", "结束日期" }
                                    input { class: "input", r#type: "date", value: "2024-10-15" }
                                }
                            }
                            div { class: "field",
                                div { class: "control",
                                    label { class: "label", "数据类型" }
                                    div { class: "select is-fullwidth",
                                        select {
                                            option { value: "all", "全部数据" }
                                            option { value: "user", "用户数据" }
                                            option { value: "order", "订单数据" }
                                            option { value: "farm", "农场数据" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "field is-grouped is-grouped-right",
                        div { class: "control",
                            button { class: "button is-primary", "生成报告" }
                            button { class: "button", "导出数据" }
                        }
                    }
                    div { class: "columns",
                        div { class: "column",
                            div { class: "card",
                                header { class: "card-header",
                                    p { class: "card-header-title", "新增用户" }
                                }
                                div { class: "card-content",
                                    div { class: "content",
                                        p { class: "title is-4", "156" }
                                        p { class: "subtitle is-6", "+12.5% 环比增长" }
                                    }
                                }
                            }
                        }
                        div { class: "column",
                            div { class: "card",
                                header { class: "card-header",
                                    p { class: "card-header-title", "订单数量" }
                                }
                                div { class: "card-content",
                                    div { class: "content",
                                        p { class: "title is-4", "89" }
                                        p { class: "subtitle is-6", "+23.6% 环比增长" }
                                    }
                                }
                            }
                        }
                        div { class: "column",
                            div { class: "card",
                                header { class: "card-header",
                                    p { class: "card-header-title", "总收入" }
                                }
                                div { class: "card-content",
                                    div { class: "content",
                                        p { class: "title is-4", "¥128,560" }
                                        p { class: "subtitle is-6", "+15.8% 同比增长" }
                                    }
                                }
                            }
                        }
                        div { class: "column",
                            div { class: "card",
                                header { class: "card-header",
                                    p { class: "card-header-title", "用户活跃度" }
                                }
                                div { class: "card-content",
                                    div { class: "content",
                                        p { class: "title is-4", "78.5%" }
                                        p { class: "subtitle is-6", "-2.3% 环比下降" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "columns",
                        div { class: "column",
                            div { class: "card",
                                header { class: "card-header",
                                    p { class: "card-header-title", "用户增长趋势" }
                                }
                                div { class: "card-content",
                                    div { class: "content",
                                        p { class: "has-text-centered", "用户增长趋势图表" }
                                    }
                                }
                            }
                        }
                        div { class: "column",
                            div { class: "card",
                                header { class: "card-header",
                                    p { class: "card-header-title", "订单分布分析" }
                                }
                                div { class: "card-content",
                                    div { class: "content",
                                        p { class: "has-text-centered", "订单分布饼图" }
                                    }
                                }
                            }
                        }
                        div { class: "column",
                            div { class: "card",
                                header { class: "card-header",
                                    p { class: "card-header-title", "收入趋势分析" }
                                }
                                div { class: "card-content",
                                    div { class: "content",
                                        p { class: "has-text-centered", "收入趋势折线图" }
                                    }
                                }
                            }
                        }
                        div { class: "column",
                            div { class: "card",
                                header { class: "card-header",
                                    p { class: "card-header-title", "农场运营效率" }
                                }
                                div { class: "card-content",
                                    div { class: "content",
                                        p { class: "has-text-centered", "农场效率柱状图" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "card",
                        header { class: "card-header",
                            p { class: "card-header-title", "关键指标数据表" }
                        }
                        div { class: "card-content",
                            table { class: "table is-fullwidth",
                                thead {
                                    tr {
                                        th { "指标名称" }
                                        th { "当前值" }
                                        th { "环比变化" }
                                        th { "同比变化" }
                                        th { "行业平均" }
                                    }
                                }
                                tbody {
                                    tr {
                                        td { "用户注册转化率" }
                                        td { "15.8%" }
                                        td { class: "has-text-success", "+2.3%" }
                                        td { class: "has-text-success", "+5.6%" }
                                        td { "12.4%" }
                                    }
                                    tr {
                                        td { "订单完成率" }
                                        td { "92.7%" }
                                        td { class: "has-text-success", "+1.2%" }
                                        td { class: "has-text-success", "+3.8%" }
                                        td { "88.5%" }
                                    }
                                    tr {
                                        td { "用户留存率" }
                                        td { "65.3%" }
                                        td { class: "has-text-danger", "-1.5%" }
                                        td { class: "has-text-success", "+2.1%" }
                                        td { "58.9%" }
                                    }
                                    tr {
                                        td { "平均客单价" }
                                        td { "¥1,445" }
                                        td { class: "has-text-success", "+8.7%" }
                                        td { class: "has-text-success", "+15.2%" }
                                        td { "¥1,120" }
                                    }
                                }
                            }
                            div { class: "field is-grouped is-grouped-right",
                                div { class: "control",
                                    button { class: "button is-primary", "生成详细报告" }
                                    button { class: "button", "导出Excel" }
                                    button { class: "button", "打印报告" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
