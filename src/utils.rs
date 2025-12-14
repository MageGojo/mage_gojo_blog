use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageLang {
    CN,
    US,
    JP,
}

impl fmt::Display for PageLang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PageLang::CN => "zh-CN",
            PageLang::US => "en-US",
            PageLang::JP => "ja-JP",
        };
        write!(f, "{}", s)
    }
}
