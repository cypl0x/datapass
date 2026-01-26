use crate::error::{DatapassError, Result};
use crate::types::DataUsage;
use scraper::{Html, Selector};

/// Parse HTML content to extract data usage information
pub fn parse_html(html: &str) -> Result<DataUsage> {
    let document = Html::parse_document(html);

    // Extract plan name from title
    let plan_name = extract_plan_name(&document)?;

    // Extract data usage from the active data pass
    let (remaining_gb, total_gb) = extract_data_usage(&document)?;

    Ok(DataUsage::new(remaining_gb, total_gb, Some(plan_name)))
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
        .ok_or_else(|| DatapassError::ParseError("Could not extract plan name from title".to_string()))?;

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

    Err(DatapassError::DataNotFound("Could not find data usage information".to_string()))
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
        assert!(result.is_ok(), "Failed to parse German format HTML: {:?}", result.err());

        let data = result.unwrap();
        assert_eq!(data.remaining_gb, 38.36);
        assert_eq!(data.total_gb, 51.0);
        assert_eq!(data.plan_name, Some("MagentaMobil Prepaid XL".to_string()));
    }

    #[test]
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
