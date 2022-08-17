use anyhow::Result;

use crate::telegram_data::TelegramData;

pub(crate) struct TelegramFile {}

impl TelegramFile {
    const PART_CAPTION: &'static str = "caption";
    const PART_CHAT_ID: &'static str = "chat_id";
    const PART_PARSE_MODE: &'static str = "parse_mode";
    const PART_PHOTO: &'static str = "photo";

    pub fn with_photo_bits(
        data: &TelegramData,
        photo: Vec<u8>,
    ) -> Result<reqwest::multipart::Form> {
        let part_chat_id = reqwest::multipart::Part::text(data.chat_id.clone());
        let mut form = reqwest::multipart::Form::new().part(Self::PART_CHAT_ID, part_chat_id);

        if let Some(ref caption) = data.caption {
            let part_caption = reqwest::multipart::Part::text(caption.clone());
            form = form.part(Self::PART_CAPTION, part_caption);
        }

        let part_parse_mode = reqwest::multipart::Part::text(data.parse_mode.clone());
        let part_photo = reqwest::multipart::Part::bytes(photo)
            .file_name("photo.png")
            .mime_str("image/png")?;
        form = form
            .part(Self::PART_PARSE_MODE, part_parse_mode)
            .part(Self::PART_PHOTO, part_photo);

        Ok(form)
    }

    //pub fn with_photo_path(photo: PathBuf) -> Result<reqwest::multipart::Form> {
    //    let form = reqwest::multipart::Form::new().file(Self::PART_PHOTO, photo)?;

    //    Ok(form)
    //}
}
