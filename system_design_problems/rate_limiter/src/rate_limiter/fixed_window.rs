use std::time::{SystemTime, UNIX_EPOCH};

use redis::AsyncCommands as _;

pub struct FixedWindow {
    pub window_size: u64,
    pub max_limit: u64,
}

impl super::RateLimiterStrategy for FixedWindow {
    async fn rate_limit(
        self,
        redis: &mut redis::aio::ConnectionManager,
        key: &str,
    ) -> Result<bool, ()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let window = (now / self.window_size) * self.window_size;
        let key = FixedWindow::build_key(key, &window.to_string());

        let count: u64 = redis.incr(key.clone(), 1).await.map_err(|_| ())?;
        if count == 1 {
            let expiry = self.window_size - (now - window);
            println!("expiry {:?}", expiry.clone());
            let _: () = redis
                .expire(key.clone(), expiry as i64)
                .await
                .map_err(|_| ())?;
        }
        if count > self.max_limit {
            println!("rate limited {key} ---------------  {count} ",);

            Ok(false)
        } else {
            println!("passed  {:?} ---------------  {:?} ", key, count);

            Ok(true)
        }
    }
}
