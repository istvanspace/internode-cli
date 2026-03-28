use serde_json::{json, Value};

use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

const BASE: &str = "/internode-tools/cli/oi/statuses";

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
    description: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({ "name": name, "oiteam_id": team });
    if let Some(d) = description { body["description"] = Value::String(d.to_string()); }
    let resp = client.post(BASE, &body).await?;
    output::print_success(resp);
    Ok(())
}

pub async fn update(
    id: &str,
    name: Option<&str>,
    description: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({});
    if let Some(n) = name { body["name"] = Value::String(n.to_string()); }
    if let Some(d) = description { body["description"] = Value::String(d.to_string()); }
    let resp = client.patch(&format!("{BASE}/{id}"), &body).await?;
    output::print_success(resp);
    Ok(())
}

pub async fn delete(id: &str) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let resp = client.delete(&format!("{BASE}/{id}")).await?;
    output::print_success(resp);
    Ok(())
}

fn urlenc(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
