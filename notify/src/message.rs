use std::path::PathBuf;

use anyhow::Result;
use tokio::fs;

pub enum Message {
    TitleBody { title: String, body: String },
    PhotoCaption { photo: Vec<u8>, caption: String },
}

impl Message {
    pub fn new_title_body(title: String, body: String) -> Self {
        Message::TitleBody { title, body }
    }

    pub async fn new_title_body_file(title: String, body_path: PathBuf) -> Result<Self> {
        let body = fs::read_to_string(body_path).await?;

        Ok(Message::TitleBody { title, body })
    }

    pub async fn new_photo_caption(photo_path: PathBuf, caption: String) -> Result<Self> {
        let photo = fs::read(photo_path).await?;

        Ok(Message::PhotoCaption { photo, caption })
    }
}
