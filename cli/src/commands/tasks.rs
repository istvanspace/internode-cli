use serde_json::{json, Value};

use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

const BASE: &str = "/internode-tools/cli/oi/tasks";

pub async fn list(
    team: Option<&str>,
    project: Option<&str>,
    status: Option<&str>,
    assignee: Option<&str>,
    priority: Option<&str>,
    search: Option<&str>,
    topic: Option<&str>,
    intent: Option<&str>,
    topic_category: Option<&str>,
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
    if let Some(a) = assignee { params.push(format!("assignee={}", urlenc(a))); }
    if let Some(t) = team { params.push(format!("team_id={}", urlenc(t))); }
    if let Some(p) = project { params.push(format!("project_id={}", urlenc(p))); }
    if let Some(t) = topic { params.push(format!("topic_id={}", urlenc(t))); }
    if let Some(i) = intent { params.push(format!("intent_id={}", urlenc(i))); }
    if let Some(tc) = topic_category { params.push(format!("topic_category={}", urlenc(tc))); }
    let qs = if params.is_empty() { String::new() } else { format!("?{}", params.join("&")) };
    let resp = client.get(&format!("{BASE}{qs}")).await?;
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
    team: Option<&str>,
    project: Option<&str>,
    user_notes: Option<&str>,
    blocked_by_reason: Option<&str>,
    task_type: Option<&str>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({});
    if let Some(t) = title { body["task_title"] = Value::String(t.to_string()); }
    if let Some(d) = description { body["description"] = Value::String(d.to_string()); }
    if let Some(p) = priority { body["priority"] = Value::String(p.to_string()); }
    if let Some(a) = assignee { body["assignee_email"] = Value::String(a.to_string()); }
    if let Some(dd) = due_date { body["due_date"] = Value::String(dd.to_string()); }
    if let Some(s) = status { body["oistatus_id"] = Value::String(s.to_string()); }
    if let Some(t) = team { body["oiteam_id"] = Value::String(t.to_string()); }
    if let Some(p) = project { body["oiproject_id"] = Value::String(p.to_string()); }
    if let Some(n) = user_notes { body["user_notes"] = Value::String(n.to_string()); }
    if let Some(b) = blocked_by_reason { body["blocked_by_reason"] = Value::String(b.to_string()); }
    if let Some(tt) = task_type { body["task_type"] = Value::String(tt.to_string()); }
    let resp = client.patch(&format!("{BASE}/{id}"), &body).await?;
    output::print_success(resp);
    Ok(())
}

fn urlenc(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
