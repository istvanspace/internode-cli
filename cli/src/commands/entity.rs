use serde_json::json;

use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

pub async fn get(ids: Vec<String>) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let body = json!({ "entity_ids": ids });
    let resp = client.post("/internode-tools/cli/oi/entities", &body).await?;
    output::print_success(resp);
    Ok(())
}
