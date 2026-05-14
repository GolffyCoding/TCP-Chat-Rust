use std::time::{Duration, Instant};
use std::future::Future;
use std::pin::Pin;

pub struct TimeoutManager {
    idle_timeout: Duration,
    heartbeat_interval: Duration,
}

impl TimeoutManager {
    pub fn new(idle_timeout_secs: u64) -> Self {
        Self {
            idle_timeout: Duration::from_secs(idle_timeout_secs),
            heartbeat_interval: Duration::from_secs(30),
        }
    }

    pub fn idle_timeout(&self) -> Duration {
        self.idle_timeout
    }

    pub fn heartbeat_interval(&self) -> Duration {
        self.heartbeat_interval
    }

    pub fn create_idle_timer(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let duration = self.idle_timeout;
        Box::pin(async move {
            tokio::time::sleep(duration).await;
        })
    }
}

pub struct ClientTimer {
    last_activity: Instant,
}

impl Default for ClientTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientTimer {
    pub fn new() -> Self {
        Self {
            last_activity: Instant::now(),
        }
    }

    pub fn update_activity(&mut self) {
        self.last_activity = Instant::now();
    }

    pub fn is_expired(&self, timeout: Duration) -> bool {
        self.last_activity.elapsed() > timeout
    }
}