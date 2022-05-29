use super::*;

pub struct ThreadCountClear {
    pub app_state: Data<AppState>,
}

impl ThreadCountClear {
    fn tick(&mut self, _ctx: &mut <Self as Actor>::Context) {
        let a = Data::clone(&self.app_state);
        tokio::spawn(async move {
            a.update_thread_count_cache().await;
        });
    }
}

impl Actor for ThreadCountClear {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        IntervalFunc::new(
            std::time::Duration::from_secs(config::THREAD_COUNT_CACHE_CLEAR_INTERVAL),
            Self::tick,
        )
        .finish()
        .spawn(ctx);
    }
}
