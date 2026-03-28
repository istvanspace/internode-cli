use serde_json::json;

use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

pub async fn search(
    query: &str,
) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let body = json!({ "query": query });
    let resp = client.post("/internode-tools/cli/oi/search", &body).await?;
    output::print_success(resp);
    Ok(())
}
