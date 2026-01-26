use crate::types::DataUsage;
use owo_colors::OwoColorize;

/// Display format options
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Human,
    Json,
    Used,
    Total,
    Remaining,
    Percentage,
    Plan,
}

/// Display data usage in the specified format
pub fn display(data: &DataUsage, format: OutputFormat, use_color: bool) {
    match format {
        OutputFormat::Human => display_human(data, use_color),
        OutputFormat::Json => display_json(data),
        OutputFormat::Used => println!("{:.2}", data.used_gb),
        OutputFormat::Total => println!("{:.2}", data.total_gb),
        OutputFormat::Remaining => println!("{:.2}", data.remaining_gb),
        OutputFormat::Percentage => println!("{:.2}", data.percentage),
        OutputFormat::Plan => {
            if let Some(plan) = &data.plan_name {
                println!("{}", plan);
            }
        }
    }
}

/// Display data usage in human-readable format with optional colors
fn display_human(data: &DataUsage, use_color: bool) {
    let plan_line = if let Some(plan) = &data.plan_name {
        if use_color {
            format!("Plan: {}\n", plan.bold())
        } else {
            format!("Plan: {}\n", plan)
        }
    } else {
        String::new()
    };

    let used_str = format!("{:.2} GB", data.used_gb);
    let total_str = format!("{:.2} GB", data.total_gb);
    let remaining_str = format!("{:.2} GB", data.remaining_gb);

    let (used_display, total_display, remaining_display) = if use_color {
        (
            used_str.bright_blue().to_string(),
            total_str.bright_white().to_string(),
            remaining_str.bright_green().to_string(),
        )
    } else {
        (used_str, total_str, remaining_str)
    };

    print!("{}", plan_line);
    println!("Used:      {} ({:.2}%)", used_display, data.percentage);
    println!("Total:     {} (100%)", total_display);
    println!(
        "Remaining: {} ({:.2}%)",
        remaining_display,
        data.remaining_percentage()
    );

    // Display progress bar
    display_progress_bar(data, use_color);
}

/// Display a progress bar showing usage
fn display_progress_bar(data: &DataUsage, use_color: bool) {
    let bar_width = 40;
    let filled = ((data.percentage / 100.0) * bar_width as f64).round() as usize;
    let filled = filled.min(bar_width); // Ensure we don't exceed bar width
    let empty = bar_width - filled;

    if use_color {
        use owo_colors::OwoColorize;

        // Determine color based on remaining percentage
        let filled_str = if data.remaining_percentage() > 50.0 {
            "█".repeat(filled).green().to_string()
        } else if data.remaining_percentage() > 20.0 {
            "█".repeat(filled).yellow().to_string()
        } else {
            "█".repeat(filled).red().to_string()
        };

        let empty_str = "░".repeat(empty).dimmed().to_string();

        println!("{}{} {:.2}%", filled_str, empty_str, data.percentage);
    } else {
        println!(
            "{}{} {:.2}%",
            "█".repeat(filled),
            "░".repeat(empty),
            data.percentage
        );
    }

    println!(); // Add newline after progress bar
}

/// Display data usage in JSON format
fn display_json(data: &DataUsage) {
    match serde_json::to_string_pretty(data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}
