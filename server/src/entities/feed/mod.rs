mod feed_type;
mod picture;
mod status_count;
mod target_feed;
mod urls_in_text;

pub use feed_type::FeedType;
pub use picture::Picture;
pub use status_count::StatsCount;
pub use target_feed::TargetFeed;
pub use urls_in_text::UrlsInText;

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
