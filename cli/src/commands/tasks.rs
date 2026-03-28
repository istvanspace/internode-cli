use serde_json::{json, Value};

use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

const BASE: &str = "/internode-tools/cli/oi/tasks";

pub async fn list(
    team: Option<&str>,
    project: Option<&str>,
    status: Option<&str>,
    _assignee: Option<&str>,
    priority: Option<&str>,
    search: Option<&str>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut params = vec![];
    if let Some(l) = limit { params.push(format!("limit={l}")); }
    if let Some(o) = offset { params.push(format!("offset={o}")); }
    if let Some(s) = search { params.push(format!("search={}", urlenc(s))); }
    if let Some(s) = status { params.push(format!("status={}", urlenc(s))); }
    if let Some(p) = priority { params.push(format!("priority={}", urlenc(p))); }
    if let Some(t) = team { params.push(format!("team_id={}", urlenc(t))); }
    if let Some(p) = project { params.push(format!("project_id={}", urlenc(p))); }
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
    priority: Option<&str>,
    assignee: Option<&str>,
    due_date: Option<&str>,
    _team: Option<&str>,
    _project: Option<&str>,
    _status: Option<&str>,
    _decision: Option<&str>,
    task_type: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({ "task_title": title });
    if let Some(d) = description { body["description"] = Value::String(d.to_string()); }
    if let Some(p) = priority { body["priority"] = Value::String(p.to_string()); }
    if let Some(a) = assignee { body["assignee_email"] = Value::String(a.to_string()); }
    if let Some(dd) = due_date { body["due_date"] = Value::String(dd.to_string()); }
    if let Some(tt) = task_type { body["task_type"] = Value::String(tt.to_string()); }
    let resp = client.post(BASE, &body).await?;
    output::print_success(resp);
    Ok(())
}

pub async fn update(
    id: &str,
    title: Option<&str>,
    description: Option<&str>,
    priority: Option<&str>,
    assignee: Option<&str>,
    due_date: Option<&str>,
    status: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({});
    if let Some(t) = title { body["task_title"] = Value::String(t.to_string()); }
    if let Some(d) = description { body["description"] = Value::String(d.to_string()); }
    if let Some(p) = priority { body["priority"] = Value::String(p.to_string()); }
    if let Some(a) = assignee { body["assignee_email"] = Value::String(a.to_string()); }
    if let Some(dd) = due_date { body["due_date"] = Value::String(dd.to_string()); }
    if let Some(s) = status { body["oistatus_id"] = Value::String(s.to_string()); }
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
