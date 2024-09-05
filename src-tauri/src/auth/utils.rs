use std::{error::Error, io};
use tauri::AppHandle;
use url::Url;

pub fn get_code_from_url(url: String) -> Result<String, Box<dyn Error>> {
    let url = Url::parse(&url)?;
    match url.query_pairs().find(|(key, _)| key == "code") {
        Some((_, value)) => Ok(value.into_owned()),
        None => Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "No code found"))),
    }
}

pub fn open_custom_auth_window(app: &AppHandle) {
    let url = reqwest::Url::parse_with_params(
        "https://id.twitch.tv/oauth2/authorize",
        &[
            ("client_id", "cig4pc07b7bxo207x8158v58r1i5pf"),
            ("response_type", "code"),
            ("redirect_uri", "http://localhost:8457"),
            ("scope", "channel:manage:predictions moderator:manage:announcements"),
        ],
    )
    .unwrap();

    tauri::WindowBuilder::new(app, "twitch_bot_auth", tauri::WindowUrl::External(url))
        .title("Flow - Custom Bot")
        .center()
        .always_on_top(true)
        .focused(true)
        .inner_size(500.0, 700.0)
        .build()
        .expect("failed to build window");
}
