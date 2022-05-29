use super::*;

#[derive(Default, Copy, Clone, Deserialize, Serialize)]
pub struct AppSettings {
    pub spamhaus_rejection: bool,
}
