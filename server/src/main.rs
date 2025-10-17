mod components;

use dioxus::prelude::*;
use components::Dashboard;

const LAY_UI_CSS: Asset = asset!("/assets/layui.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(components::Dashboard)] #[route("/")] DashboardPage {},
    #[route("/farm")] Farm {},
    #[route("/market")] Market {},
    #[route("/profile")] Profile {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: LAY_UI_CSS }
        div{
            Router::<Route> {}
            }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            h1 { "神农开心农场 - Shennong Happy Farm" }
            p { "基于物联网技术的远程种养体验平台" }
        }
    }
}

/// Dashboard page
#[component]
fn DashboardPage() -> Element {
    rsx! {
        Dashboard {}
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
