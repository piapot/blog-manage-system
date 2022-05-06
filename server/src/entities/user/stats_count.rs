use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatsCount {
    pub following_count: usize,  // 关注数
    pub followed_count: usize,   // 被关注数
    pub liked: usize,            // 点赞数
    pub topic_created: usize,    // 发布话题数
    pub topic_subscribed: usize, // 订阅话题数
}

impl StatsCount {
    pub fn new() -> Self {
        Self::default()
    }
}
