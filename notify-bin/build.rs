use std::env;
use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};
use configparser::ini::Ini;
use notify::credentials::Credentials;

const CREDENTIALS_FILE: &str = "credentials.ini";

const TELEGRAM_SECTION: &str = "telegram";
const TELEGRAM_TOKEN: &str = "token";
const TELEGRAM_CHAT_ID: &str = "chat_id";

const FIREBASE_SECTION: &str = "firebase";
const FIREBASE_AUTH_KEY: &str = "auth_key";
const FIREBASE_TOPIC: &str = "topic";

fn get_value(config: &Ini, section: &str, key: &str) -> Result<String> {
    config
        .get(section, key)
        .ok_or_else(|| anyhow!("missing {section} {key}"))
}

fn load_credentials() -> Result<Credentials> {
    let mut config = Ini::new();

    if let Err(e) = config.load(CREDENTIALS_FILE) {
        return Err(anyhow!("failed to load {}: {}", CREDENTIALS_FILE, e));
    }

    let telegram_token = get_value(&config, TELEGRAM_SECTION, TELEGRAM_TOKEN)?;
    let telegram_chat_id = get_value(&config, TELEGRAM_SECTION, TELEGRAM_CHAT_ID)?;

    let firebase_auth_key = get_value(&config, FIREBASE_SECTION, FIREBASE_AUTH_KEY)?;
    let firebase_topic = get_value(&config, FIREBASE_SECTION, FIREBASE_TOPIC)?;

    let credentials = Credentials::new(
        telegram_token,
        telegram_chat_id,
        firebase_auth_key,
        firebase_topic,
    );

    Ok(credentials)
}

fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("get_credentials.rs");

    let credentials = load_credentials()?;

    let generated_code = format!(
        "
        use notify::credentials::Credentials;

        fn get_credentials() -> Credentials {{
            let telegram_token = {}.to_string();
            let telegram_chat_id = {}.to_string();
            let firebase_auth_key = {}.to_string();
            let firebase_topic = {}.to_string();

            Credentials::new(telegram_token, telegram_chat_id, firebase_auth_key, firebase_topic)
        }}
        ",
        credentials.telegram_token(),
        credentials.telegram_chat_id(),
        credentials.firebase_auth_key(),
        credentials.firebase_topic(),
    );

    fs::write(dest_path, generated_code).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=credentials.ini");

    Ok(())
}
