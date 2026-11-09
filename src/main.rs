use std::io::stdout;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn ui(frame: &mut ratatui::Frame) {
    let area = frame.area();
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(2),
    ])
    .areas(area);

    let logo = Line::from(vec![
        Span::styled(" ▓▒░ ", Style::default().fg(Color::Rgb(245, 169, 127))),
        Span::styled(
            "s m u d g e",
            Style::default()
                .fg(Color::Rgb(238, 212, 159))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ░▒▓", Style::default().fg(Color::Rgb(245, 169, 127))),
        Span::styled(
            "   a tiny terminal sketchpad",
            Style::default().fg(Color::Rgb(110, 110, 120)),
        ),
    ]);
    frame.render_widget(Paragraph::new(logo), header);

    let [tools, canvas, palette] = Layout::horizontal([
        Constraint::Length(20),
        Constraint::Min(20),
        Constraint::Length(26),
    ])
    .areas(body);

    render_panel(frame, tools, "tools", "✎ pencil\n/ line\n▭ rect\n◯ circle");
    render_panel(frame, canvas, "canvas", "");
    render_panel(frame, palette, "ink", "██  ██  ██  ██");

    let hints = Line::from(vec![
        Span::styled(" q", Style::default().fg(Color::Rgb(238, 212, 159))),
        Span::styled(" quit ", Style::default().fg(Color::Rgb(110, 110, 120))),
        Span::styled("?: ", Style::default().fg(Color::Rgb(238, 212, 159))),
        Span::styled("help", Style::default().fg(Color::Rgb(110, 110, 120))),
    ]);
    frame.render_widget(Paragraph::new(hints), footer);
}

fn render_panel(frame: &mut ratatui::Frame, area: Rect, title: &str, text: &str) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(62, 62, 74)))
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(Color::Rgb(158, 158, 172)),
        ));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(
        Paragraph::new(text).style(Style::default().fg(Color::Rgb(190, 190, 200))),
        inner,
    );
}
