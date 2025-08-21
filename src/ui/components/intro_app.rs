use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::intro_content::{get_step_content, get_step_title};

pub struct IntroApp {
    current_step: usize,
    total_steps: usize,
    should_quit: bool,
    should_continue_to_config: bool,
}

impl IntroApp {
    pub fn new() -> Self {
        Self {
            current_step: 0,
            total_steps: 3,
            should_quit: false,
            should_continue_to_config: false,
        }
    }

    pub fn next_step(&mut self) {
        if self.current_step < self.total_steps - 1 {
            self.current_step += 1;
        } else {
            self.should_continue_to_config = true;
        }
    }

    pub fn prev_step(&mut self) {
        if self.current_step > 0 {
            self.current_step -= 1;
        }
    }

    pub fn skip_intro(&mut self) {
        self.should_quit = true;
    }

    pub fn should_continue(&self) -> bool {
        self.should_continue_to_config
    }

    pub fn should_exit(&self) -> bool {
        self.should_quit
    }

    pub fn render(&self, f: &mut Frame) {
        let area = f.area();

        // Main content layout
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(10),   // Content
                Constraint::Length(4), // Progress + Help
            ])
            .split(area);

        // Title
        self.render_title(f, layout[0]);

        // Main content
        self.render_content(f, layout[1]);

        // Bottom section
        let bottom_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Length(2)])
            .split(layout[2]);

        // Progress
        self.render_progress(f, bottom_layout[0]);

        // Help
        self.render_help(f, bottom_layout[1]);
    }

    fn render_title(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let title = Paragraph::new(format!(
            "CCometixLine Intro - v{}",
            env!("CARGO_PKG_VERSION")
        ))
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);
        f.render_widget(title, area);
    }

    fn render_content(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let content = get_step_content(self.current_step);
        let content_widget = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(get_step_title(self.current_step)),
            )
            .wrap(ratatui::widgets::Wrap { trim: true })
            .alignment(Alignment::Left);
        f.render_widget(content_widget, area);
    }

    fn render_progress(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let progress_text = format!("Step {} of {}", self.current_step + 1, self.total_steps);

        let progress_dots = (0..self.total_steps)
            .map(|i| {
                if i == self.current_step {
                    Span::styled("●", Style::default().fg(Color::Cyan))
                } else {
                    Span::styled("○", Style::default().fg(Color::DarkGray))
                }
            })
            .fold(Vec::new(), |mut acc, span| {
                if !acc.is_empty() {
                    acc.push(Span::raw(" "));
                }
                acc.push(span);
                acc
            });

        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let progress_bar = Paragraph::new(Line::from(progress_dots)).alignment(Alignment::Center);

        let text = Paragraph::new(progress_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);

        f.render_widget(progress_bar, content_layout[0]);
        f.render_widget(text, content_layout[1]);
    }

    fn render_help(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let help_text = if self.current_step == 0 {
            "[→/Enter] Next  [Esc] Skip"
        } else if self.current_step >= self.total_steps - 1 {
            "[←] Back  [Enter] Start Config  [Esc] Exit"
        } else {
            "[←] Back  [→/Enter] Next  [Esc] Skip"
        };

        let help = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::TOP))
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);

        f.render_widget(help, area);
    }
}