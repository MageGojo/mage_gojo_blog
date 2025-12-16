use dioxus::prelude::*;

use crate::utils::PageLang;

#[component]
pub fn DefaultLayout(children: Element, lang: PageLang, page_title: String) -> Element {
    rsx! {

        html { lang: lang.to_string(),
            head {
                meta { charset: "UTF-8" }
                meta {
                    content: "width=device-width, initial-scale=1.0",
                    name: "viewport",
                }
                title { "{page_title}" }
                link {
                    rel:"icon",
                    r#type :"image/png",
                    href:"/assets/favicon.ico"
                }
                link { href:"/assets/output.css",rel:"stylesheet" }
                link { href:"/assets/all.min.css",rel:"stylesheet" }

                script { src:"/assets/main.js", defer:true}

            }
            body { class: "bg-yao-bg bg-fabric-pattern font-sans text-gray-700 selection:bg-yao-red selection:text-white",

                  {children}

            }
        }

    }
}
