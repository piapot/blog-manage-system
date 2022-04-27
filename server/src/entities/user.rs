use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,                               // 唯一标识
    pub username: String,                         // 用户名（加密），用于资料查询
    pub nickname: String,                         // 昵称
    pub created_at: String,                       // 创建时间
    pub updated_at: String,                       // 更新时间
    pub gender: String,                           // 性别
    pub birthday: String,                         // 出生日期
    pub brief_intro: String,                      // 简介，个性签名
    pub avatar: Avatar,                           // 头像
    pub stats_count: StatsCount,                  // 统计数据
    pub preferences: Preferences,                 // 偏好设置
    pub enabled_features: Vec<EnabledFeatures>,   // 启用的功能，如推送、推送消息
    pub disabled_features: Vec<DisabledFeatures>, // 禁用的功能，如禁用评论、点赞等
}

impl User {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            username: String::new(),
            nickname: String::new(),
            created_at: String::new(),
            updated_at: String::new(),
            gender: String::new(),
            birthday: String::new(),
            brief_intro: String::new(),
            avatar: Avatar::new(),
            stats_count: StatsCount::new(),
            preferences: Preferences::new(),
            enabled_features: Vec::new(),
            disabled_features: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Avatar {
    pub url: String,    // 头像地址
    pub width: u32,     // 头像宽度
    pub height: u32,    // 头像高度
    pub format: String, // 头像格式
}

impl Avatar {
    pub fn new() -> Self {
        Self {
            url: String::new(),
            width: 0,
            height: 0,
            format: String::new(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StatsCount {
    pub following_count: usize,  // 关注数
    pub followed_count: usize,   // 被关注数
    pub liked: usize,            // 点赞数
    pub topic_created: usize,    // 发布话题数
    pub topic_subscribed: usize, // 订阅话题数
}

impl StatsCount {
    pub fn new() -> Self {
        Self {
            following_count: 0,
            followed_count: 0,
            liked: 0,
            topic_created: 0,
            topic_subscribed: 0,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {}

impl Preferences {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EnabledFeatures {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DisabledFeatures {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        let user = User::new();
        assert_eq!(user.id, String::new());
        assert_eq!(user.username, String::new());
        assert_eq!(user.nickname, String::new());
        assert_eq!(user.created_at, String::new());
        assert_eq!(user.updated_at, String::new());
        assert_eq!(user.gender, String::new());
        assert_eq!(user.birthday, String::new());
        assert_eq!(user.avatar, Avatar::new());
        assert_eq!(user.stats_count, StatsCount::new());
        assert_eq!(user.preferences, Preferences::new());
        assert_eq!(user.enabled_features, Vec::new());
        assert_eq!(user.disabled_features, Vec::new());
    }

    #[test]
    fn test_avatar() {
        let avatar = Avatar::new();
        assert_eq!(avatar.url, "");
        assert_eq!(avatar.width, 0);
        assert_eq!(avatar.height, 0);
        assert_eq!(avatar.format, "");
    }

    #[test]
    fn test_stats_count() {
        let stats_count = StatsCount::new();
        assert_eq!(stats_count.following_count, 0);
        assert_eq!(stats_count.followed_count, 0);
        assert_eq!(stats_count.liked, 0);
        assert_eq!(stats_count.topic_created, 0);
        assert_eq!(stats_count.topic_subscribed, 0);
    }
}
