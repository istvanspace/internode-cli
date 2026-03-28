use serde_json::{json, Value};

use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

const BASE: &str = "/internode-tools/cli/oi/teams";

pub async fn list() -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let resp = client.get(BASE).await?;
    output::print_success(resp);
    Ok(())
}

pub async fn get(id: &str) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let resp = client.get(&format!("{BASE}/{id}")).await?;
    output::print_success(resp);
    Ok(())
}

pub async fn create(
    name: &str,
    key: Option<&str>,
    description: Option<&str>,
    members: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({ "name": name });
    if let Some(k) = key { body["key"] = Value::String(k.to_string()); }
    if let Some(d) = description { body["description"] = Value::String(d.to_string()); }
    if let Some(m) = members {
        let emails: Vec<Value> = m.split(',').map(|e| Value::String(e.trim().to_string())).collect();
        body["member_emails"] = Value::Array(emails);
    }
    let resp = client.post(BASE, &body).await?;
    output::print_success(resp);
    Ok(())
}

pub async fn update(
    id: &str,
    name: Option<&str>,
    key: Option<&str>,
    description: Option<&str>,
    members: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({});
    if let Some(n) = name { body["name"] = Value::String(n.to_string()); }
    if let Some(k) = key { body["key"] = Value::String(k.to_string()); }
    if let Some(d) = description { body["description"] = Value::String(d.to_string()); }
    if let Some(m) = members {
        let emails: Vec<Value> = m.split(',').map(|e| Value::String(e.trim().to_string())).collect();
        body["member_emails"] = Value::Array(emails);
    }
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
