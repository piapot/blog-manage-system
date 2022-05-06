use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Picture {
    pub url: String,    // 图片地址
    pub width: u32,     // 图片宽度
    pub height: u32,    // 图片高度
    pub format: String, // 图片格式
}

impl Picture {
    pub fn new() -> Self {
        Self::default()
    }
}
