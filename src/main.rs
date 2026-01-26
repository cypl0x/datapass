mod cli;
mod display;
mod error;
mod fetcher;
mod parser;
mod tui;
mod types;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell as ClapShell};
use clap_mangen::Man;
use cli::{Cli, Shell};
use error::Result;
use std::io;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Handle shell completions generation
    if let Some(shell) = cli.generate_completions {
        let mut cmd = Cli::command();
        let bin_name = cmd.get_name().to_string();
        let clap_shell = match shell {
            Shell::Bash => ClapShell::Bash,
            Shell::Zsh => ClapShell::Zsh,
            Shell::Fish => ClapShell::Fish,
            Shell::PowerShell => ClapShell::PowerShell,
            Shell::Elvish => ClapShell::Elvish,
        };
        generate(clap_shell, &mut cmd, bin_name, &mut io::stdout());
        return Ok(());
    }

    // Handle man page generation
    if cli.generate_man {
        let cmd = Cli::command();
        let man = Man::new(cmd);
        man.render(&mut io::stdout())?;
        return Ok(());
    }

    // Validate CLI arguments
    if let Err(e) = cli.validate() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Setup logging
    setup_logging(&cli)?;

    // Watch mode (TUI)
    if let Some(interval) = cli.watch {
        run_watch_mode(interval, &cli)?;
        return Ok(());
    }

    // Single fetch mode
    let html = if let Some(file_path) = &cli.file {
        fetcher::read_local_file(file_path)?
    } else {
        fetcher::fetch_html(cli.url.as_deref())?
    };

    let data = parser::parse_html(&html)?;
    let output_format = cli.get_output_format();
    display::display(&data, output_format, cli.color);

    Ok(())
}

fn run_watch_mode(interval: u64, cli: &Cli) -> Result<()> {
    let mut app = tui::TuiApp::new(interval);

    // Create a closure that captures the CLI config
    let url = cli.url.clone();
    let file = cli.file.clone();

    let fetch_fn = move || -> Result<types::DataUsage> {
        let html = if let Some(ref file_path) = file {
            fetcher::read_local_file(file_path)?
        } else {
            fetcher::fetch_html(url.as_deref())?
        };
        parser::parse_html(&html)
    };

    app.run(fetch_fn)?;
    Ok(())
}

fn setup_logging(cli: &Cli) -> Result<()> {
    let log_level = if cli.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    if let Some(log_file) = &cli.log {
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;

        env_logger::Builder::new()
            .filter_level(log_level)
            .target(env_logger::Target::Pipe(Box::new(log_file)))
            .init();
    } else if cli.verbose {
        env_logger::Builder::new()
            .filter_level(log_level)
            .init();
    }

    Ok(())
}
