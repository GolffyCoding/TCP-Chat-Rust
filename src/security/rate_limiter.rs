use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RateLimiter {
    max_messages: u64,
    window_duration: Duration,
    counters: Arc<Mutex<RateLimitCounters>>,
}

#[derive(Debug)]
struct RateLimitCounters {
    count: u64,
    window_start: Instant,
}

impl Default for RateLimitCounters {
    fn default() -> Self {
        Self {
            count: 0,
            window_start: Instant::now(),
        }
    }
}

impl RateLimiter {
    pub fn new(max_messages: usize, window_secs: u64) -> Self {
        Self {
            max_messages: max_messages as u64,
            window_duration: Duration::from_secs(window_secs),
            counters: Arc::new(Mutex::new(RateLimitCounters::default())),
        }
    }

    pub async fn check_rate(&self, _client_id: usize) -> bool {
        let mut counters = self.counters.lock().await;
        let now = Instant::now();

        if now.duration_since(counters.window_start) >= self.window_duration {
            counters.count = 0;
            counters.window_start = now;
        }

        if counters.count >= self.max_messages {
            return false;
        }

        counters.count += 1;
        true
    }
}