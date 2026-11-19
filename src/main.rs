mod game;

use std::{error::Error, io, time::Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use game::{Quote, RoundResult, choose_quote, evaluate, load_quotes};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};

struct App {
    quotes: Vec<Quote>,
    quote: Quote,
    input: String,
    started_at: Instant,
    result: Option<RoundResult>,
    should_exit: bool,
}

impl App {
    fn new(quotes: Vec<Quote>) -> io::Result<Self> {
        let quote =
            choose_quote(&quotes).ok_or_else(|| io::Error::other("The list of quotes is empty"))?;

        Ok(Self {
            quotes,
            quote,
            input: String::new(),
            started_at: Instant::now(),
            result: None,
            should_exit: false,
        })
    }

    fn next_round(&mut self) -> io::Result<()> {
        self.quote = choose_quote(&self.quotes)
            .ok_or_else(|| io::Error::other("The list of quotes is empty"))?;
        self.input.clear();
        self.result = None;
        self.started_at = Instant::now();
        Ok(())
    }

    fn handle_key(&mut self, code: KeyCode) -> io::Result<()> {
        match (&self.result, code) {
            (_, KeyCode::Esc) => self.should_exit = true,
            (Some(_), KeyCode::Enter) => self.next_round()?,
            (Some(_), KeyCode::Char('q')) => self.should_exit = true,
            (Some(_), _) => {}
            (None, KeyCode::Enter) => {
                self.result = Some(evaluate(
                    &self.quote.quote,
                    &self.input,
                    self.started_at.elapsed().as_secs_f64(),
                ));
            }
            (None, KeyCode::Backspace) => {
                self.input.pop();
            }
            (None, KeyCode::Char(character)) => self.input.push(character),
            (None, _) => {}
        }

        Ok(())
    }
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let quotes = load_quotes()?;
    let mut app = App::new(quotes)?;
    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &mut app);
    ratatui::restore();
    result?;
    Ok(())
}

fn run(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
    while !app.should_exit {
        terminal.draw(|frame| render(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                app.handle_key(key.code)?;
            }
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    let background = Color::Rgb(12, 18, 32);
    let panel = Color::Rgb(27, 38, 58);
    frame.render_widget(
        Block::default().style(Style::default().bg(background)),
        frame.area(),
    );

    let [_, centered_height, _] = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
    ])
    .areas(frame.area());
    let [_, content_area, _] = Layout::horizontal([
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
    ])
    .areas(centered_height);
    let [title_area, quote_area, input_area, status_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(5),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .areas(content_area);

    frame.render_widget(
        Paragraph::new("Tippen")
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(Color::Rgb(125, 211, 252))
                    .bg(panel)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .style(Style::default().bg(panel)),
            ),
        title_area,
    );

    let quote_text = Text::from(vec![
        Line::from(Span::styled(
            app.quote.quote.as_str(),
            Style::default()
                .fg(Color::Rgb(255, 230, 150))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Source: ", Style::default().fg(Color::Rgb(148, 163, 184))),
            Span::styled(
                app.quote.source.as_str(),
                Style::default().fg(Color::Rgb(226, 232, 240)),
            ),
        ]),
    ]);
    frame.render_widget(
        Paragraph::new(quote_text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .style(Style::default().bg(panel))
            .block(
                Block::default()
                    .title(" Quote ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(250, 204, 21)))
                    .style(Style::default().bg(panel)),
            ),
        quote_area,
    );

    frame.render_widget(
        Paragraph::new(app.input.as_str())
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Rgb(37, 52, 77))
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .title(" Type the quote ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(74, 222, 128)))
                    .style(Style::default().bg(panel)),
            ),
        input_area,
    );

    let status = match &app.result {
        Some(result) => format!(
            "{}  |  Time: {:.2}s  |  WPM: {:.1}  |  Enter: try again, q/Esc: quit",
            if result.correct {
                "GOD SLAIN"
            } else {
                "YOU DIED"
            },
            result.elapsed,
            result.wpm
        ),
        None => "Enter: submit  |  Esc: quit".to_owned(),
    };
    frame.render_widget(
        Paragraph::new(status)
            .style(
                Style::default()
                    .fg(Color::Rgb(186, 230, 253))
                    .bg(panel)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center),
        status_area,
    );
}
