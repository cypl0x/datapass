use crate::error::{DatapassError, Result};

const DEFAULT_URL: &str = "https://datapass.de";
// Use a real browser user agent to avoid being blocked
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36";

/// Fetch HTML content from the specified URL or default datapass.de
pub fn fetch_html(url: Option<&str>, cookie: Option<&str>) -> Result<String> {
    let target_url = url.unwrap_or(DEFAULT_URL);

    log::info!("Fetching data from: {}", target_url);

    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    let mut request = client
        .get(target_url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Cache-Control", "no-cache")
        .header("Pragma", "no-cache");

    // Add cookies if provided, otherwise use default Apollo cookies
    let cookie_str = cookie.unwrap_or("Apollo-Summation-Disabled=true; Apollo-Lang=en_DE_TMDE");
    log::debug!("Using cookies: {}", cookie_str);
    request = request.header("Cookie", cookie_str);

    let response = request.send()?;

    if !response.status().is_success() {
        return Err(DatapassError::FetchError(reqwest::Error::from(
            response.error_for_status().unwrap_err(),
        )));
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
