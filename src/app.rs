use std::{
    io,
    time::{Duration, Instant},
};

use crate::intervals::{Interval, IntervalList};
use crate::stopwatch::Stopwatch;

use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Styled, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug)]
pub struct App {
    stopwatch: Stopwatch,
    exit: bool,
}

impl App {
    pub fn new(interval_list: Option<IntervalList>) -> Self {
        let start_time = Instant::now();
        let zero_duration = Duration::new(0, 0);

        Self {
            stopwatch: Stopwatch {
                start_time,
                current_time: zero_duration,
                interval_start_time: start_time,
                interval_current_time: zero_duration,
                interval_list,
                interval_i: 0,
                intervals_elapsed: 0,
            },
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> (io::Result<()>, String) {
        while !self.exit {
            self.stopwatch.update_time();

            match terminal.draw(|frame| self.draw(frame)) {
                Ok(_) => {}
                Err(err) => return (Err(err), self.stopwatch.get_formatted_time()),
            };

            match self.handle_events() {
                Ok(_) => {}
                Err(err) => return (Err(err), self.stopwatch.get_formatted_time()),
            };
        }

        (Ok(()), self.stopwatch.get_formatted_time())
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
            Line::from(Span::styled(self.stopwatch.get_formatted_time(), text_style)),
            Line::from(Span::styled(self.stopwatch.get_formatted_interval_time(), text_style)),
        ];
        Paragraph::new(counter_text)
            .centered()
            // .block(Block::new().borders(Borders::ALL))
            .render(area, buf);
    }
}
