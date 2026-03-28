use serde_json::{json, Value};

use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

const BASE: &str = "/internode-tools/cli/oi/decisions";

pub async fn get(id: &str, with_related: bool) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let qs = if with_related { "?with_related=true" } else { "" };
    let resp = client.get(&format!("{BASE}/{id}{qs}")).await?;
    output::print_success(resp);
    Ok(())
}

pub async fn update(
    id: &str,
    title: Option<&str>,
    description: Option<&str>,
    status: Option<&str>,
    priority: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({});
    if let Some(t) = title { body["decision_title"] = Value::String(t.to_string()); }
    if let Some(d) = description { body["description"] = Value::String(d.to_string()); }
    if let Some(s) = status { body["decision_status"] = Value::String(s.to_string()); }
    if let Some(p) = priority { body["priority"] = Value::String(p.to_string()); }
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
