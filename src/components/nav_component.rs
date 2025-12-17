use dioxus::prelude::*;

/// 导航项结构定义
///
/// - `href`    : 点击后跳转的目标地址（静态生命周期，适合 SSR 场景）
/// - `icon`    : Font Awesome 图标 class，可包含 Tailwind 颜色类
/// - `label`   : 导航显示文本
/// - `submenu` : 子菜单（用于下拉菜单），None 表示无子级
pub struct NavItems {
    pub href: &'static str,
    pub icon: &'static str,
    pub label: &'static str,
    pub submenu: Option<&'static [NavItems]>,
}
/// 「风物」下拉菜单
///
/// 用于展示文化相关分类内容：
/// - 美食
/// - 服饰
/// - 器物 / 文化符号
///
/// 该菜单作为 NAV_ITEMS 中「风物」项的子菜单使用
pub const WIND_SUBMENU: &[NavItems] = &[
    NavItems {
        href: "/culture/food",
        icon: "fas fa-bowl-rice text-orange-400",
        label: "五彩美食",
        submenu: None,
    },
    NavItems {
        href: "/culture/clothing",
        icon: "fas fa-shirt text-indigo-400",
        label: "蜡染服饰",
        submenu: None,
    },
    NavItems {
        href: "/culture/drum",
        icon: "fas fa-drum text-yellow-400",
        label: "铜鼓文化",
        submenu: None,
    },
];

/// 语言切换子菜单
///
/// 当前用于 UI 级别的语言选择入口，
/// 后续可在 Actix 中通过中间件 / Cookie / Session 实现真正的国际化切换
///
/// 路径格式：`/lang/{code}`
pub const LANG_SUBMENU: &[NavItems] = &[
    NavItems {
        href: "/lang/zh",
        icon: "fas fa-language text-emerald-400",
        label: "中文",
        submenu: None,
    },
    NavItems {
        href: "/lang/en",
        icon: "fas fa-globe-americas text-sky-400",
        label: "English",
        submenu: None,
    },
    NavItems {
        href: "/lang/ja",
        icon: "fas fa-torii-gate text-rose-400",
        label: "日本語",
        submenu: None,
    },
];
/// 顶部主导航配置
///
/// 顺序即页面中导航显示顺序：
/// 1. 首页
/// 2. 风物（含子菜单）
/// 3. 归档
/// 4. 语言（含子菜单）
pub const NAV_ITEMS: &[NavItems] = &[
    NavItems {
        href: "/",
        icon: "fas fa-torii-gate",
        label: "首页",
        submenu: None,
    },
    NavItems {
        href: "#",
        icon: "fas fa-sun icon-drum animate-spin-slow",
        label: "风物",
        submenu: Some(WIND_SUBMENU),
    },
    NavItems {
        href: "#",
        icon: "fas fa-layer-group",
        label: "归档",
        submenu: None,
    },
    NavItems {
        href: "#",
        icon: "fas fa-language",
        label: "语言",
        submenu: Some(LANG_SUBMENU),
    },
];

#[component]
pub fn NavComponent() -> Element {
    rsx! {

            nav {
                class: "fixed top-0 w-full z-50 transition-all duration-300 nav-transparent text-white bg-black/20 backdrop-blur-lg supports-[backdrop-filter]:bg-black/30",
                id: "navbar",

                div { class: "max-w-7xl mx-auto px-6 flex justify-between items-center",
                    div { class: "flex items-center gap-3 font-serif font-bold text-base md:text-2xl tracking-widest cursor-pointer",
                        div { class: "w-6 h-6 md:w-8 md:h-8 rounded-full border-2 border-white flex items-center justify-center",
                            img { src:"/assets/logo.png",alt:"网站LOGO 图" }
                        }
                        span { "瑶山手札" }
                    }
                    // 电脑端样式
                    div { class: "hidden md:flex space-x-8 text-sm font-medium tracking-wide items-center",


                        {NAV_ITEMS.iter().map(|item| {
                            if let Some(submenu) = item.submenu {
                                rsx! {
                                    div { class: "relative group",
                                        button { class: "nav-link focus:outline-none",
                                            i { class: "{item.icon}" }
                                            "{item.label}"
                                            i { class: "fas fa-chevron-right text-xs ml-1 opacity-70
                                                    transition-all duration-300 ease-out
                                                    group-hover:rotate-90 group-hover:translate-y-0.5" }
                                        }
                                        div { class: "dropdown-menu absolute left-0 mt-2 w-40 bg-white rounded-lg shadow-xl overflow-hidden py-2 text-gray-700 z-50",
                                           { submenu.iter().map(|sub| rsx! {
                                                a {
                                                    class: "block px-4 py-2 hover:bg-red-50 hover:text-yao-red transition flex items-center gap-2",
                                                    href: "{sub.href}",
                                                    i { class: "{sub.icon}" }
                                                    "{sub.label}"
                                                }
                                           })}
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    a { class: "nav-link group", href: "{item.href}",
                                        i { class: "{item.icon} text-gray-300 group-hover:text-white" }
                                        "{item.label}"
                                    }
                                }
                            }
                        })
    }
                    }
                    // === 手机端结构 (优化版) ===

                    // 1. 汉堡按钮 (ID: mobile-menu-btn)
                    div {
                        id: "mobile-menu-btn",
                        class: "md:hidden text-xl cursor-pointer text-gray-700 hover:text-yao-yellow transition p-2 relative z-50",
                        // z-50 确保按钮在遮罩层之上，这样菜单打开时点击按钮可以关闭
                        i { class: "fas fa-bars pointer-events-none transition-transform duration-300" }
                    }

                    // 2. 遮罩层 (ID: mobile-overlay)
                    // 初始 hidden。打开时：bg-black/40 (半透明黑) + backdrop-blur-sm (毛玻璃)
                    div {
                        id: "mobile-overlay",
                        class: "hidden fixed inset-0 bg-black/20 backdrop-blur-sm z-40 transition-opacity duration-300 opacity-0",
                        "aria-hidden": "true",
                    }

                    // 3. 右侧浮动菜单面板 (ID: mobile-menu-panel)
                    // 初始 hidden。absolute 定位在导航栏下方右侧
                    div {
                        id: "mobile-menu-panel",
                        class: "hidden absolute top-full right-0 mt-2 w-64 bg-white/95 backdrop-blur-md rounded-bl-xl shadow-2xl z-50 origin-top-right transform transition-all duration-300 scale-95 opacity-0 border border-white/20",

                        {NAV_ITEMS.iter().enumerate().map(|(idx, item)| {
                            let submenu_id = format!("mobile-submenu-{}", idx);

                            rsx! {
                                div { class: "border-b border-gray-100/50 last:border-0",
                                    if let Some(submenu) = &item.submenu {
                                        // === 有二级菜单 ===
                                        div {
                                            // 1. 触发器
                                            button {
                                                class: "submenu-toggle w-full flex items-center justify-between px-5 py-4 text-gray-700 font-medium hover:bg-gray-50/50 transition focus:outline-none group",
                                                "data-target": "{submenu_id}",

                                                span { class: "flex items-center gap-3 pointer-events-none",
                                                    i { class: "{item.icon} w-5 text-center text-gray-400 group-hover:text-yao-yellow transition-colors" }
                                                    "{item.label}"
                                                }
                                                // 箭头
                                                i { class: "arrow-icon fas fa-chevron-right text-xs text-gray-300 transition-transform duration-300 ease-[cubic-bezier(0.4,0,0.2,1)] pointer-events-none" }
                                            }

                                            // 2. 子菜单容器 (关键：max-h-0 overflow-hidden 用于动画)
                                            div {
                                                id: "{submenu_id}",
                                                class: "submenu-content overflow-hidden transition-[max-height] duration-500 ease-[cubic-bezier(0.4,0,0.2,1)] max-h-0",

                                                // 内部容器 (bg-gray-50)
                                                div { class: "bg-gray-50/80 inner-content pb-2",
                                                    {submenu.iter().map(|sub| rsx! {
                                                        a {
                                                            class: "block pl-12 pr-5 py-3 text-sm text-gray-600 hover:text-yao-red hover:bg-red-50/50 transition flex items-center gap-2",
                                                            href: "{sub.href}",
                                                            i { class: "{sub.icon} text-xs opacity-60" }
                                                            "{sub.label}"
                                                        }
                                                    })}
                                                }
                                            }
                                        }
                                    } else {
                                        // === 普通链接 ===
                                        a {
                                            class: "block px-5 py-4 text-gray-700 font-medium hover:bg-gray-50/50 transition flex items-center gap-3 group",
                                            href: "{item.href}",
                                            i { class: "{item.icon} w-5 text-center text-gray-400 group-hover:text-yao-yellow transition-colors" }
                                            "{item.label}"
                                        }
                                    }
                                }
                            }
                        })}
                    }
                }
            }
        }
}
