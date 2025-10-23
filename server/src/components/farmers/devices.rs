use dioxus::prelude::*;


#[derive(Debug, Clone)]
struct Device {
    device_type: String,
    name: String,
    power_consumption: f32,
    status: String,
    installation_location: String,
    unique_id: String,
    year_of_activation: String,
    farm_name: String,
}

#[allow(non_snake_case)]
#[component]
pub fn DeviceManagement() -> Element {
    let devices = use_signal(||vec![
        Device {
            device_type: "传感器".to_string(),
            name: "温度传感器".to_string(),
            power_consumption: 0.5,
            status: "在线".to_string(),
            installation_location: "农场1".to_string(),
            unique_id: "1234567890".to_string(),
            year_of_activation: "2023".to_string(),
            farm_name: "阳光生态农场".to_string(),
        },
    ]);

    rsx! {
        div {
            class: "columns",
            div {
                class: "column",
                label {
                    class: "label",
                    "所属农场"
                },
                div {
                    class: "control",
                    div {
                        class: "select is-fullwidth",
                        select {
                            option { "全部" }
                            option { "阳光生态农场" }
                            option { "绿野仙踪农场" }
                            option { "南山有机农场" }
                        }
                    }
                }
            },
            div {
                class: "column is-2",
                label {
                    class: "label",
                    "设备类型"
                },
                div {
                    class: "control",
                    div {
                        class: "select is-fullwidth",
                        select {
                            option { "全部" }
                            option { "传感器" }
                            option { "控制器" }
                            option { "摄像头" }
                            option { "喂食机" }
                        }
                    }
                }
            },
            div {
                class: "column is-2",
                label {
                    class: "label",
                    "状态"
                },
                div {
                    class: "control",
                    div {
                        class: "select is-fullwidth",
                        select {
                            option { "全部" }
                            option { "在线" }
                            option { "离线" }
                            option { "故障" }
                        }
                    }
                }
            },
            div {
                class: "column is-2",
                label {
                    class: "label",
                    "启用年份"
                },
                div {
                    class: "control",
                    div {
                        class: "select is-fullwidth",
                        select {
                            option { "全部" }
                            option { "2023" }
                            option { "2024" }
                            option { "2025" }
                        }
                    }
                }
            },
            div {
                class: "column is-3",
                label {
                    class: "label",
                    "搜索设备名称/ID"
                },
                div {
                    class: "control",
                    input {
                        class: "input",
                        r#type: "text",
                        placeholder: "输入关键词"
                    }
                }
            }
        }
        div {
            class: "card",
            div {
                class: "card-content",
                div {
                    class: "buttons is-pulled-right",
                    button {
                        class: "button is-light",
                        "重置"
                    }
                    button {
                        class: "button is-primary",
                        "查询"
                    }
                }
                div {
                    class: "table-wrapper",
                    table {
                        class: "table is-fullwidth is-striped is-hoverable is-narrow",
                        thead {
                            tr {
                                th { "设备类型" }
                                th { "设备名称" }
                                th {
                                    class: "has-text-right",
                                    "功耗 (W)"
                                }
                                th { "状态" }
                                th { "安装位置" }
                                th { "唯一编号" }
                                th { "启用年份" }
                                th { "所属农场" }
                                th {
                                    class: "has-text-right",
                                    "操作"
                                }
                            }
                        }
                        tbody {
                            for device in devices.iter(){
                                tr {
                                    td { "{device.device_type}" }
                                    td { "{device.name}" }
                                    td {
                                        class: "has-text-right",
                                        "{device.power_consumption}"
                                    }
                                    td {
                                        span {
                                            class: "tag status-{device.status.to_lowercase()}",
                                            "{device.status}"
                                        }
                                    }
                                    td { "{device.installation_location}" }
                                    td { "{device.unique_id}" }
                                    td { "{device.year_of_activation}" }
                                    td { "{device.farm_name}" }
                                    td {
                                        class: "has-text-right",
                                        button {
                                            class: "button is-light",
                                            onclick: move |_| {
                                                // 处理详情逻辑
                                            },
                                            "详情"
                                        }
                                        button {
                                            class: "button is-primary",
                                            onclick: move |_| {
                                                // 处理操作逻辑
                                            },
                                            "操作"
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