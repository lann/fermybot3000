use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SlackSlashCommand {
    token: String,
    team_id: String,
    team_domain: String,
    channel_id: String,
    channel_name: String,
    user_id: String,
    user_name: String,
    command: String,
    api_app_id: String,
    // is_enterprise_install: bool,
    response_url: String,
    trigger_id: String,
}

#[derive(Serialize)]
struct SlackSlashResponse {
    response_type: ResponseType,
    text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
enum ResponseType {
    Ephemeral,
    InChannel,
}

/// A simple Spin HTTP component.
#[http_component]
fn fermybot3000(req: Request) -> Result<Response> {
    let slash_cmd: Option<SlackSlashCommand> = req
        .body()
        .as_deref()
        .map(serde_urlencoded::from_bytes)
        .transpose()?;
    println!("Command: {slash_cmd:?}");
    let resp = SlackSlashResponse {
        response_type: ResponseType::InChannel,
        text: "🎶 🕷️ 🦇, 🕷️ 🦇 🎶".to_string(),
    };
    let resp_bytes = serde_json::to_vec(&resp)?;
    println!("Response: {}", String::from_utf8_lossy(&resp_bytes));
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(resp_bytes.into()))?)
}
