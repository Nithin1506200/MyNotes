use std::time::{SystemTime, UNIX_EPOCH};

use redis::AsyncCommands as _;

pub struct SlidingWindowCounter {
    // duration in seconds
    pub window_size: u64,
    pub max_limit: u64,
    pub window_slots: u8,
}

impl super::RateLimiterStrategy for SlidingWindowCounter {
    async fn rate_limit(
        self,
        redis: &mut redis::aio::ConnectionManager,
        key: &str,
    ) -> Result<bool, ()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let window_slot_size: u64 = self.window_size / self.window_slots as u64;
        let window_slot = (now / window_slot_size) * window_slot_size;
        let window_key: String = SlidingWindowCounter::build_key(key, &window_slot.to_string());

        let current_count: u64 = redis.incr(window_key.clone(), 1).await.map_err(|_| ())?;
        if current_count == 1 {
            let expiry = self.window_size - (now - window_slot);
            println!("expiry {:?}", expiry.clone());
            let _: () = redis
                .expire(window_key.clone(), expiry as i64)
                .await
                .map_err(|_| ())?;
        }
        let total_count = current_count + {
            let window_slots: Vec<_> = (1..self.window_slots)
                .map(|i| {
                    SlidingWindowCounter::build_key(
                        key,
                        &(window_slot - i as u64 * window_slot_size).to_string(),
                    )
                })
                .collect();
            let v: Result<Vec<Option<u64>>, _> = redis.mget(window_slots).await;
            let v = v.map_err(|_| ())?;
            let e: u64 = v.iter().filter_map(|x| x.clone()).sum();
            e
        };
        if total_count > self.max_limit {
            println!("rate limited {window_key} ---------------  {total_count} ",);

            Ok(false)
        } else {
            println!(
                "passed  {:?} ---------------  {:?} ",
                window_key, total_count
            );

            Ok(true)
        }
    }
}
