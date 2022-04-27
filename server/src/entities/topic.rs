use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,         // 唯一标识
    pub topic_name: String, // 话题名称
}

impl Topic {
    pub fn new(id: String, topic_name: String) -> Self {
        Self { id, topic_name }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic() {
        let topic = Topic::new(String::from("123"), String::from("topic_name"));
        assert_eq!(topic.id, "123");
        assert_eq!(topic.topic_name, "topic_name");
    }
}
