use std::{
    io,
    time::{Duration, Instant},
};

use crate::intervals::IntervalList;

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
    start_time: Instant,
    current_time: Duration,
    interval_start_time: Instant,
    interval_current_time: Duration,
    intervals: Option<IntervalList>,
    interval_i: Option<usize>,
    exit: bool,
}

impl App {
    pub fn new(intervals: Option<IntervalList>) -> Self {
        let start_time = Instant::now();
        let zero_duration = Duration::new(0, 0);

        Self {
            start_time,
            current_time: zero_duration,
            interval_start_time: start_time,
            interval_current_time: zero_duration,
            intervals,
            interval_i: None,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> (io::Result<()>, String) {
        while !self.exit {
            self.update_time();

            match terminal.draw(|frame| self.draw(frame)) {
                Ok(_) => {}
                Err(err) => return (Err(err), self.get_formatted_time()),
            };

            match self.handle_events() {
                Ok(_) => {}
                Err(err) => return (Err(err), self.get_formatted_time()),
            };
        }

        (Ok(()), self.get_formatted_time())
    }

    fn update_time(&mut self) {
        self.current_time = self.start_time.elapsed();
        self.interval_current_time = self.interval_start_time.elapsed();
    }

    fn get_formatted_time(&self) -> String {
        let total_s = self.current_time.as_secs();

        let h = total_s / 3600;
        let m = (total_s % 3600) / 60;
        let s = total_s % 60;
        let ms = self.current_time.subsec_millis() / 100;

        format!("{:02}:{:02}:{:02}.{}", h, m, s, ms)
    }
    fn get_formatted_interval_time(&self) -> String {
        let total_s = self.interval_current_time.as_secs();

        let h = total_s / 3600;
        let m = (total_s % 3600) / 60;
        let s = total_s % 60;
        let ms = self.current_time.subsec_millis() / 100;

        format!("interval time: {:02}:{:02}:{:02}.{} (remove this)", h, m, s, ms)
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

        let text_colour = match &self.intervals {
            Some(interval_list) => interval_list.intervals[self.interval_i.unwrap_or(0)].colour,
            None => Color::White,
        };
        let text_style = text_colour;

        // let counter_text = Text::from(self.get_formatted_time()).style(text_colour);
        let counter_text = vec![
            Line::from(Span::styled(self.get_formatted_time(), text_style)),
            Line::from(Span::styled(self.get_formatted_interval_time(), text_style)),
        ];
        Paragraph::new(counter_text)
            .centered()
            // .block(Block::new().borders(Borders::ALL))
            .render(area, buf);
    }
}
