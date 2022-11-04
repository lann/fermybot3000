use anyhow::{Context, Result};
use bytes::Bytes;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{Request, Response},
    http_component, redis,
};

// e.g. redis://<username>:<password>@<hostname>:<port>
const REDIS_URL: &str = include_str!("../redis_url");

/// A simple Spin HTTP component.
#[http_component]
fn fermybot3000(req: Request) -> Result<Response> {
    match req.uri().path() {
        "/slack/spyderbat" => spyderbat(),
        "/slack/incr" => incr(req),
        "/debug" => debug(req),
        _ => simple_response(StatusCode::NOT_FOUND, "nope"),
    }
}

fn spyderbat() -> Result<Response> {
    slack_response(&SlackSlashResponse {
        response_type: ResponseType::InChannel,
        text: "🎶 🕷️ 🦇, 🕷️ 🦇 🎶".to_string(),
    })
}

fn incr(req: Request) -> Result<Response> {
    let body = req.body().as_deref().context("no body")?;
    let cmd: SlackSlashCommand = serde_urlencoded::from_bytes(body)?;
    let what = cmd.text.split_whitespace().collect::<Vec<_>>().join(" ");
    if what.is_empty() {
        return simple_response(StatusCode::OK, "incr what?");
    }
    let key = format!("incr:{}", what.to_lowercase());
    let val =
        redis::incr(REDIS_URL, &key).map_err(|err| anyhow::format_err!("redis error: {err:?}"))?;
    let mut text = format!("{what} is now {val}");
    if val > 9 {
        text.push_str("; wow!")
    }
    slack_response(&SlackSlashResponse {
        response_type: ResponseType::InChannel,
        text,
    })
}

fn debug(req: Request) -> Result<Response> {
    let slash_cmd: Option<SlackSlashCommand> = req
        .body()
        .as_deref()
        .map(serde_urlencoded::from_bytes)
        .transpose()?;
    println!("Command: {slash_cmd:?}");
    Ok(http::Response::new(None))
}

fn slack_response(resp: &SlackSlashResponse) -> Result<Response> {
    let resp_bytes = serde_json::to_vec(resp)?;
    simple_response(StatusCode::OK, resp_bytes)
}

fn simple_response(status: StatusCode, body: impl Into<Bytes>) -> Result<Response> {
    Ok(http::Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Some(body.into()))?)
}

// https://api.slack.com/interactivity/slash-commands#app_command_handling
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[allow(dead_code)]
struct SlackSlashCommand {
    token: String,
    command: String,
    text: String,
    response_url: String,
    trigger_id: String,
    user_id: String,
    user_name: String,
    team_id: String,
    team_domain: String,
    channel_id: String,
    channel_name: String,
    api_app_id: String,
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
