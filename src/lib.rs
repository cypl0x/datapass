// Library interface for testing

pub mod cli;
pub mod display;
pub mod error;
pub mod fetcher;
pub mod parser;
pub mod tui;
pub mod types;

pub use error::{DatapassError, Result};
pub use types::DataUsage;

/// Main entry point for library usage
pub fn get_data_usage(url: Option<&str>) -> Result<DataUsage> {
    let html = fetcher::fetch_html(url)?;
    parser::parse_html(&html)
}

/// Parse data usage from HTML string
pub fn parse_data_usage(html: &str) -> Result<DataUsage> {
    parser::parse_html(html)
}

/// Read and parse data usage from local file
pub fn get_data_usage_from_file(file_path: &str) -> Result<DataUsage> {
    let html = fetcher::read_local_file(file_path)?;
    parser::parse_html(&html)
}
