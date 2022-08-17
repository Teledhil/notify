use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use notify::message::Message;
use notify::Notify;

include!(concat!(env!("OUT_DIR"), "/get_credentials.rs"));

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Body of the message
    text: Option<String>,

    /// Title of the message
    #[clap(short, long, value_parser, default_value_t=get_hostname())]
    title: String,

    /// Path to a file. Contents of the file will be sent as text
    #[clap(long, value_parser, conflicts_with("text"))]
    text_file: Option<PathBuf>,

    /// Path to a picture
    #[clap(short, long, value_parser)]
    photo: Option<PathBuf>,
}

fn get_hostname() -> String {
    hostname::get()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let body = args.text.unwrap_or_default();

    let message = if let Some(photo_path) = args.photo {
        Message::new_photo_caption(photo_path, body).await?
    } else {
        Message::new_title_body(args.title, body)
    };

    let credentials = get_credentials();

    let notify = Notify::new(&credentials);

    notify.send(&message).await?;

    Ok(())
}
