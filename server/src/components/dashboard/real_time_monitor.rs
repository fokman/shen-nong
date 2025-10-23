use dioxus::prelude::*;


#[allow(non_snake_case)]
pub fn RealTimeMonitor() -> Element {
    rsx! {
        section {
            class: "section",
            div {
                class: "columns",
                div {
                    class: "column is-full",
                    div {
                        class: "card",
                        div {
                            class: "card-content",
                            div {
                                class: "media",
                                div {
                                    class: "media-content",
                                    p {
                                        class: "title is-4",
                                        "1号农场 - 主摄像头"
                                    }
                                    p {
                                        class: "subtitle is-6",
                                        "在线"
                                    }
                                }
                            }
                            div {
                                class: "content",
                                div {
                                    class: "has-background-black has-text-white has-text-centered is-flex is-align-items-center is-justify-content-center",
                                    style: "height: 400px; position: relative;",
                                    "📺 摄像头实时画面位置",
                                    div {
                                        style: "position: absolute; top: 0.5rem; left: 0.5rem;",
                                        div {
                                            class: "buttons",
                                            button {
                                                class: "button is-primary",
                                                "农场1号"
                                            }
                                            button {
                                                class: "button is-primary",
                                                "农场2号"
                                            }
                                            button {
                                                class: "button is-primary",
                                                "农场3号"
                                            }
                                            button {
                                                class: "button is-primary",
                                                "农场4号"
                                            }
                                        }
                                    }
                                    div {
                                        style: "position: absolute; bottom: 0.5rem; right: 0.5rem;",
                                        div {
                                            class: "buttons",
                                            button {
                                                class: "button is-primary",
                                                "上"
                                            }
                                            button {
                                                class: "button is-primary",
                                                "下"
                                            }
                                            button {
                                                class: "button is-primary",
                                                "左"
                                            }
                                            button {
                                                class: "button is-primary",
                                                "右"
                                            }
                                        }
                                        div {
                                            class: "buttons",
                                            style: "margin-top: 0.5rem;",
                                            button {
                                                class: "button is-primary",
                                                "放大"
                                            }
                                            button {
                                                class: "button is-primary",
                                                "缩小"
                                            }
                                            button {
                                                class: "button is-primary",
                                                "重置"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "columns",
                div {
                    class: "column is-full",
                    h2 {
                        class: "subtitle",
                        "传感器实时数据"
                    }
                    div {
                        class: "columns",
                        div {
                            class: "column",
                            div {
                                class: "card",
                                div {
                                    class: "card-content",
                                    p {
                                        class: "title is-4",
                                        "温度传感器"
                                    }
                                    p {
                                        class: "subtitle is-6",
                                        "25.6°C"
                                    }
                                    div {
                                        class: "content",
                                        "📊 温度趋势图表"
                                    }
                                    p {
                                        class: "tag is-success",
                                        "正常"
                                    }
                                }
                            }
                        }
                        div {
                            class: "column",
                            div {
                                class: "card",
                                div {
                                    class: "card-content",
                                    p {
                                        class: "title is-4",
                                        "湿度传感器"
                                    }
                                    p {
                                        class: "subtitle is-6",
                                        "65.2%"
                                    }
                                    div {
                                        class: "content",
                                        "📊 湿度趋势图表"
                                    }
                                    p {
                                        class: "tag is-success",
                                        "正常"
                                    }
                                }
                            }
                        }
                        div {
                            class: "column",
                            div {
                                class: "card",
                                div {
                                    class: "card-content",
                                    p {
                                        class: "title is-4",
                                        "光照传感器"
                                    }
                                    p {
                                        class: "subtitle is-6",
                                        "850 lux"
                                    }
                                    div {
                                        class: "content",
                                        "📊 光照趋势图表"
                                    }
                                    p {
                                        class: "tag is-success",
                                        "正常"
                                    }
                                }
                            }
                        }
                        div {
                            class: "column",
                            div {
                                class: "card",
                                div {
                                    class: "card-content",
                                    p {
                                        class: "title is-4",
                                        "土壤湿度"
                                    }
                                    p {
                                        class: "subtitle is-6",
                                        "42.3%"
                                    }
                                    div {
                                        class: "content",
                                        "📊 土壤湿度图表"
                                    }
                                    p {
                                        class: "tag is-warning",
                                        "偏低"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "columns",
                div {
                    class: "column is-full",
                    div {
                        class: "box",
                        h2 {
                            class: "subtitle",
                            "实时预警"
                        }
                        div {
                            class: "notification is-danger",
                            button {
                                class: "delete",
                            }
                            strong {
                                "2号农场温度异常："
                            }
                            " 28.3°C，建议降温处理"
                        }
                        div {
                            class: "notification is-warning",
                            button {
                                class: "delete",
                            }
                            strong {
                                "2号农场土壤湿度偏低："
                            }
                            " 建议灌溉"
                        }
                        div {
                            class: "notification is-success",
                            button {
                                class: "delete",
                            }
                            strong {
                                "1号农场运行正常"
                            }
                        }
                    }
                }
            }
        }
    }

}
