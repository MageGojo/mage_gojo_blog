use dioxus::prelude::*;

use crate::{
    components::{FooterComponet, HeadersComponent, MainComponet, NavComponent},
    layouts::default::DefaultLayout,
};

// 首页视图
#[component]
pub fn HomeView() -> Element {
    rsx! {

        DefaultLayout{
            page_title:"Magican's Blog".to_string(),
            children:
         rsx!{NavComponent{  }

         HeadersComponent  {  }

         MainComponet {  }

         FooterComponet  {  }},
         lang:crate::utils::PageLang::CN
    }}
}
