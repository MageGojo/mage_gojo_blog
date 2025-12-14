use dioxus::prelude::*;

#[component]
pub fn NavComponent() -> Element {
    rsx! {

        nav {
            class: "fixed top-0 w-full z-50 transition-all duration-300 nav-transparent text-white",
            id: "navbar",
            div { class: "max-w-7xl mx-auto px-6 flex justify-between items-center",
                div { class: "flex items-center gap-3 font-serif font-bold text-2xl tracking-widest cursor-pointer",
                    div { class: "w-8 h-8 rounded-full border-2 border-white flex items-center justify-center",
                        i { class: "fas fa-bullhorn transform -rotate-45 text-sm" }
                    }
                    span { "瑶山手札" }
                }
                div { class: "hidden md:flex space-x-8 text-sm font-medium tracking-wide items-center",
                    a { class: "nav-link group", href: "#",
                        i { class: "fas fa-torii-gate text-gray-300 group-hover:text-white" }
                        "\n                首页\n            "
                    }
                    div { class: "relative group",
                        button { class: "nav-link focus:outline-none",
                            i { class: "fas fa-sun icon-drum animate-spin-slow" }
                            "\n                    风物\n                    "
                            i { class: "fas fa-chevron-down text-xs ml-1 opacity-70" }
                        }
                        div { class: "dropdown-menu absolute left-0 mt-2 w-40 bg-white rounded-lg shadow-xl overflow-hidden py-2 text-gray-700 z-50",
                            a {
                                class: "block px-4 py-2 hover:bg-red-50 hover:text-yao-red transition flex items-center gap-2",
                                href: "#",
                                span { class: "icon-rice" }
                                " 五彩美食\n                    "
                            }
                            a {
                                class: "block px-4 py-2 hover:bg-red-50 hover:text-yao-red transition flex items-center gap-2",
                                href: "#",
                                i { class: "fas fa-tshirt text-indigo-500" }
                                "\n                        蜡染服饰\n                    "
                            }
                            a {
                                class: "block px-4 py-2 hover:bg-red-50 hover:text-yao-red transition flex items-center gap-2",
                                href: "#",
                                i { class: "fas fa-drum text-yellow-500" }
                                "\n                        铜鼓文化\n                    "
                            }
                        }
                    }
                    a { class: "nav-link group", href: "#",
                        i { class: "fas fa-layer-group text-gray-300 group-hover:text-white" }
                        "\n                归档\n            "
                    }
                    a { class: "nav-link group", href: "#",
                        i { class: "fas fa-pen-nib text-gray-300 group-hover:text-white" }
                        "\n                留言\n            "
                    }
                    a { class: "nav-link group", href: "#",
                        i { class: "fas fa-user-astronaut text-gray-300 group-hover:text-white" }
                        "\n                关于\n            "
                    }
                }
                div { class: "md:hidden text-xl cursor-pointer hover:text-yao-yellow transition",
                    i { class: "fas fa-bars" }
                }
            }
        }
    }
}
