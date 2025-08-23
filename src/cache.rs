use redis::AsyncCommands;
use serde_json;
use crate::models::User;

pub struct Cache {
    client: redis::Client,
}

impl Cache {
    pub fn new(redis_url: &str) -> Self {
        let client = redis::Client::open(redis_url).unwrap();
        Self { client }
    }

    pub async fn get_user(&self, id: i64) -> Option<User> {
        let mut con = self.client.get_async_connection().await.unwrap();
        let result: Option<String> = con.get(id).await.unwrap();
        result.map(|s| serde_json::from_str(&s).unwrap())
    }

    pub async fn set_user(&self, id: i64, user: &User) {
        let mut con = self.client.get_async_connection().await.unwrap();
        let user_json = serde_json::to_string(user).unwrap();
        con.set(id, user_json).await.unwrap();
    }
}

