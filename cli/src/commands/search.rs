use serde_json::json;

use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

pub async fn search(
    query: &str,
    top_k: Option<i64>,
    min_score: Option<f64>,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let mut body = json!({ "query": query });
    if let Some(k) = top_k { body["top_k"] = json!(k); }
    if let Some(s) = min_score { body["min_score"] = json!(s); }
    let resp = client.post("/internode-tools/cli/oi/search", &body).await?;
    output::print_success(resp);
    Ok(())
}
