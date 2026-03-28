use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

pub async fn get(id: &str, with_related: bool) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let qs = if with_related { "?with_related=true" } else { "" };
    let resp = client.get(&format!("/internode-tools/cli/oi/entity/{id}{qs}")).await?;
    output::print_success(resp);
    Ok(())
}
