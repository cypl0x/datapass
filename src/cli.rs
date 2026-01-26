use clap::{Parser, ValueEnum};

#[allow(unused_imports)]
use clap::CommandFactory;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "datapass")]
#[command(about = "CLI tool to fetch and display mobile data usage from datapass.de")]
pub struct Cli {
    /// Output format
    #[arg(short, long, value_enum, default_value = "human")]
    pub format: Format,

    /// Enable colored output
    #[arg(short, long, default_value_t = false)]
    pub color: bool,

    /// Print only the used data in GB
    #[arg(long, conflicts_with = "format")]
    pub used: bool,

    /// Print only the total data in GB
    #[arg(long, conflicts_with = "format")]
    pub total: bool,

    /// Print only the remaining data in GB
    #[arg(long, conflicts_with = "format")]
    pub remaining: bool,

    /// Print only the usage percentage
    #[arg(long, conflicts_with = "format")]
    pub percentage: bool,

    /// Print only the plan name
    #[arg(long, conflicts_with = "format")]
    pub plan: bool,

    /// Watch mode: refresh every N seconds
    #[arg(short, long, value_name = "SECONDS")]
    pub watch: Option<u64>,

    /// Custom URL to fetch from (default: https://datapass.de)
    #[arg(short, long)]
    pub url: Option<String>,

    /// Read from local HTML file instead of fetching
    #[arg(short = 'F', long, value_name = "FILE")]
    pub file: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Log to file
    #[arg(long, value_name = "FILE")]
    pub log: Option<String>,

    /// Generate shell completions for the specified shell
    #[arg(long, value_name = "SHELL", value_enum)]
    pub generate_completions: Option<Shell>,

    /// Generate man page
    #[arg(long)]
    pub generate_man: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Format {
    /// Human-readable format with progress bar
    Human,
    /// JSON format
    Json,
}

impl Cli {
    pub fn validate(&self) -> Result<(), String> {
        if self.watch.is_some() && (self.used || self.total || self.remaining || self.percentage || self.plan) {
            return Err("Watch mode is not compatible with single value output flags".to_string());
        }

        if self.file.is_some() && self.url.is_some() {
            return Err("Cannot specify both --file and --url".to_string());
        }

        Ok(())
    }

    pub fn get_output_format(&self) -> crate::display::OutputFormat {
        use crate::display::OutputFormat;

        if self.used {
            OutputFormat::Used
        } else if self.total {
            OutputFormat::Total
        } else if self.remaining {
            OutputFormat::Remaining
        } else if self.percentage {
            OutputFormat::Percentage
        } else if self.plan {
            OutputFormat::Plan
        } else {
            match self.format {
                Format::Human => OutputFormat::Human,
                Format::Json => OutputFormat::Json,
            }
        }
    }
}
