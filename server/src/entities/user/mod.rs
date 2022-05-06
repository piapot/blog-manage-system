mod avatar;
mod disabled_features;
mod enabled_features;
mod preferences;
mod stats_count;

pub use avatar::Avatar;
pub use disabled_features::DisabledFeatures;
pub use enabled_features::EnabledFeatures;
pub use preferences::Preferences;
pub use stats_count::StatsCount;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
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
        Self::default()
    }
}
