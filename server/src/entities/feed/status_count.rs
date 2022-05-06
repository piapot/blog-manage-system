use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatsCount {
    pub like_count: u32,    // 点赞数
    pub comment_count: u32, // 评论数
    pub share_count: u32,   // 分享数
}

impl StatsCount {
    pub fn new() -> Self {
        Self::default()
    }
}
