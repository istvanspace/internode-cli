use serde_json::{json, Value};

use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

const BASE: &str = "/internode-tools/cli/oi/topics";

pub async fn list(
    limit: Option<i64>,
    offset: Option<i64>,
    search: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut params = vec![];
    if let Some(l) = limit { params.push(format!("limit={l}")); }
    if let Some(o) = offset { params.push(format!("offset={o}")); }
    if let Some(s) = search { params.push(format!("search={}", urlenc(s))); }
    let qs = if params.is_empty() { String::new() } else { format!("?{}", params.join("&")) };
    let resp = client.get(&format!("{BASE}{qs}")).await?;
    output::print_success(resp);
    Ok(())
}

pub async fn get(id: &str, with_related: bool) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let qs = if with_related { "?with_related=true" } else { "" };
    let resp = client.get(&format!("{BASE}/{id}{qs}")).await?;
    output::print_success(resp);
    Ok(())
}

pub async fn create(
    title: &str,
    description: Option<&str>,
    conclusion: Option<&str>,
    conclusion_type: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({ "topic_title": title });
    if let Some(d) = description { body["topic_description"] = Value::String(d.to_string()); }
    if let Some(c) = conclusion { body["topic_conclusion"] = Value::String(c.to_string()); }
    if let Some(ct) = conclusion_type { body["topic_conclusion_type"] = Value::String(ct.to_string()); }
    let resp = client.post(BASE, &body).await?;
    output::print_success(resp);
    Ok(())
}

pub async fn update(
    id: &str,
    title: Option<&str>,
    description: Option<&str>,
    conclusion: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({});
    if let Some(t) = title { body["topic_title"] = Value::String(t.to_string()); }
    if let Some(d) = description { body["topic_description"] = Value::String(d.to_string()); }
    if let Some(c) = conclusion { body["topic_conclusion"] = Value::String(c.to_string()); }
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
