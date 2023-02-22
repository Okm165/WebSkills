pub use redis;
use redis::{Client, RedisResult};

pub fn get_client() -> RedisResult<Client> {
    Client::open(std::env::var("REDIS_URL").unwrap_or_default().as_str())
}

#[cfg(test)]
pub mod tests;
