use crate::components::{sn_bread_crumb,sn_button,sn_input,SN_Page};
use dioxus::prelude::*;

#[component]
pub fn UserList() -> Element {
    rsx! {
        sn_bread_crumb {
            title: "用户管理",
            home: "首页",
            span1: "用户管理",
            span2: "用户列表",
            span3: "用户列表",
            class: "is-active"
        },
        div {
            div {
                class: "tab-content",
                div {
                    class: "card",
                    div {
                        class: "card-content",
                        div {
                            class: "columns",
                            sn_input{
                                id:"user_name",
                                placeholder: "用户名"
                            },
                            sn_input{
                                id:"user_mobile",
                                placeholder: "手机号"
                            },
                            sn_input{
                                id:"user_email",
                                placeholder: "邮箱"
                            }
                        }
                        // 用户列表表格
                        div {
                            class: "table-container",
                            div {
                                class: "buttons are-small is-pulled-right",
                                sn_button{
                                    class: "button is-primary",
                                    icon: "🔍",
                                    text: "搜索"
                                },
                                sn_button{
                                    class: "button is-primary",
                                    icon: "➕",
                                    text: "新增用户"
                                },
                                sn_button{
                                    class: "button is-primary",
                                    icon: "🗑️",
                                    text: "批量删除"
                                }
                            }
                            table {
                                class: "table is-fullwidth is-striped is-hoverable",
                                thead {
                                    tr {
                                        th {
                                            input {
                                                r#type: "checkbox"
                                            }
                                        }
                                        th {
                                            "用户ID"
                                        }
                                        th {
                                            "用户名"
                                        }
                                        th {
                                            class: "hide-mobile",
                                            "真实姓名"
                                        }
                                        th {
                                            "手机号"
                                        }
                                        th {
                                            "邮箱"
                                        }
                                        th {
                                            "角色"
                                        }
                                        th {
                                            "状态"
                                        }
                                        th {
                                            class: "hide-mobile",
                                            "创建时间"
                                        }
                                        th {
                                            class: "hide-mobile",
                                            "注册时间"
                                        }
                                        th {
                                            class: "hide-mobile",
                                            "最后登录时间"
                                        }
                                        th {
                                            "操作"
                                        }
                                    }
                                }
                                tbody {
                                    tr {
                                        td {
                                            input {
                                                r#type: "checkbox"
                                            }
                                        }
                                        td {
                                            "1001"
                                        }
                                        td {
                                            div {
                                                class: "media",
                                                div {
                                                    class: "media-content",
                                                    p {
                                                        class: "has-text-weight-bold",
                                                        "admin"
                                                    }
                                                }
                                            }
                                        }
                                        td {
                                            class: "hide-mobile",
                                            "王小明"
                                        }
                                        td {
                                            "13800138000"
                                        }
                                        td {
                                            "admin@example.com"
                                        }
                                        td {
                                            span {
                                                class: "tag is-info",
                                                "超级管理员"
                                            }
                                        }
                                        td {
                                            span {
                                                class: "tag is-success",
                                                "正常"
                                            }
                                        }
                                        td {
                                            class: "hide-mobile",
                                            "2023-12-01"
                                        }
                                        td {
                                            class: "hide-mobile",
                                            "2023-12-01"
                                        }
                                        td {
                                            class: "hide-mobile",
                                            "2023-12-02"
                                        }
                                        td {
                                            div {
                                                class: "buttons are-small",
                                                button {
                                                    class: "button is-link",
                                                    title: "编辑",
                                                    "✏️"
                                                }
                                                button {
                                                    class: "button is-danger",
                                                    title: "删除",
                                                    "🗑️"
                                                }
                                            }
                                        }
                                    }
                                    tr {
                                        td {
                                            input {
                                                r#type: "checkbox"
                                            }
                                        }
                                        td {
                                            "1002"
                                        }
                                        td {
                                            div {
                                                class: "media",
                                                div {
                                                    class: "media-content",
                                                    p {
                                                        class: "has-text-weight-bold",
                                                        "zhangsan"
                                                    }
                                                }
                                            }
                                        }
                                        td {
                                            class: "hide-mobile",
                                            "张三"
                                        }
                                        td {
                                            "13800138001"
                                        }
                                        td {
                                            "zhangsan@example.com"
                                        }
                                        td {
                                            span {
                                                class: "tag is-warning",
                                                "农场管理员"
                                            }
                                        }
                                        td {
                                            span {
                                                class: "tag is-success",
                                                "正常"
                                            }
                                        }
                                        td {
                                            class: "hide-mobile",
                                            "2023-12-02"
                                        }
                                        td {
                                            class: "hide-mobile",
                                            "2023-12-02"
                                        }
                                        td {
                                            class: "hide-mobile",
                                            "2023-12-03"
                                        }
                                        td {
                                            div {
                                                class: "buttons are-small",
                                                button {
                                                    class: "button is-link",
                                                    title: "编辑",
                                                    "✏️"
                                                }
                                                button {
                                                    class: "button is-danger",
                                                    title: "删除",
                                                    "🗑️"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        // 分页导航
                        SN_Page{}
                    }
                }
            }
        }
    }
}
