use crate::error::{DatapassError, Result};
use crate::types::DataUsage;
use scraper::{Html, Selector};

/// Parse HTML content to extract data usage information
pub fn parse_html(html: &str) -> Result<DataUsage> {
    let document = Html::parse_document(html);

    // Check if this is an authentication/redirect page
    if is_auth_required_page(&document) {
        return Err(DatapassError::DataNotFound(
            "Authentication required. The website requires:\n  \
            - Access from Telekom mobile network, OR\n  \
            - Valid login session\n  \
            \nTo test locally, use: --file <saved-html-file>"
                .to_string(),
        ));
    }

    // Extract plan name from title
    let plan_name = extract_plan_name(&document)?;

    // Extract data usage from the active data pass
    let (remaining_gb, total_gb) = extract_data_usage(&document)?;

    // Extract validity date (optional)
    let valid_until = extract_valid_until(&document);

    Ok(DataUsage::new(
        remaining_gb,
        total_gb,
        Some(plan_name),
        valid_until,
    ))
}

/// Extract plan name from the HTML title
fn extract_plan_name(document: &Html) -> Result<String> {
    let title_selector = Selector::parse("title")
        .map_err(|e| DatapassError::ParseError(format!("Invalid selector: {:?}", e)))?;

    let title = document
        .select(&title_selector)
        .next()
        .ok_or_else(|| DatapassError::DataNotFound("Title not found".to_string()))?
        .text()
        .collect::<String>();

    // Title format: "Data usage - MagentaMobil Prepaid L"
    // Note: May contain non-breaking spaces that need to be normalized
    let plan_name = title
        .split('-')
        .nth(1)
        .map(|s| {
            // Replace non-breaking spaces with regular spaces and trim
            s.replace('\u{00A0}', " ").trim().to_string()
        })
        .ok_or_else(|| {
            DatapassError::ParseError("Could not extract plan name from title".to_string())
        })?;

    Ok(plan_name)
}

/// Extract data usage (remaining and total GB) from the active data pass
fn extract_data_usage(document: &Html) -> Result<(f64, f64)> {
    // Look for the active data pass section
    let section_selector = Selector::parse("section.data-pass-instance")
        .map_err(|e| DatapassError::ParseError(format!("Invalid selector: {:?}", e)))?;

    let remaining_selector = Selector::parse("div.remaining-volume-value")
        .map_err(|e| DatapassError::ParseError(format!("Invalid selector: {:?}", e)))?;

    let total_selector = Selector::parse("div.start-volume")
        .map_err(|e| DatapassError::ParseError(format!("Invalid selector: {:?}", e)))?;

    // Find the first active pass (not the summation)
    for section in document.select(&section_selector) {
        // Skip if this is the summation section
        if let Some(id) = section.value().attr("id") {
            if id == "summationPass" {
                continue;
            }
        }

        // Try to find remaining and total values in this section
        let remaining_text = section
            .select(&remaining_selector)
            .next()
            .map(|elem| elem.text().collect::<String>().trim().to_string());

        let total_text = section
            .select(&total_selector)
            .next()
            .map(|elem| elem.text().collect::<String>().trim().to_string());

        if let (Some(remaining), Some(total)) = (remaining_text, total_text) {
            // Parse numbers, handling both German (comma) and English (period) formats
            let remaining_gb: f64 = parse_number(&remaining)?;
            let total_gb: f64 = parse_number(&total)?;

            return Ok((remaining_gb, total_gb));
        }
    }

    Err(DatapassError::DataNotFound(
        "Could not find data usage information".to_string(),
    ))
}

/// Extract validity date from the HTML (optional)
/// Looks for "Valid until:" or "Gültig bis:" in div.info-row elements
fn extract_valid_until(document: &Html) -> Option<String> {
    let info_row_selector = Selector::parse("div.info-row").ok()?;

    for elem in document.select(&info_row_selector) {
        let text = elem.text().collect::<String>();

        // Check for both German and English variants
        if text.contains("Valid until:") || text.contains("Gültig bis:") {
            // Extract the date part after the colon
            let date = text
                .split(':')
                .nth(1)
                .map(|s| s.trim().to_string())?;

            return Some(date);
        }
    }

    None
}

/// Check if the page is an authentication/redirect page
fn is_auth_required_page(document: &Html) -> bool {
    // Check for common redirect/auth indicators
    let body_text = document
        .root_element()
        .text()
        .collect::<String>()
        .to_lowercase();

    // German: "Direkter Zugriff auf die Seite nicht möglich"
    // German: "Weiterleitung" (Redirect)
    // English: "Direct access to the page not possible"
    body_text.contains("direkter zugriff")
        || body_text.contains("direct access to the page not possible")
        || (body_text.contains("weiterleitung") && body_text.contains("nicht möglich"))
}

/// Parse a number string, handling both German (comma) and English (period) decimal formats
fn parse_number(s: &str) -> Result<f64> {
    // Replace German decimal comma with English period
    let normalized = s.replace(',', ".");
    normalized
        .parse()
        .map_err(|e| DatapassError::ParseError(format!("Invalid number value '{}': {}", s, e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_german_numbers() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Datennutzung - MagentaMobil Prepaid XL</title>
            </head>
            <body>
                <section class="data-pass-instance" id="test-pass">
                    <div class="remaining-volume-value">38,36</div>
                    <div class="start-volume">51</div>
                </section>
            </body>
            </html>
        "#;

        let result = parse_html(html);
        assert!(
            result.is_ok(),
            "Failed to parse German format HTML: {:?}",
            result.err()
        );

        let data = result.unwrap();
        assert_eq!(data.remaining_gb, 38.36);
        assert_eq!(data.total_gb, 51.0);
        assert_eq!(data.plan_name, Some("MagentaMobil Prepaid XL".to_string()));
    }

    #[test]
    #[ignore = "Requires test file not available in Nix build"]
    fn test_parse_test_html() {
        let html = std::fs::read_to_string("test/Data usage - MagentaMobil Prepaid L.html")
            .expect("Failed to read test file");

        let result = parse_html(&html).expect("Failed to parse HTML");

        assert_eq!(result.remaining_gb, 2.88);
        assert_eq!(result.total_gb, 25.0);
        assert_eq!(result.used_gb, 22.12);
        assert_eq!(result.plan_name, Some("MagentaMobil Prepaid L".to_string()));

        // Check percentage calculation
        assert!((result.percentage - 88.48).abs() < 0.1);
        assert!((result.remaining_percentage() - 11.52).abs() < 0.1);
    }
}
