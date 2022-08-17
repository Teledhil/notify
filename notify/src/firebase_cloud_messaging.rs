use anyhow::{anyhow, Result};

use crate::firebase_data::FirebaseData;
use crate::firebase_header::FirebaseHeader;
use crate::message::Message;

pub(crate) struct FirebaseCloudMessaging {
    auth_key: String,
    topic: String,
}

impl FirebaseCloudMessaging {
    const URL: &'static str = "https://fcm.googleapis.com/fcm/send";

    pub fn new(auth_key: String, topic: String) -> Self {
        Self { auth_key, topic }
    }

    pub async fn send(&self, message: &Message) -> Result<reqwest::Response> {
        let data = match message {
            Message::TitleBody { title, body } => {
                FirebaseData::new(self.topic.clone(), title.clone(), body.clone())
            }
            Message::PhotoCaption {
                photo: _,
                caption: _,
            } => {
                return Err(anyhow!("No photos sent with firebase"));
            }
        };

        let headers = FirebaseHeader::headers(self.auth_key.clone());

        let client = reqwest::Client::new();
        let response = client
            .post(Self::URL)
            .json(&data)
            .headers(headers)
            .send()
            .await?;

        Ok(response)
    }
}
