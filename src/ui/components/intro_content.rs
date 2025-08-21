use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};

pub fn get_step_title(current_step: usize) -> &'static str {
    match current_step {
        0 => " Welcome ",
        1 => " Nerd Font Test ",
        2 => " Claude Code Configuration ",
        _ => " Intro ",
    }
}

pub fn get_step_content(current_step: usize) -> Text<'static> {
    match current_step {
        0 => Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("Welcome to "),
                Span::styled(
                    "CCometixLine",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("! 🚀"),
            ]),
            Line::from(""),
            Line::from("A high-performance statusline tool for Claude Code,"),
            Line::from("built with Rust for maximum speed and reliability."),
            Line::from(""),
            Line::from("This interactive guide will show you:"),
            Line::from("• What CCometixLine does"),
            Line::from("• Key features and capabilities"),
            Line::from("• How to get started with configuration"),
            Line::from(""),
            Line::from("Press Enter or → to continue!"),
        ]),

        1 => Text::from(vec![
            Line::from("Nerd Font Display Test"),
            Line::from(""),
            Line::from("Can you see these icons clearly and distinctly?"),
            Line::from(""),
            Line::from(vec![
                Span::styled("\u{e26d}", Style::default().fg(Color::Yellow)),
                Span::raw(" ← Should be a Haleclipse"),
            ]),
            Line::from(vec![
                Span::styled("\u{f024b}", Style::default().fg(Color::Magenta)),
                Span::raw(" ← Should be a folder icon"),
            ]),
            Line::from(vec![
                Span::styled("\u{f02a2}", Style::default().fg(Color::Blue)),
                Span::raw(" ← Should be a git branch icon"),
            ]),
            Line::from(""),
            Line::from("Powerline separators:"),
            Line::from(vec![
                Span::styled("\u{e0b0}", Style::default().fg(Color::Cyan)),
                Span::raw(" ← Should be angular separators"),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "If you see boxes (□) or question marks (?)",
                Style::default().fg(Color::Red),
            )]),
            Line::from(vec![Span::styled(
                "instead of distinct icons, we recommend installing",
                Style::default().fg(Color::Red),
            )]),
            Line::from(vec![
                Span::styled(
                    "Maple Mono",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" for the best experience.", Style::default().fg(Color::Red)),
            ]),
        ]),

        2 => Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Claude Code Configuration",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from("Add to Claude Code settings.json:"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Linux/macOS:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("{", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("\"statusLine\": {", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("\"type\": \"command\",", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("\"command\": \"~/.claude/ccline/ccline\",", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("\"padding\": 0", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("}", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("}", Style::default().fg(Color::Green)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Windows:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("{", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("\"statusLine\": {", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("\"type\": \"command\",", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("\"command\": \"%USERPROFILE%\\\\.claude\\\\ccline\\\\ccline.exe\",", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("\"padding\": 0", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("}", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("}", Style::default().fg(Color::Green)),
            ]),
            Line::from(""),
            Line::from("Would you like to start configuring your statusline now?"),
        ]),

        _ => Text::from(""),
    }
}