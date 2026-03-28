use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

pub async fn context(max_tokens: Option<i64>) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let qs = match max_tokens {
        Some(t) => format!("?max_tokens={t}"),
        None => String::new(),
    };
    let resp = client.get(&format!("/internode-tools/cli/oi/context{qs}")).await?;
    output::print_success(resp);
    Ok(())
}
