use dioxus::prelude::*;

#[component]
pub fn MainComponet() -> Element {
    rsx! {

        main {
            class: "max-w-[1200px] mx-auto px-4 py-16 relative z-10",
            id: "content",
            div { class: "flex flex-col lg:flex-row gap-10",
                div { class: "w-full lg:w-3/4 space-y-10",
                    article { class: "bg-white rounded-2xl overflow-hidden shadow-sm hover:shadow-xl transition-all duration-300 flex flex-col md:flex-row h-auto md:h-72 group border border-gray-100",
                        div { class: "p-8 md:w-3/5 flex flex-col justify-between order-2 md:order-1",
                            div {
                                div { class: "flex items-center text-xs text-gray-400 mb-3 gap-2",
                                    span { class: "bg-red-50 text-yao-red px-2 py-0.5 rounded font-bold",
                                        i { class: "fas fa-thumbtack mr-1" }
                                        "置顶"
                                    }
                                    span {
                                        i { class: "far fa-clock" }
                                        " 2025-05-20"
                                    }
                                    span { class: "pl-2 border-l border-gray-200",
                                        i { class: "fas fa-folder text-yao-yellow" }
                                        "\n                                瑶族服饰"
                                    }
                                }
                                h2 { class: "text-2xl font-serif font-bold text-gray-800 group-hover:text-yao-red transition mb-3",
                                    "\n                            几何与色彩的史诗：布努瑶的针线记忆\n                        "
                                }
                                p { class: "text-gray-500 text-sm leading-relaxed line-clamp-2",
                                    "\n                            在大山深处，每一件瑶族服饰都是一部无字的史书。蝴蝶纹象征着起源，蜘蛛纹代表着智慧...\n                        "
                                }
                            }
                            div { class: "mt-4 flex items-center justify-between",
                                a { class: "btn-yao", href: "#",
                                    i { class: "fas fa-feather-alt text-xs" }
                                    "\n                            阅读全文\n                        "
                                }
                            }
                        }
                        div { class: "md:w-2/5 order-1 md:order-2 relative overflow-hidden",
                            img {
                                alt: "Cover",
                                class: "w-full h-full object-cover transition duration-700 group-hover:scale-110",
                                src: "https://images.unsplash.com/photo-1606293926075-69a00febf280?ixlib=rb-4.0.3&auto=format&fit=crop&w=800&q=80",
                            }
                        }
                    }
                    article { class: "bg-white rounded-2xl overflow-hidden shadow-sm hover:shadow-xl transition-all duration-300 flex flex-col md:flex-row h-auto md:h-72 group border border-gray-100",
                        div { class: "md:w-2/5 relative overflow-hidden",
                            img {
                                alt: "Food",
                                class: "w-full h-full object-cover transition duration-700 group-hover:scale-110",
                                src: "https://images.unsplash.com/photo-1543353071-873f17a7a088?ixlib=rb-4.0.3&auto=format&fit=crop&w=800&q=80",
                            }
                            div { class: "absolute top-3 left-3 bg-white/90 backdrop-blur px-3 py-1 rounded-full text-xs font-bold text-gray-700 shadow flex items-center gap-1",
                                span { class: "icon-rice" }
                                " 特色美食\n                    "
                            }
                        }
                        div { class: "p-8 md:w-3/5 flex flex-col justify-between",
                            div {
                                div { class: "flex items-center text-xs text-gray-400 mb-3 gap-2",
                                    span {
                                        i { class: "far fa-clock" }
                                        " 2025-03-03"
                                    }
                                    span { class: "pl-2 border-l border-gray-200",
                                        i { class: "fas fa-utensils text-yao-yellow" }
                                        "\n                                传统技艺"
                                    }
                                }
                                h2 { class: "text-2xl font-serif font-bold text-gray-800 group-hover:text-yao-red transition mb-3",
                                    "\n                            寻味七百弄：清明五彩饭制作全解\n                        "
                                }
                                p { class: "text-gray-500 text-sm leading-relaxed line-clamp-2",
                                    "\n                            枫叶煮黑，红蓝草煮红，密蒙花煮黄。这不仅是食物，更是布努人对色彩的极致运用。\n                        "
                                }
                            }
                            div { class: "mt-4",
                                a { class: "btn-yao", href: "#",
                                    i { class: "fas fa-utensils text-xs" }
                                    "\n                            阅读全文\n                        "
                                }
                            }
                        }
                    }
                    div { class: "flex justify-center mt-8 gap-2",
                        button { class: "w-10 h-10 rounded-full bg-white border hover:bg-yao-red hover:text-white transition flex items-center justify-center shadow-sm",
                            i { class: "fas fa-chevron-left" }
                        }
                        button { class: "w-10 h-10 rounded-full bg-yao-red text-white flex items-center justify-center shadow-md",
                            "\n                    1\n                "
                        }
                        button { class: "w-10 h-10 rounded-full bg-white border hover:bg-yao-red hover:text-white transition flex items-center justify-center shadow-sm",
                            "\n                    2\n                "
                        }
                        button { class: "w-10 h-10 rounded-full bg-white border hover:bg-yao-red hover:text-white transition flex items-center justify-center shadow-sm",
                            i { class: "fas fa-chevron-right" }
                        }
                    }
                }
                aside { class: "w-full lg:w-1/4 space-y-8",
                    div { class: "bg-white rounded-2xl shadow-sm p-6 text-center border-t-4 border-yao-blue group hover:shadow-md transition relative overflow-hidden",
                        div { class: "absolute top-0 right-0 opacity-5 text-6xl text-gray-300 transform rotate-12",
                            i { class: "fas fa-feather-alt" }
                        }
                        div { class: "relative inline-block mb-3",
                            img {
                                alt: "Avatar",
                                class: "w-24 h-24 rounded-full object-cover mx-auto p-1 border-2 border-dashed border-yao-red",
                                src: "https://images.unsplash.com/photo-1534528741775-53994a69daeb?ixlib=rb-4.0.3&auto=format&fit=crop&w=200&q=80",
                            }
                        }
                        h3 { class: "text-xl font-bold font-serif text-gray-800", "阿瑶" }
                        p { class: "text-xs text-gray-400 mt-1", "布努瑶文化记录者" }
                        div { class: "flex justify-center gap-4 mt-6",
                            a {
                                class: "w-9 h-9 rounded-lg bg-gray-50 flex items-center justify-center text-gray-400 hover:bg-yao-blue hover:text-white transition transform hover:scale-110 shadow-sm",
                                href: "#",
                                i { class: "fab fa-weixin" }
                            }
                            a {
                                class: "w-9 h-9 rounded-lg bg-gray-50 flex items-center justify-center text-gray-400 hover:bg-yao-red hover:text-white transition transform hover:scale-110 shadow-sm",
                                href: "#",
                                i { class: "fab fa-weibo" }
                            }
                            a {
                                class: "w-9 h-9 rounded-lg bg-gray-50 flex items-center justify-center text-gray-400 hover:bg-black hover:text-white transition transform hover:scale-110 shadow-sm",
                                href: "#",
                                i { class: "fab fa-github" }
                            }
                        }
                    }
                    div { class: "bg-white rounded-2xl shadow-sm p-5",
                        h4 { class: "font-bold text-gray-800 text-sm mb-4 flex items-center",
                            i { class: "fas fa-chart-pie text-yao-yellow mr-2" }
                            "\n                    部落格数据\n                "
                        }
                        div { class: "grid grid-cols-2 gap-3 text-center text-sm",
                            div { class: "bg-gray-50 py-2 rounded border border-gray-100",
                                div { class: "text-yao-dark font-bold text-lg", "25" }
                                div { class: "text-xs text-gray-400", "文章" }
                            }
                            div { class: "bg-gray-50 py-2 rounded border border-gray-100",
                                div { class: "text-yao-dark font-bold text-lg", "12" }
                                div { class: "text-xs text-gray-400", "分类" }
                            }
                            div { class: "bg-gray-50 py-2 rounded border border-gray-100",
                                div { class: "text-yao-dark font-bold text-lg", "108" }
                                div { class: "text-xs text-gray-400", "天数" }
                            }
                            div { class: "bg-gray-50 py-2 rounded border border-gray-100",
                                div { class: "text-yao-dark font-bold text-lg", "5k+" }
                                div { class: "text-xs text-gray-400", "访客" }
                            }
                        }
                    }
                    div { class: "bg-white rounded-2xl shadow-sm p-5",
                        h4 { class: "font-bold text-gray-800 text-sm mb-4 flex items-center",
                            i { class: "fas fa-tags text-yao-green mr-2" }
                            "\n                    标签印象\n                "
                        }
                        div { class: "flex flex-wrap gap-2",
                            a {
                                class: "px-2 py-1 bg-red-50 text-yao-red text-xs rounded hover:bg-yao-red hover:text-white transition",
                                href: "#",
                                "瑶族 (10)"
                            }
                            a {
                                class: "px-2 py-1 bg-yellow-50 text-yellow-600 text-xs rounded hover:bg-yellow-500 hover:text-white transition",
                                href: "#",
                                "铜鼓 (5)"
                            }
                            a {
                                class: "px-2 py-1 bg-blue-50 text-blue-600 text-xs rounded hover:bg-blue-600 hover:text-white transition",
                                href: "#",
                                "蜡染 (8)"
                            }
                            a {
                                class: "px-2 py-1 bg-green-50 text-green-600 text-xs rounded hover:bg-green-600 hover:text-white transition",
                                href: "#",
                                "七百弄 (3)"
                            }
                        }
                    }
                }
            }
        }
    }
}
