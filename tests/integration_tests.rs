use datapass::{get_data_usage_from_file, parse_data_usage};

const TEST_HTML_PATH: &str = "test/Data usage - MagentaMobil Prepaid L.html";

#[test]
#[ignore = "Requires test file not available in Nix build"]
fn test_parse_test_file() {
    let result = get_data_usage_from_file(TEST_HTML_PATH);
    assert!(
        result.is_ok(),
        "Failed to parse test HTML file: {:?}",
        result.err()
    );

    let data = result.unwrap();
    assert_eq!(data.remaining_gb, 2.88, "Remaining GB mismatch");
    assert_eq!(data.total_gb, 25.0, "Total GB mismatch");
    assert_eq!(data.used_gb, 22.12, "Used GB mismatch");

    // Check percentage (allow small floating point error)
    assert!(
        (data.percentage - 88.48).abs() < 0.01,
        "Usage percentage mismatch"
    );
    assert!(
        (data.remaining_percentage() - 11.52).abs() < 0.01,
        "Remaining percentage mismatch"
    );

    assert_eq!(
        data.plan_name,
        Some("MagentaMobil Prepaid L".to_string()),
        "Plan name mismatch"
    );
}

#[test]
fn test_data_usage_calculations() {
    use datapass::DataUsage;

    let data = DataUsage::new(10.0, 50.0, Some("Test Plan".to_string()), None);

    assert_eq!(data.used_gb, 40.0);
    assert_eq!(data.percentage, 80.0);
    assert_eq!(data.remaining_percentage(), 20.0);
}

#[test]
fn test_parse_minimal_html() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Data usage - Test Plan</title>
        </head>
        <body>
            <section class="data-pass-instance" id="test-pass">
                <div class="remaining-volume-value">5.5</div>
                <div class="start-volume">10</div>
            </section>
        </body>
        </html>
    "#;

    let result = parse_data_usage(html);
    assert!(
        result.is_ok(),
        "Failed to parse minimal HTML: {:?}",
        result.err()
    );

    let data = result.unwrap();
    assert_eq!(data.remaining_gb, 5.5);
    assert_eq!(data.total_gb, 10.0);
    assert_eq!(data.plan_name, Some("Test Plan".to_string()));
}

#[test]
fn test_parse_missing_data() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Data usage - Test</title>
        </head>
        <body>
            <p>No data here</p>
        </body>
        </html>
    "#;

    let result = parse_data_usage(html);
    assert!(result.is_err(), "Should fail when data is missing");
}

#[test]
fn test_parse_missing_title() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head></head>
        <body>
            <section class="data-pass-instance">
                <div class="remaining-volume-value">5</div>
                <div class="start-volume">10</div>
            </section>
        </body>
        </html>
    "#;

    let result = parse_data_usage(html);
    assert!(result.is_err(), "Should fail when title is missing");
}

#[test]
fn test_parse_invalid_numbers() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Data usage - Test</title>
        </head>
        <body>
            <section class="data-pass-instance" id="test">
                <div class="remaining-volume-value">invalid</div>
                <div class="start-volume">10</div>
            </section>
        </body>
        </html>
    "#;

    let result = parse_data_usage(html);
    assert!(result.is_err(), "Should fail when numbers are invalid");
}

#[test]
fn test_skip_summation_pass() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Data usage - Multi Pass Plan</title>
        </head>
        <body>
            <section class="data-pass-instance" id="summationPass">
                <div class="remaining-volume-value">15</div>
                <div class="start-volume">50</div>
            </section>
            <section class="data-pass-instance" id="actual-pass">
                <div class="remaining-volume-value">10</div>
                <div class="start-volume">30</div>
            </section>
        </body>
        </html>
    "#;

    let result = parse_data_usage(html);
    assert!(result.is_ok());

    let data = result.unwrap();
    // Should use the actual pass, not the summation
    assert_eq!(data.remaining_gb, 10.0);
    assert_eq!(data.total_gb, 30.0);
}

#[test]
fn test_parse_validity_date() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Data usage - Test Plan</title>
        </head>
        <body>
            <section class="data-pass-instance" id="test-pass">
                <div class="remaining-volume-value">5.5</div>
                <div class="start-volume">10</div>
                <div class="info-row">Valid until: 12. February 2026</div>
            </section>
        </body>
        </html>
    "#;

    let result = parse_data_usage(html);
    assert!(result.is_ok(), "Failed to parse HTML: {:?}", result.err());

    let data = result.unwrap();
    assert_eq!(data.valid_until, Some("12. February 2026".to_string()));
}

#[test]
fn test_parse_validity_date_german() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Data usage - Test Plan</title>
        </head>
        <body>
            <section class="data-pass-instance" id="test-pass">
                <div class="remaining-volume-value">5.5</div>
                <div class="start-volume">10</div>
                <div class="info-row">Gültig bis: 21. Februar 2026</div>
            </section>
        </body>
        </html>
    "#;

    let result = parse_data_usage(html);
    assert!(result.is_ok(), "Failed to parse HTML: {:?}", result.err());

    let data = result.unwrap();
    assert_eq!(data.valid_until, Some("21. Februar 2026".to_string()));
}
