use crate::error::{DatapassError, Result};

const DEFAULT_URL: &str = "https://datapass.de";
const USER_AGENT: &str = "datapass-cli/0.1.0";

/// Fetch HTML content from the specified URL or default datapass.de
pub fn fetch_html(url: Option<&str>) -> Result<String> {
    let target_url = url.unwrap_or(DEFAULT_URL);

    log::info!("Fetching data from: {}", target_url);

    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = client
        .get(target_url)
        .send()?;

    if !response.status().is_success() {
        return Err(DatapassError::FetchError(
            reqwest::Error::from(response.error_for_status().unwrap_err())
        ));
    }

    let html = response.text()?;
    log::debug!("Successfully fetched {} bytes", html.len());

    Ok(html)
}

/// Read HTML from a local file (useful for testing)
pub fn read_local_file(path: &str) -> Result<String> {
    log::info!("Reading HTML from local file: {}", path);
    let html = std::fs::read_to_string(path)?;
    Ok(html)
}
