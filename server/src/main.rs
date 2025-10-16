mod components;

use dioxus::prelude::*;
use admin_menu::Header;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Header)]
    #[route("/")]
    Home {},
    #[route("/farm")]
    Farm {},
    #[route("/market")]
    Market {},
    #[route("/profile")]
    Profile {},
    #[route("/dashboard")]
    Dashboard {},
}

const MAIN_CSS: Asset = asset!("/assets/layui.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}




#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            h1 { "神农开心农场 - Shennong Happy Farm" }
            p { "基于物联网技术的远程种养体验平台" }
            div { id: "links",
                Link {
                    to: Route::Farm {},
                    class: "nav-link",
                    "🌱 我的农场"
                }
                Link {
                    to: Route::Market {},
                    class: "nav-link",
                    "🛒 农贸市场"
                }
                Link {
                    to: Route::Profile {},
                    class: "nav-link",
                    "👤 个人中心"
                }
                Link {
                    to: Route::Dashboard {},
                    class: "nav-link",
                    "📊 数据中心"
                }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

/// Farm page
#[component]
fn Farm() -> Element {
    rsx! {
        h2 { "我的农场" }
        div {
            id: "farm-view",
            "欢迎来到您的虚拟农场！在这里您可以远程种植和养殖。"
        }
    }
}

/// Market page
#[component]
fn Market() -> Element {
    rsx! {
        h2 { "农贸市场" }
        div {
            id: "market-view",
            "购买或销售农产品、禽蛋等商品。"
        }
    }
}

/// Profile page
#[component]
fn Profile() -> Element {
    rsx! {
        h2 { "个人中心" }
        div {
            id: "profile-view",
            "管理您的个人信息和农场设置。"
        }
    }
}

/// Dashboard page
#[component]
fn Dashboard() -> Element {
    rsx! {
        h2 { "数据中心" }
        div {
            id: "dashboard-view",
            "查看农场数据统计和分析报告。"
        }
    }
}
