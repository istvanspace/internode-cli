use crate::client::ApiClient;
use crate::error::CliError;
use crate::output;

const BASE: &str = "/internode-tools/cli/oi/statuses";

pub async fn list(team: Option<&str>) -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let qs = match team {
        Some(t) => format!("?oiteam_id={}", urlenc(t)),
        None => String::new(),
    };
    let resp = client.get(&format!("{BASE}{qs}")).await?;
    output::print_success(resp);
    Ok(())
}

fn urlenc(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
