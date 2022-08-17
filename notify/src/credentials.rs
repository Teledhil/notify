pub struct Credentials {
    telegram_token: String,
    telegram_chat_id: String,
    firebase_auth_key: String,
    firebase_topic: String,
}

impl Credentials {
    pub fn new(
        telegram_token: String,
        telegram_chat_id: String,
        firebase_auth_key: String,
        firebase_topic: String,
    ) -> Self {
        Self {
            telegram_token,
            telegram_chat_id,
            firebase_auth_key,
            firebase_topic,
        }
    }

    pub fn telegram_token(&self) -> &str {
        &self.telegram_token
    }

    pub fn telegram_chat_id(&self) -> &str {
        &self.telegram_chat_id
    }

    pub fn firebase_auth_key(&self) -> &str {
        &self.firebase_auth_key
    }

    pub fn firebase_topic(&self) -> &str {
        &self.firebase_topic
    }
}
