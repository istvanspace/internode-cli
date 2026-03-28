use crate::client::ApiClient;
use crate::config::{load_config, save_config};
use crate::error::CliError;
use crate::output;

pub async fn configure(api_key: &str, api_url: Option<&str>) -> Result<(), CliError> {
    if !api_key.starts_with("ink_") {
        return Err(CliError::BadInput(
            "Invalid API key format. Keys start with 'ink_'. Get one from Settings > CLI API Key in the Internode portal.".into(),
        ));
    }

    let mut config = load_config();
    config.api_key = api_key.to_string();
    if let Some(url) = api_url {
        config.api_url = url.to_string();
    }
    save_config(&config)?;

    output::print_success(serde_json::json!({
        "status": "configured",
        "api_url": config.api_url,
        "key_prefix": &api_key[..api_key.len().min(12)],
    }));
    output::stderr_msg("API key saved. Run `internode auth status` to verify.");
    Ok(())
}

pub async fn status() -> Result<(), CliError> {
    let client = ApiClient::new()?;
    let resp = client.get("/internode-tools/cli/auth/status").await?;
    output::print_success(resp);
    Ok(())
}
