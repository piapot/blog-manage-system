use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FeedType {
    OriginalFeed, // 初始帖子，可发送文字、图片、链接
}

impl Default for FeedType {
    fn default() -> Self {
        Self::OriginalFeed
    }
}

impl From<&str> for FeedType {
    fn from(feed_type: &str) -> Self {
        use FeedType::*;
        match feed_type {
            "ORIGINAL_FEED" => OriginalFeed,
            _ => panic!("unknown feed type"),
        }
    }
}
