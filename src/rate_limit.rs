use std::cell::Cell;
use tokio::time::{delay_for, delay_until, Duration, Instant};

use std::sync::Arc;
use tokio::sync::Mutex;

/// Rate limiter for `RedditApp` requests
#[derive(Clone)]
pub enum RateLimiter {
    Batched(RateLimiterBatched),
    Paced(RateLimiterPaced),
    Off,
}

impl RateLimiter {
    pub const RESET_HEADER: &'static str = "x-ratelimit-reset";
    pub const USED_HEADER: &'static str = "x-ratelimit-used";
    pub const REMANING_HEADER: &'static str = "x-ratelimit-remaining";

    /// Creates a new rate limiter using [RateLimiterBatched]
    pub fn new_batched() -> RateLimiter {
        RateLimiter::Batched(RateLimiterBatched::new())
    }

    /// Creates a new rate limiter using [RateLimiterPaced]
    pub fn new_paced() -> RateLimiter {
        RateLimiter::Paced(RateLimiterPaced::new())
    }

    pub fn should_wait(&self) -> bool {
        match self {
            RateLimiter::Batched(r) => r.should_wait(),
            RateLimiter::Paced(r) => r.should_wait(),
            _ => false,
        }
    }

    pub async fn wait(&self) {
        match self {
            RateLimiter::Batched(r) => r.wait().await,
            RateLimiter::Paced(r) => r.wait().await,
            _ => {}
        }
    }

    pub fn should_update(&self) -> bool {
        match self {
            RateLimiter::Off => false,
            _ => true,
        }
    }

    pub fn update(&self, tracker: RateLimiterTracker) {
        match self {
            RateLimiter::Batched(r) => r.update(tracker),
            RateLimiter::Paced(r) => r.update(tracker),
            _ => {}
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct RateLimiterTracker {
    remaning: i32,
    used: i32,
    reset_time: Instant,
}

impl RateLimiterTracker {
    pub fn new() -> Self {
        RateLimiterTracker {
            remaning: 100,
            used: 0,
            reset_time: Instant::now() + Duration::from_secs(60),
        }
    }

    pub fn new_safe() -> Arc<Mutex<Cell<Self>>> {
        Arc::from(Mutex::new(Cell::new(Self::new())))
    }

    pub fn from_values(remaning: i32, used: i32, reset_time: i32) -> Self {
        RateLimiterTracker {
            remaning: remaning,
            used: used,
            reset_time: Instant::now() + Duration::from_secs(reset_time as u64),
        }
    }

    pub fn set_remaning_time(&mut self, remaning: i32) {
        self.reset_time = Instant::now() + Duration::from_secs(remaning as u64);
    }
}
#[derive(Clone)]
pub struct RateLimiterBatched(Arc<Mutex<Cell<RateLimiterTracker>>>);

impl RateLimiterBatched {
    pub fn new() -> Self {
        RateLimiterBatched(RateLimiterTracker::new_safe())
    }

    pub fn should_wait(&self) -> bool {
        if let Ok(cell) = self.0.try_lock() {
            let inst = cell.get();
            inst.remaning == 0
        } else {
            false
        }
    }

    pub async fn wait(&self) {
        let reset_time = {
            let cell = self.0.lock().await;
            let inst = cell.get();
            inst.reset_time
        };
        delay_until(reset_time).await;
    }

    pub fn update(&self, tracker: RateLimiterTracker) {
        if let Ok(cell) = self.0.try_lock() {
            let mut inst = cell.get();
            inst.used = tracker.used;
            inst.remaning = tracker.remaning;
            inst.set_remaning_time(tracker.remaning);
            cell.set(inst);
        }
    }
}
#[derive(Clone)]
pub struct RateLimiterPaced(Arc<Mutex<Cell<RateLimiterTracker>>>);

impl RateLimiterPaced {
    pub fn new() -> Self {
        RateLimiterPaced(RateLimiterTracker::new_safe())
    }

    pub fn should_wait(&self) -> bool {
        true
    }

    pub async fn wait(&self) {
        let (remaning, reset_time) = {
            let cell = self.0.lock().await;
            let inst = cell.get();
            (inst.remaning, inst.reset_time)
        };

        let wait_for_s = (reset_time - Instant::now()).as_secs() as f32 / remaning as f32;
        if wait_for_s > 0.0 {
            delay_for(Duration::from_secs(wait_for_s as u64)).await;
        }
    }

    pub fn update(&self, tracker: RateLimiterTracker) {
        if let Ok(cell) = self.0.try_lock() {
            let mut inst = cell.get();
            inst.used = tracker.used;
            inst.remaning = tracker.remaning;
            inst.set_remaning_time(tracker.remaning);
            cell.set(inst);
        }
    }
}
