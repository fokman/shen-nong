use dioxus::{prelude::*};

#[component]
pub fn CropManagement() -> Element {
    rsx! {
    div{
        class:"columns",
        div {
            class:"column is-three-quarters",
            div{
                class:"box",
                set_title {title:"作物概览"},
                div {
                    class:"columns is-multiline",
                    crops_item {
                        crop_name:"樱桃小番茄",
                        image:"",
                        status:"已种植",
                        progress:"80",
                        progress_text:"80%",
                    },
                    crops_item {
                        crop_name:"小白菜",
                        image:"",
                        status:"已种植",
                        progress:"80",
                        progress_text:"80%",
                    },
                    crops_item {
                        crop_name:"胡萝卜",
                        image:"",
                        status:"已种植",
                        progress:"80",
                        progress_text:"80%",
                    },
                    crops_item {
                        crop_name:"土豆",
                        image:"",
                        status:"需要施肥",
                        progress:"40",
                        progress_text:"40%",
                    },
                }
                //环境数据
                div {
                    class:"box",
                    set_title {
                        title:"环境数据",
                    },
                    div {
                        class:"columns is-multiline",
                        environment_data {
                            data_type:"温度",
                            value:"25℃",
                        },
                        environment_data {
                            data_type:"湿度",
                            value:"50%",
                        },
                        environment_data {
                            data_type:"风速",
                            value:"2.3 m/s",
                        },
                        environment_data {
                            data_type:"光照",
                            value:"800 lux",
                        },
                        environment_data {
                            data_type:"土壤湿度",
                            value:"60%",
                        },
                        environment_data {
                            data_type:"土壤酸碱",
                            value:"6.8PH",
                        }
                    }
                }
                //操作面板
                div {
                    class:"box",
                    set_title {
                        title:"操作面板",
                    },
                    div {
                        class:"columns is-multiline",
                        operation_button {
                            button_text:"浇水",
                        },
                        operation_button {
                            button_text:"施肥",
                        },
                        operation_button {
                            button_text:"修剪",
                        },
                        operation_button {
                            button_text:"拍照",
                        },
                        operation_button {
                            button_text:"除草",
                        },
                        operation_button {
                            button_text:"除虫",
                        },
                    }
                }
             }
        }
        //天气
        div {
            class:"column",
            div {
                class:"box",
                div{
                    class:"has-text-centered",
                    p {
                        class:"title is-4 has-text-grey-dark",
                        "26°C"
                    },
                    p {
                        class:"subtitle is-6 has-text-grey",
                        "晴朗，微风"
                    },
                    p {
                        class:"is-size-6 has-text-grey",
                        "北京农场"
                    }
                }

            }
            //今日任务
                div {
                    class:"box",
                    set_title {
                        title:"今日任务",
                    },
                    div {
                        class:"content",
                        task_item {
                            task_name:"给小白菜浇水",
                            time:"09:00",
                        },
                        task_item {
                            task_name:"给胡萝卜施肥",
                            time:"10:00",
                        },
                    }
                }
                div {
                    class:"box",
                    set_title {
                        title:"生长提示",
                    },
                    div {
                        class:"content",
                        set_article {
                            text:"小白菜需要充足的阳光和水分，请确保每天浇水一次。",
                        },
                        set_article{
                            text:"胡萝卜需要充足的肥料，请确保每天施肥一次。"
                        }
                    }

                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct CropItemProps {
    crop_name: String,
    image: String,
    status: String,
    progress: String,
    progress_text: String,
}
//作物概览
fn crops_item(props: CropItemProps) -> Element {
    rsx! {
        div{
            class:"column is-one-quarter",
            div{
                class:"card",
                div{
                    class:"card-content",
                    div{
                        class:"has-background-success-light has-text-centered py-5 mb-3",
                        span{
                            class:"icon is-large has-text-success",
                            i{
                                class:"fas fa-apple-alt fa-2x",
                            }
                        }
                    },
                    p{
                        class:"title is-5 has-text-grey-dark",
                        "{props.crop_name}",
                    },
                    div{
                        class:"content",
                        span{
                            class:"tag is-success is-light",
                            "{props.status}"
                        },
                        div{
                            class:"mt-3",
                            progress{
                                class:"progress is-success is-small",
                                max:"100",
                                value:"{props.progress}",
                                "{props.progress}"
                            },
                            p{
                                class:"has-text-right is-size-7 has-text-grey",
                                "{props.progress_text}"
                            },
                        }
                    }
                }
            }
        }
    }
}

//环境数
#[derive(PartialEq, Props, Clone)]
struct EnvironmentData {
    data_type: String,
    value: String,
}
fn environment_data(props: EnvironmentData) -> Element {
    rsx! {
        div{
            class:"column is-2",
            div{
                class:"box has-background-light has-text-centered",
                p{
                    class:"title is-5 has-text-grey-dark",
                    "{props.value}",
                },
                p{
                    class:"subtitle is-7 has-text-grey",
                    "{props.data_type}",
                },
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct TitleProps {
    title: String,
}

fn set_title(props: TitleProps) -> Element {
    rsx! {
        h2{
            class:"title is-4 has-text-grey-dark",
            "{props.title}",
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct BtnProps {
    button_text: String,
}
fn operation_button(props: BtnProps) -> Element {
    rsx! {
        div{
            class:"column is-2",
            button{
                class:"button is-success is-fullwidth",
                "{props.button_text}",
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct TaskItemProps {
    task_name: String,
    time: String,
}

fn task_item(props: TaskItemProps) -> Element {
    rsx! {
        div {
            class:"media mb-4",
            div {
                class:"media-left",
                input {
                    type:"checkbox",
                    class:"checkbox media-left",
                }
            }
            div {
                class:"media-content",
                p {
                    class:"has-text-grey-dark",
                    "{props.task_name}",
                }
                span {
                    class:"tag is-light is-small",
                    "{props.time}",
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct PromptPros {
    text: String,
}
fn set_article(props: PromptPros) -> Element {
    rsx! {
        article {
            class:"message is-success",
            div {
                class:"message-body",
                "{props.text}",
            }
        }
    }
}
