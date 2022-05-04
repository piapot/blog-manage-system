use super::{topic::Topic, user::User};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    pub id: String,                      // 唯一标识
    pub user: User,                      // 用户信息
    pub content: String,                 // 内容
    pub created_at: String,              // 创建时间
    pub liked: bool,                     // 是否点赞
    pub feed_type: FeedType,             // 帖子类型
    pub stats_count: StatsCount,         // 统计数据
    pub pictures: Vec<Picture>,          // 图片
    pub urls_in_text: Vec<UrlsInText>,   // 内容中的链接
    pub target_feed: Option<TargetFeed>, // 目标帖子，如转发的帖子
    pub topic: Option<Topic>,            // 话题
}

impl Feed {
    pub fn new() -> Self {
        Self::default()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed() {
        let feed = Feed::new();
        assert_eq!(feed.id, String::from(""));
        assert_eq!(feed.user, User::new());
        assert_eq!(feed.content, String::from(""));
        assert_eq!(feed.created_at, String::from(""));
        assert_eq!(feed.liked, false);
        assert_eq!(feed.feed_type, FeedType::OriginalFeed);
        assert_eq!(feed.stats_count, StatsCount::new());
        assert_eq!(feed.pictures, Vec::new());
        assert_eq!(feed.urls_in_text, Vec::new());
        assert_eq!(feed.target_feed, None);
        assert_eq!(feed.topic, None);
    }

    #[test]
    fn test_picture() {
        let picture = Picture::new();
        assert_eq!(picture.url, String::from(""));
        assert_eq!(picture.width, 0);
        assert_eq!(picture.height, 0);
        assert_eq!(picture.format, String::from(""));
    }

    #[test]
    fn test_target_feed() {
        let target_feed = TargetFeed::new();
        assert_eq!(target_feed.id, String::from(""));
        assert_eq!(target_feed.feed_type, FeedType::OriginalFeed);
    }

    #[test]
    fn test_stats_count() {
        let stats_count = StatsCount::new();
        assert_eq!(stats_count.like_count, 0);
        assert_eq!(stats_count.comment_count, 0);
        assert_eq!(stats_count.share_count, 0);
    }

    #[test]
    fn test_urls_in_text() {
        let urls_in_text = UrlsInText::new();
        assert_eq!(urls_in_text.original_url, String::from(""));
        assert_eq!(urls_in_text.title, String::from(""));
    }

    #[test]
    fn test_feed_type() {
        let feed_type = FeedType::from("ORIGINAL_FEED");
        assert_eq!(feed_type, FeedType::OriginalFeed);
    }
}
