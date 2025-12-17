use dioxus::prelude::*;

#[component]
pub fn HeadersComponent() -> Element {
    rsx! {

        header { class: "relative h-screen w-full overflow-hidden flex items-center justify-center bg-gray-900",
            div { class: "absolute inset-0",
                img {
                    alt: "Mountain",
                    class: "w-full h-full object-cover opacity-60",
                    src: "/assets/background.avif",
                }
                div { class: "absolute inset-0 bg-gradient-to-b from-slate-900/60 via-transparent to-yao-bg/20" }
            }
            div { class: "relative z-10 text-center text-white",
                h1 { class: "text-4xl md:text-8xl font-serif font-bold mb-8 drop-shadow-2xl tracking-widest text-transparent bg-clip-text bg-gradient-to-b from-white to-gray-300 ",
                    "心系瑶山"
                }
                div { class: "inline-flex items-center bg-black/20 backdrop-blur-md border border-white/20 px-8 py-3 rounded-full shadow-2xl",
                    span {
                        class: "text-sm md:text-xl font-light text-gray-100 tracking-widest font-serif",
                        id: "typewriter-text",
                    }
                    span { class: "cursor-stick" }
                }
            }
            svg {
                class: "waves h-16 md:h-28",
                preserve_aspect_ratio: "none",
                shape_rendering: "auto",
                view_box: "0 24 150 28",
                "xlink": "http://www.w3.org/1999/xlink",
                xmlns: "http://www.w3.org/2000/svg",
                defs {
                    path {
                        d: "M-160 44c30 0 58-18 88-18s 58 18 88 18 58-18 88-18 58 18 88 18 v44h-352z",
                        id: "gentle-wave",
                    }
                }
                g { class: "parallax",
                    use { href: "#gentle-wave", x: "48", y: "0" }
                    use { href: "#gentle-wave", x: "48", y: "3" }
                    use { href: "#gentle-wave", x: "48", y: "5",class:"hidden md:block" }
                    use { href: "#gentle-wave", x: "48", y: "7",class:"hidden md:block" }
                }
            }
            div {
                class: "absolute bottom-8 left-1/2 -translate-x-1/2 z-30 cursor-pointer",
                "onclick": "scrollToContent()",
                div { class: "w-9 h-9 mb:w-12 mb:h-12 flex items-center justify-center rounded-full bg-white/15 backdrop-blur-md shadow-lg ring-1 ring-white/30 hover:ring-white/60 transition-all duration-300 animate-bounce",
                    i { class: "fa fa-angle-down text-xl md:text-2xl text-white/90 hover:text-white" }

                }
            }
        }
    }
}
