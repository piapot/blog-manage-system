use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Avatar {
    pub url: String,    // 头像地址
    pub width: u32,     // 头像宽度
    pub height: u32,    // 头像高度
    pub format: String, // 头像格式
}

impl Avatar {
    pub fn new() -> Self {
        Self::default()
    }
}
