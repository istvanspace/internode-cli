use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

const BASE: &str = "/internode-tools/cli/oi/decisions";

pub async fn list(
    search: Option<&str>,
    limit: Option<i64>,
    offset: Option<i64>,
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

fn urlenc(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
