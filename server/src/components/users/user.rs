use dioxus::prelude::*;


const USER_CSS: Asset = asset!("/assets/user.css");
#[component]
pub fn UserList() -> Element {
    
    // 搜索表单状态
    let mut username = use_signal(|| String::new());
    let mut phone = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    
    // 分页状态
    let mut page_size = use_signal(|| "10".to_string());
    
    // 示例用户数据
    let users = vec![
        ("U10001", "张三", "138****0001", "zhangsan@example.com", "普通用户", "正常", "2024-01-15", "2024-10-15 14:30"),
        ("U10002", "李四", "139****0002", "lisi@example.com", "农场主", "正常", "2024-02-20", "2024-10-15 13:15"),
        ("U10003", "王五", "136****0003", "wangwu@example.com", "VIP用户", "冻结", "2024-03-10", "2024-10-14 16:40"),
        ("U10004", "赵六", "135****0004", "zhaoliu@example.com", "管理员", "正常", "2024-04-05", "2024-10-15 11:45"),
    ];
    
    rsx! {
        document::Link { rel: "stylesheet", href: USER_CSS }
        div { class: "user-management-container",
            div { class: "card",
                div { class: "card-header", "用户管理" }
                div { class: "card-body",
                    // 搜索表单
                    form { class: "search-form",
                        div { class: "form-row",
                            div { class: "form-group",
                                div { class: "input-group",
                                    div { class: "input-label", "用户名" }
                                    input {
                                        r#type: "text",
                                        class: "input-field",
                                        placeholder: "请输入用户名",
                                        value: "{username}",
                                        oninput: move |e| username.set(e.value())
                                    }
                                }
                            }
                            div { class: "form-group",
                                div { class: "input-group",
                                    div { class: "input-label", "手机号" }
                                    input {
                                        r#type: "text",
                                        class: "input-field",
                                        placeholder: "请输入手机号",
                                        value: "{phone}",
                                        oninput: move |e| phone.set(e.value())
                                    }
                                }
                            }
                            div { class: "form-group",
                                div { class: "input-group",
                                    div { class: "input-label", "邮箱" }
                                    input {
                                        r#type: "text",
                                        class: "input-field",
                                        placeholder: "请输入邮箱",
                                        value: "{email}",
                                        oninput: move |e| email.set(e.value())
                                    }
                                }
                            }
                            div { class: "form-group",
                                button {
                                    r#type: "button",
                                    class: "btn btn-primary",
                                    style: "width: 100%;",
                                    onclick: move |_| {
                                        // 处理查询逻辑
                                    },
                                    span { class: "btn-icon", "🔍" }
                                    span { "查询" }
                                }
                            }
                        }
                    }
                }
                
                // 工具栏
                div { class: "toolbar",
                    button {
                        r#type: "button",
                        class: "btn btn-info",
                        onclick: move |_| {
                            // 处理导出数据逻辑
                        },
                        span { class: "btn-icon", "📤" }
                        span { "导出数据" }
                    }
                    button {
                        r#type: "button",
                        class: "btn btn-warning",
                        onclick: move |_| {
                            // 处理添加用户逻辑
                        },
                        span { class: "btn-icon", "➕" }
                        span { "添加用户" }
                    }
                }
                
                // 数据表格
                div { class: "table-container",
                    table { class: "table",
                        thead {
                            tr {
                                th { style: "width: 80px;", "用户ID" }
                                th { style: "width: 200px;", "用户信息" }
                                th { style: "width: 120px;", "手机号" }
                                th { style: "width: 200px;", "邮箱" }
                                th { style: "width: 100px;", "角色" }
                                th { style: "width: 100px;", "状态" }
                                th { style: "width: 120px;", "注册时间" }
                                th { style: "width: 140px;", "最后登录" }
                                th { style: "width: 200px;", "操作" }
                            }
                        }
                        tbody {
                            for (id, name, phone_num, email_addr, role, status, reg_time, last_login) in users.iter() {
                                tr {
                                    td { "{id}" }
                                    td {
                                        div { class: "user-info",
                                            div { class: "user-details",
                                                div { class: "user-name", "{name}" }
                                            }
                                        }
                                    }
                                    td { "{phone_num}" }
                                    td { "{email_addr}" }
                                    td { "{role}" }
                                    td { "{status}" }
                                    td { "{reg_time}" }
                                    td { "{last_login}" }
                                    td { class: "action-buttons",
                                        button {
                                            r#type: "button",
                                            class: "btn btn-info btn-sm",
                                            onclick: move |_| {
                                                // 处理编辑逻辑
                                            },
                                            span { "编辑" }
                                        }
                                        if *status == "正常" {
                                            button {
                                                r#type: "button",
                                                class: "btn btn-primary btn-sm",
                                                onclick: move |_| {
                                                    // 处理重置密码逻辑
                                                },
                                                span { "重置密码" }
                                            }
                                        } else if *status == "冻结" {
                                            button {
                                                r#type: "button",
                                                class: "btn btn-warning btn-sm",
                                                onclick: move |_| {
                                                    // 处理解冻逻辑
                                                },
                                                span { "解冻" }
                                            }
                                        }
                                        if *role == "管理员" {
                                            button {
                                                r#type: "button",
                                                class: "btn btn-danger btn-sm",
                                                onclick: move |_| {
                                                    // 处理权限逻辑
                                                },
                                                span { "权限" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // 分页
                div { class: "pagination", id: "pagination",
                    span { class: "pagination-info",
                        "共 "
                        strong { "120" }
                        " 条记录"
                    }
                    button {
                        class: "pagination-btn",
                        id: "prevBtn",
                        onclick: move |_| {
                            // 处理上一页逻辑
                        },
                        "上一页"
                    }
                    div { id: "pageNumbers" }
                    button {
                        class: "pagination-btn",
                        id: "nextBtn",
                        onclick: move |_| {
                            // 处理下一页逻辑
                        },
                        "下一页"
                    }
                    span { class: "pagination-info", "每页显示" }
                    select {
                        class: "pagination-select",
                        id: "pageSizeSelect",
                        value: "{page_size}",
                        onchange: move |e| page_size.set(e.value()),
                        option { value: "10", "10条" }
                        option { value: "20", "20条" }
                        option { value: "50", "50条" }
                        option { value: "100", "100条" }
                    }
                }
            }
        }
    }
}