use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct FirebaseData {
    data: Data,
    notification: Notification,
    priority: String,
    to: String,
    light_settings: LightSettings,
}

impl FirebaseData {
    const PRIORITY: &'static str = "high";

    pub fn new(to: String, title: String, body: String) -> Self {
        let data = Data::new();
        let priority = Self::PRIORITY.to_string();
        let notification = Notification::new(title, body);
        let light_settings = LightSettings::new();

        Self {
            data,
            notification,
            priority,
            to,
            light_settings,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    click_action: String,
    id: u32,
    status: String,
}

impl Data {
    const FLUTTER_NOTIFICATION_CLICK: &'static str = "FLUTTER_NOTIFICATION_CLICK";
    const STATUS: &'static str = "done";

    fn new() -> Self {
        let click_action = Self::FLUTTER_NOTIFICATION_CLICK.to_string();
        let id = 1;
        let status = Self::STATUS.to_string();

        Self {
            click_action,
            id,
            status,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Notification {
    title: String,
    body: String,
}

impl Notification {
    fn new(title: String, body: String) -> Self {
        Self { title, body }
    }
}

#[derive(Serialize, Deserialize)]
struct LightSettings {
    color: Color,
    light_on_duration: String,
    light_off_duration: String,
}

impl LightSettings {
    const DURATION: &'static str = "3.5s";

    fn new() -> Self {
        let color = Color::new();
        let light_on_duration = Self::DURATION.to_string();
        let light_off_duration = Self::DURATION.to_string();

        Self {
            color,
            light_on_duration,
            light_off_duration,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Color {
    red: u32,
    green: u32,
    blue: u32,
    alpha: u32,
}

impl Color {
    const RED: u32 = 0;
    const GREEN: u32 = 0;
    const BLUE: u32 = 128;
    const ALPHA: u32 = 255;

    fn new() -> Self {
        let red = Self::RED;
        let green = Self::GREEN;
        let blue = Self::BLUE;
        let alpha = Self::ALPHA;

        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}
