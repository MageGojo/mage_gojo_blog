use dioxus::prelude::*;

#[component]
pub fn FooterComponet() -> Element {
    rsx! {

        footer { class: "bg-yao-dark text-gray-400 py-12 border-t-4 border-yao-red mt-12",
            div { class: "max-w-4xl mx-auto text-center space-y-4",
                div { class: "flex justify-center items-center gap-2 text-xl text-white font-serif",
                    i { class: "fas fa-bullhorn transform -rotate-45" }
                    "\n            瑶山手札\n        "
                }
                p { class: "text-sm", "记录布努瑶的每一个清晨与黄昏" }
                div { class: "text-xs opacity-60",
                    "\n            © 2025 Bunuyal Blog. Design with\n            "
                    i { class: "fas fa-heart text-yao-red" }
                    " by Ah Yao.\n        "
                }
            }
        }
    }
}
