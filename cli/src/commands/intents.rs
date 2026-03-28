use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

const BASE: &str = "/internode-tools/cli/oi/intents";

pub async fn list(limit: Option<i64>, offset: Option<i64>) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut params = vec![];
    if let Some(l) = limit { params.push(format!("limit={l}")); }
    if let Some(o) = offset { params.push(format!("offset={o}")); }
    let qs = if params.is_empty() { String::new() } else { format!("?{}", params.join("&")) };
    let resp = client.get(&format!("{BASE}{qs}")).await?;
    output::print_success(resp);
    Ok(())
}
