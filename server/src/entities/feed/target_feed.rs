use super::FeedType;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetFeed {
    pub id: String,          // 唯一标识
    pub feed_type: FeedType, // 目标帖子类型
}

impl TargetFeed {
    pub fn new() -> Self {
        Self::default()
    }
}
