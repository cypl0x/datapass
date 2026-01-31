use crate::error::Result;
use crate::types::DataUsage;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

pub struct TuiApp {
    refresh_interval: Duration,
    last_update: Instant,
    data: Option<DataUsage>,
    error: Option<String>,
}

impl TuiApp {
    pub fn new(refresh_interval: u64) -> Self {
        Self {
            refresh_interval: Duration::from_secs(refresh_interval),
            last_update: Instant::now(),
            data: None,
            error: None,
        }
    }

    pub fn update_data(&mut self, data: Result<DataUsage>) {
        match data {
            Ok(usage) => {
                self.data = Some(usage);
                self.error = None;
            }
            Err(e) => {
                self.error = Some(format!("Error: {}", e));
            }
        }
        self.last_update = Instant::now();
    }

    pub fn should_refresh(&self) -> bool {
        self.last_update.elapsed() >= self.refresh_interval
    }

    pub fn run<F>(&mut self, mut fetch_fn: F) -> Result<()>
    where
        F: FnMut() -> Result<DataUsage>,
    {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Initial fetch
        self.update_data(fetch_fn());

        // Main loop
        loop {
            terminal.draw(|f| self.draw(f))?;

            // Check for refresh
            if self.should_refresh() {
                self.update_data(fetch_fn());
            }

            // Handle input with timeout
            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            KeyCode::Char('r') => {
                                self.update_data(fetch_fn());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Length(7),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        // Title
        self.render_title(frame, chunks[0]);

        // Plan name and validity
        self.render_plan(frame, chunks[1]);

        // Data usage info
        self.render_data_info(frame, chunks[2]);

        // Progress gauge
        self.render_gauge(frame, chunks[3]);

        // Help/Status
        self.render_help(frame, chunks[4]);
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new("Mobile Data Usage Monitor")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, area);
    }

    fn render_plan(&self, frame: &mut Frame, area: Rect) {
        let text = if let Some(ref data) = self.data {
            let plan_name = data.plan_name.as_deref().unwrap_or("Unknown Plan");
            let mut lines = vec![Line::from(Span::styled(
                plan_name,
                Style::default().fg(Color::Yellow),
            ))];

            if let Some(ref valid_until) = data.valid_until {
                lines.push(Line::from(Span::styled(
                    format!("Valid until: {}", valid_until),
                    Style::default().fg(Color::Cyan),
                )));
            }

            lines
        } else {
            vec![Line::from("Loading...")]
        };

        let plan = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Plan"));
        frame.render_widget(plan, area);
    }

    fn render_data_info(&self, frame: &mut Frame, area: Rect) {
        let text = if let Some(ref error) = self.error {
            vec![Line::from(Span::styled(
                error,
                Style::default().fg(Color::Red),
            ))]
        } else if let Some(ref data) = self.data {
            if data.is_unlimited {
                // Display unlimited plan info
                vec![Line::from(vec![
                    Span::styled("Data:      ", Style::default().fg(Color::White)),
                    Span::styled(
                        "unlimited",
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])]
            } else {
                // Display standard metered plan info
                vec![
                    Line::from(vec![
                        Span::styled("Used:      ", Style::default().fg(Color::White)),
                        Span::styled(
                            format!("{:.2} GB ({:.2}%)", data.used_gb, data.percentage),
                            Style::default().fg(Color::Blue),
                        ),
                    ]),
                    Line::from(vec![
                        Span::styled("Total:     ", Style::default().fg(Color::White)),
                        Span::styled(
                            format!("{:.2} GB (100%)", data.total_gb),
                            Style::default().fg(Color::White),
                        ),
                    ]),
                    Line::from(vec![
                        Span::styled("Remaining: ", Style::default().fg(Color::White)),
                        Span::styled(
                            format!(
                                "{:.2} GB ({:.2}%)",
                                data.remaining_gb,
                                data.remaining_percentage()
                            ),
                            Style::default().fg(Color::Green),
                        ),
                    ]),
                ]
            }
        } else {
            vec![Line::from("Loading data...")]
        };

        let info =
            Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Data Usage"));
        frame.render_widget(info, area);
    }

    fn render_gauge(&self, frame: &mut Frame, area: Rect) {
        let (ratio, label, color) = if let Some(ref data) = self.data {
            if data.is_unlimited {
                // For unlimited plans, show 100% with green color
                (1.0, "unlimited".to_string(), Color::Green)
            } else {
                // For metered plans, show actual usage
                let ratio = data.percentage / 100.0;
                let label = format!("{:.2}% Used", data.percentage);
                let color = if data.remaining_percentage() > 50.0 {
                    Color::Green
                } else if data.remaining_percentage() > 20.0 {
                    Color::Yellow
                } else {
                    Color::Red
                };
                (ratio, label, color)
            }
        } else {
            (0.0, "Loading...".to_string(), Color::Gray)
        };

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Progress"))
            .gauge_style(Style::default().fg(color).bg(Color::Black))
            .ratio(ratio)
            .label(label);
        frame.render_widget(gauge, area);
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let elapsed = self.last_update.elapsed().as_secs();
        let next_refresh = if elapsed < self.refresh_interval.as_secs() {
            self.refresh_interval.as_secs() - elapsed
        } else {
            0
        };

        let help_text = format!(
            "Press 'q' or ESC to quit | 'r' to refresh now | Next auto-refresh in {}s",
            next_refresh
        );

        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(help, area);
    }
}
