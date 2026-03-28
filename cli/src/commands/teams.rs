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
