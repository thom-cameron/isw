use std::{
    io,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::stopwatch::Stopwatch;

use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    DefaultTerminal, Frame,
};

const REFRESH_RATE_HZ: f32 = 60.;

#[derive(Debug)]
pub struct App {
    stopwatch: Stopwatch,
    exit: bool,
}

impl App {
    pub fn new(stopwatch: Stopwatch) -> Self {
        Self {
            stopwatch,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> (io::Result<()>, String) {
        let loop_duration = Duration::from_millis(((1.0 / REFRESH_RATE_HZ) * 1000.0) as u64);

        while !self.exit {
            let loop_start_time = Instant::now();

            self.stopwatch.update_time();

            match terminal.draw(|frame| self.draw(frame)) {
                Ok(_) => {}
                Err(err) => return (Err(err), self.stopwatch.to_string()),
            };

            match self.handle_events() {
                Ok(_) => {}
                Err(err) => return (Err(err), self.stopwatch.to_string()),
            };

            sleep(loop_duration - loop_start_time.elapsed());
        }

        (Ok(()), self.stopwatch.to_string())
    }

    /// draws the next frame of the tui
    fn draw(&self, frame: &mut Frame) { frame.render_widget(self, frame.area()); }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        // return immediately if no key is pressed
        if !poll(Duration::from_secs(0))? {
            return Ok(());
        }

        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    /// map keys to functionality
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('p') | KeyCode::Char(' ') => self.stopwatch.toggle_pause(),
            KeyCode::Char('r') => self.stopwatch.reset(),
            _ => {}
        }
    }

    /// exit the main loop of the app
    fn exit(&mut self) { self.exit = true; }
}

impl Widget for &App {
    /// draw the tui of the app
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text_style = Style::new()
            .fg(match self.stopwatch.get_current_interval() {
                Some(interval) => interval.colour,
                None => Color::White,
            })
            .add_modifier(Modifier::BOLD);

        let counter_text = vec![
            Line::from(Span::styled(self.stopwatch.to_string(), text_style)),
            Line::from(Span::raw(self.stopwatch.get_status_string())),
        ];

        // lay out the stopwatch text in the central rows of the tui
        let [area] = Layout::vertical([Constraint::Length(counter_text.len() as u16)])
            .flex(Flex::Center)
            .areas(area);
        Paragraph::new(counter_text).centered().render(area, buf);
    }
}
