use anyhow::Result;

pub mod credentials;
mod firebase_cloud_messaging;
mod firebase_data;
mod firebase_header;
pub mod message;
mod telegram;
mod telegram_data;
mod telegram_file;

use credentials::Credentials;
use firebase_cloud_messaging::FirebaseCloudMessaging;
use message::Message;
use telegram::Telegram;

pub struct Notify {
    telegram: Telegram,
    firebase: FirebaseCloudMessaging,
}

impl Notify {
    pub fn new(credentials: &Credentials) -> Self {
        let telegram = Telegram::new(
            credentials.telegram_token().to_string(),
            credentials.telegram_chat_id().to_string(),
        );
        let firebase = FirebaseCloudMessaging::new(
            credentials.firebase_auth_key().to_string(),
            credentials.firebase_topic().to_string(),
        );

        Self { telegram, firebase }
    }

    pub async fn send(&self, message: &Message) -> Result<()> {
        let (_telegram_response, _firebase_response) =
            tokio::join!(self.telegram.send(message), self.firebase.send(message),);

        //println!("Telegram response: {telegram_response:?}");
        //println!(
        //"Telegram response body: {}",
        //telegram_response?.text().await?
        //);
        //println!("Firebase response: {firebase_response:?}");

        Ok(())
    }
}
