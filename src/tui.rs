use std::{io, time::Duration};

use crate::stopwatch::Stopwatch;

use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Color,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

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
        while !self.exit {
            self.stopwatch.update_time();

            match terminal.draw(|frame| self.draw(frame)) {
                Ok(_) => {}
                Err(err) => return (Err(err), self.stopwatch.to_string()),
            };

            match self.handle_events() {
                Ok(_) => {}
                Err(err) => return (Err(err), self.stopwatch.to_string()),
            };
        }

        (Ok(()), self.stopwatch.to_string())
    }

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

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('p') | KeyCode::Char(' ') => self.stopwatch.toggle_pause(),
            _ => {}
        }
    }

    fn exit(&mut self) { self.exit = true; }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [area] = Layout::vertical([Constraint::Percentage(25)])
            .flex(Flex::Center)
            .areas(area);

        let text_colour = match self.stopwatch.get_current_interval() {
            Some(interval) => interval.colour,
            None => Color::White,
        };
        let text_style = text_colour;

        let counter_text = vec![
            Line::from(Span::styled(self.stopwatch.to_string(), text_style)),
            Line::from(format!("intervals: {}", self.stopwatch.intervals_elapsed)),
            Line::from(format!("interval cycles: {}", self.stopwatch.interval_cycles_elapsed)),
            Line::from(format!("paused: {}", self.stopwatch.paused)),
        ];
        Paragraph::new(counter_text)
            .centered()
            // .block(Block::new().borders(Borders::ALL))
            .render(area, buf);
    }
}
