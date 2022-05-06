use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlsInText {
    pub original_url: String, // 原始链接
    pub title: String,        // 标题
}

impl UrlsInText {
    pub fn new() -> Self {
        Self::default()
    }
}
