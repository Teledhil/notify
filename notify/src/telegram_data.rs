use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct TelegramData {
    pub chat_id: String,
    text: Option<String>,
    pub parse_mode: String,
    pub caption: Option<String>,
}

impl TelegramData {
    const MODE_MARKDOWN: &'static str = "MarkdownV2";

    pub fn with_text(title: String, body: String, chat_id: String) -> Self {
        let text = format!("*{}*\n{}", title, body);

        let text = Some(Self::escape(text));

        let parse_mode = Self::MODE_MARKDOWN.to_string();
        let caption = None;

        Self {
            chat_id,
            text,
            parse_mode,
            caption,
        }
    }

    pub fn with_caption(caption: String, chat_id: String) -> Self {
        let text = None;
        let parse_mode = Self::MODE_MARKDOWN.to_string();
        let caption = Some(Self::escape(caption));

        Self {
            chat_id,
            text,
            parse_mode,
            caption,
        }
    }

    fn escape(text: String) -> String {
        text.chars()
            .map(|x| match x {
                '.' => "\\.".to_string(),
                '-' => "\\-".to_string(),
                '[' => "\\[".to_string(),
                ']' => "\\]".to_string(),
                '(' => "\\(".to_string(),
                ')' => "\\)".to_string(),
                '%' => "\\%".to_string(),
                '=' => "\\=".to_string(),
                ':' => "\\:".to_string(),
                '>' => "\\>".to_string(),
                '<' => "\\<".to_string(),
                _ => x.to_string(),
            })
            .collect()
    }
}
