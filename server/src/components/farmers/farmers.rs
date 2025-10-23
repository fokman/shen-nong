use dioxus::prelude::*;

#[component]
pub fn FarmList() -> Element {
    rsx! {
        div {
            class: "columns is-gapless",
            main {
                class: "column",
                div {
                    class: "tabs is-boxed px-4",
                    ul {
                        li {
                            class: "is-active",
                            a { "地图视图" }
                        }
                        li {
                            a { "列表视图" }
                        }
                        li {
                            a { "统计视图" }
                        }
                    }
                }
                div {
                    class: "box mx-4 has-background-light",
                    style: "height:400px;display:flex;align-items:center;justify-content:center;color:#7a7a7a",
                    "农场地理位置分布地图"
                }
                div {
                    class: "columns is-mobile is-align-items-center mx-4 my-4",
                    div {
                        class: "column",
                        div {
                            class: "field has-addons",
                            div {
                                class: "control",
                                input {
                                    class: "input",
                                    r#type: "text",
                                    placeholder: "搜索农场名称、地址..."
                                }
                            }
                            div {
                                class: "control",
                                button {
                                    class: "button is-info",
                                    span {
                                        class: "icon",
                                        i {
                                            class: "fas fa-search"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "column is-narrow",
                        button {
                            class: "button is-success",
                            span {
                                class: "icon",
                                i {
                                    class: "fas fa-plus"
                                }
                            }
                            span { "新建农场" }
                        }
                    }
                }
                div {
                    class: "table-container mx-4",
                    table {
                        class: "table is-fullwidth is-hoverable",
                        thead {
                            tr {
                                th { "农场ID" }
                                th { "农场名称" }
                                th { "地理位置" }
                                th { "农场类型" }
                                th { "负责人" }
                                th { "状态" }
                                th { "创建时间" }
                                th {
                                    width: "160",
                                    "操作"
                                }
                            }
                        }
                        tbody {
                            tr {
                                td { "F001" }
                                td { "北京顺义示范农场" }
                                td { "北京市顺义区" }
                                td { "综合农场" }
                                td { "张三" }
                                td {
                                    span {
                                        class: "tag is-success is-light",
                                        "运营中"
                                    }
                                }
                                td { "2024-01-15" }
                                td {
                                    div {
                                        class: "buttons are-small",
                                        button {
                                            class: "button is-info",
                                            "查看"
                                        }
                                        button {
                                            "编辑"
                                        }
                                        button {
                                            class: "button is-danger",
                                            "删除"
                                        }
                                    }
                                }
                            }
                            tr {
                                td { "F002" }
                                td { "上海崇明生态农场" }
                                td { "上海市崇明区" }
                                td { "有机农场" }
                                td { "李四" }
                                td {
                                    span {
                                        class: "tag is-success is-light",
                                        "运营中"
                                    }
                                }
                                td { "2024-02-20" }
                                td {
                                    div {
                                        class: "buttons are-small",
                                        button {
                                            class: "button is-info",
                                            "查看"
                                        }
                                        button {
                                            "编辑"
                                        }
                                        button {
                                            class: "button is-danger",
                                            "删除"
                                        }
                                    }
                                }
                            }
                            tr {
                                td { "F003" }
                                td { "广州从化种植基地" }
                                td { "广州市从化区" }
                                td { "种植基地" }
                                td { "王五" }
                                td {
                                    span {
                                        class: "tag is-warning is-light",
                                        "维护中"
                                    }
                                }
                                td { "2024-03-10" }
                                td {
                                    div {
                                        class: "buttons are-small",
                                        button {
                                            class: "button is-info",
                                            "查看"
                                        }
                                        button {
                                            "编辑"
                                        }
                                        button {
                                            class: "button is-danger",
                                            "删除"
                                        }
                                    }
                                }
                            }
                            tr {
                                td { "F004" }
                                td { "成都郫都养殖场" }
                                td { "成都市郫都区" }
                                td { "养殖场" }
                                td { "赵六" }
                                td {
                                    span {
                                        class: "tag is-success is-light",
                                        "运营中"
                                    }
                                }
                                td { "2024-04-05" }
                                td {
                                    div {
                                        class: "buttons are-small",
                                        button {
                                            class: "button is-info",
                                            "查看"
                                        }
                                        button {
                                            "编辑"
                                        }
                                        button {
                                            class: "button is-danger",
                                            "删除"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                nav {
                    class: "pagination is-small is-right mx-4 mb-4",
                    role: "navigation",
                    button {
                        class: "pagination-previous",
                        disabled: "",
                        "上一页"
                    }
                    button {
                        class: "pagination-next",
                        "下一页"
                    }
                    ul {
                        class: "pagination-list",
                        li {
                            span {
                                class: "pagination-ellipsis",
                                "第 1 页，共 5 页"
                            }
                        }
                    }
                }
            }
        }
    }
}