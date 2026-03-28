use serde_json::{json, Value};

use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

const BASE: &str = "/internode-tools/cli/oi/projects";

pub async fn list(team: Option<&str>) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let qs = match team {
        Some(t) => format!("?oiteam_id={}", urlenc(t)),
        None => String::new(),
    };
    let resp = client.get(&format!("{BASE}{qs}")).await?;
    output::print_success(resp);
    Ok(())
}

pub async fn create(
    name: &str,
    team: &str,
    key: Option<&str>,
    description: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({ "name": name, "oiteam_id": team });
    if let Some(k) = key { body["key"] = Value::String(k.to_string()); }
    if let Some(d) = description { body["description"] = Value::String(d.to_string()); }
    let resp = client.post(BASE, &body).await?;
    output::print_success(resp);
    Ok(())
}

fn urlenc(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
