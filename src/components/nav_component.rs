use dioxus::prelude::*;


/// 导航项结构定义
///
/// - `href`    : 点击后跳转的目标地址（静态生命周期，适合 SSR 场景）
/// - `icon`    : Font Awesome 图标 class，可包含 Tailwind 颜色类
/// - `label`   : 导航显示文本
/// - `submenu` : 子菜单（用于下拉菜单），None 表示无子级
pub struct NavItems{
    pub href:&'static str,
    pub icon:&'static str,
    pub label:&'static str,
    pub submenu:Option<&'static [NavItems]>

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
pub const LANG_SUBMENU:&[NavItems] = &[
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
    NavItems { href: "/", icon: "fas fa-torii-gate", label: "首页", submenu: None },
    NavItems { href: "#", icon: "fas fa-sun icon-drum animate-spin-slow", label: "风物", submenu: Some(WIND_SUBMENU) },
    NavItems { href: "#", icon: "fas fa-layer-group", label: "归档", submenu: None },
    NavItems { href: "#", icon: "fas fa-language", label: "语言", submenu: Some(LANG_SUBMENU) },
];


#[component]
pub fn NavComponent() -> Element {


    rsx! {

        nav {
            class: "fixed top-0 w-full z-50 transition-all duration-300 nav-transparent text-white",
            id: "navbar",
            div { class: "max-w-7xl mx-auto px-6 flex justify-between items-center",
                div { class: "flex items-center gap-3 font-serif font-bold text-2xl tracking-widest cursor-pointer",
                    div { class: "w-8 h-8 rounded-full border-2 border-white flex items-center justify-center",
                        img { src:"/assets/logo.png",alt:"网站LOGO 图" }
                    }
                    span { "瑶山手札" }
                }
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
                div { class: "md:hidden text-xl cursor-pointer hover:text-yao-yellow transition",
                    i { class: "fas fa-bars" }
                }
            }
        }
    }
}
