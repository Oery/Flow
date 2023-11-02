use crate::state::{AppState, Streamer};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use std::io;
use tauri::{AppHandle, Manager, State};
use windows::core::HSTRING;
use windows::Security::Credentials::{PasswordCredential, PasswordVault};

#[allow(dead_code)]
#[derive(Deserialize)]
struct ValidateTokenResponse {
    client_id: String,
    login: String,
    scopes: Vec<String>,
    user_id: String,
    expires_in: i64,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct RefreshTokenResponse {
    token: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct GetStreamerInfoResponse {
    data: Vec<User>,
}

pub fn save_token(resource: &str, token: &str) -> Result<(), Box<dyn Error>> {
    let vault = PasswordVault::new()?;
    let resource_hstring = HSTRING::from(resource);
    let flow_hstring = HSTRING::from("Flow");
    let token_hstring = HSTRING::from(token);

    let credential_result = vault.Retrieve(&resource_hstring, &flow_hstring);
    match credential_result {
        Ok(credential) => {
            vault.Remove(&credential)?;
            let new_credential = PasswordCredential::CreatePasswordCredential(
                &resource_hstring,
                &flow_hstring,
                &token_hstring,
            )?;
            vault.Add(&new_credential)?;
            println!("Token saved successfully");
        }
        Err(_) => {
            let new_credential = PasswordCredential::CreatePasswordCredential(
                &resource_hstring,
                &flow_hstring,
                &token_hstring,
            )?;
            vault.Add(&new_credential)?;
        }
    }

    Ok(())
}

pub fn get_token(resource: &str) -> Result<String, Box<dyn Error>> {
    let vault = PasswordVault::new()?;
    let credential = vault.Retrieve(&HSTRING::from(resource), &HSTRING::from("Flow"))?;
    credential.RetrievePassword()?;
    Ok(credential.Password()?.to_string_lossy())
}

pub fn delete_token(resource: String) -> Result<(), Box<dyn Error>> {
    let vault = PasswordVault::new()?;
    let credential_result = vault.Retrieve(&HSTRING::from(resource), &HSTRING::from("Flow"));

    if let Ok(credential) = credential_result {
        vault.Remove(&credential)?;
    }

    // TODO UPDATE CONTEXT AND DISCONNECT USER

    Ok(())
}

pub fn revoke_twitch_token(token: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let params = [
        ("client_id", "cig4pc07b7bxo207x8158v58r1i5pf"),
        ("token", &token),
    ];

    let response = client
        .post("https://id.twitch.tv/oauth2/revoke")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .unwrap();

    if !response.status().is_success() {
        let error_text = response.text().unwrap();
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, error_text)));
    }

    delete_token("TwitchOAuthToken".to_string())?;
    Ok(())
}

pub fn validate_twitch_token(token: &String, app: &AppHandle) -> Result<(), Box<dyn Error>> {
    println!("Validating Token... {}", token);
    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://id.twitch.tv/oauth2/validate")
        .header(AUTHORIZATION, format!("OAuth {token}"))
        .send()
        .unwrap();

    if !response.status().is_success() {
        if response.status() == StatusCode::UNAUTHORIZED {
            println!("TOKEN EXPIRED");
            refresh_twitch_token(&token, app)?;
        }

        let error_text = response.text().unwrap();
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, error_text)));
    }

    let text = response.text()?;
    let json: ValidateTokenResponse = serde_json::from_str(&text)?;

    println!("Request successful");
    println!("{}", text);

    save_token("TwitchOAuthToken", &token)?;
    get_streamer_info(token, &json.user_id, app)?;
    Ok(())
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
struct User {
    id: String,
    login: String,
    display_name: String,
    #[serde(rename = "type")]
    type_field: String,
    broadcaster_type: String,
    description: String,
    profile_image_url: String,
    offline_image_url: String,
    view_count: i32,
    created_at: String,
}

fn get_streamer_info(token: &String, id: &String, app: &AppHandle) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let params = [("id", id)];

    let url = reqwest::Url::parse_with_params("https://api.twitch.tv/helix/users", &params)?;
    println!("URL : {}", url);

    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .header("Client-Id", "cig4pc07b7bxo207x8158v58r1i5pf")
        .send()
        .unwrap();

    if !response.status().is_success() {
        let error_text = response.text().unwrap();
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, error_text)));
    }

    let text = response.text()?;
    let json: GetStreamerInfoResponse = serde_json::from_str(&text)?;

    println!("Request successful");
    println!("{}", text);

    let twitch_streamer: User = json.data[0].clone();
    let streamer = Streamer {
        display_name: twitch_streamer.display_name,
        id: twitch_streamer.id,
        color: Streamer::default().color,
        avatar_url: twitch_streamer.profile_image_url,
    };

    let app_state: State<AppState> = app.state();
    let mut locked_app_state = app_state.data.write().unwrap();
    *locked_app_state = {
        let mut app_state = locked_app_state.clone();
        app_state.twitch_access_token = token.clone();
        app_state.streamer = streamer;
        let _ = app.emit_all("loading-end", true);
        let _ = app.emit_all("update-context", &app_state);
        app_state
    };

    // TODO Update le contexte avec le streamer

    Ok(())
}

fn refresh_twitch_token(expired_token: &String, app: &AppHandle) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let data = json!({
        "expired_token": expired_token
    });
    println!("Refreshing Token");
    println!("{}", expired_token.to_string());

    let response = client
        .post("https://api.oery.dev/oauth/twitch/refresh")
        .json(&data)
        .send()
        .unwrap();

    if !response.status().is_success() {
        // TODO Handle Flow API Error
        // if response.status() == StatusCode::UNAUTHORIZED {}

        let error_text = response.text()?;
        println!("REQUETE A FOIREE {}", error_text);
        return Err(Box::new(io::Error::new(io::ErrorKind::Other, error_text)));
    }

    let body: RefreshTokenResponse = response.json()?;
    save_token("TwitchOAuthToken", &body.token)?;
    validate_twitch_token(&body.token, app)?;
    Ok(())
}

// TODO REFRESH TOKEN ONCE ON 401
