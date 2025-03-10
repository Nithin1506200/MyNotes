use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

struct Influencer {
    id: String,
    followers: HashSet<String>,
}
impl Influencer {
    pub fn new(id: &String) -> Influencer {
        Influencer {
            id: id.clone(),
            followers: HashSet::new(),
        }
    }
}

struct User {
    id: String,
    feed: HashSet,
}
impl User {
    pub fn new(id: &String) -> User {
        User { id: id.clone() }
    }
}
struct Post {
    id: String,
    content: String,
    created_by: String,
    followed_by: HashSet<String>,
}
struct Hastag {
    id: String,
    content: String,
    // created_by: String,
    followed_by: HashSet<String>,
}
struct InstaPost {
    influencer: HashMap<String, Influencer>,
    user: HashMap<String, User>,
    post: HashMap<String, Post>,
    hashtag: HashMap<String, Hastag>,
}

enum Actor {
    User(String),
    Influencer(String),
}
enum Follow {
    Post(String),
    Hashtag(String),
}
impl InstaPost {
    pub fn new() -> InstaPost {
        InstaPost {
            influencer: HashMap::new(),
            user: HashMap::new(),
            post: HashMap::new(),
            hashtag: HashMap::new(),
        }
    }
    fn create_influencer(&mut self, id: &String) -> Result<(), String> {
        match self.get_influencer(&id) {
            Some(_) => Err("USER ALREADY EXISTS".to_string()),
            None => {
                self.influencer.insert(id.clone(), Influencer::new(&id));
                Ok(())
            }
        }
    }
    fn create_user(&mut self, id: &String) -> Result<(), String> {
        match self.get_user(&id) {
            Some(_) => Err("USER ALREADY EXISTS".to_string()),
            None => {
                self.influencer.insert(id.clone(), Influencer::new(&id));
                Ok(())
            }
        }
    }
    fn get_user(&self, id: &String) -> Option<&User> {
        self.user.get(id)
    }
    fn get_influencer(&self, id: &String) -> Option<&Influencer> {
        self.influencer.get(id)
    }

    fn follow_influencer(&mut self, user: String, influencer: String) -> Result<(), String> {
        match (&self.get_user(&user), self.get_influencer(&influencer)) {
            (None, Some(_)) => Err("Influencer doesn't exist!".to_string()),
            (None, None) => Err("User doesn't exist!".to_string()),
            (Some(_), None) => Err("User and influencer doesn't exist!".to_string()),
            (Some(user), Some(influencers)) => {
                // influencer.followers.push(user.id.clone());
                // influencers.followers.insert(user.id.clone());
                Ok(())
            }
        }
    }

    pub fn create(&mut self, actor: Actor) -> Result<(), String> {
        match actor {
            Actor::User(id) => self.create_user(&id),
            Actor::Influencer(id) => self.create_influencer(&id),
        }
    }
    pub fn follow(&mut self, flollow: Follow) -> Result<(), String> {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
