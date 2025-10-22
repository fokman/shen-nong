use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct BreadcrumbProps {
    title: String,
    home: String,
    span1: String,
    span2: String,
    span3: String,
    class: String,

}

pub fn sn_bread_crumb(props: BreadcrumbProps) -> Element {
    rsx! {div {
            class: "section",
            h1 {
                class: "title",
                "{props.title}"
            }
            nav {
                class: "breadcrumb",
                aria_label: "breadcrumbs",
                ul {
                    li {
                        a {
                            href: "#",
                            span {
                                "{props.home}"
                            }
                        }
                    }
                    li {
                        a {
                            href: "#",
                            span {
                                class: "icon is-small",
                                i {
                                    class: "fas fa-book",
                                    aria_hidden: "true",
                                    ""
                                }
                            }
                            span {
                                "{props.span1}"
                            }
                        }
                    }
                    li {
                        a {
                            href: "#",
                            span {
                                class: "icon is-small",
                                i {
                                    class: "fas fa-puzzle-piece",
                                    aria_hidden: "true",
                                    ""
                                }
                            }
                            span {
                                "{props.span2}"
                            }
                        }
                    }
                    li {
                        class: "is-active",
                        a {
                            href: "#",
                            span {
                                class: "icon is-small",
                                i {
                                    class: "fas fa-thumbs-up",
                                    aria_hidden: "true",
                                    ""
                                }
                            }
                            span {
                                "{props.span3}"
                            }
                        }
                    }
                }
            }
        }
    }
}