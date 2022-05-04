use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,          // 唯一标识
    pub topic_name: String,  // 话题名称
    pub description: String, // 话题描述
}

impl Topic {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic() {
        let topic = Topic::new();
        assert_eq!(topic.id, String::new());
        assert_eq!(topic.topic_name, String::new());
        assert_eq!(topic.description, String::new());
    }
}
