use super::*;
pub use std::{
    collections::HashMap,
    net::IpAddr,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::RwLock;

mod app_settings;
mod stats;
pub use app_settings::AppSettings;
pub use stats::StatisticsCache;

pub struct AppState {
    // IDEA: Use RCU pattern instead of RWLock. (There should be two arcs, which will be automatically switched on Read mode)
    stats_updatng: AtomicBool,
    stats_last_updated: AtomicU64,
    pub last_posts: Mutex<HashMap<IpAddr, Instant>>,
    pub stats: RwLock<StatisticsCache>,
    pub dcaptchas: Mutex<HashMap<String, NaiveDateTime>>,
    pub threads_count_cache: Mutex<HashMap<(String, String, u64), (u64, NaiveDateTime)>>,
    pub settings: Mutex<AppSettings>,
}

impl AppState {
    pub async fn new(conn: Conn) -> Result<(Conn, Self), MysqlError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let (conn, stats) = StatisticsCache::get(conn).await?;

        Ok((
            conn,
            Self {
                stats: RwLock::new(stats),
                stats_updatng: AtomicBool::new(false),
                stats_last_updated: AtomicU64::new(now),
                last_posts: Mutex::new(HashMap::new()),
                dcaptchas: Default::default(),
                settings: Mutex::new(AppSettings::default()),
                threads_count_cache: Default::default(),
            },
        ))
    }

    pub async fn set_thread_count_cache(
        &self,
        (lang, board, num): (impl Into<String>, impl Into<String>, u64),
        b: u64,
    ) -> Option<(u64, NaiveDateTime)> {
        self.threads_count_cache
            .lock()
            .await
            .insert((lang.into(), board.into(), num), (b, time_now()))
    }

    pub async fn set_thread_counts_cache(&self, list: &[((String, String, u64), u64)]) {
        let mut lock = self.threads_count_cache.lock().await;
        for ((lang, board, num), v) in list {
            lock.insert((lang.into(), board.into(), *num), (*v, time_now()));
        }
    }

    pub async fn update_thread_count_cache(&self) {
        let mut lock = self.threads_count_cache.lock().await;
        let mut new = HashMap::new();

        for (key, v) in lock.iter() {
            if v.1 + chrono::Duration::seconds(config::THREAD_COUNT_CACHE_LIFETIME) > time_now() {
                new.insert(key.clone(), v.clone());
            }
        }

        *lock = new;
    }

    pub async fn update_dcaptchas(&self) {
        let mut lock = self.dcaptchas.lock().await;
        let mut new = HashMap::new();

        for (key, datetime) in lock.iter() {
            if *datetime + chrono::Duration::seconds(config::DIGIT_CAPTCHA_LIFETIME) > time_now() {
                new.insert(key.clone(), datetime.clone());
            }
        }

        *lock = new;
    }

    pub async fn validate_dcaptcha(&self, s: &str) -> bool {
        self.dcaptchas.lock().await.remove(s).is_some()
    }

    pub async fn gen_dcaptcha(&self) -> (String, Vec<u8>) {
        use captcha::{
            filters::{Dots, Noise, Wave},
            Captcha,
        };
        let (s, v) = Captcha::new()
            .add_chars(5)
            .apply_filter(Noise::new(0.2))
            .apply_filter(Wave::new(1.0, 10.0).horizontal())
            .apply_filter(Wave::new(1.0, 10.0).vertical())
            .view(220, 90)
            .apply_filter(Dots::new(3))
            .as_tuple()
            .unwrap();

        self.dcaptchas.lock().await.insert(s.clone(), time_now());

        (s, v)
    }

    // IDEA: just spawn another future, maybe field with conn is required.
    pub async fn stats(&self, conn: Conn) -> Result<(Conn, StatisticsCache), MysqlError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        if self.stats_last_updated.load(Ordering::SeqCst) + config::APPSTATE_STATS_CACHE_INTERVAL
            < now
            && !self.stats_updatng.load(Ordering::SeqCst)
        {
            // update
            self.stats_updatng.store(true, Ordering::SeqCst);
            let (conn, stats) = StatisticsCache::get(conn).await?;
            *self.stats.write().await = stats.clone();
            self.stats_last_updated.store(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                Ordering::SeqCst,
            );
            self.stats_updatng.store(false, Ordering::SeqCst);
            Ok((conn, stats))
        } else {
            // read
            Ok((conn, self.stats.read().await.clone()))
        }
    }
}
