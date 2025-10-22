use dioxus::prelude::*;

#[component]
pub fn SN_Page() -> Element {
    rsx! {
        nav {
            class: "pagination is-small",
            role: "navigation",
            aria_label: "pagination",
            a {
                href: "#",
                class: "pagination-previous",
                "Previous"
            }
            a {
                href: "#",
                class: "pagination-next",
                "Next page"
            }
            ul {
                class: "pagination-list",
                li {
                    a {
                        href: "#",
                        class: "pagination-link",
                        aria_label: "Goto page 1",
                        "1"
                    }
                }
                li {
                    span {
                        class: "pagination-ellipsis",
                        "..."
                    }
                }
                li {
                    a {
                        href: "#",
                        class: "pagination-link",
                        aria_label: "Goto page 45",
                        "45"
                    }
                }
                li {
                    a {
                        href: "#",
                        class: "pagination-link is-current",
                        aria_label: "Page 46",
                        aria_current: "page",
                        "46"
                    }
                }
                li {
                    a {
                        href: "#",
                        class: "pagination-link",
                        aria_label: "Goto page 47",
                        "47"
                    }
                }
                li {
                    span {
                        class: "pagination-ellipsis",
                        "..."
                    }
                }
                li {
                    a {
                        href: "#",
                        class: "pagination-link",
                        aria_label: "Goto page 86",
                        "86"
                    }
                }
            }
        }
    }
}