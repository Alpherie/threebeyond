use super::*;

pub struct DCaptchaClear {
    pub app_state: Data<AppState>,
}

impl DCaptchaClear {
    fn tick(&mut self, _ctx: &mut <Self as Actor>::Context) {
        let a = Data::clone(&self.app_state);
        tokio::spawn(async move {
            a.update_dcaptchas().await;
        });
    }
}

impl Actor for DCaptchaClear {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        IntervalFunc::new(
            std::time::Duration::from_secs(config::DIGIT_CAPTCHA_CLEAR_INTERVAL),
            Self::tick,
        )
        .finish()
        .spawn(ctx);
    }
}
