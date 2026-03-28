use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

pub async fn taxonomy() -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let resp = client.get("/internode-tools/cli/oi/teams").await?;
    output::print_success(resp);
    Ok(())
}
