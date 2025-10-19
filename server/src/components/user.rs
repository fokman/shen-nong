use dioxus::prelude::*;

const USER_MANAGEMENT_CSS: Asset = asset!("/assets/user.css");

#[component]
pub fn UserManagement() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: USER_MANAGEMENT_CSS }

        div { class: "user-management-container",
            // 页面标题
            div { class: "layui-card",
                div { class: "layui-card-header", "用户管理" }
                div { class: "layui-card-body",
                    // 工具栏
                    div { class: "layui-form",
                        div { class: "layui-row layui-col-space10",
                            div { class: "layui-col-xs3",
                                div { class: "layui-input-group",
                                    div { class: "layui-input-split layui-input-prefix", "用户名" }
                                    input { r#type: "text", placeholder: "带任意前置内容", class: "layui-input" }
                                }
                            }
                            div { class: "layui-col-xs3",
                                div { class: "layui-input-group",
                                    div { class: "layui-input-split layui-input-prefix", "手机号" }
                                    input { r#type: "text", placeholder: "请输入手机号", class: "layui-input" }
                                }
                            }
                            div { class: "layui-col-xs3",
                                div { class: "layui-input-group",
                                    div { class: "layui-input-split layui-input-prefix", "邮箱" }
                                    input { r#type: "text", placeholder: "请输入邮箱", class: "layui-input" }
                                }
                            }
                            div { class: "layui-col-xs3",
                                div { class: "layui-input-group",
                                    button { r#type: "button", class: "layui-btn layui-bg-blue", "查询" }
                                }
                            }
                        }
                    }
                }
                // 操作按钮
                div { class: "layui-form-item",
                    div { class: "layui-inline", style: "float: right;",
                        button { r#type: "button", class: "layui-btn layui-btn-normal",
                            i { class: "layui-icon layui-icon-export" }
                            " 导出数据"
                        }
                        button { r#type: "button", class: "layui-btn layui-btn-warm",
                            i { class: "layui-icon layui-icon-add-1" }
                            " 添加用户"
                        }
                    }
                }
            }

            // 数据表格
            table { class: "layui-table", 
                // lay-skin: "line",
                colgroup {
                    col { width: "80" }
                    col { width: "180" }
                    col { width: "120" }
                    col { width: "180" }
                    col { width: "100" }
                    col { width: "80" }
                    col { width: "120" }
                    col { width: "140" }
                    col { width: "180" }
                }
                thead {
                    tr {
                        th { "用户ID" }
                        th { "用户信息" }
                        th { "手机号" }
                        th { "邮箱" }
                        th { "角色" }
                        th { "状态" }
                        th { "注册时间" }
                        th { "最后登录" }
                        th { "操作" }
                    }
                }
                tbody {
                    // 空表格行 - 不包含模拟数据
                    tr {
                        td { colspan: "9", style: "text-align: center; padding: 40px; color: #999;", 
                            "暂无用户数据" 
                        }
                    }
                }
            }

            // 分页
            div { id: "userPage", style: "text-align: center; margin-top: 20px;",
                // 分页组件将在后续实现
            }
        }
    }
}

// 用户头像组件
#[component]
fn UserAvatar(initials: String) -> Element {
    rsx! {
        div { class: "user-avatar", 
            "{initials}"
        }
    }
}

// 用户信息组件
#[component]
fn UserInfo(name: String, role: String) -> Element {
    rsx! {
        div { class: "user-info", 
            UserAvatar { initials: name.chars().next().unwrap_or('?').to_string() }
            div {
                div { style: "font-weight: 500;", "{name}" }
                div { style: "font-size: 12px; color: #999;", "{role}" }
            }
        }
    }
}