use anyhow::Result;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use spin_sdk::http::Response;

use crate::simple_response;

pub fn slack_response(resp: &SlackSlashResponse) -> Result<Response> {
    let resp_bytes = serde_json::to_vec(resp)?;
    simple_response(StatusCode::OK, resp_bytes)
}

// https://api.slack.com/interactivity/slash-commands#app_command_handling
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[allow(dead_code)]
pub struct SlackSlashCommand {
    pub token: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
    pub trigger_id: String,
    pub user_id: String,
    pub user_name: String,
    pub team_id: String,
    pub team_domain: String,
    pub channel_id: String,
    pub channel_name: String,
    pub api_app_id: String,
}

#[derive(Serialize)]
pub struct SlackSlashResponse {
    pub response_type: ResponseType,
    pub text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum ResponseType {
    Ephemeral,
    InChannel,
}
