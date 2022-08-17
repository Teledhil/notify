use anyhow::Result;

use crate::message::Message;
use crate::telegram_data::TelegramData;
use crate::telegram_file::TelegramFile;

pub(crate) struct Telegram {
    token: String,
    chat_id: String,
}

impl Telegram {
    const BASE_URL: &'static str = "https://api.telegram.org/bot";
    //const BASE_URL: &'static str = "http://127.0.0.1:8000/bot";
    const COMMAND_MESSAGE: &'static str = "/sendMessage";
    const COMMAND_PHOTO: &'static str = "/sendPhoto";

    pub fn new(token: String, chat_id: String) -> Self {
        Self { token, chat_id }
    }

    pub async fn send(&self, message: &Message) -> Result<reqwest::Response> {
        let client = reqwest::Client::new();

        let url = match message {
            Message::TitleBody { title: _, body: _ } => {
                format!("{}{}{}", Self::BASE_URL, self.token, Self::COMMAND_MESSAGE)
            }
            Message::PhotoCaption {
                photo: _,
                caption: _,
            } => format!("{}{}{}", Self::BASE_URL, self.token, Self::COMMAND_PHOTO),
        };
        let mut request = client.post(url.clone());

        let data = match message {
            Message::TitleBody { title, body } => {
                TelegramData::with_text(title.clone(), body.clone(), self.chat_id.clone())
            }
            Message::PhotoCaption { photo: _, caption } => {
                TelegramData::with_caption(caption.clone(), self.chat_id.clone())
            }
        };
        request = request.json(&data);

        if let Message::PhotoCaption { photo, caption: _ } = message {
            let form = TelegramFile::with_photo_bits(&data, photo.clone())?;
            let mut request = client.post(url);
            request = request.multipart(form);
            //println!("Telegram: Sending photo request: {request:?}");
            let response = request.send().await?;
            //println!("Sending photo");
            return Ok(response);
        }

        //println!("Telegram: Sending request: {request:?}");

        let response = request.send().await?;

        Ok(response)
    }
}
