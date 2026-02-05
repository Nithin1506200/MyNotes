use std::collections::{HashMap, HashSet};

struct Topic {
    name: String,
    subscribers: HashSet<String>,
    topics: Vec<String>,
}
struct PubSub {
    subscripbers: HashSet<String>,
    topic: HashMap<String, Topic>,
}

impl PubSub {
    fn new() -> Self {
        PubSub {
            subscripbers: HashSet::new(),
            topic: HashMap::new(),
        }
    }
    // -------------
    fn create_topic(&mut self, topic_name: &str) -> Result<(), String> {
        if topic_name.is_empty() {
            return Err("Topic name cannot be empty".to_string());
        }
        if self.topic.contains_key(topic_name) {
            return Err("Topic already exists".to_string());
        }
        let topic = Topic {
            name: topic_name.to_string(),
            subscribers: HashSet::new(),
            topics: Vec::new(),
        };
        self.topic.insert(topic_name.to_string(), topic);
        Ok(())
    }
    fn publish(&mut self, topic_name: &str, message: &str) -> Result<(), String> {
        if let Some(topic) = self.topic.get(topic_name) {
            for subscriber in &topic.subscribers {
                println!("Sending message to {}: {}", subscriber, message);
            }
            Ok(())
        } else {
            Err("Topic does not exist".to_string())
        }
    }

    fn subscribe
}
fn main() {
    println!("Hello, world!");
}
